const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut xds__type__v3__CelExpression_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CelExpression {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CelExpression>
}

impl ::protobuf::Message for CelExpression {
  type MessageView<'msg> = CelExpressionView<'msg>;
  type MessageMut<'msg> = CelExpressionMut<'msg>;
}

impl ::std::default::Default for CelExpression {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CelExpression {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CelExpression` is `Sync` because it does not implement interior mutability.
//    Neither does `CelExpressionMut`.
unsafe impl ::std::marker::Sync for CelExpression {}

// SAFETY:
// - `CelExpression` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CelExpression {}

impl ::protobuf::Proxied for CelExpression {
  type View<'msg> = CelExpressionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CelExpression {}

impl ::protobuf::MutProxied for CelExpression {
  type Mut<'msg> = CelExpressionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CelExpressionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CelExpression>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CelExpressionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CelExpressionView<'msg> {
  type Message = CelExpression;
}

impl ::std::fmt::Debug for CelExpressionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CelExpressionView<'_> {
  fn default() -> CelExpressionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CelExpression>> for CelExpressionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CelExpression>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CelExpressionView<'msg> {

  pub fn to_owned(&self) -> CelExpression {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // parsed_expr: optional message google.api.expr.v1alpha1.ParsedExpr
  pub fn has_parsed_expr(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn parsed_expr_opt(self) -> ::std::option::Option<crate::xds::generated::google::api::expr::v1alpha1::syntax::ParsedExprView<'msg>> {
    self.has_parsed_expr().then(|| self.parsed_expr())
  }
  pub fn parsed_expr(self) -> crate::xds::generated::google::api::expr::v1alpha1::syntax::ParsedExprView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::google::api::expr::v1alpha1::syntax::ParsedExprView::default())
  }

  // checked_expr: optional message google.api.expr.v1alpha1.CheckedExpr
  pub fn has_checked_expr(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn checked_expr_opt(self) -> ::std::option::Option<crate::xds::generated::google::api::expr::v1alpha1::checked::CheckedExprView<'msg>> {
    self.has_checked_expr().then(|| self.checked_expr())
  }
  pub fn checked_expr(self) -> crate::xds::generated::google::api::expr::v1alpha1::checked::CheckedExprView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::google::api::expr::v1alpha1::checked::CheckedExprView::default())
  }

  // cel_expr_parsed: optional message cel.expr.ParsedExpr
  pub fn has_cel_expr_parsed(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn cel_expr_parsed_opt(self) -> ::std::option::Option<crate::xds::generated::cel::expr::syntax::ParsedExprView<'msg>> {
    self.has_cel_expr_parsed().then(|| self.cel_expr_parsed())
  }
  pub fn cel_expr_parsed(self) -> crate::xds::generated::cel::expr::syntax::ParsedExprView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::cel::expr::syntax::ParsedExprView::default())
  }

  // cel_expr_checked: optional message cel.expr.CheckedExpr
  pub fn has_cel_expr_checked(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn cel_expr_checked_opt(self) -> ::std::option::Option<crate::xds::generated::cel::expr::checked::CheckedExprView<'msg>> {
    self.has_cel_expr_checked().then(|| self.cel_expr_checked())
  }
  pub fn cel_expr_checked(self) -> crate::xds::generated::cel::expr::checked::CheckedExprView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::cel::expr::checked::CheckedExprView::default())
  }

  // cel_expr_string: optional string
  pub fn cel_expr_string(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  pub fn expr_specifier(self) -> super::cel_expression::ExprSpecifierOneof<'msg> {
    match self.expr_specifier_case() {
      super::cel_expression::ExprSpecifierCase::ParsedExpr =>
          super::cel_expression::ExprSpecifierOneof::ParsedExpr(self.parsed_expr()),
      super::cel_expression::ExprSpecifierCase::CheckedExpr =>
          super::cel_expression::ExprSpecifierOneof::CheckedExpr(self.checked_expr()),
      _ => super::cel_expression::ExprSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn expr_specifier_case(self) -> super::cel_expression::ExprSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::cel_expression::ExprSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `CelExpressionView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CelExpressionView<'_> {}

// SAFETY:
// - `CelExpressionView` is `Send` because while its alive a `CelExpressionMut` cannot.
// - `CelExpressionView` does not use thread-local data.
unsafe impl ::std::marker::Send for CelExpressionView<'_> {}

impl<'msg> ::protobuf::AsView for CelExpressionView<'msg> {
  type Proxied = CelExpression;
  fn as_view(&self) -> ::protobuf::View<'msg, CelExpression> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CelExpressionView<'msg> {
  fn into_view<'shorter>(self) -> CelExpressionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CelExpression> for CelExpressionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CelExpression {
    let mut dst = CelExpression::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CelExpression> for CelExpressionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CelExpression {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CelExpression {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CelExpressionView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CelExpressionMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CelExpressionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CelExpression>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CelExpressionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CelExpressionMut<'msg> {
  type Message = CelExpression;
}

impl ::std::fmt::Debug for CelExpressionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CelExpression>> for CelExpressionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CelExpression>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CelExpressionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CelExpression> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CelExpression {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // parsed_expr: optional message google.api.expr.v1alpha1.ParsedExpr
  pub fn has_parsed_expr(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_parsed_expr(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn parsed_expr_opt(&self) -> ::std::option::Option<crate::xds::generated::google::api::expr::v1alpha1::syntax::ParsedExprView<'_>> {
    self.has_parsed_expr().then(|| self.parsed_expr())
  }
  pub fn parsed_expr(&self) -> crate::xds::generated::google::api::expr::v1alpha1::syntax::ParsedExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::google::api::expr::v1alpha1::syntax::ParsedExprView::default())
  }
  pub fn parsed_expr_mut(&mut self) -> crate::xds::generated::google::api::expr::v1alpha1::syntax::ParsedExprMut<'_> {
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
  pub fn set_parsed_expr(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::google::api::expr::v1alpha1::syntax::ParsedExpr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // checked_expr: optional message google.api.expr.v1alpha1.CheckedExpr
  pub fn has_checked_expr(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_checked_expr(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn checked_expr_opt(&self) -> ::std::option::Option<crate::xds::generated::google::api::expr::v1alpha1::checked::CheckedExprView<'_>> {
    self.has_checked_expr().then(|| self.checked_expr())
  }
  pub fn checked_expr(&self) -> crate::xds::generated::google::api::expr::v1alpha1::checked::CheckedExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::google::api::expr::v1alpha1::checked::CheckedExprView::default())
  }
  pub fn checked_expr_mut(&mut self) -> crate::xds::generated::google::api::expr::v1alpha1::checked::CheckedExprMut<'_> {
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
  pub fn set_checked_expr(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::google::api::expr::v1alpha1::checked::CheckedExpr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // cel_expr_parsed: optional message cel.expr.ParsedExpr
  pub fn has_cel_expr_parsed(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_cel_expr_parsed(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn cel_expr_parsed_opt(&self) -> ::std::option::Option<crate::xds::generated::cel::expr::syntax::ParsedExprView<'_>> {
    self.has_cel_expr_parsed().then(|| self.cel_expr_parsed())
  }
  pub fn cel_expr_parsed(&self) -> crate::xds::generated::cel::expr::syntax::ParsedExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::cel::expr::syntax::ParsedExprView::default())
  }
  pub fn cel_expr_parsed_mut(&mut self) -> crate::xds::generated::cel::expr::syntax::ParsedExprMut<'_> {
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
  pub fn set_cel_expr_parsed(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::cel::expr::syntax::ParsedExpr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // cel_expr_checked: optional message cel.expr.CheckedExpr
  pub fn has_cel_expr_checked(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_cel_expr_checked(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn cel_expr_checked_opt(&self) -> ::std::option::Option<crate::xds::generated::cel::expr::checked::CheckedExprView<'_>> {
    self.has_cel_expr_checked().then(|| self.cel_expr_checked())
  }
  pub fn cel_expr_checked(&self) -> crate::xds::generated::cel::expr::checked::CheckedExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::cel::expr::checked::CheckedExprView::default())
  }
  pub fn cel_expr_checked_mut(&mut self) -> crate::xds::generated::cel::expr::checked::CheckedExprMut<'_> {
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
  pub fn set_cel_expr_checked(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::cel::expr::checked::CheckedExpr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // cel_expr_string: optional string
  pub fn cel_expr_string(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_cel_expr_string(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  pub fn expr_specifier(&self) -> super::cel_expression::ExprSpecifierOneof<'_> {
    match &self.expr_specifier_case() {
      super::cel_expression::ExprSpecifierCase::ParsedExpr =>
          super::cel_expression::ExprSpecifierOneof::ParsedExpr(self.parsed_expr()),
      super::cel_expression::ExprSpecifierCase::CheckedExpr =>
          super::cel_expression::ExprSpecifierOneof::CheckedExpr(self.checked_expr()),
      _ => super::cel_expression::ExprSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn expr_specifier_case(&self) -> super::cel_expression::ExprSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::cel_expression::ExprSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `CelExpressionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CelExpressionMut<'_> {}

// SAFETY:
// - `CelExpressionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CelExpressionMut<'_> {}

impl<'msg> ::protobuf::AsView for CelExpressionMut<'msg> {
  type Proxied = CelExpression;
  fn as_view(&self) -> ::protobuf::View<'_, CelExpression> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CelExpressionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CelExpression>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CelExpressionMut<'msg> {
  type MutProxied = CelExpression;
  fn as_mut(&mut self) -> CelExpressionMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CelExpressionMut<'msg> {
  fn into_mut<'shorter>(self) -> CelExpressionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CelExpression {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CelExpression> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CelExpressionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CelExpressionMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // parsed_expr: optional message google.api.expr.v1alpha1.ParsedExpr
  pub fn has_parsed_expr(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_parsed_expr(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn parsed_expr_opt(&self) -> ::std::option::Option<crate::xds::generated::google::api::expr::v1alpha1::syntax::ParsedExprView<'_>> {
    self.has_parsed_expr().then(|| self.parsed_expr())
  }
  pub fn parsed_expr(&self) -> crate::xds::generated::google::api::expr::v1alpha1::syntax::ParsedExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::google::api::expr::v1alpha1::syntax::ParsedExprView::default())
  }
  pub fn parsed_expr_mut(&mut self) -> crate::xds::generated::google::api::expr::v1alpha1::syntax::ParsedExprMut<'_> {
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
  pub fn set_parsed_expr(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::google::api::expr::v1alpha1::syntax::ParsedExpr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // checked_expr: optional message google.api.expr.v1alpha1.CheckedExpr
  pub fn has_checked_expr(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_checked_expr(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn checked_expr_opt(&self) -> ::std::option::Option<crate::xds::generated::google::api::expr::v1alpha1::checked::CheckedExprView<'_>> {
    self.has_checked_expr().then(|| self.checked_expr())
  }
  pub fn checked_expr(&self) -> crate::xds::generated::google::api::expr::v1alpha1::checked::CheckedExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::google::api::expr::v1alpha1::checked::CheckedExprView::default())
  }
  pub fn checked_expr_mut(&mut self) -> crate::xds::generated::google::api::expr::v1alpha1::checked::CheckedExprMut<'_> {
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
  pub fn set_checked_expr(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::google::api::expr::v1alpha1::checked::CheckedExpr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // cel_expr_parsed: optional message cel.expr.ParsedExpr
  pub fn has_cel_expr_parsed(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_cel_expr_parsed(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn cel_expr_parsed_opt(&self) -> ::std::option::Option<crate::xds::generated::cel::expr::syntax::ParsedExprView<'_>> {
    self.has_cel_expr_parsed().then(|| self.cel_expr_parsed())
  }
  pub fn cel_expr_parsed(&self) -> crate::xds::generated::cel::expr::syntax::ParsedExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::cel::expr::syntax::ParsedExprView::default())
  }
  pub fn cel_expr_parsed_mut(&mut self) -> crate::xds::generated::cel::expr::syntax::ParsedExprMut<'_> {
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
  pub fn set_cel_expr_parsed(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::cel::expr::syntax::ParsedExpr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // cel_expr_checked: optional message cel.expr.CheckedExpr
  pub fn has_cel_expr_checked(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_cel_expr_checked(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn cel_expr_checked_opt(&self) -> ::std::option::Option<crate::xds::generated::cel::expr::checked::CheckedExprView<'_>> {
    self.has_cel_expr_checked().then(|| self.cel_expr_checked())
  }
  pub fn cel_expr_checked(&self) -> crate::xds::generated::cel::expr::checked::CheckedExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::cel::expr::checked::CheckedExprView::default())
  }
  pub fn cel_expr_checked_mut(&mut self) -> crate::xds::generated::cel::expr::checked::CheckedExprMut<'_> {
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
  pub fn set_cel_expr_checked(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::cel::expr::checked::CheckedExpr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // cel_expr_string: optional string
  pub fn cel_expr_string(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_cel_expr_string(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  pub fn expr_specifier(&self) -> super::cel_expression::ExprSpecifierOneof<'_> {
    match &self.expr_specifier_case() {
      super::cel_expression::ExprSpecifierCase::ParsedExpr =>
          super::cel_expression::ExprSpecifierOneof::ParsedExpr(self.parsed_expr()),
      super::cel_expression::ExprSpecifierCase::CheckedExpr =>
          super::cel_expression::ExprSpecifierOneof::CheckedExpr(self.checked_expr()),
      _ => super::cel_expression::ExprSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn expr_specifier_case(&self) -> super::cel_expression::ExprSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::cel_expression::ExprSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl CelExpression

impl ::std::ops::Drop for CelExpression {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CelExpression {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CelExpression {
  type Proxied = Self;
  fn as_view(&self) -> CelExpressionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CelExpression {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CelExpressionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CelExpression {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::xds__type__v3__CelExpression_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33331X^!|#");
        ::protobuf::__internal::runtime::link_mini_table(
            super::xds__type__v3__CelExpression_msg_init.0, &[<crate::xds::generated::google::api::expr::v1alpha1::syntax::ParsedExpr as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::google::api::expr::v1alpha1::checked::CheckedExpr as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::cel::expr::syntax::ParsedExpr as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::cel::expr::checked::CheckedExpr as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::xds__type__v3__CelExpression_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CelExpression {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CelExpression {
  type Msg = CelExpression;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CelExpression> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CelExpression {
  type Msg = CelExpression;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CelExpression> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CelExpressionMut<'_> {
  type Msg = CelExpression;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CelExpression> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CelExpressionMut<'_> {
  type Msg = CelExpression;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CelExpression> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CelExpressionView<'_> {
  type Msg = CelExpression;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CelExpression> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CelExpressionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod cel_expression {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ExprSpecifierOneof<'msg> {
  ParsedExpr(::protobuf::View<'msg, crate::xds::generated::google::api::expr::v1alpha1::syntax::ParsedExpr>) = 1,
  CheckedExpr(::protobuf::View<'msg, crate::xds::generated::google::api::expr::v1alpha1::checked::CheckedExpr>) = 2,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ExprSpecifierCase {
  ParsedExpr = 1,
  CheckedExpr = 2,

  not_set = 0
}

impl ExprSpecifierCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ExprSpecifierCase> {
    match v {
      0 => Some(ExprSpecifierCase::not_set),
      1 => Some(ExprSpecifierCase::ParsedExpr),
      2 => Some(ExprSpecifierCase::CheckedExpr),
      _ => None
    }
  }
}
}  // pub mod cel_expression


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut xds__type__v3__CelExtractString_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CelExtractString {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CelExtractString>
}

impl ::protobuf::Message for CelExtractString {
  type MessageView<'msg> = CelExtractStringView<'msg>;
  type MessageMut<'msg> = CelExtractStringMut<'msg>;
}

impl ::std::default::Default for CelExtractString {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CelExtractString {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CelExtractString` is `Sync` because it does not implement interior mutability.
//    Neither does `CelExtractStringMut`.
unsafe impl ::std::marker::Sync for CelExtractString {}

// SAFETY:
// - `CelExtractString` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CelExtractString {}

impl ::protobuf::Proxied for CelExtractString {
  type View<'msg> = CelExtractStringView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CelExtractString {}

impl ::protobuf::MutProxied for CelExtractString {
  type Mut<'msg> = CelExtractStringMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CelExtractStringView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CelExtractString>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CelExtractStringView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CelExtractStringView<'msg> {
  type Message = CelExtractString;
}

impl ::std::fmt::Debug for CelExtractStringView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CelExtractStringView<'_> {
  fn default() -> CelExtractStringView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CelExtractString>> for CelExtractStringView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CelExtractString>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CelExtractStringView<'msg> {

  pub fn to_owned(&self) -> CelExtractString {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // expr_extract: optional message xds.type.v3.CelExpression
  pub fn has_expr_extract(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn expr_extract_opt(self) -> ::std::option::Option<super::CelExpressionView<'msg>> {
    self.has_expr_extract().then(|| self.expr_extract())
  }
  pub fn expr_extract(self) -> super::CelExpressionView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::CelExpressionView::default())
  }

  // default_value: optional message google.protobuf.StringValue
  pub fn has_default_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn default_value_opt(self) -> ::std::option::Option<::protobuf_well_known_types::StringValueView<'msg>> {
    self.has_default_value().then(|| self.default_value())
  }
  pub fn default_value(self) -> ::protobuf_well_known_types::StringValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StringValueView::default())
  }

}

// SAFETY:
// - `CelExtractStringView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CelExtractStringView<'_> {}

// SAFETY:
// - `CelExtractStringView` is `Send` because while its alive a `CelExtractStringMut` cannot.
// - `CelExtractStringView` does not use thread-local data.
unsafe impl ::std::marker::Send for CelExtractStringView<'_> {}

impl<'msg> ::protobuf::AsView for CelExtractStringView<'msg> {
  type Proxied = CelExtractString;
  fn as_view(&self) -> ::protobuf::View<'msg, CelExtractString> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CelExtractStringView<'msg> {
  fn into_view<'shorter>(self) -> CelExtractStringView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CelExtractString> for CelExtractStringView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CelExtractString {
    let mut dst = CelExtractString::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CelExtractString> for CelExtractStringMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CelExtractString {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CelExtractString {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CelExtractStringView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CelExtractStringMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CelExtractStringMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CelExtractString>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CelExtractStringMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CelExtractStringMut<'msg> {
  type Message = CelExtractString;
}

impl ::std::fmt::Debug for CelExtractStringMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CelExtractString>> for CelExtractStringMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CelExtractString>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CelExtractStringMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CelExtractString> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CelExtractString {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // expr_extract: optional message xds.type.v3.CelExpression
  pub fn has_expr_extract(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_expr_extract(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn expr_extract_opt(&self) -> ::std::option::Option<super::CelExpressionView<'_>> {
    self.has_expr_extract().then(|| self.expr_extract())
  }
  pub fn expr_extract(&self) -> super::CelExpressionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::CelExpressionView::default())
  }
  pub fn expr_extract_mut(&mut self) -> super::CelExpressionMut<'_> {
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
  pub fn set_expr_extract(&mut self,
    val: impl ::protobuf::IntoProxied<super::CelExpression>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // default_value: optional message google.protobuf.StringValue
  pub fn has_default_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_default_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn default_value_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StringValueView<'_>> {
    self.has_default_value().then(|| self.default_value())
  }
  pub fn default_value(&self) -> ::protobuf_well_known_types::StringValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StringValueView::default())
  }
  pub fn default_value_mut(&mut self) -> ::protobuf_well_known_types::StringValueMut<'_> {
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
  pub fn set_default_value(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::StringValue>) {

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
// - `CelExtractStringMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CelExtractStringMut<'_> {}

// SAFETY:
// - `CelExtractStringMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CelExtractStringMut<'_> {}

impl<'msg> ::protobuf::AsView for CelExtractStringMut<'msg> {
  type Proxied = CelExtractString;
  fn as_view(&self) -> ::protobuf::View<'_, CelExtractString> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CelExtractStringMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CelExtractString>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CelExtractStringMut<'msg> {
  type MutProxied = CelExtractString;
  fn as_mut(&mut self) -> CelExtractStringMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CelExtractStringMut<'msg> {
  fn into_mut<'shorter>(self) -> CelExtractStringMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CelExtractString {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CelExtractString> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CelExtractStringView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CelExtractStringMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // expr_extract: optional message xds.type.v3.CelExpression
  pub fn has_expr_extract(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_expr_extract(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn expr_extract_opt(&self) -> ::std::option::Option<super::CelExpressionView<'_>> {
    self.has_expr_extract().then(|| self.expr_extract())
  }
  pub fn expr_extract(&self) -> super::CelExpressionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::CelExpressionView::default())
  }
  pub fn expr_extract_mut(&mut self) -> super::CelExpressionMut<'_> {
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
  pub fn set_expr_extract(&mut self,
    val: impl ::protobuf::IntoProxied<super::CelExpression>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // default_value: optional message google.protobuf.StringValue
  pub fn has_default_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_default_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn default_value_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StringValueView<'_>> {
    self.has_default_value().then(|| self.default_value())
  }
  pub fn default_value(&self) -> ::protobuf_well_known_types::StringValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StringValueView::default())
  }
  pub fn default_value_mut(&mut self) -> ::protobuf_well_known_types::StringValueMut<'_> {
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
  pub fn set_default_value(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::StringValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl CelExtractString

impl ::std::ops::Drop for CelExtractString {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CelExtractString {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CelExtractString {
  type Proxied = Self;
  fn as_view(&self) -> CelExtractStringView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CelExtractString {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CelExtractStringMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CelExtractString {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::xds__type__v3__CelExtractString_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::xds__type__v3__CelExtractString_msg_init.0, &[<super::CelExpression as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::StringValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::xds__type__v3__CelExtractString_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CelExtractString {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CelExtractString {
  type Msg = CelExtractString;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CelExtractString> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CelExtractString {
  type Msg = CelExtractString;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CelExtractString> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CelExtractStringMut<'_> {
  type Msg = CelExtractString;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CelExtractString> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CelExtractStringMut<'_> {
  type Msg = CelExtractString;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CelExtractString> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CelExtractStringView<'_> {
  type Msg = CelExtractString;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CelExtractString> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CelExtractStringMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



