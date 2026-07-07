const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut cel__expr__CheckedExpr_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CheckedExpr {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CheckedExpr>
}

impl ::protobuf::Message for CheckedExpr {
  type MessageView<'msg> = CheckedExprView<'msg>;
  type MessageMut<'msg> = CheckedExprMut<'msg>;
}

impl ::std::default::Default for CheckedExpr {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CheckedExpr {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CheckedExpr` is `Sync` because it does not implement interior mutability.
//    Neither does `CheckedExprMut`.
unsafe impl ::std::marker::Sync for CheckedExpr {}

// SAFETY:
// - `CheckedExpr` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CheckedExpr {}

impl ::protobuf::Proxied for CheckedExpr {
  type View<'msg> = CheckedExprView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CheckedExpr {}

impl ::protobuf::MutProxied for CheckedExpr {
  type Mut<'msg> = CheckedExprMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CheckedExprView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CheckedExpr>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CheckedExprView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CheckedExprView<'msg> {
  type Message = CheckedExpr;
}

impl ::std::fmt::Debug for CheckedExprView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CheckedExprView<'_> {
  fn default() -> CheckedExprView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CheckedExpr>> for CheckedExprView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CheckedExpr>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CheckedExprView<'msg> {

  pub fn to_owned(&self) -> CheckedExpr {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // reference_map: repeated message cel.expr.CheckedExpr.ReferenceMapEntry
  pub fn reference_map(self)
    -> ::protobuf::MapView<'msg, i64, super::Reference> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(0)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<i64, super::Reference>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // type_map: repeated message cel.expr.CheckedExpr.TypeMapEntry
  pub fn type_map(self)
    -> ::protobuf::MapView<'msg, i64, super::Type> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(1)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<i64, super::Type>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // source_info: optional message cel.expr.SourceInfo
  pub fn has_source_info(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn source_info_opt(self) -> ::std::option::Option<crate::xds::generated::cel::expr::syntax::SourceInfoView<'msg>> {
    self.has_source_info().then(|| self.source_info())
  }
  pub fn source_info(self) -> crate::xds::generated::cel::expr::syntax::SourceInfoView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::cel::expr::syntax::SourceInfoView::default())
  }

  // expr_version: optional string
  pub fn expr_version(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // expr: optional message cel.expr.Expr
  pub fn has_expr(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn expr_opt(self) -> ::std::option::Option<crate::xds::generated::cel::expr::syntax::ExprView<'msg>> {
    self.has_expr().then(|| self.expr())
  }
  pub fn expr(self) -> crate::xds::generated::cel::expr::syntax::ExprView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::cel::expr::syntax::ExprView::default())
  }

}

// SAFETY:
// - `CheckedExprView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CheckedExprView<'_> {}

// SAFETY:
// - `CheckedExprView` is `Send` because while its alive a `CheckedExprMut` cannot.
// - `CheckedExprView` does not use thread-local data.
unsafe impl ::std::marker::Send for CheckedExprView<'_> {}

impl<'msg> ::protobuf::AsView for CheckedExprView<'msg> {
  type Proxied = CheckedExpr;
  fn as_view(&self) -> ::protobuf::View<'msg, CheckedExpr> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CheckedExprView<'msg> {
  fn into_view<'shorter>(self) -> CheckedExprView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CheckedExpr> for CheckedExprView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CheckedExpr {
    let mut dst = CheckedExpr::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CheckedExpr> for CheckedExprMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CheckedExpr {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CheckedExpr {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CheckedExprView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CheckedExprMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CheckedExprMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CheckedExpr>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CheckedExprMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CheckedExprMut<'msg> {
  type Message = CheckedExpr;
}

impl ::std::fmt::Debug for CheckedExprMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CheckedExpr>> for CheckedExprMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CheckedExpr>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CheckedExprMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CheckedExpr> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CheckedExpr {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // reference_map: repeated message cel.expr.CheckedExpr.ReferenceMapEntry
  pub fn reference_map(&self)
    -> ::protobuf::MapView<'_, i64, super::Reference> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(0)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<i64, super::Reference>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn reference_map_mut(&mut self)
    -> ::protobuf::MapMut<'_, i64, super::Reference> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          0, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_reference_map(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<i64, super::Reference>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // type_map: repeated message cel.expr.CheckedExpr.TypeMapEntry
  pub fn type_map(&self)
    -> ::protobuf::MapView<'_, i64, super::Type> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(1)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<i64, super::Type>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn type_map_mut(&mut self)
    -> ::protobuf::MapMut<'_, i64, super::Type> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          1, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_type_map(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<i64, super::Type>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // source_info: optional message cel.expr.SourceInfo
  pub fn has_source_info(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_source_info(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn source_info_opt(&self) -> ::std::option::Option<crate::xds::generated::cel::expr::syntax::SourceInfoView<'_>> {
    self.has_source_info().then(|| self.source_info())
  }
  pub fn source_info(&self) -> crate::xds::generated::cel::expr::syntax::SourceInfoView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::cel::expr::syntax::SourceInfoView::default())
  }
  pub fn source_info_mut(&mut self) -> crate::xds::generated::cel::expr::syntax::SourceInfoMut<'_> {
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
  pub fn set_source_info(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::cel::expr::syntax::SourceInfo>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // expr_version: optional string
  pub fn expr_version(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_expr_version(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // expr: optional message cel.expr.Expr
  pub fn has_expr(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_expr(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn expr_opt(&self) -> ::std::option::Option<crate::xds::generated::cel::expr::syntax::ExprView<'_>> {
    self.has_expr().then(|| self.expr())
  }
  pub fn expr(&self) -> crate::xds::generated::cel::expr::syntax::ExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::cel::expr::syntax::ExprView::default())
  }
  pub fn expr_mut(&mut self) -> crate::xds::generated::cel::expr::syntax::ExprMut<'_> {
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
  pub fn set_expr(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::cel::expr::syntax::Expr>) {

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
// - `CheckedExprMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CheckedExprMut<'_> {}

// SAFETY:
// - `CheckedExprMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CheckedExprMut<'_> {}

impl<'msg> ::protobuf::AsView for CheckedExprMut<'msg> {
  type Proxied = CheckedExpr;
  fn as_view(&self) -> ::protobuf::View<'_, CheckedExpr> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CheckedExprMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CheckedExpr>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CheckedExprMut<'msg> {
  type MutProxied = CheckedExpr;
  fn as_mut(&mut self) -> CheckedExprMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CheckedExprMut<'msg> {
  fn into_mut<'shorter>(self) -> CheckedExprMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CheckedExpr {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CheckedExpr> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CheckedExprView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CheckedExprMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // reference_map: repeated message cel.expr.CheckedExpr.ReferenceMapEntry
  pub fn reference_map(&self)
    -> ::protobuf::MapView<'_, i64, super::Reference> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(0)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<i64, super::Reference>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn reference_map_mut(&mut self)
    -> ::protobuf::MapMut<'_, i64, super::Reference> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          0, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_reference_map(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<i64, super::Reference>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // type_map: repeated message cel.expr.CheckedExpr.TypeMapEntry
  pub fn type_map(&self)
    -> ::protobuf::MapView<'_, i64, super::Type> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(1)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<i64, super::Type>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn type_map_mut(&mut self)
    -> ::protobuf::MapMut<'_, i64, super::Type> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          1, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_type_map(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<i64, super::Type>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // source_info: optional message cel.expr.SourceInfo
  pub fn has_source_info(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_source_info(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn source_info_opt(&self) -> ::std::option::Option<crate::xds::generated::cel::expr::syntax::SourceInfoView<'_>> {
    self.has_source_info().then(|| self.source_info())
  }
  pub fn source_info(&self) -> crate::xds::generated::cel::expr::syntax::SourceInfoView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::cel::expr::syntax::SourceInfoView::default())
  }
  pub fn source_info_mut(&mut self) -> crate::xds::generated::cel::expr::syntax::SourceInfoMut<'_> {
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
  pub fn set_source_info(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::cel::expr::syntax::SourceInfo>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // expr_version: optional string
  pub fn expr_version(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_expr_version(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // expr: optional message cel.expr.Expr
  pub fn has_expr(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_expr(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn expr_opt(&self) -> ::std::option::Option<crate::xds::generated::cel::expr::syntax::ExprView<'_>> {
    self.has_expr().then(|| self.expr())
  }
  pub fn expr(&self) -> crate::xds::generated::cel::expr::syntax::ExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::cel::expr::syntax::ExprView::default())
  }
  pub fn expr_mut(&mut self) -> crate::xds::generated::cel::expr::syntax::ExprMut<'_> {
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
  pub fn set_expr(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::cel::expr::syntax::Expr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl CheckedExpr

impl ::std::ops::Drop for CheckedExpr {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CheckedExpr {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CheckedExpr {
  type Proxied = Self;
  fn as_view(&self) -> CheckedExprView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CheckedExpr {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CheckedExprMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CheckedExpr {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::cel__expr__CheckedExpr_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$aGG331X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::cel__expr__CheckedExpr_msg_init.0, &[<super::checked_expr::ReferenceMapEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::checked_expr::TypeMapEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::cel::expr::syntax::Expr as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::cel::expr::syntax::SourceInfo as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::cel__expr__CheckedExpr_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CheckedExpr {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CheckedExpr {
  type Msg = CheckedExpr;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CheckedExpr> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CheckedExpr {
  type Msg = CheckedExpr;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CheckedExpr> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CheckedExprMut<'_> {
  type Msg = CheckedExpr;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CheckedExpr> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CheckedExprMut<'_> {
  type Msg = CheckedExpr;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CheckedExpr> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CheckedExprView<'_> {
  type Msg = CheckedExpr;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CheckedExpr> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CheckedExprMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod checked_expr {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut cel__expr__CheckedExpr__ReferenceMapEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct ReferenceMapEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ReferenceMapEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::checked_expr::cel__expr__CheckedExpr__ReferenceMapEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%+P3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::checked_expr::cel__expr__CheckedExpr__ReferenceMapEntry_msg_init.0, &[<super::super::Reference as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::checked_expr::cel__expr__CheckedExpr__ReferenceMapEntry_msg_init.0)
      }).0
    }
  }
}
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut cel__expr__CheckedExpr__TypeMapEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct TypeMapEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TypeMapEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::checked_expr::cel__expr__CheckedExpr__TypeMapEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%+P3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::checked_expr::cel__expr__CheckedExpr__TypeMapEntry_msg_init.0, &[<super::super::Type as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::checked_expr::cel__expr__CheckedExpr__TypeMapEntry_msg_init.0)
      }).0
    }
  }
}

}  // pub mod checked_expr


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut cel__expr__Type_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Type {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Type>
}

impl ::protobuf::Message for Type {
  type MessageView<'msg> = TypeView<'msg>;
  type MessageMut<'msg> = TypeMut<'msg>;
}

impl ::std::default::Default for Type {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Type {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Type` is `Sync` because it does not implement interior mutability.
//    Neither does `TypeMut`.
unsafe impl ::std::marker::Sync for Type {}

// SAFETY:
// - `Type` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Type {}

impl ::protobuf::Proxied for Type {
  type View<'msg> = TypeView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Type {}

impl ::protobuf::MutProxied for Type {
  type Mut<'msg> = TypeMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TypeView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Type>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TypeView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TypeView<'msg> {
  type Message = Type;
}

impl ::std::fmt::Debug for TypeView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TypeView<'_> {
  fn default() -> TypeView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Type>> for TypeView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Type>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TypeView<'msg> {

  pub fn to_owned(&self) -> Type {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // dyn: optional message google.protobuf.Empty
  pub fn has_dyn(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn dyn_opt(self) -> ::std::option::Option<::protobuf_well_known_types::EmptyView<'msg>> {
    self.has_dyn().then(|| self.r#dyn())
  }
  pub fn r#dyn(self) -> ::protobuf_well_known_types::EmptyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::EmptyView::default())
  }

  // null: optional enum google.protobuf.NullValue
  pub fn has_null(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn null_opt(self) -> ::std::option::Option<::protobuf_well_known_types::NullValue> {
    self.has_null().then(|| self.null())
  }
  pub fn null(self) -> ::protobuf_well_known_types::NullValue {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (::protobuf_well_known_types::NullValue::NullValue).into()
      ).try_into().unwrap()
    }
  }

  // primitive: optional enum cel.expr.Type.PrimitiveType
  pub fn has_primitive(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn primitive_opt(self) -> ::std::option::Option<super::r#type::PrimitiveType> {
    self.has_primitive().then(|| self.primitive())
  }
  pub fn primitive(self) -> super::r#type::PrimitiveType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (super::r#type::PrimitiveType::Unspecified).into()
      ).try_into().unwrap()
    }
  }

  // wrapper: optional enum cel.expr.Type.PrimitiveType
  pub fn has_wrapper(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn wrapper_opt(self) -> ::std::option::Option<super::r#type::PrimitiveType> {
    self.has_wrapper().then(|| self.wrapper())
  }
  pub fn wrapper(self) -> super::r#type::PrimitiveType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (super::r#type::PrimitiveType::Unspecified).into()
      ).try_into().unwrap()
    }
  }

  // well_known: optional enum cel.expr.Type.WellKnownType
  pub fn has_well_known(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn well_known_opt(self) -> ::std::option::Option<super::r#type::WellKnownType> {
    self.has_well_known().then(|| self.well_known())
  }
  pub fn well_known(self) -> super::r#type::WellKnownType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::r#type::WellKnownType::Unspecified).into()
      ).try_into().unwrap()
    }
  }

  // list_type: optional message cel.expr.Type.ListType
  pub fn has_list_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn list_type_opt(self) -> ::std::option::Option<super::r#type::ListTypeView<'msg>> {
    self.has_list_type().then(|| self.list_type())
  }
  pub fn list_type(self) -> super::r#type::ListTypeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::r#type::ListTypeView::default())
  }

  // map_type: optional message cel.expr.Type.MapType
  pub fn has_map_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn map_type_opt(self) -> ::std::option::Option<super::r#type::MapTypeView<'msg>> {
    self.has_map_type().then(|| self.map_type())
  }
  pub fn map_type(self) -> super::r#type::MapTypeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::r#type::MapTypeView::default())
  }

  // function: optional message cel.expr.Type.FunctionType
  pub fn has_function(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn function_opt(self) -> ::std::option::Option<super::r#type::FunctionTypeView<'msg>> {
    self.has_function().then(|| self.function())
  }
  pub fn function(self) -> super::r#type::FunctionTypeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::r#type::FunctionTypeView::default())
  }

  // message_type: optional string
  pub fn has_message_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn message_type_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_message_type().then(|| self.message_type())
  }
  pub fn message_type(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // type_param: optional string
  pub fn has_type_param(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn type_param_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_type_param().then(|| self.type_param())
  }
  pub fn type_param(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // type: optional message cel.expr.Type
  pub fn has_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn type_opt(self) -> ::std::option::Option<super::TypeView<'msg>> {
    self.has_type().then(|| self.r#type())
  }
  pub fn r#type(self) -> super::TypeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TypeView::default())
  }

  // error: optional message google.protobuf.Empty
  pub fn has_error(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn error_opt(self) -> ::std::option::Option<::protobuf_well_known_types::EmptyView<'msg>> {
    self.has_error().then(|| self.error())
  }
  pub fn error(self) -> ::protobuf_well_known_types::EmptyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::EmptyView::default())
  }

  // abstract_type: optional message cel.expr.Type.AbstractType
  pub fn has_abstract_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn abstract_type_opt(self) -> ::std::option::Option<super::r#type::AbstractTypeView<'msg>> {
    self.has_abstract_type().then(|| self.abstract_type())
  }
  pub fn abstract_type(self) -> super::r#type::AbstractTypeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::r#type::AbstractTypeView::default())
  }

  pub fn type_kind(self) -> super::r#type::TypeKindOneof<'msg> {
    match self.type_kind_case() {
      super::r#type::TypeKindCase::Dyn =>
          super::r#type::TypeKindOneof::Dyn(self.r#dyn()),
      super::r#type::TypeKindCase::Null =>
          super::r#type::TypeKindOneof::Null(self.null()),
      super::r#type::TypeKindCase::Primitive =>
          super::r#type::TypeKindOneof::Primitive(self.primitive()),
      super::r#type::TypeKindCase::Wrapper =>
          super::r#type::TypeKindOneof::Wrapper(self.wrapper()),
      super::r#type::TypeKindCase::WellKnown =>
          super::r#type::TypeKindOneof::WellKnown(self.well_known()),
      super::r#type::TypeKindCase::ListType =>
          super::r#type::TypeKindOneof::ListType(self.list_type()),
      super::r#type::TypeKindCase::MapType =>
          super::r#type::TypeKindOneof::MapType(self.map_type()),
      super::r#type::TypeKindCase::Function =>
          super::r#type::TypeKindOneof::Function(self.function()),
      super::r#type::TypeKindCase::MessageType =>
          super::r#type::TypeKindOneof::MessageType(self.message_type()),
      super::r#type::TypeKindCase::TypeParam =>
          super::r#type::TypeKindOneof::TypeParam(self.type_param()),
      super::r#type::TypeKindCase::Type =>
          super::r#type::TypeKindOneof::Type(self.r#type()),
      super::r#type::TypeKindCase::Error =>
          super::r#type::TypeKindOneof::Error(self.error()),
      super::r#type::TypeKindCase::AbstractType =>
          super::r#type::TypeKindOneof::AbstractType(self.abstract_type()),
      _ => super::r#type::TypeKindOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn type_kind_case(self) -> super::r#type::TypeKindCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::r#type::TypeKindCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `TypeView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TypeView<'_> {}

// SAFETY:
// - `TypeView` is `Send` because while its alive a `TypeMut` cannot.
// - `TypeView` does not use thread-local data.
unsafe impl ::std::marker::Send for TypeView<'_> {}

impl<'msg> ::protobuf::AsView for TypeView<'msg> {
  type Proxied = Type;
  fn as_view(&self) -> ::protobuf::View<'msg, Type> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TypeView<'msg> {
  fn into_view<'shorter>(self) -> TypeView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Type> for TypeView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Type {
    let mut dst = Type::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Type> for TypeMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Type {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Type {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TypeView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TypeMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TypeMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Type>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TypeMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TypeMut<'msg> {
  type Message = Type;
}

impl ::std::fmt::Debug for TypeMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Type>> for TypeMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Type>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TypeMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Type> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Type {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // dyn: optional message google.protobuf.Empty
  pub fn has_dyn(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_dyn(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn dyn_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::EmptyView<'_>> {
    self.has_dyn().then(|| self.r#dyn())
  }
  pub fn r#dyn(&self) -> ::protobuf_well_known_types::EmptyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::EmptyView::default())
  }
  pub fn dyn_mut(&mut self) -> ::protobuf_well_known_types::EmptyMut<'_> {
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
  pub fn set_dyn(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Empty>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // null: optional enum google.protobuf.NullValue
  pub fn has_null(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_null(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn null_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::NullValue> {
    self.has_null().then(|| self.null())
  }
  pub fn null(&self) -> ::protobuf_well_known_types::NullValue {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (::protobuf_well_known_types::NullValue::NullValue).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_null(&mut self, val: ::protobuf_well_known_types::NullValue) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        1, val.into()
      )
    }
  }

  // primitive: optional enum cel.expr.Type.PrimitiveType
  pub fn has_primitive(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_primitive(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn primitive_opt(&self) -> ::std::option::Option<super::r#type::PrimitiveType> {
    self.has_primitive().then(|| self.primitive())
  }
  pub fn primitive(&self) -> super::r#type::PrimitiveType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (super::r#type::PrimitiveType::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_primitive(&mut self, val: super::r#type::PrimitiveType) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

  // wrapper: optional enum cel.expr.Type.PrimitiveType
  pub fn has_wrapper(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_wrapper(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn wrapper_opt(&self) -> ::std::option::Option<super::r#type::PrimitiveType> {
    self.has_wrapper().then(|| self.wrapper())
  }
  pub fn wrapper(&self) -> super::r#type::PrimitiveType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (super::r#type::PrimitiveType::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_wrapper(&mut self, val: super::r#type::PrimitiveType) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        3, val.into()
      )
    }
  }

  // well_known: optional enum cel.expr.Type.WellKnownType
  pub fn has_well_known(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_well_known(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn well_known_opt(&self) -> ::std::option::Option<super::r#type::WellKnownType> {
    self.has_well_known().then(|| self.well_known())
  }
  pub fn well_known(&self) -> super::r#type::WellKnownType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::r#type::WellKnownType::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_well_known(&mut self, val: super::r#type::WellKnownType) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        4, val.into()
      )
    }
  }

  // list_type: optional message cel.expr.Type.ListType
  pub fn has_list_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_list_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn list_type_opt(&self) -> ::std::option::Option<super::r#type::ListTypeView<'_>> {
    self.has_list_type().then(|| self.list_type())
  }
  pub fn list_type(&self) -> super::r#type::ListTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::r#type::ListTypeView::default())
  }
  pub fn list_type_mut(&mut self) -> super::r#type::ListTypeMut<'_> {
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
  pub fn set_list_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::r#type::ListType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // map_type: optional message cel.expr.Type.MapType
  pub fn has_map_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_map_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn map_type_opt(&self) -> ::std::option::Option<super::r#type::MapTypeView<'_>> {
    self.has_map_type().then(|| self.map_type())
  }
  pub fn map_type(&self) -> super::r#type::MapTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::r#type::MapTypeView::default())
  }
  pub fn map_type_mut(&mut self) -> super::r#type::MapTypeMut<'_> {
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
  pub fn set_map_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::r#type::MapType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // function: optional message cel.expr.Type.FunctionType
  pub fn has_function(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_function(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn function_opt(&self) -> ::std::option::Option<super::r#type::FunctionTypeView<'_>> {
    self.has_function().then(|| self.function())
  }
  pub fn function(&self) -> super::r#type::FunctionTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::r#type::FunctionTypeView::default())
  }
  pub fn function_mut(&mut self) -> super::r#type::FunctionTypeMut<'_> {
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
  pub fn set_function(&mut self,
    val: impl ::protobuf::IntoProxied<super::r#type::FunctionType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // message_type: optional string
  pub fn has_message_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_message_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn message_type_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_message_type().then(|| self.message_type())
  }
  pub fn message_type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_message_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val);
    }
  }

  // type_param: optional string
  pub fn has_type_param(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_type_param(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn type_param_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_type_param().then(|| self.type_param())
  }
  pub fn type_param(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_type_param(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val);
    }
  }

  // type: optional message cel.expr.Type
  pub fn has_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn type_opt(&self) -> ::std::option::Option<super::TypeView<'_>> {
    self.has_type().then(|| self.r#type())
  }
  pub fn r#type(&self) -> super::TypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TypeView::default())
  }
  pub fn type_mut(&mut self) -> super::TypeMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         10, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::Type>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // error: optional message google.protobuf.Empty
  pub fn has_error(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_error(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn error_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::EmptyView<'_>> {
    self.has_error().then(|| self.error())
  }
  pub fn error(&self) -> ::protobuf_well_known_types::EmptyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::EmptyView::default())
  }
  pub fn error_mut(&mut self) -> ::protobuf_well_known_types::EmptyMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         11, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_error(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Empty>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // abstract_type: optional message cel.expr.Type.AbstractType
  pub fn has_abstract_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_abstract_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn abstract_type_opt(&self) -> ::std::option::Option<super::r#type::AbstractTypeView<'_>> {
    self.has_abstract_type().then(|| self.abstract_type())
  }
  pub fn abstract_type(&self) -> super::r#type::AbstractTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::r#type::AbstractTypeView::default())
  }
  pub fn abstract_type_mut(&mut self) -> super::r#type::AbstractTypeMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         12, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_abstract_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::r#type::AbstractType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val
      );
    }
  }

  pub fn type_kind(&self) -> super::r#type::TypeKindOneof<'_> {
    match &self.type_kind_case() {
      super::r#type::TypeKindCase::Dyn =>
          super::r#type::TypeKindOneof::Dyn(self.r#dyn()),
      super::r#type::TypeKindCase::Null =>
          super::r#type::TypeKindOneof::Null(self.null()),
      super::r#type::TypeKindCase::Primitive =>
          super::r#type::TypeKindOneof::Primitive(self.primitive()),
      super::r#type::TypeKindCase::Wrapper =>
          super::r#type::TypeKindOneof::Wrapper(self.wrapper()),
      super::r#type::TypeKindCase::WellKnown =>
          super::r#type::TypeKindOneof::WellKnown(self.well_known()),
      super::r#type::TypeKindCase::ListType =>
          super::r#type::TypeKindOneof::ListType(self.list_type()),
      super::r#type::TypeKindCase::MapType =>
          super::r#type::TypeKindOneof::MapType(self.map_type()),
      super::r#type::TypeKindCase::Function =>
          super::r#type::TypeKindOneof::Function(self.function()),
      super::r#type::TypeKindCase::MessageType =>
          super::r#type::TypeKindOneof::MessageType(self.message_type()),
      super::r#type::TypeKindCase::TypeParam =>
          super::r#type::TypeKindOneof::TypeParam(self.type_param()),
      super::r#type::TypeKindCase::Type =>
          super::r#type::TypeKindOneof::Type(self.r#type()),
      super::r#type::TypeKindCase::Error =>
          super::r#type::TypeKindOneof::Error(self.error()),
      super::r#type::TypeKindCase::AbstractType =>
          super::r#type::TypeKindOneof::AbstractType(self.abstract_type()),
      _ => super::r#type::TypeKindOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn type_kind_case(&self) -> super::r#type::TypeKindCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::r#type::TypeKindCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `TypeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TypeMut<'_> {}

// SAFETY:
// - `TypeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TypeMut<'_> {}

impl<'msg> ::protobuf::AsView for TypeMut<'msg> {
  type Proxied = Type;
  fn as_view(&self) -> ::protobuf::View<'_, Type> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TypeMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Type>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TypeMut<'msg> {
  type MutProxied = Type;
  fn as_mut(&mut self) -> TypeMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TypeMut<'msg> {
  fn into_mut<'shorter>(self) -> TypeMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Type {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Type> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TypeView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TypeMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // dyn: optional message google.protobuf.Empty
  pub fn has_dyn(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_dyn(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn dyn_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::EmptyView<'_>> {
    self.has_dyn().then(|| self.r#dyn())
  }
  pub fn r#dyn(&self) -> ::protobuf_well_known_types::EmptyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::EmptyView::default())
  }
  pub fn dyn_mut(&mut self) -> ::protobuf_well_known_types::EmptyMut<'_> {
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
  pub fn set_dyn(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Empty>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // null: optional enum google.protobuf.NullValue
  pub fn has_null(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_null(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn null_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::NullValue> {
    self.has_null().then(|| self.null())
  }
  pub fn null(&self) -> ::protobuf_well_known_types::NullValue {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (::protobuf_well_known_types::NullValue::NullValue).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_null(&mut self, val: ::protobuf_well_known_types::NullValue) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        1, val.into()
      )
    }
  }

  // primitive: optional enum cel.expr.Type.PrimitiveType
  pub fn has_primitive(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_primitive(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn primitive_opt(&self) -> ::std::option::Option<super::r#type::PrimitiveType> {
    self.has_primitive().then(|| self.primitive())
  }
  pub fn primitive(&self) -> super::r#type::PrimitiveType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (super::r#type::PrimitiveType::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_primitive(&mut self, val: super::r#type::PrimitiveType) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

  // wrapper: optional enum cel.expr.Type.PrimitiveType
  pub fn has_wrapper(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_wrapper(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn wrapper_opt(&self) -> ::std::option::Option<super::r#type::PrimitiveType> {
    self.has_wrapper().then(|| self.wrapper())
  }
  pub fn wrapper(&self) -> super::r#type::PrimitiveType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (super::r#type::PrimitiveType::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_wrapper(&mut self, val: super::r#type::PrimitiveType) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        3, val.into()
      )
    }
  }

  // well_known: optional enum cel.expr.Type.WellKnownType
  pub fn has_well_known(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_well_known(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn well_known_opt(&self) -> ::std::option::Option<super::r#type::WellKnownType> {
    self.has_well_known().then(|| self.well_known())
  }
  pub fn well_known(&self) -> super::r#type::WellKnownType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::r#type::WellKnownType::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_well_known(&mut self, val: super::r#type::WellKnownType) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        4, val.into()
      )
    }
  }

  // list_type: optional message cel.expr.Type.ListType
  pub fn has_list_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_list_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn list_type_opt(&self) -> ::std::option::Option<super::r#type::ListTypeView<'_>> {
    self.has_list_type().then(|| self.list_type())
  }
  pub fn list_type(&self) -> super::r#type::ListTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::r#type::ListTypeView::default())
  }
  pub fn list_type_mut(&mut self) -> super::r#type::ListTypeMut<'_> {
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
  pub fn set_list_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::r#type::ListType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // map_type: optional message cel.expr.Type.MapType
  pub fn has_map_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_map_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn map_type_opt(&self) -> ::std::option::Option<super::r#type::MapTypeView<'_>> {
    self.has_map_type().then(|| self.map_type())
  }
  pub fn map_type(&self) -> super::r#type::MapTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::r#type::MapTypeView::default())
  }
  pub fn map_type_mut(&mut self) -> super::r#type::MapTypeMut<'_> {
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
  pub fn set_map_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::r#type::MapType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // function: optional message cel.expr.Type.FunctionType
  pub fn has_function(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_function(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn function_opt(&self) -> ::std::option::Option<super::r#type::FunctionTypeView<'_>> {
    self.has_function().then(|| self.function())
  }
  pub fn function(&self) -> super::r#type::FunctionTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::r#type::FunctionTypeView::default())
  }
  pub fn function_mut(&mut self) -> super::r#type::FunctionTypeMut<'_> {
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
  pub fn set_function(&mut self,
    val: impl ::protobuf::IntoProxied<super::r#type::FunctionType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // message_type: optional string
  pub fn has_message_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_message_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn message_type_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_message_type().then(|| self.message_type())
  }
  pub fn message_type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_message_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val);
    }
  }

  // type_param: optional string
  pub fn has_type_param(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_type_param(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn type_param_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_type_param().then(|| self.type_param())
  }
  pub fn type_param(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_type_param(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val);
    }
  }

  // type: optional message cel.expr.Type
  pub fn has_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn type_opt(&self) -> ::std::option::Option<super::TypeView<'_>> {
    self.has_type().then(|| self.r#type())
  }
  pub fn r#type(&self) -> super::TypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TypeView::default())
  }
  pub fn type_mut(&mut self) -> super::TypeMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         10, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::Type>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // error: optional message google.protobuf.Empty
  pub fn has_error(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_error(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn error_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::EmptyView<'_>> {
    self.has_error().then(|| self.error())
  }
  pub fn error(&self) -> ::protobuf_well_known_types::EmptyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::EmptyView::default())
  }
  pub fn error_mut(&mut self) -> ::protobuf_well_known_types::EmptyMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         11, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_error(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Empty>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // abstract_type: optional message cel.expr.Type.AbstractType
  pub fn has_abstract_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_abstract_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn abstract_type_opt(&self) -> ::std::option::Option<super::r#type::AbstractTypeView<'_>> {
    self.has_abstract_type().then(|| self.abstract_type())
  }
  pub fn abstract_type(&self) -> super::r#type::AbstractTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::r#type::AbstractTypeView::default())
  }
  pub fn abstract_type_mut(&mut self) -> super::r#type::AbstractTypeMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         12, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_abstract_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::r#type::AbstractType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val
      );
    }
  }

  pub fn type_kind(&self) -> super::r#type::TypeKindOneof<'_> {
    match &self.type_kind_case() {
      super::r#type::TypeKindCase::Dyn =>
          super::r#type::TypeKindOneof::Dyn(self.r#dyn()),
      super::r#type::TypeKindCase::Null =>
          super::r#type::TypeKindOneof::Null(self.null()),
      super::r#type::TypeKindCase::Primitive =>
          super::r#type::TypeKindOneof::Primitive(self.primitive()),
      super::r#type::TypeKindCase::Wrapper =>
          super::r#type::TypeKindOneof::Wrapper(self.wrapper()),
      super::r#type::TypeKindCase::WellKnown =>
          super::r#type::TypeKindOneof::WellKnown(self.well_known()),
      super::r#type::TypeKindCase::ListType =>
          super::r#type::TypeKindOneof::ListType(self.list_type()),
      super::r#type::TypeKindCase::MapType =>
          super::r#type::TypeKindOneof::MapType(self.map_type()),
      super::r#type::TypeKindCase::Function =>
          super::r#type::TypeKindOneof::Function(self.function()),
      super::r#type::TypeKindCase::MessageType =>
          super::r#type::TypeKindOneof::MessageType(self.message_type()),
      super::r#type::TypeKindCase::TypeParam =>
          super::r#type::TypeKindOneof::TypeParam(self.type_param()),
      super::r#type::TypeKindCase::Type =>
          super::r#type::TypeKindOneof::Type(self.r#type()),
      super::r#type::TypeKindCase::Error =>
          super::r#type::TypeKindOneof::Error(self.error()),
      super::r#type::TypeKindCase::AbstractType =>
          super::r#type::TypeKindOneof::AbstractType(self.abstract_type()),
      _ => super::r#type::TypeKindOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn type_kind_case(&self) -> super::r#type::TypeKindCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::r#type::TypeKindCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl Type

impl ::std::ops::Drop for Type {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Type {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Type {
  type Proxied = Self;
  fn as_view(&self) -> TypeView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Type {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TypeMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Type {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::cel__expr__Type_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3....3331T1T33a3^!|#|$|%|&|(|)|*|+|,|-|.|0");
        super::r#type::cel__expr__Type__AbstractType_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1XG");
        super::r#type::cel__expr__Type__FunctionType_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3G");
        super::r#type::cel__expr__Type__ListType_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        super::r#type::cel__expr__Type__MapType_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::cel__expr__Type_msg_init.0, &[<::protobuf_well_known_types::Empty as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            super::r#type::cel__expr__Type__ListType_msg_init.0,
            super::r#type::cel__expr__Type__MapType_msg_init.0,
            super::r#type::cel__expr__Type__FunctionType_msg_init.0,
            super::cel__expr__Type_msg_init.0,
            <::protobuf_well_known_types::Empty as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            super::r#type::cel__expr__Type__AbstractType_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::r#type::cel__expr__Type__AbstractType_msg_init.0, &[super::cel__expr__Type_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::r#type::cel__expr__Type__FunctionType_msg_init.0, &[super::cel__expr__Type_msg_init.0,
            super::cel__expr__Type_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::r#type::cel__expr__Type__ListType_msg_init.0, &[super::cel__expr__Type_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::r#type::cel__expr__Type__MapType_msg_init.0, &[super::cel__expr__Type_msg_init.0,
            super::cel__expr__Type_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::cel__expr__Type_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Type {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Type {
  type Msg = Type;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Type> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Type {
  type Msg = Type;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Type> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TypeMut<'_> {
  type Msg = Type;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Type> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TypeMut<'_> {
  type Msg = Type;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Type> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TypeView<'_> {
  type Msg = Type;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Type> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TypeMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod r#type {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut cel__expr__Type__ListType_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ListType {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ListType>
}

impl ::protobuf::Message for ListType {
  type MessageView<'msg> = ListTypeView<'msg>;
  type MessageMut<'msg> = ListTypeMut<'msg>;
}

impl ::std::default::Default for ListType {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ListType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ListType` is `Sync` because it does not implement interior mutability.
//    Neither does `ListTypeMut`.
unsafe impl ::std::marker::Sync for ListType {}

// SAFETY:
// - `ListType` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ListType {}

impl ::protobuf::Proxied for ListType {
  type View<'msg> = ListTypeView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ListType {}

impl ::protobuf::MutProxied for ListType {
  type Mut<'msg> = ListTypeMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ListTypeView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListType>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListTypeView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ListTypeView<'msg> {
  type Message = ListType;
}

impl ::std::fmt::Debug for ListTypeView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ListTypeView<'_> {
  fn default() -> ListTypeView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ListType>> for ListTypeView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListType>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListTypeView<'msg> {

  pub fn to_owned(&self) -> ListType {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // elem_type: optional message cel.expr.Type
  pub fn has_elem_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn elem_type_opt(self) -> ::std::option::Option<super::super::TypeView<'msg>> {
    self.has_elem_type().then(|| self.elem_type())
  }
  pub fn elem_type(self) -> super::super::TypeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::TypeView::default())
  }

}

// SAFETY:
// - `ListTypeView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ListTypeView<'_> {}

// SAFETY:
// - `ListTypeView` is `Send` because while its alive a `ListTypeMut` cannot.
// - `ListTypeView` does not use thread-local data.
unsafe impl ::std::marker::Send for ListTypeView<'_> {}

impl<'msg> ::protobuf::AsView for ListTypeView<'msg> {
  type Proxied = ListType;
  fn as_view(&self) -> ::protobuf::View<'msg, ListType> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListTypeView<'msg> {
  fn into_view<'shorter>(self) -> ListTypeView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ListType> for ListTypeView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListType {
    let mut dst = ListType::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ListType> for ListTypeMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListType {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ListType {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ListTypeView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ListTypeMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ListTypeMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListType>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListTypeMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ListTypeMut<'msg> {
  type Message = ListType;
}

impl ::std::fmt::Debug for ListTypeMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ListType>> for ListTypeMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListType>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListTypeMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ListType> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ListType {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // elem_type: optional message cel.expr.Type
  pub fn has_elem_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_elem_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn elem_type_opt(&self) -> ::std::option::Option<super::super::TypeView<'_>> {
    self.has_elem_type().then(|| self.elem_type())
  }
  pub fn elem_type(&self) -> super::super::TypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::TypeView::default())
  }
  pub fn elem_type_mut(&mut self) -> super::super::TypeMut<'_> {
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
  pub fn set_elem_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Type>) {

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
// - `ListTypeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ListTypeMut<'_> {}

// SAFETY:
// - `ListTypeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ListTypeMut<'_> {}

impl<'msg> ::protobuf::AsView for ListTypeMut<'msg> {
  type Proxied = ListType;
  fn as_view(&self) -> ::protobuf::View<'_, ListType> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListTypeMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ListType>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ListTypeMut<'msg> {
  type MutProxied = ListType;
  fn as_mut(&mut self) -> ListTypeMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ListTypeMut<'msg> {
  fn into_mut<'shorter>(self) -> ListTypeMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ListType {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ListType> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ListTypeView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ListTypeMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // elem_type: optional message cel.expr.Type
  pub fn has_elem_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_elem_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn elem_type_opt(&self) -> ::std::option::Option<super::super::TypeView<'_>> {
    self.has_elem_type().then(|| self.elem_type())
  }
  pub fn elem_type(&self) -> super::super::TypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::TypeView::default())
  }
  pub fn elem_type_mut(&mut self) -> super::super::TypeMut<'_> {
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
  pub fn set_elem_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Type>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl ListType

impl ::std::ops::Drop for ListType {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ListType {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ListType {
  type Proxied = Self;
  fn as_view(&self) -> ListTypeView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ListType {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ListTypeMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ListType {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::super::Type as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::r#type::cel__expr__Type__ListType_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListType {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListType {
  type Msg = ListType;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListType> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListType {
  type Msg = ListType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListType> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListTypeMut<'_> {
  type Msg = ListType;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListType> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListTypeMut<'_> {
  type Msg = ListType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListType> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListTypeView<'_> {
  type Msg = ListType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListType> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListTypeMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut cel__expr__Type__MapType_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MapType {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MapType>
}

impl ::protobuf::Message for MapType {
  type MessageView<'msg> = MapTypeView<'msg>;
  type MessageMut<'msg> = MapTypeMut<'msg>;
}

impl ::std::default::Default for MapType {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MapType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MapType` is `Sync` because it does not implement interior mutability.
//    Neither does `MapTypeMut`.
unsafe impl ::std::marker::Sync for MapType {}

// SAFETY:
// - `MapType` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for MapType {}

impl ::protobuf::Proxied for MapType {
  type View<'msg> = MapTypeView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MapType {}

impl ::protobuf::MutProxied for MapType {
  type Mut<'msg> = MapTypeMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MapTypeView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MapType>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MapTypeView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MapTypeView<'msg> {
  type Message = MapType;
}

impl ::std::fmt::Debug for MapTypeView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MapTypeView<'_> {
  fn default() -> MapTypeView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MapType>> for MapTypeView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MapType>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MapTypeView<'msg> {

  pub fn to_owned(&self) -> MapType {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // key_type: optional message cel.expr.Type
  pub fn has_key_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn key_type_opt(self) -> ::std::option::Option<super::super::TypeView<'msg>> {
    self.has_key_type().then(|| self.key_type())
  }
  pub fn key_type(self) -> super::super::TypeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::TypeView::default())
  }

  // value_type: optional message cel.expr.Type
  pub fn has_value_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn value_type_opt(self) -> ::std::option::Option<super::super::TypeView<'msg>> {
    self.has_value_type().then(|| self.value_type())
  }
  pub fn value_type(self) -> super::super::TypeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::TypeView::default())
  }

}

// SAFETY:
// - `MapTypeView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for MapTypeView<'_> {}

// SAFETY:
// - `MapTypeView` is `Send` because while its alive a `MapTypeMut` cannot.
// - `MapTypeView` does not use thread-local data.
unsafe impl ::std::marker::Send for MapTypeView<'_> {}

impl<'msg> ::protobuf::AsView for MapTypeView<'msg> {
  type Proxied = MapType;
  fn as_view(&self) -> ::protobuf::View<'msg, MapType> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MapTypeView<'msg> {
  fn into_view<'shorter>(self) -> MapTypeView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MapType> for MapTypeView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MapType {
    let mut dst = MapType::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MapType> for MapTypeMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MapType {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for MapType {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MapTypeView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MapTypeMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MapTypeMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MapType>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MapTypeMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MapTypeMut<'msg> {
  type Message = MapType;
}

impl ::std::fmt::Debug for MapTypeMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MapType>> for MapTypeMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MapType>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MapTypeMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MapType> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> MapType {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // key_type: optional message cel.expr.Type
  pub fn has_key_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_key_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn key_type_opt(&self) -> ::std::option::Option<super::super::TypeView<'_>> {
    self.has_key_type().then(|| self.key_type())
  }
  pub fn key_type(&self) -> super::super::TypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::TypeView::default())
  }
  pub fn key_type_mut(&mut self) -> super::super::TypeMut<'_> {
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
  pub fn set_key_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Type>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // value_type: optional message cel.expr.Type
  pub fn has_value_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_value_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn value_type_opt(&self) -> ::std::option::Option<super::super::TypeView<'_>> {
    self.has_value_type().then(|| self.value_type())
  }
  pub fn value_type(&self) -> super::super::TypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::TypeView::default())
  }
  pub fn value_type_mut(&mut self) -> super::super::TypeMut<'_> {
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
  pub fn set_value_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Type>) {

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
// - `MapTypeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for MapTypeMut<'_> {}

// SAFETY:
// - `MapTypeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for MapTypeMut<'_> {}

impl<'msg> ::protobuf::AsView for MapTypeMut<'msg> {
  type Proxied = MapType;
  fn as_view(&self) -> ::protobuf::View<'_, MapType> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MapTypeMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MapType>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for MapTypeMut<'msg> {
  type MutProxied = MapType;
  fn as_mut(&mut self) -> MapTypeMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MapTypeMut<'msg> {
  fn into_mut<'shorter>(self) -> MapTypeMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MapType {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MapType> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MapTypeView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MapTypeMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // key_type: optional message cel.expr.Type
  pub fn has_key_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_key_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn key_type_opt(&self) -> ::std::option::Option<super::super::TypeView<'_>> {
    self.has_key_type().then(|| self.key_type())
  }
  pub fn key_type(&self) -> super::super::TypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::TypeView::default())
  }
  pub fn key_type_mut(&mut self) -> super::super::TypeMut<'_> {
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
  pub fn set_key_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Type>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // value_type: optional message cel.expr.Type
  pub fn has_value_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_value_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn value_type_opt(&self) -> ::std::option::Option<super::super::TypeView<'_>> {
    self.has_value_type().then(|| self.value_type())
  }
  pub fn value_type(&self) -> super::super::TypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::TypeView::default())
  }
  pub fn value_type_mut(&mut self) -> super::super::TypeMut<'_> {
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
  pub fn set_value_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Type>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl MapType

impl ::std::ops::Drop for MapType {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MapType {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MapType {
  type Proxied = Self;
  fn as_view(&self) -> MapTypeView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MapType {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MapTypeMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MapType {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::super::Type as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::r#type::cel__expr__Type__MapType_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MapType {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MapType {
  type Msg = MapType;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MapType> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MapType {
  type Msg = MapType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MapType> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MapTypeMut<'_> {
  type Msg = MapType;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MapType> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MapTypeMut<'_> {
  type Msg = MapType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MapType> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MapTypeView<'_> {
  type Msg = MapType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MapType> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MapTypeMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut cel__expr__Type__FunctionType_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct FunctionType {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<FunctionType>
}

impl ::protobuf::Message for FunctionType {
  type MessageView<'msg> = FunctionTypeView<'msg>;
  type MessageMut<'msg> = FunctionTypeMut<'msg>;
}

impl ::std::default::Default for FunctionType {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for FunctionType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `FunctionType` is `Sync` because it does not implement interior mutability.
//    Neither does `FunctionTypeMut`.
unsafe impl ::std::marker::Sync for FunctionType {}

// SAFETY:
// - `FunctionType` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for FunctionType {}

impl ::protobuf::Proxied for FunctionType {
  type View<'msg> = FunctionTypeView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for FunctionType {}

impl ::protobuf::MutProxied for FunctionType {
  type Mut<'msg> = FunctionTypeMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FunctionTypeView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FunctionType>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FunctionTypeView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FunctionTypeView<'msg> {
  type Message = FunctionType;
}

impl ::std::fmt::Debug for FunctionTypeView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FunctionTypeView<'_> {
  fn default() -> FunctionTypeView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, FunctionType>> for FunctionTypeView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FunctionType>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FunctionTypeView<'msg> {

  pub fn to_owned(&self) -> FunctionType {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // result_type: optional message cel.expr.Type
  pub fn has_result_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn result_type_opt(self) -> ::std::option::Option<super::super::TypeView<'msg>> {
    self.has_result_type().then(|| self.result_type())
  }
  pub fn result_type(self) -> super::super::TypeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::TypeView::default())
  }

  // arg_types: repeated message cel.expr.Type
  pub fn arg_types(self) -> ::protobuf::RepeatedView<'msg, super::super::Type> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::Type>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `FunctionTypeView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for FunctionTypeView<'_> {}

// SAFETY:
// - `FunctionTypeView` is `Send` because while its alive a `FunctionTypeMut` cannot.
// - `FunctionTypeView` does not use thread-local data.
unsafe impl ::std::marker::Send for FunctionTypeView<'_> {}

impl<'msg> ::protobuf::AsView for FunctionTypeView<'msg> {
  type Proxied = FunctionType;
  fn as_view(&self) -> ::protobuf::View<'msg, FunctionType> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FunctionTypeView<'msg> {
  fn into_view<'shorter>(self) -> FunctionTypeView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<FunctionType> for FunctionTypeView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FunctionType {
    let mut dst = FunctionType::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<FunctionType> for FunctionTypeMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FunctionType {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for FunctionType {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FunctionTypeView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FunctionTypeMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FunctionTypeMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FunctionType>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FunctionTypeMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FunctionTypeMut<'msg> {
  type Message = FunctionType;
}

impl ::std::fmt::Debug for FunctionTypeMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, FunctionType>> for FunctionTypeMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FunctionType>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FunctionTypeMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, FunctionType> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> FunctionType {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // result_type: optional message cel.expr.Type
  pub fn has_result_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_result_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn result_type_opt(&self) -> ::std::option::Option<super::super::TypeView<'_>> {
    self.has_result_type().then(|| self.result_type())
  }
  pub fn result_type(&self) -> super::super::TypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::TypeView::default())
  }
  pub fn result_type_mut(&mut self) -> super::super::TypeMut<'_> {
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
  pub fn set_result_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Type>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // arg_types: repeated message cel.expr.Type
  pub fn arg_types(&self) -> ::protobuf::RepeatedView<'_, super::super::Type> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::Type>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn arg_types_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::Type> {
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
  pub fn set_arg_types(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::Type>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `FunctionTypeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for FunctionTypeMut<'_> {}

// SAFETY:
// - `FunctionTypeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for FunctionTypeMut<'_> {}

impl<'msg> ::protobuf::AsView for FunctionTypeMut<'msg> {
  type Proxied = FunctionType;
  fn as_view(&self) -> ::protobuf::View<'_, FunctionType> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FunctionTypeMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, FunctionType>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for FunctionTypeMut<'msg> {
  type MutProxied = FunctionType;
  fn as_mut(&mut self) -> FunctionTypeMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FunctionTypeMut<'msg> {
  fn into_mut<'shorter>(self) -> FunctionTypeMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl FunctionType {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, FunctionType> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FunctionTypeView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FunctionTypeMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // result_type: optional message cel.expr.Type
  pub fn has_result_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_result_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn result_type_opt(&self) -> ::std::option::Option<super::super::TypeView<'_>> {
    self.has_result_type().then(|| self.result_type())
  }
  pub fn result_type(&self) -> super::super::TypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::TypeView::default())
  }
  pub fn result_type_mut(&mut self) -> super::super::TypeMut<'_> {
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
  pub fn set_result_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Type>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // arg_types: repeated message cel.expr.Type
  pub fn arg_types(&self) -> ::protobuf::RepeatedView<'_, super::super::Type> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::Type>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn arg_types_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::Type> {
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
  pub fn set_arg_types(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::Type>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl FunctionType

impl ::std::ops::Drop for FunctionType {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for FunctionType {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for FunctionType {
  type Proxied = Self;
  fn as_view(&self) -> FunctionTypeView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for FunctionType {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FunctionTypeMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FunctionType {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::super::Type as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::r#type::cel__expr__Type__FunctionType_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FunctionType {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FunctionType {
  type Msg = FunctionType;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FunctionType> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FunctionType {
  type Msg = FunctionType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FunctionType> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FunctionTypeMut<'_> {
  type Msg = FunctionType;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FunctionType> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FunctionTypeMut<'_> {
  type Msg = FunctionType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FunctionType> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FunctionTypeView<'_> {
  type Msg = FunctionType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FunctionType> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FunctionTypeMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut cel__expr__Type__AbstractType_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AbstractType {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AbstractType>
}

impl ::protobuf::Message for AbstractType {
  type MessageView<'msg> = AbstractTypeView<'msg>;
  type MessageMut<'msg> = AbstractTypeMut<'msg>;
}

impl ::std::default::Default for AbstractType {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AbstractType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AbstractType` is `Sync` because it does not implement interior mutability.
//    Neither does `AbstractTypeMut`.
unsafe impl ::std::marker::Sync for AbstractType {}

// SAFETY:
// - `AbstractType` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for AbstractType {}

impl ::protobuf::Proxied for AbstractType {
  type View<'msg> = AbstractTypeView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AbstractType {}

impl ::protobuf::MutProxied for AbstractType {
  type Mut<'msg> = AbstractTypeMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AbstractTypeView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AbstractType>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AbstractTypeView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AbstractTypeView<'msg> {
  type Message = AbstractType;
}

impl ::std::fmt::Debug for AbstractTypeView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AbstractTypeView<'_> {
  fn default() -> AbstractTypeView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AbstractType>> for AbstractTypeView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AbstractType>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AbstractTypeView<'msg> {

  pub fn to_owned(&self) -> AbstractType {
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

  // parameter_types: repeated message cel.expr.Type
  pub fn parameter_types(self) -> ::protobuf::RepeatedView<'msg, super::super::Type> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::Type>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `AbstractTypeView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AbstractTypeView<'_> {}

// SAFETY:
// - `AbstractTypeView` is `Send` because while its alive a `AbstractTypeMut` cannot.
// - `AbstractTypeView` does not use thread-local data.
unsafe impl ::std::marker::Send for AbstractTypeView<'_> {}

impl<'msg> ::protobuf::AsView for AbstractTypeView<'msg> {
  type Proxied = AbstractType;
  fn as_view(&self) -> ::protobuf::View<'msg, AbstractType> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AbstractTypeView<'msg> {
  fn into_view<'shorter>(self) -> AbstractTypeView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AbstractType> for AbstractTypeView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AbstractType {
    let mut dst = AbstractType::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AbstractType> for AbstractTypeMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AbstractType {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for AbstractType {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AbstractTypeView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AbstractTypeMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AbstractTypeMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AbstractType>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AbstractTypeMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AbstractTypeMut<'msg> {
  type Message = AbstractType;
}

impl ::std::fmt::Debug for AbstractTypeMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AbstractType>> for AbstractTypeMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AbstractType>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AbstractTypeMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AbstractType> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> AbstractType {
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

  // parameter_types: repeated message cel.expr.Type
  pub fn parameter_types(&self) -> ::protobuf::RepeatedView<'_, super::super::Type> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::Type>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn parameter_types_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::Type> {
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
  pub fn set_parameter_types(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::Type>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `AbstractTypeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AbstractTypeMut<'_> {}

// SAFETY:
// - `AbstractTypeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AbstractTypeMut<'_> {}

impl<'msg> ::protobuf::AsView for AbstractTypeMut<'msg> {
  type Proxied = AbstractType;
  fn as_view(&self) -> ::protobuf::View<'_, AbstractType> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AbstractTypeMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AbstractType>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AbstractTypeMut<'msg> {
  type MutProxied = AbstractType;
  fn as_mut(&mut self) -> AbstractTypeMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AbstractTypeMut<'msg> {
  fn into_mut<'shorter>(self) -> AbstractTypeMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AbstractType {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AbstractType> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AbstractTypeView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AbstractTypeMut<'_> {
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

  // parameter_types: repeated message cel.expr.Type
  pub fn parameter_types(&self) -> ::protobuf::RepeatedView<'_, super::super::Type> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::Type>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn parameter_types_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::Type> {
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
  pub fn set_parameter_types(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::Type>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl AbstractType

impl ::std::ops::Drop for AbstractType {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AbstractType {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AbstractType {
  type Proxied = Self;
  fn as_view(&self) -> AbstractTypeView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AbstractType {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AbstractTypeMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AbstractType {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::super::Type as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::r#type::cel__expr__Type__AbstractType_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AbstractType {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AbstractType {
  type Msg = AbstractType;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AbstractType> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AbstractType {
  type Msg = AbstractType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AbstractType> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AbstractTypeMut<'_> {
  type Msg = AbstractType;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AbstractType> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AbstractTypeMut<'_> {
  type Msg = AbstractType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AbstractType> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AbstractTypeView<'_> {
  type Msg = AbstractType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AbstractType> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AbstractTypeMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PrimitiveType(i32);

#[allow(non_upper_case_globals)]
impl PrimitiveType {
  pub const Unspecified: PrimitiveType = PrimitiveType(0);
  pub const Bool: PrimitiveType = PrimitiveType(1);
  pub const Int64: PrimitiveType = PrimitiveType(2);
  pub const Uint64: PrimitiveType = PrimitiveType(3);
  pub const Double: PrimitiveType = PrimitiveType(4);
  pub const String: PrimitiveType = PrimitiveType(5);
  pub const Bytes: PrimitiveType = PrimitiveType(6);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unspecified",
      1 => "Bool",
      2 => "Int64",
      3 => "Uint64",
      4 => "Double",
      5 => "String",
      6 => "Bytes",
      _ => return None
    })
  }
}

impl ::std::convert::From<PrimitiveType> for i32 {
  fn from(val: PrimitiveType) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for PrimitiveType {
  fn from(val: i32) -> PrimitiveType {
    Self(val)
  }
}

impl ::std::default::Default for PrimitiveType {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for PrimitiveType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "PrimitiveType::{}", constant_name)
    } else {
      write!(f, "PrimitiveType::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for PrimitiveType {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for PrimitiveType {}

impl ::protobuf::Proxied for PrimitiveType {
  type View<'a> = PrimitiveType;
}

impl ::protobuf::AsView for PrimitiveType {
  type Proxied = PrimitiveType;

  fn as_view(&self) -> PrimitiveType {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PrimitiveType {
  fn into_view<'shorter>(self) -> PrimitiveType where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for PrimitiveType {
  const NAME: &'static str = "PrimitiveType";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4|5|6)
  }
}

impl ::protobuf::__internal::EntityType for PrimitiveType {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WellKnownType(i32);

#[allow(non_upper_case_globals)]
impl WellKnownType {
  pub const Unspecified: WellKnownType = WellKnownType(0);
  pub const Any: WellKnownType = WellKnownType(1);
  pub const Timestamp: WellKnownType = WellKnownType(2);
  pub const Duration: WellKnownType = WellKnownType(3);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unspecified",
      1 => "Any",
      2 => "Timestamp",
      3 => "Duration",
      _ => return None
    })
  }
}

impl ::std::convert::From<WellKnownType> for i32 {
  fn from(val: WellKnownType) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for WellKnownType {
  fn from(val: i32) -> WellKnownType {
    Self(val)
  }
}

impl ::std::default::Default for WellKnownType {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for WellKnownType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "WellKnownType::{}", constant_name)
    } else {
      write!(f, "WellKnownType::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for WellKnownType {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for WellKnownType {}

impl ::protobuf::Proxied for WellKnownType {
  type View<'a> = WellKnownType;
}

impl ::protobuf::AsView for WellKnownType {
  type Proxied = WellKnownType;

  fn as_view(&self) -> WellKnownType {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for WellKnownType {
  fn into_view<'shorter>(self) -> WellKnownType where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for WellKnownType {
  const NAME: &'static str = "WellKnownType";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3)
  }
}

impl ::protobuf::__internal::EntityType for WellKnownType {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum TypeKindOneof<'msg> {
  Dyn(::protobuf::View<'msg, ::protobuf_well_known_types::Empty>) = 1,
  Null(::protobuf::View<'msg, ::protobuf_well_known_types::NullValue>) = 2,
  Primitive(::protobuf::View<'msg, super::super::r#type::PrimitiveType>) = 3,
  Wrapper(::protobuf::View<'msg, super::super::r#type::PrimitiveType>) = 4,
  WellKnown(::protobuf::View<'msg, super::super::r#type::WellKnownType>) = 5,
  ListType(::protobuf::View<'msg, super::super::r#type::ListType>) = 6,
  MapType(::protobuf::View<'msg, super::super::r#type::MapType>) = 7,
  Function(::protobuf::View<'msg, super::super::r#type::FunctionType>) = 8,
  MessageType(&'msg ::protobuf::ProtoStr) = 9,
  TypeParam(&'msg ::protobuf::ProtoStr) = 10,
  Type(::protobuf::View<'msg, super::super::Type>) = 11,
  Error(::protobuf::View<'msg, ::protobuf_well_known_types::Empty>) = 12,
  AbstractType(::protobuf::View<'msg, super::super::r#type::AbstractType>) = 14,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum TypeKindCase {
  Dyn = 1,
  Null = 2,
  Primitive = 3,
  Wrapper = 4,
  WellKnown = 5,
  ListType = 6,
  MapType = 7,
  Function = 8,
  MessageType = 9,
  TypeParam = 10,
  Type = 11,
  Error = 12,
  AbstractType = 14,

  not_set = 0
}

impl TypeKindCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<TypeKindCase> {
    match v {
      0 => Some(TypeKindCase::not_set),
      1 => Some(TypeKindCase::Dyn),
      2 => Some(TypeKindCase::Null),
      3 => Some(TypeKindCase::Primitive),
      4 => Some(TypeKindCase::Wrapper),
      5 => Some(TypeKindCase::WellKnown),
      6 => Some(TypeKindCase::ListType),
      7 => Some(TypeKindCase::MapType),
      8 => Some(TypeKindCase::Function),
      9 => Some(TypeKindCase::MessageType),
      10 => Some(TypeKindCase::TypeParam),
      11 => Some(TypeKindCase::Type),
      12 => Some(TypeKindCase::Error),
      14 => Some(TypeKindCase::AbstractType),
      _ => None
    }
  }
}
}  // pub mod r#type


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut cel__expr__Decl_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Decl {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Decl>
}

impl ::protobuf::Message for Decl {
  type MessageView<'msg> = DeclView<'msg>;
  type MessageMut<'msg> = DeclMut<'msg>;
}

impl ::std::default::Default for Decl {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Decl {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Decl` is `Sync` because it does not implement interior mutability.
//    Neither does `DeclMut`.
unsafe impl ::std::marker::Sync for Decl {}

// SAFETY:
// - `Decl` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Decl {}

impl ::protobuf::Proxied for Decl {
  type View<'msg> = DeclView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Decl {}

impl ::protobuf::MutProxied for Decl {
  type Mut<'msg> = DeclMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DeclView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Decl>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DeclView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DeclView<'msg> {
  type Message = Decl;
}

impl ::std::fmt::Debug for DeclView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DeclView<'_> {
  fn default() -> DeclView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Decl>> for DeclView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Decl>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DeclView<'msg> {

  pub fn to_owned(&self) -> Decl {
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

  // ident: optional message cel.expr.Decl.IdentDecl
  pub fn has_ident(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn ident_opt(self) -> ::std::option::Option<super::decl::IdentDeclView<'msg>> {
    self.has_ident().then(|| self.ident())
  }
  pub fn ident(self) -> super::decl::IdentDeclView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::decl::IdentDeclView::default())
  }

  // function: optional message cel.expr.Decl.FunctionDecl
  pub fn has_function(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn function_opt(self) -> ::std::option::Option<super::decl::FunctionDeclView<'msg>> {
    self.has_function().then(|| self.function())
  }
  pub fn function(self) -> super::decl::FunctionDeclView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::decl::FunctionDeclView::default())
  }

  pub fn decl_kind(self) -> super::decl::DeclKindOneof<'msg> {
    match self.decl_kind_case() {
      super::decl::DeclKindCase::Ident =>
          super::decl::DeclKindOneof::Ident(self.ident()),
      super::decl::DeclKindCase::Function =>
          super::decl::DeclKindOneof::Function(self.function()),
      _ => super::decl::DeclKindOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn decl_kind_case(self) -> super::decl::DeclKindCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::decl::DeclKindCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `DeclView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DeclView<'_> {}

// SAFETY:
// - `DeclView` is `Send` because while its alive a `DeclMut` cannot.
// - `DeclView` does not use thread-local data.
unsafe impl ::std::marker::Send for DeclView<'_> {}

impl<'msg> ::protobuf::AsView for DeclView<'msg> {
  type Proxied = Decl;
  fn as_view(&self) -> ::protobuf::View<'msg, Decl> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DeclView<'msg> {
  fn into_view<'shorter>(self) -> DeclView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Decl> for DeclView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Decl {
    let mut dst = Decl::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Decl> for DeclMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Decl {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Decl {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DeclView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DeclMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DeclMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Decl>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DeclMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DeclMut<'msg> {
  type Message = Decl;
}

impl ::std::fmt::Debug for DeclMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Decl>> for DeclMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Decl>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DeclMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Decl> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Decl {
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

  // ident: optional message cel.expr.Decl.IdentDecl
  pub fn has_ident(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_ident(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn ident_opt(&self) -> ::std::option::Option<super::decl::IdentDeclView<'_>> {
    self.has_ident().then(|| self.ident())
  }
  pub fn ident(&self) -> super::decl::IdentDeclView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::decl::IdentDeclView::default())
  }
  pub fn ident_mut(&mut self) -> super::decl::IdentDeclMut<'_> {
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
  pub fn set_ident(&mut self,
    val: impl ::protobuf::IntoProxied<super::decl::IdentDecl>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // function: optional message cel.expr.Decl.FunctionDecl
  pub fn has_function(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_function(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn function_opt(&self) -> ::std::option::Option<super::decl::FunctionDeclView<'_>> {
    self.has_function().then(|| self.function())
  }
  pub fn function(&self) -> super::decl::FunctionDeclView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::decl::FunctionDeclView::default())
  }
  pub fn function_mut(&mut self) -> super::decl::FunctionDeclMut<'_> {
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
  pub fn set_function(&mut self,
    val: impl ::protobuf::IntoProxied<super::decl::FunctionDecl>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn decl_kind(&self) -> super::decl::DeclKindOneof<'_> {
    match &self.decl_kind_case() {
      super::decl::DeclKindCase::Ident =>
          super::decl::DeclKindOneof::Ident(self.ident()),
      super::decl::DeclKindCase::Function =>
          super::decl::DeclKindOneof::Function(self.function()),
      _ => super::decl::DeclKindOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn decl_kind_case(&self) -> super::decl::DeclKindCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::decl::DeclKindCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `DeclMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DeclMut<'_> {}

// SAFETY:
// - `DeclMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DeclMut<'_> {}

impl<'msg> ::protobuf::AsView for DeclMut<'msg> {
  type Proxied = Decl;
  fn as_view(&self) -> ::protobuf::View<'_, Decl> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DeclMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Decl>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DeclMut<'msg> {
  type MutProxied = Decl;
  fn as_mut(&mut self) -> DeclMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DeclMut<'msg> {
  fn into_mut<'shorter>(self) -> DeclMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Decl {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Decl> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DeclView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DeclMut<'_> {
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

  // ident: optional message cel.expr.Decl.IdentDecl
  pub fn has_ident(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_ident(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn ident_opt(&self) -> ::std::option::Option<super::decl::IdentDeclView<'_>> {
    self.has_ident().then(|| self.ident())
  }
  pub fn ident(&self) -> super::decl::IdentDeclView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::decl::IdentDeclView::default())
  }
  pub fn ident_mut(&mut self) -> super::decl::IdentDeclMut<'_> {
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
  pub fn set_ident(&mut self,
    val: impl ::protobuf::IntoProxied<super::decl::IdentDecl>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // function: optional message cel.expr.Decl.FunctionDecl
  pub fn has_function(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_function(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn function_opt(&self) -> ::std::option::Option<super::decl::FunctionDeclView<'_>> {
    self.has_function().then(|| self.function())
  }
  pub fn function(&self) -> super::decl::FunctionDeclView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::decl::FunctionDeclView::default())
  }
  pub fn function_mut(&mut self) -> super::decl::FunctionDeclMut<'_> {
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
  pub fn set_function(&mut self,
    val: impl ::protobuf::IntoProxied<super::decl::FunctionDecl>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn decl_kind(&self) -> super::decl::DeclKindOneof<'_> {
    match &self.decl_kind_case() {
      super::decl::DeclKindCase::Ident =>
          super::decl::DeclKindOneof::Ident(self.ident()),
      super::decl::DeclKindCase::Function =>
          super::decl::DeclKindOneof::Function(self.function()),
      _ => super::decl::DeclKindOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn decl_kind_case(&self) -> super::decl::DeclKindCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::decl::DeclKindCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl Decl

impl ::std::ops::Drop for Decl {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Decl {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Decl {
  type Proxied = Self;
  fn as_view(&self) -> DeclView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Decl {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DeclMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Decl {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::cel__expr__Decl_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X33^#|$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::cel__expr__Decl_msg_init.0, &[<super::decl::IdentDecl as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::decl::FunctionDecl as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::cel__expr__Decl_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Decl {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Decl {
  type Msg = Decl;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Decl> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Decl {
  type Msg = Decl;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Decl> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DeclMut<'_> {
  type Msg = Decl;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Decl> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeclMut<'_> {
  type Msg = Decl;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Decl> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeclView<'_> {
  type Msg = Decl;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Decl> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DeclMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod decl {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut cel__expr__Decl__IdentDecl_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct IdentDecl {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<IdentDecl>
}

impl ::protobuf::Message for IdentDecl {
  type MessageView<'msg> = IdentDeclView<'msg>;
  type MessageMut<'msg> = IdentDeclMut<'msg>;
}

impl ::std::default::Default for IdentDecl {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for IdentDecl {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `IdentDecl` is `Sync` because it does not implement interior mutability.
//    Neither does `IdentDeclMut`.
unsafe impl ::std::marker::Sync for IdentDecl {}

// SAFETY:
// - `IdentDecl` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for IdentDecl {}

impl ::protobuf::Proxied for IdentDecl {
  type View<'msg> = IdentDeclView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for IdentDecl {}

impl ::protobuf::MutProxied for IdentDecl {
  type Mut<'msg> = IdentDeclMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct IdentDeclView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, IdentDecl>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for IdentDeclView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for IdentDeclView<'msg> {
  type Message = IdentDecl;
}

impl ::std::fmt::Debug for IdentDeclView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for IdentDeclView<'_> {
  fn default() -> IdentDeclView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, IdentDecl>> for IdentDeclView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, IdentDecl>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> IdentDeclView<'msg> {

  pub fn to_owned(&self) -> IdentDecl {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // type: optional message cel.expr.Type
  pub fn has_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn type_opt(self) -> ::std::option::Option<super::super::TypeView<'msg>> {
    self.has_type().then(|| self.r#type())
  }
  pub fn r#type(self) -> super::super::TypeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::TypeView::default())
  }

  // value: optional message cel.expr.Constant
  pub fn has_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn value_opt(self) -> ::std::option::Option<crate::xds::generated::cel::expr::syntax::ConstantView<'msg>> {
    self.has_value().then(|| self.value())
  }
  pub fn value(self) -> crate::xds::generated::cel::expr::syntax::ConstantView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::cel::expr::syntax::ConstantView::default())
  }

  // doc: optional string
  pub fn doc(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `IdentDeclView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for IdentDeclView<'_> {}

// SAFETY:
// - `IdentDeclView` is `Send` because while its alive a `IdentDeclMut` cannot.
// - `IdentDeclView` does not use thread-local data.
unsafe impl ::std::marker::Send for IdentDeclView<'_> {}

impl<'msg> ::protobuf::AsView for IdentDeclView<'msg> {
  type Proxied = IdentDecl;
  fn as_view(&self) -> ::protobuf::View<'msg, IdentDecl> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for IdentDeclView<'msg> {
  fn into_view<'shorter>(self) -> IdentDeclView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<IdentDecl> for IdentDeclView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> IdentDecl {
    let mut dst = IdentDecl::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<IdentDecl> for IdentDeclMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> IdentDecl {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for IdentDecl {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for IdentDeclView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for IdentDeclMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct IdentDeclMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, IdentDecl>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for IdentDeclMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for IdentDeclMut<'msg> {
  type Message = IdentDecl;
}

impl ::std::fmt::Debug for IdentDeclMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, IdentDecl>> for IdentDeclMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, IdentDecl>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> IdentDeclMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, IdentDecl> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> IdentDecl {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // type: optional message cel.expr.Type
  pub fn has_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn type_opt(&self) -> ::std::option::Option<super::super::TypeView<'_>> {
    self.has_type().then(|| self.r#type())
  }
  pub fn r#type(&self) -> super::super::TypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::TypeView::default())
  }
  pub fn type_mut(&mut self) -> super::super::TypeMut<'_> {
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
  pub fn set_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Type>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // value: optional message cel.expr.Constant
  pub fn has_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn value_opt(&self) -> ::std::option::Option<crate::xds::generated::cel::expr::syntax::ConstantView<'_>> {
    self.has_value().then(|| self.value())
  }
  pub fn value(&self) -> crate::xds::generated::cel::expr::syntax::ConstantView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::cel::expr::syntax::ConstantView::default())
  }
  pub fn value_mut(&mut self) -> crate::xds::generated::cel::expr::syntax::ConstantMut<'_> {
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
  pub fn set_value(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::cel::expr::syntax::Constant>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // doc: optional string
  pub fn doc(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_doc(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}

// SAFETY:
// - `IdentDeclMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for IdentDeclMut<'_> {}

// SAFETY:
// - `IdentDeclMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for IdentDeclMut<'_> {}

impl<'msg> ::protobuf::AsView for IdentDeclMut<'msg> {
  type Proxied = IdentDecl;
  fn as_view(&self) -> ::protobuf::View<'_, IdentDecl> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for IdentDeclMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, IdentDecl>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for IdentDeclMut<'msg> {
  type MutProxied = IdentDecl;
  fn as_mut(&mut self) -> IdentDeclMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for IdentDeclMut<'msg> {
  fn into_mut<'shorter>(self) -> IdentDeclMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl IdentDecl {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, IdentDecl> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> IdentDeclView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> IdentDeclMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // type: optional message cel.expr.Type
  pub fn has_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn type_opt(&self) -> ::std::option::Option<super::super::TypeView<'_>> {
    self.has_type().then(|| self.r#type())
  }
  pub fn r#type(&self) -> super::super::TypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::TypeView::default())
  }
  pub fn type_mut(&mut self) -> super::super::TypeMut<'_> {
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
  pub fn set_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::Type>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // value: optional message cel.expr.Constant
  pub fn has_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn value_opt(&self) -> ::std::option::Option<crate::xds::generated::cel::expr::syntax::ConstantView<'_>> {
    self.has_value().then(|| self.value())
  }
  pub fn value(&self) -> crate::xds::generated::cel::expr::syntax::ConstantView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::cel::expr::syntax::ConstantView::default())
  }
  pub fn value_mut(&mut self) -> crate::xds::generated::cel::expr::syntax::ConstantMut<'_> {
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
  pub fn set_value(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::cel::expr::syntax::Constant>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // doc: optional string
  pub fn doc(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_doc(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}  // impl IdentDecl

impl ::std::ops::Drop for IdentDecl {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for IdentDecl {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for IdentDecl {
  type Proxied = Self;
  fn as_view(&self) -> IdentDeclView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for IdentDecl {
  type MutProxied = Self;
  fn as_mut(&mut self) -> IdentDeclMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for IdentDecl {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::decl::cel__expr__Decl__IdentDecl_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$331X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::decl::cel__expr__Decl__IdentDecl_msg_init.0, &[<super::super::Type as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::cel::expr::syntax::Constant as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::decl::cel__expr__Decl__IdentDecl_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for IdentDecl {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for IdentDecl {
  type Msg = IdentDecl;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<IdentDecl> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for IdentDecl {
  type Msg = IdentDecl;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<IdentDecl> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for IdentDeclMut<'_> {
  type Msg = IdentDecl;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<IdentDecl> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for IdentDeclMut<'_> {
  type Msg = IdentDecl;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<IdentDecl> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for IdentDeclView<'_> {
  type Msg = IdentDecl;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<IdentDecl> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for IdentDeclMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut cel__expr__Decl__FunctionDecl_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct FunctionDecl {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<FunctionDecl>
}

impl ::protobuf::Message for FunctionDecl {
  type MessageView<'msg> = FunctionDeclView<'msg>;
  type MessageMut<'msg> = FunctionDeclMut<'msg>;
}

impl ::std::default::Default for FunctionDecl {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for FunctionDecl {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `FunctionDecl` is `Sync` because it does not implement interior mutability.
//    Neither does `FunctionDeclMut`.
unsafe impl ::std::marker::Sync for FunctionDecl {}

// SAFETY:
// - `FunctionDecl` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for FunctionDecl {}

impl ::protobuf::Proxied for FunctionDecl {
  type View<'msg> = FunctionDeclView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for FunctionDecl {}

impl ::protobuf::MutProxied for FunctionDecl {
  type Mut<'msg> = FunctionDeclMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FunctionDeclView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FunctionDecl>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FunctionDeclView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FunctionDeclView<'msg> {
  type Message = FunctionDecl;
}

impl ::std::fmt::Debug for FunctionDeclView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FunctionDeclView<'_> {
  fn default() -> FunctionDeclView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, FunctionDecl>> for FunctionDeclView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FunctionDecl>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FunctionDeclView<'msg> {

  pub fn to_owned(&self) -> FunctionDecl {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // overloads: repeated message cel.expr.Decl.FunctionDecl.Overload
  pub fn overloads(self) -> ::protobuf::RepeatedView<'msg, super::super::decl::function_decl::Overload> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::decl::function_decl::Overload>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `FunctionDeclView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for FunctionDeclView<'_> {}

// SAFETY:
// - `FunctionDeclView` is `Send` because while its alive a `FunctionDeclMut` cannot.
// - `FunctionDeclView` does not use thread-local data.
unsafe impl ::std::marker::Send for FunctionDeclView<'_> {}

impl<'msg> ::protobuf::AsView for FunctionDeclView<'msg> {
  type Proxied = FunctionDecl;
  fn as_view(&self) -> ::protobuf::View<'msg, FunctionDecl> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FunctionDeclView<'msg> {
  fn into_view<'shorter>(self) -> FunctionDeclView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<FunctionDecl> for FunctionDeclView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FunctionDecl {
    let mut dst = FunctionDecl::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<FunctionDecl> for FunctionDeclMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FunctionDecl {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for FunctionDecl {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FunctionDeclView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FunctionDeclMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FunctionDeclMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FunctionDecl>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FunctionDeclMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FunctionDeclMut<'msg> {
  type Message = FunctionDecl;
}

impl ::std::fmt::Debug for FunctionDeclMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, FunctionDecl>> for FunctionDeclMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FunctionDecl>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FunctionDeclMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, FunctionDecl> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> FunctionDecl {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // overloads: repeated message cel.expr.Decl.FunctionDecl.Overload
  pub fn overloads(&self) -> ::protobuf::RepeatedView<'_, super::super::decl::function_decl::Overload> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::decl::function_decl::Overload>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn overloads_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::decl::function_decl::Overload> {
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
  pub fn set_overloads(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::decl::function_decl::Overload>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `FunctionDeclMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for FunctionDeclMut<'_> {}

// SAFETY:
// - `FunctionDeclMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for FunctionDeclMut<'_> {}

impl<'msg> ::protobuf::AsView for FunctionDeclMut<'msg> {
  type Proxied = FunctionDecl;
  fn as_view(&self) -> ::protobuf::View<'_, FunctionDecl> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FunctionDeclMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, FunctionDecl>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for FunctionDeclMut<'msg> {
  type MutProxied = FunctionDecl;
  fn as_mut(&mut self) -> FunctionDeclMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FunctionDeclMut<'msg> {
  fn into_mut<'shorter>(self) -> FunctionDeclMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl FunctionDecl {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, FunctionDecl> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FunctionDeclView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FunctionDeclMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // overloads: repeated message cel.expr.Decl.FunctionDecl.Overload
  pub fn overloads(&self) -> ::protobuf::RepeatedView<'_, super::super::decl::function_decl::Overload> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::decl::function_decl::Overload>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn overloads_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::decl::function_decl::Overload> {
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
  pub fn set_overloads(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::decl::function_decl::Overload>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl FunctionDecl

impl ::std::ops::Drop for FunctionDecl {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for FunctionDecl {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for FunctionDecl {
  type Proxied = Self;
  fn as_view(&self) -> FunctionDeclView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for FunctionDecl {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FunctionDeclMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FunctionDecl {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::decl::cel__expr__Decl__FunctionDecl_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::decl::cel__expr__Decl__FunctionDecl_msg_init.0, &[<super::super::decl::function_decl::Overload as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::decl::cel__expr__Decl__FunctionDecl_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FunctionDecl {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FunctionDecl {
  type Msg = FunctionDecl;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FunctionDecl> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FunctionDecl {
  type Msg = FunctionDecl;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FunctionDecl> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FunctionDeclMut<'_> {
  type Msg = FunctionDecl;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FunctionDecl> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FunctionDeclMut<'_> {
  type Msg = FunctionDecl;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FunctionDecl> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FunctionDeclView<'_> {
  type Msg = FunctionDecl;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FunctionDecl> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FunctionDeclMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod function_decl {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut cel__expr__Decl__FunctionDecl__Overload_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Overload {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Overload>
}

impl ::protobuf::Message for Overload {
  type MessageView<'msg> = OverloadView<'msg>;
  type MessageMut<'msg> = OverloadMut<'msg>;
}

impl ::std::default::Default for Overload {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Overload {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Overload` is `Sync` because it does not implement interior mutability.
//    Neither does `OverloadMut`.
unsafe impl ::std::marker::Sync for Overload {}

// SAFETY:
// - `Overload` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Overload {}

impl ::protobuf::Proxied for Overload {
  type View<'msg> = OverloadView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Overload {}

impl ::protobuf::MutProxied for Overload {
  type Mut<'msg> = OverloadMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct OverloadView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Overload>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OverloadView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for OverloadView<'msg> {
  type Message = Overload;
}

impl ::std::fmt::Debug for OverloadView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for OverloadView<'_> {
  fn default() -> OverloadView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Overload>> for OverloadView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Overload>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OverloadView<'msg> {

  pub fn to_owned(&self) -> Overload {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // overload_id: optional string
  pub fn overload_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // params: repeated message cel.expr.Type
  pub fn params(self) -> ::protobuf::RepeatedView<'msg, super::super::super::Type> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::super::Type>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // type_params: repeated string
  pub fn type_params(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // result_type: optional message cel.expr.Type
  pub fn has_result_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn result_type_opt(self) -> ::std::option::Option<super::super::super::TypeView<'msg>> {
    self.has_result_type().then(|| self.result_type())
  }
  pub fn result_type(self) -> super::super::super::TypeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::TypeView::default())
  }

  // is_instance_function: optional bool
  pub fn is_instance_function(self) -> bool {
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

  // doc: optional string
  pub fn doc(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `OverloadView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for OverloadView<'_> {}

// SAFETY:
// - `OverloadView` is `Send` because while its alive a `OverloadMut` cannot.
// - `OverloadView` does not use thread-local data.
unsafe impl ::std::marker::Send for OverloadView<'_> {}

impl<'msg> ::protobuf::AsView for OverloadView<'msg> {
  type Proxied = Overload;
  fn as_view(&self) -> ::protobuf::View<'msg, Overload> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OverloadView<'msg> {
  fn into_view<'shorter>(self) -> OverloadView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Overload> for OverloadView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Overload {
    let mut dst = Overload::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Overload> for OverloadMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Overload {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Overload {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for OverloadView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for OverloadMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct OverloadMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Overload>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OverloadMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for OverloadMut<'msg> {
  type Message = Overload;
}

impl ::std::fmt::Debug for OverloadMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Overload>> for OverloadMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Overload>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OverloadMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Overload> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Overload {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // overload_id: optional string
  pub fn overload_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_overload_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // params: repeated message cel.expr.Type
  pub fn params(&self) -> ::protobuf::RepeatedView<'_, super::super::super::Type> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::super::Type>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn params_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::super::Type> {
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
  pub fn set_params(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::super::Type>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // type_params: repeated string
  pub fn type_params(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn type_params_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_type_params(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // result_type: optional message cel.expr.Type
  pub fn has_result_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_result_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn result_type_opt(&self) -> ::std::option::Option<super::super::super::TypeView<'_>> {
    self.has_result_type().then(|| self.result_type())
  }
  pub fn result_type(&self) -> super::super::super::TypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::TypeView::default())
  }
  pub fn result_type_mut(&mut self) -> super::super::super::TypeMut<'_> {
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
  pub fn set_result_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::Type>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // is_instance_function: optional bool
  pub fn is_instance_function(&self) -> bool {
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
  pub fn set_is_instance_function(&mut self, val: bool) {
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

  // doc: optional string
  pub fn doc(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_doc(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

}

// SAFETY:
// - `OverloadMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for OverloadMut<'_> {}

// SAFETY:
// - `OverloadMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for OverloadMut<'_> {}

impl<'msg> ::protobuf::AsView for OverloadMut<'msg> {
  type Proxied = Overload;
  fn as_view(&self) -> ::protobuf::View<'_, Overload> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OverloadMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Overload>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for OverloadMut<'msg> {
  type MutProxied = Overload;
  fn as_mut(&mut self) -> OverloadMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for OverloadMut<'msg> {
  fn into_mut<'shorter>(self) -> OverloadMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Overload {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Overload> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> OverloadView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> OverloadMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // overload_id: optional string
  pub fn overload_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_overload_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // params: repeated message cel.expr.Type
  pub fn params(&self) -> ::protobuf::RepeatedView<'_, super::super::super::Type> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::super::Type>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn params_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::super::Type> {
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
  pub fn set_params(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::super::Type>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // type_params: repeated string
  pub fn type_params(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn type_params_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_type_params(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // result_type: optional message cel.expr.Type
  pub fn has_result_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_result_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn result_type_opt(&self) -> ::std::option::Option<super::super::super::TypeView<'_>> {
    self.has_result_type().then(|| self.result_type())
  }
  pub fn result_type(&self) -> super::super::super::TypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::TypeView::default())
  }
  pub fn result_type_mut(&mut self) -> super::super::super::TypeMut<'_> {
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
  pub fn set_result_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::Type>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // is_instance_function: optional bool
  pub fn is_instance_function(&self) -> bool {
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
  pub fn set_is_instance_function(&mut self, val: bool) {
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

  // doc: optional string
  pub fn doc(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_doc(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

}  // impl Overload

impl ::std::ops::Drop for Overload {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Overload {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Overload {
  type Proxied = Self;
  fn as_view(&self) -> OverloadView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Overload {
  type MutProxied = Self;
  fn as_mut(&mut self) -> OverloadMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Overload {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::decl::function_decl::cel__expr__Decl__FunctionDecl__Overload_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1XGET3/P1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::decl::function_decl::cel__expr__Decl__FunctionDecl__Overload_msg_init.0, &[<super::super::super::Type as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::super::Type as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::decl::function_decl::cel__expr__Decl__FunctionDecl__Overload_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Overload {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Overload {
  type Msg = Overload;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Overload> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Overload {
  type Msg = Overload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Overload> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for OverloadMut<'_> {
  type Msg = Overload;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Overload> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OverloadMut<'_> {
  type Msg = Overload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Overload> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OverloadView<'_> {
  type Msg = Overload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Overload> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for OverloadMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod function_decl


#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum DeclKindOneof<'msg> {
  Ident(::protobuf::View<'msg, super::super::decl::IdentDecl>) = 2,
  Function(::protobuf::View<'msg, super::super::decl::FunctionDecl>) = 3,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum DeclKindCase {
  Ident = 2,
  Function = 3,

  not_set = 0
}

impl DeclKindCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<DeclKindCase> {
    match v {
      0 => Some(DeclKindCase::not_set),
      2 => Some(DeclKindCase::Ident),
      3 => Some(DeclKindCase::Function),
      _ => None
    }
  }
}
}  // pub mod decl


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut cel__expr__Reference_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Reference {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Reference>
}

impl ::protobuf::Message for Reference {
  type MessageView<'msg> = ReferenceView<'msg>;
  type MessageMut<'msg> = ReferenceMut<'msg>;
}

impl ::std::default::Default for Reference {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Reference {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Reference` is `Sync` because it does not implement interior mutability.
//    Neither does `ReferenceMut`.
unsafe impl ::std::marker::Sync for Reference {}

// SAFETY:
// - `Reference` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Reference {}

impl ::protobuf::Proxied for Reference {
  type View<'msg> = ReferenceView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Reference {}

impl ::protobuf::MutProxied for Reference {
  type Mut<'msg> = ReferenceMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ReferenceView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Reference>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ReferenceView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ReferenceView<'msg> {
  type Message = Reference;
}

impl ::std::fmt::Debug for ReferenceView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ReferenceView<'_> {
  fn default() -> ReferenceView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Reference>> for ReferenceView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Reference>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ReferenceView<'msg> {

  pub fn to_owned(&self) -> Reference {
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

  // overload_id: repeated string
  pub fn overload_id(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // value: optional message cel.expr.Constant
  pub fn has_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn value_opt(self) -> ::std::option::Option<crate::xds::generated::cel::expr::syntax::ConstantView<'msg>> {
    self.has_value().then(|| self.value())
  }
  pub fn value(self) -> crate::xds::generated::cel::expr::syntax::ConstantView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::cel::expr::syntax::ConstantView::default())
  }

}

// SAFETY:
// - `ReferenceView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ReferenceView<'_> {}

// SAFETY:
// - `ReferenceView` is `Send` because while its alive a `ReferenceMut` cannot.
// - `ReferenceView` does not use thread-local data.
unsafe impl ::std::marker::Send for ReferenceView<'_> {}

impl<'msg> ::protobuf::AsView for ReferenceView<'msg> {
  type Proxied = Reference;
  fn as_view(&self) -> ::protobuf::View<'msg, Reference> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ReferenceView<'msg> {
  fn into_view<'shorter>(self) -> ReferenceView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Reference> for ReferenceView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Reference {
    let mut dst = Reference::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Reference> for ReferenceMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Reference {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Reference {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ReferenceView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ReferenceMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ReferenceMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Reference>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ReferenceMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ReferenceMut<'msg> {
  type Message = Reference;
}

impl ::std::fmt::Debug for ReferenceMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Reference>> for ReferenceMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Reference>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ReferenceMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Reference> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Reference {
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

  // overload_id: repeated string
  pub fn overload_id(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn overload_id_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_overload_id(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // value: optional message cel.expr.Constant
  pub fn has_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn value_opt(&self) -> ::std::option::Option<crate::xds::generated::cel::expr::syntax::ConstantView<'_>> {
    self.has_value().then(|| self.value())
  }
  pub fn value(&self) -> crate::xds::generated::cel::expr::syntax::ConstantView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::cel::expr::syntax::ConstantView::default())
  }
  pub fn value_mut(&mut self) -> crate::xds::generated::cel::expr::syntax::ConstantMut<'_> {
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
  pub fn set_value(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::cel::expr::syntax::Constant>) {

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
// - `ReferenceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ReferenceMut<'_> {}

// SAFETY:
// - `ReferenceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ReferenceMut<'_> {}

impl<'msg> ::protobuf::AsView for ReferenceMut<'msg> {
  type Proxied = Reference;
  fn as_view(&self) -> ::protobuf::View<'_, Reference> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ReferenceMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Reference>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ReferenceMut<'msg> {
  type MutProxied = Reference;
  fn as_mut(&mut self) -> ReferenceMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ReferenceMut<'msg> {
  fn into_mut<'shorter>(self) -> ReferenceMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Reference {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Reference> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ReferenceView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ReferenceMut<'_> {
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

  // overload_id: repeated string
  pub fn overload_id(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn overload_id_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_overload_id(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // value: optional message cel.expr.Constant
  pub fn has_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn value_opt(&self) -> ::std::option::Option<crate::xds::generated::cel::expr::syntax::ConstantView<'_>> {
    self.has_value().then(|| self.value())
  }
  pub fn value(&self) -> crate::xds::generated::cel::expr::syntax::ConstantView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::cel::expr::syntax::ConstantView::default())
  }
  pub fn value_mut(&mut self) -> crate::xds::generated::cel::expr::syntax::ConstantMut<'_> {
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
  pub fn set_value(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::cel::expr::syntax::Constant>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl Reference

impl ::std::ops::Drop for Reference {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Reference {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Reference {
  type Proxied = Self;
  fn as_view(&self) -> ReferenceView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Reference {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ReferenceMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Reference {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::cel__expr__Reference_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1XaET3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::cel__expr__Reference_msg_init.0, &[<crate::xds::generated::cel::expr::syntax::Constant as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::cel__expr__Reference_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Reference {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Reference {
  type Msg = Reference;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Reference> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Reference {
  type Msg = Reference;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Reference> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ReferenceMut<'_> {
  type Msg = Reference;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Reference> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ReferenceMut<'_> {
  type Msg = Reference;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Reference> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ReferenceView<'_> {
  type Msg = Reference;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Reference> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ReferenceMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



