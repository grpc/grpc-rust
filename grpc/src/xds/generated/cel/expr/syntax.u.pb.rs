const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut cel__expr__ParsedExpr_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ParsedExpr {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ParsedExpr>
}

impl ::protobuf::Message for ParsedExpr {
  type MessageView<'msg> = ParsedExprView<'msg>;
  type MessageMut<'msg> = ParsedExprMut<'msg>;
}

impl ::std::default::Default for ParsedExpr {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ParsedExpr {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ParsedExpr` is `Sync` because it does not implement interior mutability.
//    Neither does `ParsedExprMut`.
unsafe impl ::std::marker::Sync for ParsedExpr {}

// SAFETY:
// - `ParsedExpr` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ParsedExpr {}

impl ::protobuf::Proxied for ParsedExpr {
  type View<'msg> = ParsedExprView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ParsedExpr {}

impl ::protobuf::MutProxied for ParsedExpr {
  type Mut<'msg> = ParsedExprMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ParsedExprView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ParsedExpr>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ParsedExprView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ParsedExprView<'msg> {
  type Message = ParsedExpr;
}

impl ::std::fmt::Debug for ParsedExprView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ParsedExprView<'_> {
  fn default() -> ParsedExprView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ParsedExpr>> for ParsedExprView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ParsedExpr>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ParsedExprView<'msg> {

  pub fn to_owned(&self) -> ParsedExpr {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // expr: optional message cel.expr.Expr
  pub fn has_expr(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn expr_opt(self) -> ::std::option::Option<super::ExprView<'msg>> {
    self.has_expr().then(|| self.expr())
  }
  pub fn expr(self) -> super::ExprView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ExprView::default())
  }

  // source_info: optional message cel.expr.SourceInfo
  pub fn has_source_info(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn source_info_opt(self) -> ::std::option::Option<super::SourceInfoView<'msg>> {
    self.has_source_info().then(|| self.source_info())
  }
  pub fn source_info(self) -> super::SourceInfoView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SourceInfoView::default())
  }

}

// SAFETY:
// - `ParsedExprView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ParsedExprView<'_> {}

// SAFETY:
// - `ParsedExprView` is `Send` because while its alive a `ParsedExprMut` cannot.
// - `ParsedExprView` does not use thread-local data.
unsafe impl ::std::marker::Send for ParsedExprView<'_> {}

impl<'msg> ::protobuf::AsView for ParsedExprView<'msg> {
  type Proxied = ParsedExpr;
  fn as_view(&self) -> ::protobuf::View<'msg, ParsedExpr> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ParsedExprView<'msg> {
  fn into_view<'shorter>(self) -> ParsedExprView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ParsedExpr> for ParsedExprView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ParsedExpr {
    let mut dst = ParsedExpr::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ParsedExpr> for ParsedExprMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ParsedExpr {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ParsedExpr {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ParsedExprView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ParsedExprMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ParsedExprMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ParsedExpr>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ParsedExprMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ParsedExprMut<'msg> {
  type Message = ParsedExpr;
}

impl ::std::fmt::Debug for ParsedExprMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ParsedExpr>> for ParsedExprMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ParsedExpr>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ParsedExprMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ParsedExpr> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ParsedExpr {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // expr: optional message cel.expr.Expr
  pub fn has_expr(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_expr(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn expr_opt(&self) -> ::std::option::Option<super::ExprView<'_>> {
    self.has_expr().then(|| self.expr())
  }
  pub fn expr(&self) -> super::ExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ExprView::default())
  }
  pub fn expr_mut(&mut self) -> super::ExprMut<'_> {
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
  pub fn set_expr(&mut self,
    val: impl ::protobuf::IntoProxied<super::Expr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // source_info: optional message cel.expr.SourceInfo
  pub fn has_source_info(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_source_info(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn source_info_opt(&self) -> ::std::option::Option<super::SourceInfoView<'_>> {
    self.has_source_info().then(|| self.source_info())
  }
  pub fn source_info(&self) -> super::SourceInfoView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SourceInfoView::default())
  }
  pub fn source_info_mut(&mut self) -> super::SourceInfoMut<'_> {
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
  pub fn set_source_info(&mut self,
    val: impl ::protobuf::IntoProxied<super::SourceInfo>) {

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
// - `ParsedExprMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ParsedExprMut<'_> {}

// SAFETY:
// - `ParsedExprMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ParsedExprMut<'_> {}

impl<'msg> ::protobuf::AsView for ParsedExprMut<'msg> {
  type Proxied = ParsedExpr;
  fn as_view(&self) -> ::protobuf::View<'_, ParsedExpr> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ParsedExprMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ParsedExpr>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ParsedExprMut<'msg> {
  type MutProxied = ParsedExpr;
  fn as_mut(&mut self) -> ParsedExprMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ParsedExprMut<'msg> {
  fn into_mut<'shorter>(self) -> ParsedExprMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ParsedExpr {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ParsedExpr> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ParsedExprView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ParsedExprMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // expr: optional message cel.expr.Expr
  pub fn has_expr(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_expr(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn expr_opt(&self) -> ::std::option::Option<super::ExprView<'_>> {
    self.has_expr().then(|| self.expr())
  }
  pub fn expr(&self) -> super::ExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ExprView::default())
  }
  pub fn expr_mut(&mut self) -> super::ExprMut<'_> {
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
  pub fn set_expr(&mut self,
    val: impl ::protobuf::IntoProxied<super::Expr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // source_info: optional message cel.expr.SourceInfo
  pub fn has_source_info(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_source_info(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn source_info_opt(&self) -> ::std::option::Option<super::SourceInfoView<'_>> {
    self.has_source_info().then(|| self.source_info())
  }
  pub fn source_info(&self) -> super::SourceInfoView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SourceInfoView::default())
  }
  pub fn source_info_mut(&mut self) -> super::SourceInfoMut<'_> {
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
  pub fn set_source_info(&mut self,
    val: impl ::protobuf::IntoProxied<super::SourceInfo>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl ParsedExpr

impl ::std::ops::Drop for ParsedExpr {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ParsedExpr {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ParsedExpr {
  type Proxied = Self;
  fn as_view(&self) -> ParsedExprView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ParsedExpr {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ParsedExprMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ParsedExpr {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::cel__expr__ParsedExpr_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$a33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::cel__expr__ParsedExpr_msg_init.0, &[<super::Expr as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::SourceInfo as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::cel__expr__ParsedExpr_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ParsedExpr {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ParsedExpr {
  type Msg = ParsedExpr;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ParsedExpr> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ParsedExpr {
  type Msg = ParsedExpr;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ParsedExpr> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ParsedExprMut<'_> {
  type Msg = ParsedExpr;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ParsedExpr> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ParsedExprMut<'_> {
  type Msg = ParsedExpr;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ParsedExpr> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ParsedExprView<'_> {
  type Msg = ParsedExpr;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ParsedExpr> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ParsedExprMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut cel__expr__Expr_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Expr {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Expr>
}

impl ::protobuf::Message for Expr {
  type MessageView<'msg> = ExprView<'msg>;
  type MessageMut<'msg> = ExprMut<'msg>;
}

impl ::std::default::Default for Expr {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Expr {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Expr` is `Sync` because it does not implement interior mutability.
//    Neither does `ExprMut`.
unsafe impl ::std::marker::Sync for Expr {}

// SAFETY:
// - `Expr` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Expr {}

impl ::protobuf::Proxied for Expr {
  type View<'msg> = ExprView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Expr {}

impl ::protobuf::MutProxied for Expr {
  type Mut<'msg> = ExprMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ExprView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Expr>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExprView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ExprView<'msg> {
  type Message = Expr;
}

impl ::std::fmt::Debug for ExprView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ExprView<'_> {
  fn default() -> ExprView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Expr>> for ExprView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Expr>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExprView<'msg> {

  pub fn to_owned(&self) -> Expr {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // id: optional int64
  pub fn id(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // const_expr: optional message cel.expr.Constant
  pub fn has_const_expr(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn const_expr_opt(self) -> ::std::option::Option<super::ConstantView<'msg>> {
    self.has_const_expr().then(|| self.const_expr())
  }
  pub fn const_expr(self) -> super::ConstantView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ConstantView::default())
  }

  // ident_expr: optional message cel.expr.Expr.Ident
  pub fn has_ident_expr(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn ident_expr_opt(self) -> ::std::option::Option<super::expr::IdentView<'msg>> {
    self.has_ident_expr().then(|| self.ident_expr())
  }
  pub fn ident_expr(self) -> super::expr::IdentView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::expr::IdentView::default())
  }

  // select_expr: optional message cel.expr.Expr.Select
  pub fn has_select_expr(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn select_expr_opt(self) -> ::std::option::Option<super::expr::SelectView<'msg>> {
    self.has_select_expr().then(|| self.select_expr())
  }
  pub fn select_expr(self) -> super::expr::SelectView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::expr::SelectView::default())
  }

  // call_expr: optional message cel.expr.Expr.Call
  pub fn has_call_expr(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn call_expr_opt(self) -> ::std::option::Option<super::expr::CallView<'msg>> {
    self.has_call_expr().then(|| self.call_expr())
  }
  pub fn call_expr(self) -> super::expr::CallView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::expr::CallView::default())
  }

  // list_expr: optional message cel.expr.Expr.CreateList
  pub fn has_list_expr(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn list_expr_opt(self) -> ::std::option::Option<super::expr::CreateListView<'msg>> {
    self.has_list_expr().then(|| self.list_expr())
  }
  pub fn list_expr(self) -> super::expr::CreateListView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::expr::CreateListView::default())
  }

  // struct_expr: optional message cel.expr.Expr.CreateStruct
  pub fn has_struct_expr(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn struct_expr_opt(self) -> ::std::option::Option<super::expr::CreateStructView<'msg>> {
    self.has_struct_expr().then(|| self.struct_expr())
  }
  pub fn struct_expr(self) -> super::expr::CreateStructView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::expr::CreateStructView::default())
  }

  // comprehension_expr: optional message cel.expr.Expr.Comprehension
  pub fn has_comprehension_expr(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn comprehension_expr_opt(self) -> ::std::option::Option<super::expr::ComprehensionView<'msg>> {
    self.has_comprehension_expr().then(|| self.comprehension_expr())
  }
  pub fn comprehension_expr(self) -> super::expr::ComprehensionView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::expr::ComprehensionView::default())
  }

  pub fn expr_kind(self) -> super::expr::ExprKindOneof<'msg> {
    match self.expr_kind_case() {
      super::expr::ExprKindCase::ConstExpr =>
          super::expr::ExprKindOneof::ConstExpr(self.const_expr()),
      super::expr::ExprKindCase::IdentExpr =>
          super::expr::ExprKindOneof::IdentExpr(self.ident_expr()),
      super::expr::ExprKindCase::SelectExpr =>
          super::expr::ExprKindOneof::SelectExpr(self.select_expr()),
      super::expr::ExprKindCase::CallExpr =>
          super::expr::ExprKindOneof::CallExpr(self.call_expr()),
      super::expr::ExprKindCase::ListExpr =>
          super::expr::ExprKindOneof::ListExpr(self.list_expr()),
      super::expr::ExprKindCase::StructExpr =>
          super::expr::ExprKindOneof::StructExpr(self.struct_expr()),
      super::expr::ExprKindCase::ComprehensionExpr =>
          super::expr::ExprKindOneof::ComprehensionExpr(self.comprehension_expr()),
      _ => super::expr::ExprKindOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn expr_kind_case(self) -> super::expr::ExprKindCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::expr::ExprKindCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ExprView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ExprView<'_> {}

// SAFETY:
// - `ExprView` is `Send` because while its alive a `ExprMut` cannot.
// - `ExprView` does not use thread-local data.
unsafe impl ::std::marker::Send for ExprView<'_> {}

impl<'msg> ::protobuf::AsView for ExprView<'msg> {
  type Proxied = Expr;
  fn as_view(&self) -> ::protobuf::View<'msg, Expr> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExprView<'msg> {
  fn into_view<'shorter>(self) -> ExprView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Expr> for ExprView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Expr {
    let mut dst = Expr::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Expr> for ExprMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Expr {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Expr {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExprView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExprMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ExprMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Expr>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExprMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ExprMut<'msg> {
  type Message = Expr;
}

impl ::std::fmt::Debug for ExprMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Expr>> for ExprMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Expr>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExprMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Expr> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Expr {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // id: optional int64
  pub fn id(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_id(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        0, val.into()
      )
    }
  }

  // const_expr: optional message cel.expr.Constant
  pub fn has_const_expr(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_const_expr(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn const_expr_opt(&self) -> ::std::option::Option<super::ConstantView<'_>> {
    self.has_const_expr().then(|| self.const_expr())
  }
  pub fn const_expr(&self) -> super::ConstantView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ConstantView::default())
  }
  pub fn const_expr_mut(&mut self) -> super::ConstantMut<'_> {
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
  pub fn set_const_expr(&mut self,
    val: impl ::protobuf::IntoProxied<super::Constant>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // ident_expr: optional message cel.expr.Expr.Ident
  pub fn has_ident_expr(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_ident_expr(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn ident_expr_opt(&self) -> ::std::option::Option<super::expr::IdentView<'_>> {
    self.has_ident_expr().then(|| self.ident_expr())
  }
  pub fn ident_expr(&self) -> super::expr::IdentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::expr::IdentView::default())
  }
  pub fn ident_expr_mut(&mut self) -> super::expr::IdentMut<'_> {
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
  pub fn set_ident_expr(&mut self,
    val: impl ::protobuf::IntoProxied<super::expr::Ident>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // select_expr: optional message cel.expr.Expr.Select
  pub fn has_select_expr(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_select_expr(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn select_expr_opt(&self) -> ::std::option::Option<super::expr::SelectView<'_>> {
    self.has_select_expr().then(|| self.select_expr())
  }
  pub fn select_expr(&self) -> super::expr::SelectView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::expr::SelectView::default())
  }
  pub fn select_expr_mut(&mut self) -> super::expr::SelectMut<'_> {
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
  pub fn set_select_expr(&mut self,
    val: impl ::protobuf::IntoProxied<super::expr::Select>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // call_expr: optional message cel.expr.Expr.Call
  pub fn has_call_expr(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_call_expr(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn call_expr_opt(&self) -> ::std::option::Option<super::expr::CallView<'_>> {
    self.has_call_expr().then(|| self.call_expr())
  }
  pub fn call_expr(&self) -> super::expr::CallView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::expr::CallView::default())
  }
  pub fn call_expr_mut(&mut self) -> super::expr::CallMut<'_> {
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
  pub fn set_call_expr(&mut self,
    val: impl ::protobuf::IntoProxied<super::expr::Call>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // list_expr: optional message cel.expr.Expr.CreateList
  pub fn has_list_expr(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_list_expr(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn list_expr_opt(&self) -> ::std::option::Option<super::expr::CreateListView<'_>> {
    self.has_list_expr().then(|| self.list_expr())
  }
  pub fn list_expr(&self) -> super::expr::CreateListView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::expr::CreateListView::default())
  }
  pub fn list_expr_mut(&mut self) -> super::expr::CreateListMut<'_> {
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
  pub fn set_list_expr(&mut self,
    val: impl ::protobuf::IntoProxied<super::expr::CreateList>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // struct_expr: optional message cel.expr.Expr.CreateStruct
  pub fn has_struct_expr(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_struct_expr(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn struct_expr_opt(&self) -> ::std::option::Option<super::expr::CreateStructView<'_>> {
    self.has_struct_expr().then(|| self.struct_expr())
  }
  pub fn struct_expr(&self) -> super::expr::CreateStructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::expr::CreateStructView::default())
  }
  pub fn struct_expr_mut(&mut self) -> super::expr::CreateStructMut<'_> {
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
  pub fn set_struct_expr(&mut self,
    val: impl ::protobuf::IntoProxied<super::expr::CreateStruct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // comprehension_expr: optional message cel.expr.Expr.Comprehension
  pub fn has_comprehension_expr(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_comprehension_expr(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn comprehension_expr_opt(&self) -> ::std::option::Option<super::expr::ComprehensionView<'_>> {
    self.has_comprehension_expr().then(|| self.comprehension_expr())
  }
  pub fn comprehension_expr(&self) -> super::expr::ComprehensionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::expr::ComprehensionView::default())
  }
  pub fn comprehension_expr_mut(&mut self) -> super::expr::ComprehensionMut<'_> {
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
  pub fn set_comprehension_expr(&mut self,
    val: impl ::protobuf::IntoProxied<super::expr::Comprehension>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  pub fn expr_kind(&self) -> super::expr::ExprKindOneof<'_> {
    match &self.expr_kind_case() {
      super::expr::ExprKindCase::ConstExpr =>
          super::expr::ExprKindOneof::ConstExpr(self.const_expr()),
      super::expr::ExprKindCase::IdentExpr =>
          super::expr::ExprKindOneof::IdentExpr(self.ident_expr()),
      super::expr::ExprKindCase::SelectExpr =>
          super::expr::ExprKindOneof::SelectExpr(self.select_expr()),
      super::expr::ExprKindCase::CallExpr =>
          super::expr::ExprKindOneof::CallExpr(self.call_expr()),
      super::expr::ExprKindCase::ListExpr =>
          super::expr::ExprKindOneof::ListExpr(self.list_expr()),
      super::expr::ExprKindCase::StructExpr =>
          super::expr::ExprKindOneof::StructExpr(self.struct_expr()),
      super::expr::ExprKindCase::ComprehensionExpr =>
          super::expr::ExprKindOneof::ComprehensionExpr(self.comprehension_expr()),
      _ => super::expr::ExprKindOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn expr_kind_case(&self) -> super::expr::ExprKindCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::expr::ExprKindCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ExprMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ExprMut<'_> {}

// SAFETY:
// - `ExprMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ExprMut<'_> {}

impl<'msg> ::protobuf::AsView for ExprMut<'msg> {
  type Proxied = Expr;
  fn as_view(&self) -> ::protobuf::View<'_, Expr> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExprMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Expr>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ExprMut<'msg> {
  type MutProxied = Expr;
  fn as_mut(&mut self) -> ExprMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ExprMut<'msg> {
  fn into_mut<'shorter>(self) -> ExprMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Expr {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Expr> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ExprView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ExprMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // id: optional int64
  pub fn id(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_id(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        0, val.into()
      )
    }
  }

  // const_expr: optional message cel.expr.Constant
  pub fn has_const_expr(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_const_expr(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn const_expr_opt(&self) -> ::std::option::Option<super::ConstantView<'_>> {
    self.has_const_expr().then(|| self.const_expr())
  }
  pub fn const_expr(&self) -> super::ConstantView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ConstantView::default())
  }
  pub fn const_expr_mut(&mut self) -> super::ConstantMut<'_> {
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
  pub fn set_const_expr(&mut self,
    val: impl ::protobuf::IntoProxied<super::Constant>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // ident_expr: optional message cel.expr.Expr.Ident
  pub fn has_ident_expr(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_ident_expr(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn ident_expr_opt(&self) -> ::std::option::Option<super::expr::IdentView<'_>> {
    self.has_ident_expr().then(|| self.ident_expr())
  }
  pub fn ident_expr(&self) -> super::expr::IdentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::expr::IdentView::default())
  }
  pub fn ident_expr_mut(&mut self) -> super::expr::IdentMut<'_> {
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
  pub fn set_ident_expr(&mut self,
    val: impl ::protobuf::IntoProxied<super::expr::Ident>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // select_expr: optional message cel.expr.Expr.Select
  pub fn has_select_expr(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_select_expr(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn select_expr_opt(&self) -> ::std::option::Option<super::expr::SelectView<'_>> {
    self.has_select_expr().then(|| self.select_expr())
  }
  pub fn select_expr(&self) -> super::expr::SelectView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::expr::SelectView::default())
  }
  pub fn select_expr_mut(&mut self) -> super::expr::SelectMut<'_> {
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
  pub fn set_select_expr(&mut self,
    val: impl ::protobuf::IntoProxied<super::expr::Select>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // call_expr: optional message cel.expr.Expr.Call
  pub fn has_call_expr(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_call_expr(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn call_expr_opt(&self) -> ::std::option::Option<super::expr::CallView<'_>> {
    self.has_call_expr().then(|| self.call_expr())
  }
  pub fn call_expr(&self) -> super::expr::CallView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::expr::CallView::default())
  }
  pub fn call_expr_mut(&mut self) -> super::expr::CallMut<'_> {
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
  pub fn set_call_expr(&mut self,
    val: impl ::protobuf::IntoProxied<super::expr::Call>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // list_expr: optional message cel.expr.Expr.CreateList
  pub fn has_list_expr(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_list_expr(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn list_expr_opt(&self) -> ::std::option::Option<super::expr::CreateListView<'_>> {
    self.has_list_expr().then(|| self.list_expr())
  }
  pub fn list_expr(&self) -> super::expr::CreateListView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::expr::CreateListView::default())
  }
  pub fn list_expr_mut(&mut self) -> super::expr::CreateListMut<'_> {
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
  pub fn set_list_expr(&mut self,
    val: impl ::protobuf::IntoProxied<super::expr::CreateList>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // struct_expr: optional message cel.expr.Expr.CreateStruct
  pub fn has_struct_expr(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_struct_expr(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn struct_expr_opt(&self) -> ::std::option::Option<super::expr::CreateStructView<'_>> {
    self.has_struct_expr().then(|| self.struct_expr())
  }
  pub fn struct_expr(&self) -> super::expr::CreateStructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::expr::CreateStructView::default())
  }
  pub fn struct_expr_mut(&mut self) -> super::expr::CreateStructMut<'_> {
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
  pub fn set_struct_expr(&mut self,
    val: impl ::protobuf::IntoProxied<super::expr::CreateStruct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // comprehension_expr: optional message cel.expr.Expr.Comprehension
  pub fn has_comprehension_expr(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_comprehension_expr(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn comprehension_expr_opt(&self) -> ::std::option::Option<super::expr::ComprehensionView<'_>> {
    self.has_comprehension_expr().then(|| self.comprehension_expr())
  }
  pub fn comprehension_expr(&self) -> super::expr::ComprehensionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::expr::ComprehensionView::default())
  }
  pub fn comprehension_expr_mut(&mut self) -> super::expr::ComprehensionMut<'_> {
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
  pub fn set_comprehension_expr(&mut self,
    val: impl ::protobuf::IntoProxied<super::expr::Comprehension>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  pub fn expr_kind(&self) -> super::expr::ExprKindOneof<'_> {
    match &self.expr_kind_case() {
      super::expr::ExprKindCase::ConstExpr =>
          super::expr::ExprKindOneof::ConstExpr(self.const_expr()),
      super::expr::ExprKindCase::IdentExpr =>
          super::expr::ExprKindOneof::IdentExpr(self.ident_expr()),
      super::expr::ExprKindCase::SelectExpr =>
          super::expr::ExprKindOneof::SelectExpr(self.select_expr()),
      super::expr::ExprKindCase::CallExpr =>
          super::expr::ExprKindOneof::CallExpr(self.call_expr()),
      super::expr::ExprKindCase::ListExpr =>
          super::expr::ExprKindOneof::ListExpr(self.list_expr()),
      super::expr::ExprKindCase::StructExpr =>
          super::expr::ExprKindOneof::StructExpr(self.struct_expr()),
      super::expr::ExprKindCase::ComprehensionExpr =>
          super::expr::ExprKindOneof::ComprehensionExpr(self.comprehension_expr()),
      _ => super::expr::ExprKindOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn expr_kind_case(&self) -> super::expr::ExprKindCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::expr::ExprKindCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl Expr

impl ::std::ops::Drop for Expr {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Expr {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Expr {
  type Proxied = Self;
  fn as_view(&self) -> ExprView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Expr {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ExprMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Expr {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::cel__expr__Expr_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$a+P3333333^$|%|&|(|)|*|+");
        super::expr::cel__expr__Expr__Call_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$31XG");
        super::expr::cel__expr__Expr__Comprehension_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X31X3333");
        super::expr::cel__expr__Expr__CreateList_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$NG<");
        super::expr::cel__expr__Expr__CreateStruct_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1XG");
        super::expr::create_struct::cel__expr__Expr__CreateStruct__Entry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$+P1T33/P^#|$");
        super::expr::cel__expr__Expr__Select_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$31X/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::cel__expr__Expr_msg_init.0, &[<super::Constant as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::expr::Ident as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            super::expr::cel__expr__Expr__Select_msg_init.0,
            super::expr::cel__expr__Expr__Call_msg_init.0,
            super::expr::cel__expr__Expr__CreateList_msg_init.0,
            super::expr::cel__expr__Expr__CreateStruct_msg_init.0,
            super::expr::cel__expr__Expr__Comprehension_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::expr::cel__expr__Expr__Call_msg_init.0, &[super::cel__expr__Expr_msg_init.0,
            super::cel__expr__Expr_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::expr::cel__expr__Expr__Comprehension_msg_init.0, &[super::cel__expr__Expr_msg_init.0,
            super::cel__expr__Expr_msg_init.0,
            super::cel__expr__Expr_msg_init.0,
            super::cel__expr__Expr_msg_init.0,
            super::cel__expr__Expr_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::expr::cel__expr__Expr__CreateList_msg_init.0, &[super::cel__expr__Expr_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::expr::cel__expr__Expr__CreateStruct_msg_init.0, &[super::expr::create_struct::cel__expr__Expr__CreateStruct__Entry_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::expr::create_struct::cel__expr__Expr__CreateStruct__Entry_msg_init.0, &[super::cel__expr__Expr_msg_init.0,
            super::cel__expr__Expr_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::expr::cel__expr__Expr__Select_msg_init.0, &[super::cel__expr__Expr_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::cel__expr__Expr_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Expr {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Expr {
  type Msg = Expr;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Expr> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Expr {
  type Msg = Expr;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Expr> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExprMut<'_> {
  type Msg = Expr;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Expr> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExprMut<'_> {
  type Msg = Expr;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Expr> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExprView<'_> {
  type Msg = Expr;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Expr> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExprMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod expr {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut cel__expr__Expr__Ident_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Ident {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Ident>
}

impl ::protobuf::Message for Ident {
  type MessageView<'msg> = IdentView<'msg>;
  type MessageMut<'msg> = IdentMut<'msg>;
}

impl ::std::default::Default for Ident {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Ident {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Ident` is `Sync` because it does not implement interior mutability.
//    Neither does `IdentMut`.
unsafe impl ::std::marker::Sync for Ident {}

// SAFETY:
// - `Ident` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Ident {}

impl ::protobuf::Proxied for Ident {
  type View<'msg> = IdentView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Ident {}

impl ::protobuf::MutProxied for Ident {
  type Mut<'msg> = IdentMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct IdentView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Ident>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for IdentView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for IdentView<'msg> {
  type Message = Ident;
}

impl ::std::fmt::Debug for IdentView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for IdentView<'_> {
  fn default() -> IdentView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Ident>> for IdentView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Ident>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> IdentView<'msg> {

  pub fn to_owned(&self) -> Ident {
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

}

// SAFETY:
// - `IdentView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for IdentView<'_> {}

// SAFETY:
// - `IdentView` is `Send` because while its alive a `IdentMut` cannot.
// - `IdentView` does not use thread-local data.
unsafe impl ::std::marker::Send for IdentView<'_> {}

impl<'msg> ::protobuf::AsView for IdentView<'msg> {
  type Proxied = Ident;
  fn as_view(&self) -> ::protobuf::View<'msg, Ident> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for IdentView<'msg> {
  fn into_view<'shorter>(self) -> IdentView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Ident> for IdentView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Ident {
    let mut dst = Ident::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Ident> for IdentMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Ident {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Ident {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for IdentView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for IdentMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct IdentMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Ident>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for IdentMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for IdentMut<'msg> {
  type Message = Ident;
}

impl ::std::fmt::Debug for IdentMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Ident>> for IdentMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Ident>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> IdentMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Ident> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Ident {
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

}

// SAFETY:
// - `IdentMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for IdentMut<'_> {}

// SAFETY:
// - `IdentMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for IdentMut<'_> {}

impl<'msg> ::protobuf::AsView for IdentMut<'msg> {
  type Proxied = Ident;
  fn as_view(&self) -> ::protobuf::View<'_, Ident> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for IdentMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Ident>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for IdentMut<'msg> {
  type MutProxied = Ident;
  fn as_mut(&mut self) -> IdentMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for IdentMut<'msg> {
  fn into_mut<'shorter>(self) -> IdentMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Ident {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Ident> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> IdentView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> IdentMut<'_> {
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

}  // impl Ident

impl ::std::ops::Drop for Ident {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Ident {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Ident {
  type Proxied = Self;
  fn as_view(&self) -> IdentView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Ident {
  type MutProxied = Self;
  fn as_mut(&mut self) -> IdentMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Ident {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::expr::cel__expr__Expr__Ident_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::expr::cel__expr__Expr__Ident_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::expr::cel__expr__Expr__Ident_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Ident {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Ident {
  type Msg = Ident;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Ident> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Ident {
  type Msg = Ident;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Ident> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for IdentMut<'_> {
  type Msg = Ident;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Ident> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for IdentMut<'_> {
  type Msg = Ident;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Ident> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for IdentView<'_> {
  type Msg = Ident;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Ident> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for IdentMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut cel__expr__Expr__Select_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Select {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Select>
}

impl ::protobuf::Message for Select {
  type MessageView<'msg> = SelectView<'msg>;
  type MessageMut<'msg> = SelectMut<'msg>;
}

impl ::std::default::Default for Select {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Select {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Select` is `Sync` because it does not implement interior mutability.
//    Neither does `SelectMut`.
unsafe impl ::std::marker::Sync for Select {}

// SAFETY:
// - `Select` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Select {}

impl ::protobuf::Proxied for Select {
  type View<'msg> = SelectView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Select {}

impl ::protobuf::MutProxied for Select {
  type Mut<'msg> = SelectMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SelectView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Select>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SelectView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SelectView<'msg> {
  type Message = Select;
}

impl ::std::fmt::Debug for SelectView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SelectView<'_> {
  fn default() -> SelectView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Select>> for SelectView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Select>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SelectView<'msg> {

  pub fn to_owned(&self) -> Select {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // operand: optional message cel.expr.Expr
  pub fn has_operand(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn operand_opt(self) -> ::std::option::Option<super::super::ExprView<'msg>> {
    self.has_operand().then(|| self.operand())
  }
  pub fn operand(self) -> super::super::ExprView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::ExprView::default())
  }

  // field: optional string
  pub fn field(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // test_only: optional bool
  pub fn test_only(self) -> bool {
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
// - `SelectView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SelectView<'_> {}

// SAFETY:
// - `SelectView` is `Send` because while its alive a `SelectMut` cannot.
// - `SelectView` does not use thread-local data.
unsafe impl ::std::marker::Send for SelectView<'_> {}

impl<'msg> ::protobuf::AsView for SelectView<'msg> {
  type Proxied = Select;
  fn as_view(&self) -> ::protobuf::View<'msg, Select> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SelectView<'msg> {
  fn into_view<'shorter>(self) -> SelectView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Select> for SelectView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Select {
    let mut dst = Select::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Select> for SelectMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Select {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Select {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SelectView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SelectMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SelectMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Select>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SelectMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SelectMut<'msg> {
  type Message = Select;
}

impl ::std::fmt::Debug for SelectMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Select>> for SelectMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Select>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SelectMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Select> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Select {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // operand: optional message cel.expr.Expr
  pub fn has_operand(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_operand(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn operand_opt(&self) -> ::std::option::Option<super::super::ExprView<'_>> {
    self.has_operand().then(|| self.operand())
  }
  pub fn operand(&self) -> super::super::ExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::ExprView::default())
  }
  pub fn operand_mut(&mut self) -> super::super::ExprMut<'_> {
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
  pub fn set_operand(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Expr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // field: optional string
  pub fn field(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_field(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // test_only: optional bool
  pub fn test_only(&self) -> bool {
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
  pub fn set_test_only(&mut self, val: bool) {
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
// - `SelectMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SelectMut<'_> {}

// SAFETY:
// - `SelectMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SelectMut<'_> {}

impl<'msg> ::protobuf::AsView for SelectMut<'msg> {
  type Proxied = Select;
  fn as_view(&self) -> ::protobuf::View<'_, Select> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SelectMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Select>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SelectMut<'msg> {
  type MutProxied = Select;
  fn as_mut(&mut self) -> SelectMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SelectMut<'msg> {
  fn into_mut<'shorter>(self) -> SelectMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Select {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Select> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SelectView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SelectMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // operand: optional message cel.expr.Expr
  pub fn has_operand(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_operand(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn operand_opt(&self) -> ::std::option::Option<super::super::ExprView<'_>> {
    self.has_operand().then(|| self.operand())
  }
  pub fn operand(&self) -> super::super::ExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::ExprView::default())
  }
  pub fn operand_mut(&mut self) -> super::super::ExprMut<'_> {
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
  pub fn set_operand(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Expr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // field: optional string
  pub fn field(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_field(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // test_only: optional bool
  pub fn test_only(&self) -> bool {
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
  pub fn set_test_only(&mut self, val: bool) {
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

}  // impl Select

impl ::std::ops::Drop for Select {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Select {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Select {
  type Proxied = Self;
  fn as_view(&self) -> SelectView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Select {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SelectMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Select {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::super::Expr as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::expr::cel__expr__Expr__Select_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Select {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Select {
  type Msg = Select;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Select> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Select {
  type Msg = Select;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Select> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SelectMut<'_> {
  type Msg = Select;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Select> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SelectMut<'_> {
  type Msg = Select;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Select> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SelectView<'_> {
  type Msg = Select;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Select> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SelectMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut cel__expr__Expr__Call_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Call {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Call>
}

impl ::protobuf::Message for Call {
  type MessageView<'msg> = CallView<'msg>;
  type MessageMut<'msg> = CallMut<'msg>;
}

impl ::std::default::Default for Call {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Call {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Call` is `Sync` because it does not implement interior mutability.
//    Neither does `CallMut`.
unsafe impl ::std::marker::Sync for Call {}

// SAFETY:
// - `Call` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Call {}

impl ::protobuf::Proxied for Call {
  type View<'msg> = CallView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Call {}

impl ::protobuf::MutProxied for Call {
  type Mut<'msg> = CallMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CallView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Call>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CallView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CallView<'msg> {
  type Message = Call;
}

impl ::std::fmt::Debug for CallView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CallView<'_> {
  fn default() -> CallView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Call>> for CallView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Call>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CallView<'msg> {

  pub fn to_owned(&self) -> Call {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // target: optional message cel.expr.Expr
  pub fn has_target(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn target_opt(self) -> ::std::option::Option<super::super::ExprView<'msg>> {
    self.has_target().then(|| self.target())
  }
  pub fn target(self) -> super::super::ExprView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::ExprView::default())
  }

  // function: optional string
  pub fn function(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // args: repeated message cel.expr.Expr
  pub fn args(self) -> ::protobuf::RepeatedView<'msg, super::super::Expr> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::Expr>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `CallView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CallView<'_> {}

// SAFETY:
// - `CallView` is `Send` because while its alive a `CallMut` cannot.
// - `CallView` does not use thread-local data.
unsafe impl ::std::marker::Send for CallView<'_> {}

impl<'msg> ::protobuf::AsView for CallView<'msg> {
  type Proxied = Call;
  fn as_view(&self) -> ::protobuf::View<'msg, Call> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CallView<'msg> {
  fn into_view<'shorter>(self) -> CallView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Call> for CallView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Call {
    let mut dst = Call::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Call> for CallMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Call {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Call {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CallView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CallMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CallMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Call>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CallMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CallMut<'msg> {
  type Message = Call;
}

impl ::std::fmt::Debug for CallMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Call>> for CallMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Call>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CallMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Call> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Call {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // target: optional message cel.expr.Expr
  pub fn has_target(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_target(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn target_opt(&self) -> ::std::option::Option<super::super::ExprView<'_>> {
    self.has_target().then(|| self.target())
  }
  pub fn target(&self) -> super::super::ExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::ExprView::default())
  }
  pub fn target_mut(&mut self) -> super::super::ExprMut<'_> {
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
  pub fn set_target(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Expr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // function: optional string
  pub fn function(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_function(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // args: repeated message cel.expr.Expr
  pub fn args(&self) -> ::protobuf::RepeatedView<'_, super::super::Expr> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::Expr>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn args_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::Expr> {
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
  pub fn set_args(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::Expr>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}

// SAFETY:
// - `CallMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CallMut<'_> {}

// SAFETY:
// - `CallMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CallMut<'_> {}

impl<'msg> ::protobuf::AsView for CallMut<'msg> {
  type Proxied = Call;
  fn as_view(&self) -> ::protobuf::View<'_, Call> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CallMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Call>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CallMut<'msg> {
  type MutProxied = Call;
  fn as_mut(&mut self) -> CallMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CallMut<'msg> {
  fn into_mut<'shorter>(self) -> CallMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Call {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Call> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CallView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CallMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // target: optional message cel.expr.Expr
  pub fn has_target(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_target(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn target_opt(&self) -> ::std::option::Option<super::super::ExprView<'_>> {
    self.has_target().then(|| self.target())
  }
  pub fn target(&self) -> super::super::ExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::ExprView::default())
  }
  pub fn target_mut(&mut self) -> super::super::ExprMut<'_> {
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
  pub fn set_target(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Expr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // function: optional string
  pub fn function(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_function(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // args: repeated message cel.expr.Expr
  pub fn args(&self) -> ::protobuf::RepeatedView<'_, super::super::Expr> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::Expr>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn args_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::Expr> {
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
  pub fn set_args(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::Expr>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}  // impl Call

impl ::std::ops::Drop for Call {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Call {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Call {
  type Proxied = Self;
  fn as_view(&self) -> CallView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Call {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CallMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Call {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::super::Expr as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::expr::cel__expr__Expr__Call_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Call {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Call {
  type Msg = Call;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Call> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Call {
  type Msg = Call;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Call> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CallMut<'_> {
  type Msg = Call;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Call> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CallMut<'_> {
  type Msg = Call;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Call> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CallView<'_> {
  type Msg = Call;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Call> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CallMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut cel__expr__Expr__CreateList_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CreateList {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CreateList>
}

impl ::protobuf::Message for CreateList {
  type MessageView<'msg> = CreateListView<'msg>;
  type MessageMut<'msg> = CreateListMut<'msg>;
}

impl ::std::default::Default for CreateList {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CreateList {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CreateList` is `Sync` because it does not implement interior mutability.
//    Neither does `CreateListMut`.
unsafe impl ::std::marker::Sync for CreateList {}

// SAFETY:
// - `CreateList` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CreateList {}

impl ::protobuf::Proxied for CreateList {
  type View<'msg> = CreateListView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CreateList {}

impl ::protobuf::MutProxied for CreateList {
  type Mut<'msg> = CreateListMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CreateListView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CreateList>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CreateListView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CreateListView<'msg> {
  type Message = CreateList;
}

impl ::std::fmt::Debug for CreateListView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CreateListView<'_> {
  fn default() -> CreateListView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CreateList>> for CreateListView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CreateList>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CreateListView<'msg> {

  pub fn to_owned(&self) -> CreateList {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // elements: repeated message cel.expr.Expr
  pub fn elements(self) -> ::protobuf::RepeatedView<'msg, super::super::Expr> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::Expr>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // optional_indices: repeated int32
  pub fn optional_indices(self) -> ::protobuf::RepeatedView<'msg, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `CreateListView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CreateListView<'_> {}

// SAFETY:
// - `CreateListView` is `Send` because while its alive a `CreateListMut` cannot.
// - `CreateListView` does not use thread-local data.
unsafe impl ::std::marker::Send for CreateListView<'_> {}

impl<'msg> ::protobuf::AsView for CreateListView<'msg> {
  type Proxied = CreateList;
  fn as_view(&self) -> ::protobuf::View<'msg, CreateList> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CreateListView<'msg> {
  fn into_view<'shorter>(self) -> CreateListView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CreateList> for CreateListView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CreateList {
    let mut dst = CreateList::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CreateList> for CreateListMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CreateList {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CreateList {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CreateListView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CreateListMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CreateListMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CreateList>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CreateListMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CreateListMut<'msg> {
  type Message = CreateList;
}

impl ::std::fmt::Debug for CreateListMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CreateList>> for CreateListMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CreateList>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CreateListMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CreateList> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CreateList {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // elements: repeated message cel.expr.Expr
  pub fn elements(&self) -> ::protobuf::RepeatedView<'_, super::super::Expr> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::Expr>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn elements_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::Expr> {
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
  pub fn set_elements(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::Expr>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // optional_indices: repeated int32
  pub fn optional_indices(&self) -> ::protobuf::RepeatedView<'_, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn optional_indices_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i32> {
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
  pub fn set_optional_indices(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `CreateListMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CreateListMut<'_> {}

// SAFETY:
// - `CreateListMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CreateListMut<'_> {}

impl<'msg> ::protobuf::AsView for CreateListMut<'msg> {
  type Proxied = CreateList;
  fn as_view(&self) -> ::protobuf::View<'_, CreateList> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CreateListMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CreateList>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CreateListMut<'msg> {
  type MutProxied = CreateList;
  fn as_mut(&mut self) -> CreateListMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CreateListMut<'msg> {
  fn into_mut<'shorter>(self) -> CreateListMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CreateList {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CreateList> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CreateListView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CreateListMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // elements: repeated message cel.expr.Expr
  pub fn elements(&self) -> ::protobuf::RepeatedView<'_, super::super::Expr> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::Expr>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn elements_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::Expr> {
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
  pub fn set_elements(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::Expr>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // optional_indices: repeated int32
  pub fn optional_indices(&self) -> ::protobuf::RepeatedView<'_, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn optional_indices_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i32> {
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
  pub fn set_optional_indices(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl CreateList

impl ::std::ops::Drop for CreateList {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CreateList {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CreateList {
  type Proxied = Self;
  fn as_view(&self) -> CreateListView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CreateList {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CreateListMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CreateList {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::super::Expr as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::expr::cel__expr__Expr__CreateList_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CreateList {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CreateList {
  type Msg = CreateList;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CreateList> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CreateList {
  type Msg = CreateList;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CreateList> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CreateListMut<'_> {
  type Msg = CreateList;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CreateList> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CreateListMut<'_> {
  type Msg = CreateList;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CreateList> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CreateListView<'_> {
  type Msg = CreateList;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CreateList> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CreateListMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut cel__expr__Expr__CreateStruct_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CreateStruct {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CreateStruct>
}

impl ::protobuf::Message for CreateStruct {
  type MessageView<'msg> = CreateStructView<'msg>;
  type MessageMut<'msg> = CreateStructMut<'msg>;
}

impl ::std::default::Default for CreateStruct {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CreateStruct {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CreateStruct` is `Sync` because it does not implement interior mutability.
//    Neither does `CreateStructMut`.
unsafe impl ::std::marker::Sync for CreateStruct {}

// SAFETY:
// - `CreateStruct` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CreateStruct {}

impl ::protobuf::Proxied for CreateStruct {
  type View<'msg> = CreateStructView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CreateStruct {}

impl ::protobuf::MutProxied for CreateStruct {
  type Mut<'msg> = CreateStructMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CreateStructView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CreateStruct>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CreateStructView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CreateStructView<'msg> {
  type Message = CreateStruct;
}

impl ::std::fmt::Debug for CreateStructView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CreateStructView<'_> {
  fn default() -> CreateStructView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CreateStruct>> for CreateStructView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CreateStruct>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CreateStructView<'msg> {

  pub fn to_owned(&self) -> CreateStruct {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // message_name: optional string
  pub fn message_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // entries: repeated message cel.expr.Expr.CreateStruct.Entry
  pub fn entries(self) -> ::protobuf::RepeatedView<'msg, super::super::expr::create_struct::Entry> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::expr::create_struct::Entry>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `CreateStructView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CreateStructView<'_> {}

// SAFETY:
// - `CreateStructView` is `Send` because while its alive a `CreateStructMut` cannot.
// - `CreateStructView` does not use thread-local data.
unsafe impl ::std::marker::Send for CreateStructView<'_> {}

impl<'msg> ::protobuf::AsView for CreateStructView<'msg> {
  type Proxied = CreateStruct;
  fn as_view(&self) -> ::protobuf::View<'msg, CreateStruct> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CreateStructView<'msg> {
  fn into_view<'shorter>(self) -> CreateStructView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CreateStruct> for CreateStructView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CreateStruct {
    let mut dst = CreateStruct::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CreateStruct> for CreateStructMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CreateStruct {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CreateStruct {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CreateStructView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CreateStructMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CreateStructMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CreateStruct>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CreateStructMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CreateStructMut<'msg> {
  type Message = CreateStruct;
}

impl ::std::fmt::Debug for CreateStructMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CreateStruct>> for CreateStructMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CreateStruct>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CreateStructMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CreateStruct> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CreateStruct {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // message_name: optional string
  pub fn message_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_message_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // entries: repeated message cel.expr.Expr.CreateStruct.Entry
  pub fn entries(&self) -> ::protobuf::RepeatedView<'_, super::super::expr::create_struct::Entry> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::expr::create_struct::Entry>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn entries_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::expr::create_struct::Entry> {
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
  pub fn set_entries(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::expr::create_struct::Entry>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `CreateStructMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CreateStructMut<'_> {}

// SAFETY:
// - `CreateStructMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CreateStructMut<'_> {}

impl<'msg> ::protobuf::AsView for CreateStructMut<'msg> {
  type Proxied = CreateStruct;
  fn as_view(&self) -> ::protobuf::View<'_, CreateStruct> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CreateStructMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CreateStruct>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CreateStructMut<'msg> {
  type MutProxied = CreateStruct;
  fn as_mut(&mut self) -> CreateStructMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CreateStructMut<'msg> {
  fn into_mut<'shorter>(self) -> CreateStructMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CreateStruct {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CreateStruct> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CreateStructView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CreateStructMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // message_name: optional string
  pub fn message_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_message_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // entries: repeated message cel.expr.Expr.CreateStruct.Entry
  pub fn entries(&self) -> ::protobuf::RepeatedView<'_, super::super::expr::create_struct::Entry> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::expr::create_struct::Entry>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn entries_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::expr::create_struct::Entry> {
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
  pub fn set_entries(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::expr::create_struct::Entry>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl CreateStruct

impl ::std::ops::Drop for CreateStruct {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CreateStruct {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CreateStruct {
  type Proxied = Self;
  fn as_view(&self) -> CreateStructView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CreateStruct {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CreateStructMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CreateStruct {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::super::Expr as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::expr::cel__expr__Expr__CreateStruct_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CreateStruct {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CreateStruct {
  type Msg = CreateStruct;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CreateStruct> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CreateStruct {
  type Msg = CreateStruct;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CreateStruct> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CreateStructMut<'_> {
  type Msg = CreateStruct;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CreateStruct> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CreateStructMut<'_> {
  type Msg = CreateStruct;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CreateStruct> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CreateStructView<'_> {
  type Msg = CreateStruct;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CreateStruct> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CreateStructMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod create_struct {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut cel__expr__Expr__CreateStruct__Entry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Entry {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Entry>
}

impl ::protobuf::Message for Entry {
  type MessageView<'msg> = EntryView<'msg>;
  type MessageMut<'msg> = EntryMut<'msg>;
}

impl ::std::default::Default for Entry {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Entry {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Entry` is `Sync` because it does not implement interior mutability.
//    Neither does `EntryMut`.
unsafe impl ::std::marker::Sync for Entry {}

// SAFETY:
// - `Entry` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Entry {}

impl ::protobuf::Proxied for Entry {
  type View<'msg> = EntryView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Entry {}

impl ::protobuf::MutProxied for Entry {
  type Mut<'msg> = EntryMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct EntryView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Entry>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EntryView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for EntryView<'msg> {
  type Message = Entry;
}

impl ::std::fmt::Debug for EntryView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for EntryView<'_> {
  fn default() -> EntryView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Entry>> for EntryView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Entry>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EntryView<'msg> {

  pub fn to_owned(&self) -> Entry {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // id: optional int64
  pub fn id(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // field_key: optional string
  pub fn has_field_key(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn field_key_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_field_key().then(|| self.field_key())
  }
  pub fn field_key(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // map_key: optional message cel.expr.Expr
  pub fn has_map_key(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn map_key_opt(self) -> ::std::option::Option<super::super::super::ExprView<'msg>> {
    self.has_map_key().then(|| self.map_key())
  }
  pub fn map_key(self) -> super::super::super::ExprView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::ExprView::default())
  }

  // value: optional message cel.expr.Expr
  pub fn has_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn value_opt(self) -> ::std::option::Option<super::super::super::ExprView<'msg>> {
    self.has_value().then(|| self.value())
  }
  pub fn value(self) -> super::super::super::ExprView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::ExprView::default())
  }

  // optional_entry: optional bool
  pub fn optional_entry(self) -> bool {
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

  pub fn key_kind(self) -> super::super::super::expr::create_struct::entry::KeyKindOneof<'msg> {
    match self.key_kind_case() {
      super::super::super::expr::create_struct::entry::KeyKindCase::FieldKey =>
          super::super::super::expr::create_struct::entry::KeyKindOneof::FieldKey(self.field_key()),
      super::super::super::expr::create_struct::entry::KeyKindCase::MapKey =>
          super::super::super::expr::create_struct::entry::KeyKindOneof::MapKey(self.map_key()),
      _ => super::super::super::expr::create_struct::entry::KeyKindOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn key_kind_case(self) -> super::super::super::expr::create_struct::entry::KeyKindCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::super::expr::create_struct::entry::KeyKindCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `EntryView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for EntryView<'_> {}

// SAFETY:
// - `EntryView` is `Send` because while its alive a `EntryMut` cannot.
// - `EntryView` does not use thread-local data.
unsafe impl ::std::marker::Send for EntryView<'_> {}

impl<'msg> ::protobuf::AsView for EntryView<'msg> {
  type Proxied = Entry;
  fn as_view(&self) -> ::protobuf::View<'msg, Entry> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EntryView<'msg> {
  fn into_view<'shorter>(self) -> EntryView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Entry> for EntryView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Entry {
    let mut dst = Entry::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Entry> for EntryMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Entry {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Entry {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for EntryView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for EntryMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct EntryMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Entry>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EntryMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for EntryMut<'msg> {
  type Message = Entry;
}

impl ::std::fmt::Debug for EntryMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Entry>> for EntryMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Entry>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EntryMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Entry> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Entry {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // id: optional int64
  pub fn id(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_id(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        0, val.into()
      )
    }
  }

  // field_key: optional string
  pub fn has_field_key(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_field_key(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn field_key_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_field_key().then(|| self.field_key())
  }
  pub fn field_key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_field_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // map_key: optional message cel.expr.Expr
  pub fn has_map_key(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_map_key(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn map_key_opt(&self) -> ::std::option::Option<super::super::super::ExprView<'_>> {
    self.has_map_key().then(|| self.map_key())
  }
  pub fn map_key(&self) -> super::super::super::ExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::ExprView::default())
  }
  pub fn map_key_mut(&mut self) -> super::super::super::ExprMut<'_> {
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
  pub fn set_map_key(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::Expr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // value: optional message cel.expr.Expr
  pub fn has_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn value_opt(&self) -> ::std::option::Option<super::super::super::ExprView<'_>> {
    self.has_value().then(|| self.value())
  }
  pub fn value(&self) -> super::super::super::ExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::ExprView::default())
  }
  pub fn value_mut(&mut self) -> super::super::super::ExprMut<'_> {
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
  pub fn set_value(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::Expr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // optional_entry: optional bool
  pub fn optional_entry(&self) -> bool {
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
  pub fn set_optional_entry(&mut self, val: bool) {
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

  pub fn key_kind(&self) -> super::super::super::expr::create_struct::entry::KeyKindOneof<'_> {
    match &self.key_kind_case() {
      super::super::super::expr::create_struct::entry::KeyKindCase::FieldKey =>
          super::super::super::expr::create_struct::entry::KeyKindOneof::FieldKey(self.field_key()),
      super::super::super::expr::create_struct::entry::KeyKindCase::MapKey =>
          super::super::super::expr::create_struct::entry::KeyKindOneof::MapKey(self.map_key()),
      _ => super::super::super::expr::create_struct::entry::KeyKindOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn key_kind_case(&self) -> super::super::super::expr::create_struct::entry::KeyKindCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::super::expr::create_struct::entry::KeyKindCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `EntryMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for EntryMut<'_> {}

// SAFETY:
// - `EntryMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for EntryMut<'_> {}

impl<'msg> ::protobuf::AsView for EntryMut<'msg> {
  type Proxied = Entry;
  fn as_view(&self) -> ::protobuf::View<'_, Entry> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EntryMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Entry>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for EntryMut<'msg> {
  type MutProxied = Entry;
  fn as_mut(&mut self) -> EntryMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for EntryMut<'msg> {
  fn into_mut<'shorter>(self) -> EntryMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Entry {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Entry> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> EntryView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> EntryMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // id: optional int64
  pub fn id(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_id(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        0, val.into()
      )
    }
  }

  // field_key: optional string
  pub fn has_field_key(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_field_key(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn field_key_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_field_key().then(|| self.field_key())
  }
  pub fn field_key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_field_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // map_key: optional message cel.expr.Expr
  pub fn has_map_key(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_map_key(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn map_key_opt(&self) -> ::std::option::Option<super::super::super::ExprView<'_>> {
    self.has_map_key().then(|| self.map_key())
  }
  pub fn map_key(&self) -> super::super::super::ExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::ExprView::default())
  }
  pub fn map_key_mut(&mut self) -> super::super::super::ExprMut<'_> {
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
  pub fn set_map_key(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::Expr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // value: optional message cel.expr.Expr
  pub fn has_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn value_opt(&self) -> ::std::option::Option<super::super::super::ExprView<'_>> {
    self.has_value().then(|| self.value())
  }
  pub fn value(&self) -> super::super::super::ExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::ExprView::default())
  }
  pub fn value_mut(&mut self) -> super::super::super::ExprMut<'_> {
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
  pub fn set_value(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::Expr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // optional_entry: optional bool
  pub fn optional_entry(&self) -> bool {
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
  pub fn set_optional_entry(&mut self, val: bool) {
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

  pub fn key_kind(&self) -> super::super::super::expr::create_struct::entry::KeyKindOneof<'_> {
    match &self.key_kind_case() {
      super::super::super::expr::create_struct::entry::KeyKindCase::FieldKey =>
          super::super::super::expr::create_struct::entry::KeyKindOneof::FieldKey(self.field_key()),
      super::super::super::expr::create_struct::entry::KeyKindCase::MapKey =>
          super::super::super::expr::create_struct::entry::KeyKindOneof::MapKey(self.map_key()),
      _ => super::super::super::expr::create_struct::entry::KeyKindOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn key_kind_case(&self) -> super::super::super::expr::create_struct::entry::KeyKindCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::super::expr::create_struct::entry::KeyKindCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl Entry

impl ::std::ops::Drop for Entry {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Entry {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Entry {
  type Proxied = Self;
  fn as_view(&self) -> EntryView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Entry {
  type MutProxied = Self;
  fn as_mut(&mut self) -> EntryMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Entry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::super::super::Expr as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::expr::create_struct::cel__expr__Expr__CreateStruct__Entry_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Entry {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Entry {
  type Msg = Entry;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Entry> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Entry {
  type Msg = Entry;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Entry> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EntryMut<'_> {
  type Msg = Entry;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Entry> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EntryMut<'_> {
  type Msg = Entry;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Entry> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EntryView<'_> {
  type Msg = Entry;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Entry> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EntryMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod entry {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum KeyKindOneof<'msg> {
  FieldKey(&'msg ::protobuf::ProtoStr) = 2,
  MapKey(::protobuf::View<'msg, super::super::super::super::Expr>) = 3,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum KeyKindCase {
  FieldKey = 2,
  MapKey = 3,

  not_set = 0
}

impl KeyKindCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<KeyKindCase> {
    match v {
      0 => Some(KeyKindCase::not_set),
      2 => Some(KeyKindCase::FieldKey),
      3 => Some(KeyKindCase::MapKey),
      _ => None
    }
  }
}
}  // pub mod entry


}  // pub mod create_struct

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut cel__expr__Expr__Comprehension_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Comprehension {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Comprehension>
}

impl ::protobuf::Message for Comprehension {
  type MessageView<'msg> = ComprehensionView<'msg>;
  type MessageMut<'msg> = ComprehensionMut<'msg>;
}

impl ::std::default::Default for Comprehension {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Comprehension {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Comprehension` is `Sync` because it does not implement interior mutability.
//    Neither does `ComprehensionMut`.
unsafe impl ::std::marker::Sync for Comprehension {}

// SAFETY:
// - `Comprehension` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Comprehension {}

impl ::protobuf::Proxied for Comprehension {
  type View<'msg> = ComprehensionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Comprehension {}

impl ::protobuf::MutProxied for Comprehension {
  type Mut<'msg> = ComprehensionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ComprehensionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Comprehension>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ComprehensionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ComprehensionView<'msg> {
  type Message = Comprehension;
}

impl ::std::fmt::Debug for ComprehensionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ComprehensionView<'_> {
  fn default() -> ComprehensionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Comprehension>> for ComprehensionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Comprehension>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ComprehensionView<'msg> {

  pub fn to_owned(&self) -> Comprehension {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // iter_var: optional string
  pub fn iter_var(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // iter_range: optional message cel.expr.Expr
  pub fn has_iter_range(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn iter_range_opt(self) -> ::std::option::Option<super::super::ExprView<'msg>> {
    self.has_iter_range().then(|| self.iter_range())
  }
  pub fn iter_range(self) -> super::super::ExprView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::ExprView::default())
  }

  // accu_var: optional string
  pub fn accu_var(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // accu_init: optional message cel.expr.Expr
  pub fn has_accu_init(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn accu_init_opt(self) -> ::std::option::Option<super::super::ExprView<'msg>> {
    self.has_accu_init().then(|| self.accu_init())
  }
  pub fn accu_init(self) -> super::super::ExprView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::ExprView::default())
  }

  // loop_condition: optional message cel.expr.Expr
  pub fn has_loop_condition(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn loop_condition_opt(self) -> ::std::option::Option<super::super::ExprView<'msg>> {
    self.has_loop_condition().then(|| self.loop_condition())
  }
  pub fn loop_condition(self) -> super::super::ExprView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::ExprView::default())
  }

  // loop_step: optional message cel.expr.Expr
  pub fn has_loop_step(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn loop_step_opt(self) -> ::std::option::Option<super::super::ExprView<'msg>> {
    self.has_loop_step().then(|| self.loop_step())
  }
  pub fn loop_step(self) -> super::super::ExprView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::ExprView::default())
  }

  // result: optional message cel.expr.Expr
  pub fn has_result(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn result_opt(self) -> ::std::option::Option<super::super::ExprView<'msg>> {
    self.has_result().then(|| self.result())
  }
  pub fn result(self) -> super::super::ExprView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::ExprView::default())
  }

}

// SAFETY:
// - `ComprehensionView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ComprehensionView<'_> {}

// SAFETY:
// - `ComprehensionView` is `Send` because while its alive a `ComprehensionMut` cannot.
// - `ComprehensionView` does not use thread-local data.
unsafe impl ::std::marker::Send for ComprehensionView<'_> {}

impl<'msg> ::protobuf::AsView for ComprehensionView<'msg> {
  type Proxied = Comprehension;
  fn as_view(&self) -> ::protobuf::View<'msg, Comprehension> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ComprehensionView<'msg> {
  fn into_view<'shorter>(self) -> ComprehensionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Comprehension> for ComprehensionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Comprehension {
    let mut dst = Comprehension::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Comprehension> for ComprehensionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Comprehension {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Comprehension {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ComprehensionView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ComprehensionMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ComprehensionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Comprehension>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ComprehensionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ComprehensionMut<'msg> {
  type Message = Comprehension;
}

impl ::std::fmt::Debug for ComprehensionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Comprehension>> for ComprehensionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Comprehension>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ComprehensionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Comprehension> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Comprehension {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // iter_var: optional string
  pub fn iter_var(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_iter_var(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // iter_range: optional message cel.expr.Expr
  pub fn has_iter_range(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_iter_range(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn iter_range_opt(&self) -> ::std::option::Option<super::super::ExprView<'_>> {
    self.has_iter_range().then(|| self.iter_range())
  }
  pub fn iter_range(&self) -> super::super::ExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::ExprView::default())
  }
  pub fn iter_range_mut(&mut self) -> super::super::ExprMut<'_> {
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
  pub fn set_iter_range(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Expr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // accu_var: optional string
  pub fn accu_var(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_accu_var(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // accu_init: optional message cel.expr.Expr
  pub fn has_accu_init(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_accu_init(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn accu_init_opt(&self) -> ::std::option::Option<super::super::ExprView<'_>> {
    self.has_accu_init().then(|| self.accu_init())
  }
  pub fn accu_init(&self) -> super::super::ExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::ExprView::default())
  }
  pub fn accu_init_mut(&mut self) -> super::super::ExprMut<'_> {
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
  pub fn set_accu_init(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Expr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // loop_condition: optional message cel.expr.Expr
  pub fn has_loop_condition(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_loop_condition(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn loop_condition_opt(&self) -> ::std::option::Option<super::super::ExprView<'_>> {
    self.has_loop_condition().then(|| self.loop_condition())
  }
  pub fn loop_condition(&self) -> super::super::ExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::ExprView::default())
  }
  pub fn loop_condition_mut(&mut self) -> super::super::ExprMut<'_> {
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
  pub fn set_loop_condition(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Expr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // loop_step: optional message cel.expr.Expr
  pub fn has_loop_step(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_loop_step(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn loop_step_opt(&self) -> ::std::option::Option<super::super::ExprView<'_>> {
    self.has_loop_step().then(|| self.loop_step())
  }
  pub fn loop_step(&self) -> super::super::ExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::ExprView::default())
  }
  pub fn loop_step_mut(&mut self) -> super::super::ExprMut<'_> {
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
  pub fn set_loop_step(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Expr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // result: optional message cel.expr.Expr
  pub fn has_result(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_result(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn result_opt(&self) -> ::std::option::Option<super::super::ExprView<'_>> {
    self.has_result().then(|| self.result())
  }
  pub fn result(&self) -> super::super::ExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::ExprView::default())
  }
  pub fn result_mut(&mut self) -> super::super::ExprMut<'_> {
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
  pub fn set_result(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Expr>) {

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
// - `ComprehensionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ComprehensionMut<'_> {}

// SAFETY:
// - `ComprehensionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ComprehensionMut<'_> {}

impl<'msg> ::protobuf::AsView for ComprehensionMut<'msg> {
  type Proxied = Comprehension;
  fn as_view(&self) -> ::protobuf::View<'_, Comprehension> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ComprehensionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Comprehension>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ComprehensionMut<'msg> {
  type MutProxied = Comprehension;
  fn as_mut(&mut self) -> ComprehensionMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ComprehensionMut<'msg> {
  fn into_mut<'shorter>(self) -> ComprehensionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Comprehension {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Comprehension> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ComprehensionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ComprehensionMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // iter_var: optional string
  pub fn iter_var(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_iter_var(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // iter_range: optional message cel.expr.Expr
  pub fn has_iter_range(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_iter_range(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn iter_range_opt(&self) -> ::std::option::Option<super::super::ExprView<'_>> {
    self.has_iter_range().then(|| self.iter_range())
  }
  pub fn iter_range(&self) -> super::super::ExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::ExprView::default())
  }
  pub fn iter_range_mut(&mut self) -> super::super::ExprMut<'_> {
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
  pub fn set_iter_range(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Expr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // accu_var: optional string
  pub fn accu_var(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_accu_var(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // accu_init: optional message cel.expr.Expr
  pub fn has_accu_init(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_accu_init(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn accu_init_opt(&self) -> ::std::option::Option<super::super::ExprView<'_>> {
    self.has_accu_init().then(|| self.accu_init())
  }
  pub fn accu_init(&self) -> super::super::ExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::ExprView::default())
  }
  pub fn accu_init_mut(&mut self) -> super::super::ExprMut<'_> {
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
  pub fn set_accu_init(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Expr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // loop_condition: optional message cel.expr.Expr
  pub fn has_loop_condition(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_loop_condition(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn loop_condition_opt(&self) -> ::std::option::Option<super::super::ExprView<'_>> {
    self.has_loop_condition().then(|| self.loop_condition())
  }
  pub fn loop_condition(&self) -> super::super::ExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::ExprView::default())
  }
  pub fn loop_condition_mut(&mut self) -> super::super::ExprMut<'_> {
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
  pub fn set_loop_condition(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Expr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // loop_step: optional message cel.expr.Expr
  pub fn has_loop_step(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_loop_step(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn loop_step_opt(&self) -> ::std::option::Option<super::super::ExprView<'_>> {
    self.has_loop_step().then(|| self.loop_step())
  }
  pub fn loop_step(&self) -> super::super::ExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::ExprView::default())
  }
  pub fn loop_step_mut(&mut self) -> super::super::ExprMut<'_> {
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
  pub fn set_loop_step(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Expr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // result: optional message cel.expr.Expr
  pub fn has_result(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_result(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn result_opt(&self) -> ::std::option::Option<super::super::ExprView<'_>> {
    self.has_result().then(|| self.result())
  }
  pub fn result(&self) -> super::super::ExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::ExprView::default())
  }
  pub fn result_mut(&mut self) -> super::super::ExprMut<'_> {
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
  pub fn set_result(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Expr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

}  // impl Comprehension

impl ::std::ops::Drop for Comprehension {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Comprehension {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Comprehension {
  type Proxied = Self;
  fn as_view(&self) -> ComprehensionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Comprehension {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ComprehensionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Comprehension {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::super::Expr as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::expr::cel__expr__Expr__Comprehension_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Comprehension {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Comprehension {
  type Msg = Comprehension;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Comprehension> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Comprehension {
  type Msg = Comprehension;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Comprehension> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ComprehensionMut<'_> {
  type Msg = Comprehension;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Comprehension> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ComprehensionMut<'_> {
  type Msg = Comprehension;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Comprehension> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ComprehensionView<'_> {
  type Msg = Comprehension;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Comprehension> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ComprehensionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ExprKindOneof<'msg> {
  ConstExpr(::protobuf::View<'msg, super::super::Constant>) = 3,
  IdentExpr(::protobuf::View<'msg, super::super::expr::Ident>) = 4,
  SelectExpr(::protobuf::View<'msg, super::super::expr::Select>) = 5,
  CallExpr(::protobuf::View<'msg, super::super::expr::Call>) = 6,
  ListExpr(::protobuf::View<'msg, super::super::expr::CreateList>) = 7,
  StructExpr(::protobuf::View<'msg, super::super::expr::CreateStruct>) = 8,
  ComprehensionExpr(::protobuf::View<'msg, super::super::expr::Comprehension>) = 9,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ExprKindCase {
  ConstExpr = 3,
  IdentExpr = 4,
  SelectExpr = 5,
  CallExpr = 6,
  ListExpr = 7,
  StructExpr = 8,
  ComprehensionExpr = 9,

  not_set = 0
}

impl ExprKindCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ExprKindCase> {
    match v {
      0 => Some(ExprKindCase::not_set),
      3 => Some(ExprKindCase::ConstExpr),
      4 => Some(ExprKindCase::IdentExpr),
      5 => Some(ExprKindCase::SelectExpr),
      6 => Some(ExprKindCase::CallExpr),
      7 => Some(ExprKindCase::ListExpr),
      8 => Some(ExprKindCase::StructExpr),
      9 => Some(ExprKindCase::ComprehensionExpr),
      _ => None
    }
  }
}
}  // pub mod expr


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut cel__expr__Constant_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Constant {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Constant>
}

impl ::protobuf::Message for Constant {
  type MessageView<'msg> = ConstantView<'msg>;
  type MessageMut<'msg> = ConstantMut<'msg>;
}

impl ::std::default::Default for Constant {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Constant {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Constant` is `Sync` because it does not implement interior mutability.
//    Neither does `ConstantMut`.
unsafe impl ::std::marker::Sync for Constant {}

// SAFETY:
// - `Constant` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Constant {}

impl ::protobuf::Proxied for Constant {
  type View<'msg> = ConstantView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Constant {}

impl ::protobuf::MutProxied for Constant {
  type Mut<'msg> = ConstantMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ConstantView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Constant>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ConstantView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ConstantView<'msg> {
  type Message = Constant;
}

impl ::std::fmt::Debug for ConstantView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ConstantView<'_> {
  fn default() -> ConstantView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Constant>> for ConstantView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Constant>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ConstantView<'msg> {

  pub fn to_owned(&self) -> Constant {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // null_value: optional enum google.protobuf.NullValue
  pub fn has_null_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn null_value_opt(self) -> ::std::option::Option<::protobuf_well_known_types::NullValue> {
    self.has_null_value().then(|| self.null_value())
  }
  pub fn null_value(self) -> ::protobuf_well_known_types::NullValue {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (::protobuf_well_known_types::NullValue::NullValue).into()
      ).try_into().unwrap()
    }
  }

  // bool_value: optional bool
  pub fn has_bool_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn bool_value_opt(self) -> ::std::option::Option<bool> {
    self.has_bool_value().then(|| self.bool_value())
  }
  pub fn bool_value(self) -> bool {
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

  // int64_value: optional int64
  pub fn has_int64_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn int64_value_opt(self) -> ::std::option::Option<i64> {
    self.has_int64_value().then(|| self.int64_value())
  }
  pub fn int64_value(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        2, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // uint64_value: optional uint64
  pub fn has_uint64_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn uint64_value_opt(self) -> ::std::option::Option<u64> {
    self.has_uint64_value().then(|| self.uint64_value())
  }
  pub fn uint64_value(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        3, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // double_value: optional double
  pub fn has_double_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn double_value_opt(self) -> ::std::option::Option<f64> {
    self.has_double_value().then(|| self.double_value())
  }
  pub fn double_value(self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        4, (0f64).into()
      ).try_into().unwrap()
    }
  }

  // string_value: optional string
  pub fn has_string_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn string_value_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_string_value().then(|| self.string_value())
  }
  pub fn string_value(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // bytes_value: optional bytes
  pub fn has_bytes_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn bytes_value_opt(self) -> ::std::option::Option<&'msg [u8]> {
    self.has_bytes_value().then(|| self.bytes_value())
  }
  pub fn bytes_value(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // duration_value: optional message google.protobuf.Duration
  pub fn has_duration_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn duration_value_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_duration_value().then(|| self.duration_value())
  }
  pub fn duration_value(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // timestamp_value: optional message google.protobuf.Timestamp
  pub fn has_timestamp_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn timestamp_value_opt(self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'msg>> {
    self.has_timestamp_value().then(|| self.timestamp_value())
  }
  pub fn timestamp_value(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

  pub fn constant_kind(self) -> super::constant::ConstantKindOneof<'msg> {
    match self.constant_kind_case() {
      super::constant::ConstantKindCase::NullValue =>
          super::constant::ConstantKindOneof::NullValue(self.null_value()),
      super::constant::ConstantKindCase::BoolValue =>
          super::constant::ConstantKindOneof::BoolValue(self.bool_value()),
      super::constant::ConstantKindCase::Int64Value =>
          super::constant::ConstantKindOneof::Int64Value(self.int64_value()),
      super::constant::ConstantKindCase::Uint64Value =>
          super::constant::ConstantKindOneof::Uint64Value(self.uint64_value()),
      super::constant::ConstantKindCase::DoubleValue =>
          super::constant::ConstantKindOneof::DoubleValue(self.double_value()),
      super::constant::ConstantKindCase::StringValue =>
          super::constant::ConstantKindOneof::StringValue(self.string_value()),
      super::constant::ConstantKindCase::BytesValue =>
          super::constant::ConstantKindOneof::BytesValue(self.bytes_value()),
      super::constant::ConstantKindCase::DurationValue =>
          super::constant::ConstantKindOneof::DurationValue(self.duration_value()),
      super::constant::ConstantKindCase::TimestampValue =>
          super::constant::ConstantKindOneof::TimestampValue(self.timestamp_value()),
      _ => super::constant::ConstantKindOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn constant_kind_case(self) -> super::constant::ConstantKindCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::constant::ConstantKindCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ConstantView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ConstantView<'_> {}

// SAFETY:
// - `ConstantView` is `Send` because while its alive a `ConstantMut` cannot.
// - `ConstantView` does not use thread-local data.
unsafe impl ::std::marker::Send for ConstantView<'_> {}

impl<'msg> ::protobuf::AsView for ConstantView<'msg> {
  type Proxied = Constant;
  fn as_view(&self) -> ::protobuf::View<'msg, Constant> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ConstantView<'msg> {
  fn into_view<'shorter>(self) -> ConstantView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Constant> for ConstantView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Constant {
    let mut dst = Constant::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Constant> for ConstantMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Constant {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Constant {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ConstantView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ConstantMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ConstantMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Constant>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ConstantMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ConstantMut<'msg> {
  type Message = Constant;
}

impl ::std::fmt::Debug for ConstantMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Constant>> for ConstantMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Constant>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ConstantMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Constant> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Constant {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // null_value: optional enum google.protobuf.NullValue
  pub fn has_null_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_null_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn null_value_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::NullValue> {
    self.has_null_value().then(|| self.null_value())
  }
  pub fn null_value(&self) -> ::protobuf_well_known_types::NullValue {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (::protobuf_well_known_types::NullValue::NullValue).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_null_value(&mut self, val: ::protobuf_well_known_types::NullValue) {
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

  // bool_value: optional bool
  pub fn has_bool_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_bool_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn bool_value_opt(&self) -> ::std::option::Option<bool> {
    self.has_bool_value().then(|| self.bool_value())
  }
  pub fn bool_value(&self) -> bool {
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
  pub fn set_bool_value(&mut self, val: bool) {
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

  // int64_value: optional int64
  pub fn has_int64_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_int64_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn int64_value_opt(&self) -> ::std::option::Option<i64> {
    self.has_int64_value().then(|| self.int64_value())
  }
  pub fn int64_value(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        2, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_int64_value(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        2, val.into()
      )
    }
  }

  // uint64_value: optional uint64
  pub fn has_uint64_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_uint64_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn uint64_value_opt(&self) -> ::std::option::Option<u64> {
    self.has_uint64_value().then(|| self.uint64_value())
  }
  pub fn uint64_value(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        3, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_uint64_value(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        3, val.into()
      )
    }
  }

  // double_value: optional double
  pub fn has_double_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_double_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn double_value_opt(&self) -> ::std::option::Option<f64> {
    self.has_double_value().then(|| self.double_value())
  }
  pub fn double_value(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        4, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_double_value(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        4, val.into()
      )
    }
  }

  // string_value: optional string
  pub fn has_string_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_string_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn string_value_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_string_value().then(|| self.string_value())
  }
  pub fn string_value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_string_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // bytes_value: optional bytes
  pub fn has_bytes_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_bytes_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn bytes_value_opt(&self) -> ::std::option::Option<&'_ [u8]> {
    self.has_bytes_value().then(|| self.bytes_value())
  }
  pub fn bytes_value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_bytes_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // duration_value: optional message google.protobuf.Duration
  pub fn has_duration_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_duration_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn duration_value_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_duration_value().then(|| self.duration_value())
  }
  pub fn duration_value(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn duration_value_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_duration_value(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // timestamp_value: optional message google.protobuf.Timestamp
  pub fn has_timestamp_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_timestamp_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn timestamp_value_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_timestamp_value().then(|| self.timestamp_value())
  }
  pub fn timestamp_value(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn timestamp_value_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_timestamp_value(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  pub fn constant_kind(&self) -> super::constant::ConstantKindOneof<'_> {
    match &self.constant_kind_case() {
      super::constant::ConstantKindCase::NullValue =>
          super::constant::ConstantKindOneof::NullValue(self.null_value()),
      super::constant::ConstantKindCase::BoolValue =>
          super::constant::ConstantKindOneof::BoolValue(self.bool_value()),
      super::constant::ConstantKindCase::Int64Value =>
          super::constant::ConstantKindOneof::Int64Value(self.int64_value()),
      super::constant::ConstantKindCase::Uint64Value =>
          super::constant::ConstantKindOneof::Uint64Value(self.uint64_value()),
      super::constant::ConstantKindCase::DoubleValue =>
          super::constant::ConstantKindOneof::DoubleValue(self.double_value()),
      super::constant::ConstantKindCase::StringValue =>
          super::constant::ConstantKindOneof::StringValue(self.string_value()),
      super::constant::ConstantKindCase::BytesValue =>
          super::constant::ConstantKindOneof::BytesValue(self.bytes_value()),
      super::constant::ConstantKindCase::DurationValue =>
          super::constant::ConstantKindOneof::DurationValue(self.duration_value()),
      super::constant::ConstantKindCase::TimestampValue =>
          super::constant::ConstantKindOneof::TimestampValue(self.timestamp_value()),
      _ => super::constant::ConstantKindOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn constant_kind_case(&self) -> super::constant::ConstantKindCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::constant::ConstantKindCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ConstantMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ConstantMut<'_> {}

// SAFETY:
// - `ConstantMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ConstantMut<'_> {}

impl<'msg> ::protobuf::AsView for ConstantMut<'msg> {
  type Proxied = Constant;
  fn as_view(&self) -> ::protobuf::View<'_, Constant> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ConstantMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Constant>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ConstantMut<'msg> {
  type MutProxied = Constant;
  fn as_mut(&mut self) -> ConstantMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ConstantMut<'msg> {
  fn into_mut<'shorter>(self) -> ConstantMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Constant {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Constant> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ConstantView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ConstantMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // null_value: optional enum google.protobuf.NullValue
  pub fn has_null_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_null_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn null_value_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::NullValue> {
    self.has_null_value().then(|| self.null_value())
  }
  pub fn null_value(&self) -> ::protobuf_well_known_types::NullValue {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (::protobuf_well_known_types::NullValue::NullValue).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_null_value(&mut self, val: ::protobuf_well_known_types::NullValue) {
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

  // bool_value: optional bool
  pub fn has_bool_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_bool_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn bool_value_opt(&self) -> ::std::option::Option<bool> {
    self.has_bool_value().then(|| self.bool_value())
  }
  pub fn bool_value(&self) -> bool {
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
  pub fn set_bool_value(&mut self, val: bool) {
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

  // int64_value: optional int64
  pub fn has_int64_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_int64_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn int64_value_opt(&self) -> ::std::option::Option<i64> {
    self.has_int64_value().then(|| self.int64_value())
  }
  pub fn int64_value(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        2, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_int64_value(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        2, val.into()
      )
    }
  }

  // uint64_value: optional uint64
  pub fn has_uint64_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_uint64_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn uint64_value_opt(&self) -> ::std::option::Option<u64> {
    self.has_uint64_value().then(|| self.uint64_value())
  }
  pub fn uint64_value(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        3, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_uint64_value(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        3, val.into()
      )
    }
  }

  // double_value: optional double
  pub fn has_double_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_double_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn double_value_opt(&self) -> ::std::option::Option<f64> {
    self.has_double_value().then(|| self.double_value())
  }
  pub fn double_value(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        4, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_double_value(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        4, val.into()
      )
    }
  }

  // string_value: optional string
  pub fn has_string_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_string_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn string_value_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_string_value().then(|| self.string_value())
  }
  pub fn string_value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_string_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // bytes_value: optional bytes
  pub fn has_bytes_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_bytes_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn bytes_value_opt(&self) -> ::std::option::Option<&'_ [u8]> {
    self.has_bytes_value().then(|| self.bytes_value())
  }
  pub fn bytes_value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_bytes_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // duration_value: optional message google.protobuf.Duration
  pub fn has_duration_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_duration_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn duration_value_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_duration_value().then(|| self.duration_value())
  }
  pub fn duration_value(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn duration_value_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_duration_value(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // timestamp_value: optional message google.protobuf.Timestamp
  pub fn has_timestamp_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_timestamp_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn timestamp_value_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_timestamp_value().then(|| self.timestamp_value())
  }
  pub fn timestamp_value(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn timestamp_value_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_timestamp_value(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  pub fn constant_kind(&self) -> super::constant::ConstantKindOneof<'_> {
    match &self.constant_kind_case() {
      super::constant::ConstantKindCase::NullValue =>
          super::constant::ConstantKindOneof::NullValue(self.null_value()),
      super::constant::ConstantKindCase::BoolValue =>
          super::constant::ConstantKindOneof::BoolValue(self.bool_value()),
      super::constant::ConstantKindCase::Int64Value =>
          super::constant::ConstantKindOneof::Int64Value(self.int64_value()),
      super::constant::ConstantKindCase::Uint64Value =>
          super::constant::ConstantKindOneof::Uint64Value(self.uint64_value()),
      super::constant::ConstantKindCase::DoubleValue =>
          super::constant::ConstantKindOneof::DoubleValue(self.double_value()),
      super::constant::ConstantKindCase::StringValue =>
          super::constant::ConstantKindOneof::StringValue(self.string_value()),
      super::constant::ConstantKindCase::BytesValue =>
          super::constant::ConstantKindOneof::BytesValue(self.bytes_value()),
      super::constant::ConstantKindCase::DurationValue =>
          super::constant::ConstantKindOneof::DurationValue(self.duration_value()),
      super::constant::ConstantKindCase::TimestampValue =>
          super::constant::ConstantKindOneof::TimestampValue(self.timestamp_value()),
      _ => super::constant::ConstantKindOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn constant_kind_case(&self) -> super::constant::ConstantKindCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::constant::ConstantKindCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl Constant

impl ::std::ops::Drop for Constant {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Constant {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Constant {
  type Proxied = Self;
  fn as_view(&self) -> ConstantView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Constant {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ConstantMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Constant {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::cel__expr__Constant_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$./+, 1T033^!|#|$|%|&|(|)|*|+");
        ::protobuf::__internal::runtime::link_mini_table(
            super::cel__expr__Constant_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::cel__expr__Constant_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Constant {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Constant {
  type Msg = Constant;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Constant> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Constant {
  type Msg = Constant;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Constant> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ConstantMut<'_> {
  type Msg = Constant;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Constant> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConstantMut<'_> {
  type Msg = Constant;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Constant> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConstantView<'_> {
  type Msg = Constant;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Constant> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ConstantMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod constant {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ConstantKindOneof<'msg> {
  NullValue(::protobuf::View<'msg, ::protobuf_well_known_types::NullValue>) = 1,
  BoolValue(bool) = 2,
  Int64Value(i64) = 3,
  Uint64Value(u64) = 4,
  DoubleValue(f64) = 5,
  StringValue(&'msg ::protobuf::ProtoStr) = 6,
  BytesValue(&'msg [u8]) = 7,
  DurationValue(::protobuf::View<'msg, ::protobuf_well_known_types::Duration>) = 8,
  TimestampValue(::protobuf::View<'msg, ::protobuf_well_known_types::Timestamp>) = 9,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ConstantKindCase {
  NullValue = 1,
  BoolValue = 2,
  Int64Value = 3,
  Uint64Value = 4,
  DoubleValue = 5,
  StringValue = 6,
  BytesValue = 7,
  DurationValue = 8,
  TimestampValue = 9,

  not_set = 0
}

impl ConstantKindCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ConstantKindCase> {
    match v {
      0 => Some(ConstantKindCase::not_set),
      1 => Some(ConstantKindCase::NullValue),
      2 => Some(ConstantKindCase::BoolValue),
      3 => Some(ConstantKindCase::Int64Value),
      4 => Some(ConstantKindCase::Uint64Value),
      5 => Some(ConstantKindCase::DoubleValue),
      6 => Some(ConstantKindCase::StringValue),
      7 => Some(ConstantKindCase::BytesValue),
      8 => Some(ConstantKindCase::DurationValue),
      9 => Some(ConstantKindCase::TimestampValue),
      _ => None
    }
  }
}
}  // pub mod constant


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut cel__expr__SourceInfo_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SourceInfo {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SourceInfo>
}

impl ::protobuf::Message for SourceInfo {
  type MessageView<'msg> = SourceInfoView<'msg>;
  type MessageMut<'msg> = SourceInfoMut<'msg>;
}

impl ::std::default::Default for SourceInfo {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SourceInfo {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SourceInfo` is `Sync` because it does not implement interior mutability.
//    Neither does `SourceInfoMut`.
unsafe impl ::std::marker::Sync for SourceInfo {}

// SAFETY:
// - `SourceInfo` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SourceInfo {}

impl ::protobuf::Proxied for SourceInfo {
  type View<'msg> = SourceInfoView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SourceInfo {}

impl ::protobuf::MutProxied for SourceInfo {
  type Mut<'msg> = SourceInfoMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SourceInfoView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SourceInfo>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SourceInfoView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SourceInfoView<'msg> {
  type Message = SourceInfo;
}

impl ::std::fmt::Debug for SourceInfoView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SourceInfoView<'_> {
  fn default() -> SourceInfoView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SourceInfo>> for SourceInfoView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SourceInfo>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SourceInfoView<'msg> {

  pub fn to_owned(&self) -> SourceInfo {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // syntax_version: optional string
  pub fn syntax_version(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // location: optional string
  pub fn location(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // line_offsets: repeated int32
  pub fn line_offsets(self) -> ::protobuf::RepeatedView<'msg, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // positions: repeated message cel.expr.SourceInfo.PositionsEntry
  pub fn positions(self)
    -> ::protobuf::MapView<'msg, i64, i32> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(3)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<i64, i32>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // macro_calls: repeated message cel.expr.SourceInfo.MacroCallsEntry
  pub fn macro_calls(self)
    -> ::protobuf::MapView<'msg, i64, super::Expr> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(4)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<i64, super::Expr>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // extensions: repeated message cel.expr.SourceInfo.Extension
  pub fn extensions(self) -> ::protobuf::RepeatedView<'msg, super::source_info::Extension> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::source_info::Extension>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `SourceInfoView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SourceInfoView<'_> {}

// SAFETY:
// - `SourceInfoView` is `Send` because while its alive a `SourceInfoMut` cannot.
// - `SourceInfoView` does not use thread-local data.
unsafe impl ::std::marker::Send for SourceInfoView<'_> {}

impl<'msg> ::protobuf::AsView for SourceInfoView<'msg> {
  type Proxied = SourceInfo;
  fn as_view(&self) -> ::protobuf::View<'msg, SourceInfo> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SourceInfoView<'msg> {
  fn into_view<'shorter>(self) -> SourceInfoView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SourceInfo> for SourceInfoView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SourceInfo {
    let mut dst = SourceInfo::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SourceInfo> for SourceInfoMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SourceInfo {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SourceInfo {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SourceInfoView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SourceInfoMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SourceInfoMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SourceInfo>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SourceInfoMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SourceInfoMut<'msg> {
  type Message = SourceInfo;
}

impl ::std::fmt::Debug for SourceInfoMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SourceInfo>> for SourceInfoMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SourceInfo>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SourceInfoMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SourceInfo> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SourceInfo {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // syntax_version: optional string
  pub fn syntax_version(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_syntax_version(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // location: optional string
  pub fn location(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_location(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // line_offsets: repeated int32
  pub fn line_offsets(&self) -> ::protobuf::RepeatedView<'_, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn line_offsets_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i32> {
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
  pub fn set_line_offsets(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // positions: repeated message cel.expr.SourceInfo.PositionsEntry
  pub fn positions(&self)
    -> ::protobuf::MapView<'_, i64, i32> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(3)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<i64, i32>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn positions_mut(&mut self)
    -> ::protobuf::MapMut<'_, i64, i32> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          3, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_positions(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<i64, i32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // macro_calls: repeated message cel.expr.SourceInfo.MacroCallsEntry
  pub fn macro_calls(&self)
    -> ::protobuf::MapView<'_, i64, super::Expr> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(4)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<i64, super::Expr>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn macro_calls_mut(&mut self)
    -> ::protobuf::MapMut<'_, i64, super::Expr> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          4, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_macro_calls(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<i64, super::Expr>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // extensions: repeated message cel.expr.SourceInfo.Extension
  pub fn extensions(&self) -> ::protobuf::RepeatedView<'_, super::source_info::Extension> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::source_info::Extension>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn extensions_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::source_info::Extension> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_extensions(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::source_info::Extension>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

}

// SAFETY:
// - `SourceInfoMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SourceInfoMut<'_> {}

// SAFETY:
// - `SourceInfoMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SourceInfoMut<'_> {}

impl<'msg> ::protobuf::AsView for SourceInfoMut<'msg> {
  type Proxied = SourceInfo;
  fn as_view(&self) -> ::protobuf::View<'_, SourceInfo> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SourceInfoMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SourceInfo>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SourceInfoMut<'msg> {
  type MutProxied = SourceInfo;
  fn as_mut(&mut self) -> SourceInfoMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SourceInfoMut<'msg> {
  fn into_mut<'shorter>(self) -> SourceInfoMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SourceInfo {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SourceInfo> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SourceInfoView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SourceInfoMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // syntax_version: optional string
  pub fn syntax_version(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_syntax_version(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // location: optional string
  pub fn location(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_location(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // line_offsets: repeated int32
  pub fn line_offsets(&self) -> ::protobuf::RepeatedView<'_, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn line_offsets_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i32> {
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
  pub fn set_line_offsets(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // positions: repeated message cel.expr.SourceInfo.PositionsEntry
  pub fn positions(&self)
    -> ::protobuf::MapView<'_, i64, i32> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(3)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<i64, i32>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn positions_mut(&mut self)
    -> ::protobuf::MapMut<'_, i64, i32> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          3, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_positions(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<i64, i32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // macro_calls: repeated message cel.expr.SourceInfo.MacroCallsEntry
  pub fn macro_calls(&self)
    -> ::protobuf::MapView<'_, i64, super::Expr> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(4)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<i64, super::Expr>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn macro_calls_mut(&mut self)
    -> ::protobuf::MapMut<'_, i64, super::Expr> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          4, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_macro_calls(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<i64, super::Expr>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // extensions: repeated message cel.expr.SourceInfo.Extension
  pub fn extensions(&self) -> ::protobuf::RepeatedView<'_, super::source_info::Extension> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::source_info::Extension>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn extensions_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::source_info::Extension> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_extensions(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::source_info::Extension>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

}  // impl SourceInfo

impl ::std::ops::Drop for SourceInfo {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SourceInfo {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SourceInfo {
  type Proxied = Self;
  fn as_view(&self) -> SourceInfoView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SourceInfo {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SourceInfoMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SourceInfo {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::cel__expr__SourceInfo_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$N1X1X<GGG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::cel__expr__SourceInfo_msg_init.0, &[<super::source_info::PositionsEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::source_info::MacroCallsEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::source_info::Extension as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::cel__expr__SourceInfo_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SourceInfo {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SourceInfo {
  type Msg = SourceInfo;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SourceInfo> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SourceInfo {
  type Msg = SourceInfo;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SourceInfo> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SourceInfoMut<'_> {
  type Msg = SourceInfo;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SourceInfo> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SourceInfoMut<'_> {
  type Msg = SourceInfo;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SourceInfo> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SourceInfoView<'_> {
  type Msg = SourceInfo;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SourceInfo> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SourceInfoMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod source_info {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut cel__expr__SourceInfo__PositionsEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct PositionsEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PositionsEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::source_info::cel__expr__SourceInfo__PositionsEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%+P(P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::source_info::cel__expr__SourceInfo__PositionsEntry_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::source_info::cel__expr__SourceInfo__PositionsEntry_msg_init.0)
      }).0
    }
  }
}
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut cel__expr__SourceInfo__MacroCallsEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct MacroCallsEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MacroCallsEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::source_info::cel__expr__SourceInfo__MacroCallsEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%+P3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::source_info::cel__expr__SourceInfo__MacroCallsEntry_msg_init.0, &[<super::super::Expr as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::source_info::cel__expr__SourceInfo__MacroCallsEntry_msg_init.0)
      }).0
    }
  }
}
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut cel__expr__SourceInfo__Extension_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Extension {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Extension>
}

impl ::protobuf::Message for Extension {
  type MessageView<'msg> = ExtensionView<'msg>;
  type MessageMut<'msg> = ExtensionMut<'msg>;
}

impl ::std::default::Default for Extension {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Extension {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Extension` is `Sync` because it does not implement interior mutability.
//    Neither does `ExtensionMut`.
unsafe impl ::std::marker::Sync for Extension {}

// SAFETY:
// - `Extension` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Extension {}

impl ::protobuf::Proxied for Extension {
  type View<'msg> = ExtensionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Extension {}

impl ::protobuf::MutProxied for Extension {
  type Mut<'msg> = ExtensionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ExtensionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Extension>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExtensionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ExtensionView<'msg> {
  type Message = Extension;
}

impl ::std::fmt::Debug for ExtensionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ExtensionView<'_> {
  fn default() -> ExtensionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Extension>> for ExtensionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Extension>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExtensionView<'msg> {

  pub fn to_owned(&self) -> Extension {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // id: optional string
  pub fn id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // affected_components: repeated enum cel.expr.SourceInfo.Extension.Component
  pub fn affected_components(self) -> ::protobuf::RepeatedView<'msg, super::super::source_info::extension::Component> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::source_info::extension::Component>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // version: optional message cel.expr.SourceInfo.Extension.Version
  pub fn has_version(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn version_opt(self) -> ::std::option::Option<super::super::source_info::extension::VersionView<'msg>> {
    self.has_version().then(|| self.version())
  }
  pub fn version(self) -> super::super::source_info::extension::VersionView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::source_info::extension::VersionView::default())
  }

}

// SAFETY:
// - `ExtensionView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ExtensionView<'_> {}

// SAFETY:
// - `ExtensionView` is `Send` because while its alive a `ExtensionMut` cannot.
// - `ExtensionView` does not use thread-local data.
unsafe impl ::std::marker::Send for ExtensionView<'_> {}

impl<'msg> ::protobuf::AsView for ExtensionView<'msg> {
  type Proxied = Extension;
  fn as_view(&self) -> ::protobuf::View<'msg, Extension> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExtensionView<'msg> {
  fn into_view<'shorter>(self) -> ExtensionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Extension> for ExtensionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Extension {
    let mut dst = Extension::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Extension> for ExtensionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Extension {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Extension {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExtensionView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExtensionMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ExtensionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Extension>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExtensionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ExtensionMut<'msg> {
  type Message = Extension;
}

impl ::std::fmt::Debug for ExtensionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Extension>> for ExtensionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Extension>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExtensionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Extension> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Extension {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // id: optional string
  pub fn id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // affected_components: repeated enum cel.expr.SourceInfo.Extension.Component
  pub fn affected_components(&self) -> ::protobuf::RepeatedView<'_, super::super::source_info::extension::Component> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::source_info::extension::Component>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn affected_components_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::source_info::extension::Component> {
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
  pub fn set_affected_components(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::source_info::extension::Component>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // version: optional message cel.expr.SourceInfo.Extension.Version
  pub fn has_version(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_version(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn version_opt(&self) -> ::std::option::Option<super::super::source_info::extension::VersionView<'_>> {
    self.has_version().then(|| self.version())
  }
  pub fn version(&self) -> super::super::source_info::extension::VersionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::source_info::extension::VersionView::default())
  }
  pub fn version_mut(&mut self) -> super::super::source_info::extension::VersionMut<'_> {
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
  pub fn set_version(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::source_info::extension::Version>) {

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
// - `ExtensionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ExtensionMut<'_> {}

// SAFETY:
// - `ExtensionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ExtensionMut<'_> {}

impl<'msg> ::protobuf::AsView for ExtensionMut<'msg> {
  type Proxied = Extension;
  fn as_view(&self) -> ::protobuf::View<'_, Extension> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExtensionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Extension>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ExtensionMut<'msg> {
  type MutProxied = Extension;
  fn as_mut(&mut self) -> ExtensionMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ExtensionMut<'msg> {
  fn into_mut<'shorter>(self) -> ExtensionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Extension {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Extension> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ExtensionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ExtensionMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // id: optional string
  pub fn id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // affected_components: repeated enum cel.expr.SourceInfo.Extension.Component
  pub fn affected_components(&self) -> ::protobuf::RepeatedView<'_, super::super::source_info::extension::Component> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::source_info::extension::Component>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn affected_components_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::source_info::extension::Component> {
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
  pub fn set_affected_components(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::source_info::extension::Component>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // version: optional message cel.expr.SourceInfo.Extension.Version
  pub fn has_version(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_version(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn version_opt(&self) -> ::std::option::Option<super::super::source_info::extension::VersionView<'_>> {
    self.has_version().then(|| self.version())
  }
  pub fn version(&self) -> super::super::source_info::extension::VersionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::source_info::extension::VersionView::default())
  }
  pub fn version_mut(&mut self) -> super::super::source_info::extension::VersionMut<'_> {
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
  pub fn set_version(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::source_info::extension::Version>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl Extension

impl ::std::ops::Drop for Extension {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Extension {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Extension {
  type Proxied = Self;
  fn as_view(&self) -> ExtensionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Extension {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ExtensionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Extension {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::source_info::cel__expr__SourceInfo__Extension_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$N1XB3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::source_info::cel__expr__SourceInfo__Extension_msg_init.0, &[<super::super::source_info::extension::Version as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::source_info::cel__expr__SourceInfo__Extension_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Extension {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Extension {
  type Msg = Extension;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Extension> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Extension {
  type Msg = Extension;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Extension> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExtensionMut<'_> {
  type Msg = Extension;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Extension> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtensionMut<'_> {
  type Msg = Extension;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Extension> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtensionView<'_> {
  type Msg = Extension;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Extension> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExtensionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod extension {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut cel__expr__SourceInfo__Extension__Version_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Version {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Version>
}

impl ::protobuf::Message for Version {
  type MessageView<'msg> = VersionView<'msg>;
  type MessageMut<'msg> = VersionMut<'msg>;
}

impl ::std::default::Default for Version {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Version {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Version` is `Sync` because it does not implement interior mutability.
//    Neither does `VersionMut`.
unsafe impl ::std::marker::Sync for Version {}

// SAFETY:
// - `Version` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Version {}

impl ::protobuf::Proxied for Version {
  type View<'msg> = VersionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Version {}

impl ::protobuf::MutProxied for Version {
  type Mut<'msg> = VersionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct VersionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Version>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for VersionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for VersionView<'msg> {
  type Message = Version;
}

impl ::std::fmt::Debug for VersionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for VersionView<'_> {
  fn default() -> VersionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Version>> for VersionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Version>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> VersionView<'msg> {

  pub fn to_owned(&self) -> Version {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // major: optional int64
  pub fn major(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // minor: optional int64
  pub fn minor(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        1, (0i64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `VersionView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for VersionView<'_> {}

// SAFETY:
// - `VersionView` is `Send` because while its alive a `VersionMut` cannot.
// - `VersionView` does not use thread-local data.
unsafe impl ::std::marker::Send for VersionView<'_> {}

impl<'msg> ::protobuf::AsView for VersionView<'msg> {
  type Proxied = Version;
  fn as_view(&self) -> ::protobuf::View<'msg, Version> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for VersionView<'msg> {
  fn into_view<'shorter>(self) -> VersionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Version> for VersionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Version {
    let mut dst = Version::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Version> for VersionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Version {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Version {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for VersionView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for VersionMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct VersionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Version>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for VersionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for VersionMut<'msg> {
  type Message = Version;
}

impl ::std::fmt::Debug for VersionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Version>> for VersionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Version>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> VersionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Version> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Version {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // major: optional int64
  pub fn major(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_major(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        0, val.into()
      )
    }
  }

  // minor: optional int64
  pub fn minor(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        1, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_minor(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        1, val.into()
      )
    }
  }

}

// SAFETY:
// - `VersionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for VersionMut<'_> {}

// SAFETY:
// - `VersionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for VersionMut<'_> {}

impl<'msg> ::protobuf::AsView for VersionMut<'msg> {
  type Proxied = Version;
  fn as_view(&self) -> ::protobuf::View<'_, Version> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for VersionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Version>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for VersionMut<'msg> {
  type MutProxied = Version;
  fn as_mut(&mut self) -> VersionMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for VersionMut<'msg> {
  fn into_mut<'shorter>(self) -> VersionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Version {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Version> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> VersionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> VersionMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // major: optional int64
  pub fn major(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_major(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        0, val.into()
      )
    }
  }

  // minor: optional int64
  pub fn minor(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        1, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_minor(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        1, val.into()
      )
    }
  }

}  // impl Version

impl ::std::ops::Drop for Version {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Version {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Version {
  type Proxied = Self;
  fn as_view(&self) -> VersionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Version {
  type MutProxied = Self;
  fn as_mut(&mut self) -> VersionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Version {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::source_info::extension::cel__expr__SourceInfo__Extension__Version_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$+P+P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::source_info::extension::cel__expr__SourceInfo__Extension__Version_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::source_info::extension::cel__expr__SourceInfo__Extension__Version_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Version {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Version {
  type Msg = Version;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Version> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Version {
  type Msg = Version;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Version> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for VersionMut<'_> {
  type Msg = Version;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Version> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for VersionMut<'_> {
  type Msg = Version;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Version> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for VersionView<'_> {
  type Msg = Version;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Version> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for VersionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Component(i32);

#[allow(non_upper_case_globals)]
impl Component {
  pub const Unspecified: Component = Component(0);
  pub const Parser: Component = Component(1);
  pub const TypeChecker: Component = Component(2);
  pub const Runtime: Component = Component(3);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unspecified",
      1 => "Parser",
      2 => "TypeChecker",
      3 => "Runtime",
      _ => return None
    })
  }
}

impl ::std::convert::From<Component> for i32 {
  fn from(val: Component) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for Component {
  fn from(val: i32) -> Component {
    Self(val)
  }
}

impl ::std::default::Default for Component {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for Component {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "Component::{}", constant_name)
    } else {
      write!(f, "Component::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for Component {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for Component {}

impl ::protobuf::Proxied for Component {
  type View<'a> = Component;
}

impl ::protobuf::AsView for Component {
  type Proxied = Component;

  fn as_view(&self) -> Component {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Component {
  fn into_view<'shorter>(self) -> Component where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for Component {
  const NAME: &'static str = "Component";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3)
  }
}

impl ::protobuf::__internal::EntityType for Component {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod extension


}  // pub mod source_info


