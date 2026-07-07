const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__http__v3__PathTransformation_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct PathTransformation {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PathTransformation>
}

impl ::protobuf::Message for PathTransformation {
  type MessageView<'msg> = PathTransformationView<'msg>;
  type MessageMut<'msg> = PathTransformationMut<'msg>;
}

impl ::std::default::Default for PathTransformation {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for PathTransformation {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `PathTransformation` is `Sync` because it does not implement interior mutability.
//    Neither does `PathTransformationMut`.
unsafe impl ::std::marker::Sync for PathTransformation {}

// SAFETY:
// - `PathTransformation` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for PathTransformation {}

impl ::protobuf::Proxied for PathTransformation {
  type View<'msg> = PathTransformationView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PathTransformation {}

impl ::protobuf::MutProxied for PathTransformation {
  type Mut<'msg> = PathTransformationMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PathTransformationView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PathTransformation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PathTransformationView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PathTransformationView<'msg> {
  type Message = PathTransformation;
}

impl ::std::fmt::Debug for PathTransformationView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PathTransformationView<'_> {
  fn default() -> PathTransformationView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, PathTransformation>> for PathTransformationView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PathTransformation>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PathTransformationView<'msg> {

  pub fn to_owned(&self) -> PathTransformation {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // operations: repeated message envoy.type.http.v3.PathTransformation.Operation
  pub fn operations(self) -> ::protobuf::RepeatedView<'msg, super::path_transformation::Operation> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::path_transformation::Operation>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `PathTransformationView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for PathTransformationView<'_> {}

// SAFETY:
// - `PathTransformationView` is `Send` because while its alive a `PathTransformationMut` cannot.
// - `PathTransformationView` does not use thread-local data.
unsafe impl ::std::marker::Send for PathTransformationView<'_> {}

impl<'msg> ::protobuf::AsView for PathTransformationView<'msg> {
  type Proxied = PathTransformation;
  fn as_view(&self) -> ::protobuf::View<'msg, PathTransformation> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PathTransformationView<'msg> {
  fn into_view<'shorter>(self) -> PathTransformationView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PathTransformation> for PathTransformationView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PathTransformation {
    let mut dst = PathTransformation::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PathTransformation> for PathTransformationMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PathTransformation {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for PathTransformation {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PathTransformationView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PathTransformationMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PathTransformationMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PathTransformation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PathTransformationMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PathTransformationMut<'msg> {
  type Message = PathTransformation;
}

impl ::std::fmt::Debug for PathTransformationMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, PathTransformation>> for PathTransformationMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PathTransformation>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PathTransformationMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PathTransformation> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> PathTransformation {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // operations: repeated message envoy.type.http.v3.PathTransformation.Operation
  pub fn operations(&self) -> ::protobuf::RepeatedView<'_, super::path_transformation::Operation> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::path_transformation::Operation>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn operations_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::path_transformation::Operation> {
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
  pub fn set_operations(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::path_transformation::Operation>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `PathTransformationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for PathTransformationMut<'_> {}

// SAFETY:
// - `PathTransformationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for PathTransformationMut<'_> {}

impl<'msg> ::protobuf::AsView for PathTransformationMut<'msg> {
  type Proxied = PathTransformation;
  fn as_view(&self) -> ::protobuf::View<'_, PathTransformation> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PathTransformationMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PathTransformation>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for PathTransformationMut<'msg> {
  type MutProxied = PathTransformation;
  fn as_mut(&mut self) -> PathTransformationMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PathTransformationMut<'msg> {
  fn into_mut<'shorter>(self) -> PathTransformationMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PathTransformation {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PathTransformation> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PathTransformationView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PathTransformationMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // operations: repeated message envoy.type.http.v3.PathTransformation.Operation
  pub fn operations(&self) -> ::protobuf::RepeatedView<'_, super::path_transformation::Operation> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::path_transformation::Operation>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn operations_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::path_transformation::Operation> {
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
  pub fn set_operations(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::path_transformation::Operation>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl PathTransformation

impl ::std::ops::Drop for PathTransformation {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PathTransformation {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PathTransformation {
  type Proxied = Self;
  fn as_view(&self) -> PathTransformationView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PathTransformation {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PathTransformationMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PathTransformation {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__type__http__v3__PathTransformation_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__http__v3__PathTransformation_msg_init.0, &[<super::path_transformation::Operation as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__http__v3__PathTransformation_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PathTransformation {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PathTransformation {
  type Msg = PathTransformation;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PathTransformation> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PathTransformation {
  type Msg = PathTransformation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PathTransformation> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PathTransformationMut<'_> {
  type Msg = PathTransformation;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PathTransformation> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PathTransformationMut<'_> {
  type Msg = PathTransformation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PathTransformation> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PathTransformationView<'_> {
  type Msg = PathTransformation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PathTransformation> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PathTransformationMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod path_transformation {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__http__v3__PathTransformation__Operation_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Operation {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Operation>
}

impl ::protobuf::Message for Operation {
  type MessageView<'msg> = OperationView<'msg>;
  type MessageMut<'msg> = OperationMut<'msg>;
}

impl ::std::default::Default for Operation {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Operation {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Operation` is `Sync` because it does not implement interior mutability.
//    Neither does `OperationMut`.
unsafe impl ::std::marker::Sync for Operation {}

// SAFETY:
// - `Operation` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Operation {}

impl ::protobuf::Proxied for Operation {
  type View<'msg> = OperationView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Operation {}

impl ::protobuf::MutProxied for Operation {
  type Mut<'msg> = OperationMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct OperationView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Operation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OperationView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for OperationView<'msg> {
  type Message = Operation;
}

impl ::std::fmt::Debug for OperationView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for OperationView<'_> {
  fn default() -> OperationView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Operation>> for OperationView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Operation>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OperationView<'msg> {

  pub fn to_owned(&self) -> Operation {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // normalize_path_rfc_3986: optional message envoy.type.http.v3.PathTransformation.Operation.NormalizePathRFC3986
  pub fn has_normalize_path_rfc_3986(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn normalize_path_rfc_3986_opt(self) -> ::std::option::Option<super::super::path_transformation::operation::NormalizePathRFC3986View<'msg>> {
    self.has_normalize_path_rfc_3986().then(|| self.normalize_path_rfc_3986())
  }
  pub fn normalize_path_rfc_3986(self) -> super::super::path_transformation::operation::NormalizePathRFC3986View<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::path_transformation::operation::NormalizePathRFC3986View::default())
  }

  // merge_slashes: optional message envoy.type.http.v3.PathTransformation.Operation.MergeSlashes
  pub fn has_merge_slashes(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn merge_slashes_opt(self) -> ::std::option::Option<super::super::path_transformation::operation::MergeSlashesView<'msg>> {
    self.has_merge_slashes().then(|| self.merge_slashes())
  }
  pub fn merge_slashes(self) -> super::super::path_transformation::operation::MergeSlashesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::path_transformation::operation::MergeSlashesView::default())
  }

  pub fn operation_specifier(self) -> super::super::path_transformation::operation::OperationSpecifierOneof<'msg> {
    match self.operation_specifier_case() {
      super::super::path_transformation::operation::OperationSpecifierCase::NormalizePathRfc3986 =>
          super::super::path_transformation::operation::OperationSpecifierOneof::NormalizePathRfc3986(self.normalize_path_rfc_3986()),
      super::super::path_transformation::operation::OperationSpecifierCase::MergeSlashes =>
          super::super::path_transformation::operation::OperationSpecifierOneof::MergeSlashes(self.merge_slashes()),
      _ => super::super::path_transformation::operation::OperationSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn operation_specifier_case(self) -> super::super::path_transformation::operation::OperationSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::path_transformation::operation::OperationSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `OperationView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for OperationView<'_> {}

// SAFETY:
// - `OperationView` is `Send` because while its alive a `OperationMut` cannot.
// - `OperationView` does not use thread-local data.
unsafe impl ::std::marker::Send for OperationView<'_> {}

impl<'msg> ::protobuf::AsView for OperationView<'msg> {
  type Proxied = Operation;
  fn as_view(&self) -> ::protobuf::View<'msg, Operation> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OperationView<'msg> {
  fn into_view<'shorter>(self) -> OperationView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Operation> for OperationView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Operation {
    let mut dst = Operation::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Operation> for OperationMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Operation {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Operation {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for OperationView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for OperationMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct OperationMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Operation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OperationMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for OperationMut<'msg> {
  type Message = Operation;
}

impl ::std::fmt::Debug for OperationMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Operation>> for OperationMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Operation>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OperationMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Operation> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Operation {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // normalize_path_rfc_3986: optional message envoy.type.http.v3.PathTransformation.Operation.NormalizePathRFC3986
  pub fn has_normalize_path_rfc_3986(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_normalize_path_rfc_3986(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn normalize_path_rfc_3986_opt(&self) -> ::std::option::Option<super::super::path_transformation::operation::NormalizePathRFC3986View<'_>> {
    self.has_normalize_path_rfc_3986().then(|| self.normalize_path_rfc_3986())
  }
  pub fn normalize_path_rfc_3986(&self) -> super::super::path_transformation::operation::NormalizePathRFC3986View<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::path_transformation::operation::NormalizePathRFC3986View::default())
  }
  pub fn normalize_path_rfc_3986_mut(&mut self) -> super::super::path_transformation::operation::NormalizePathRFC3986Mut<'_> {
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
  pub fn set_normalize_path_rfc_3986(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::path_transformation::operation::NormalizePathRFC3986>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // merge_slashes: optional message envoy.type.http.v3.PathTransformation.Operation.MergeSlashes
  pub fn has_merge_slashes(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_merge_slashes(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn merge_slashes_opt(&self) -> ::std::option::Option<super::super::path_transformation::operation::MergeSlashesView<'_>> {
    self.has_merge_slashes().then(|| self.merge_slashes())
  }
  pub fn merge_slashes(&self) -> super::super::path_transformation::operation::MergeSlashesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::path_transformation::operation::MergeSlashesView::default())
  }
  pub fn merge_slashes_mut(&mut self) -> super::super::path_transformation::operation::MergeSlashesMut<'_> {
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
  pub fn set_merge_slashes(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::path_transformation::operation::MergeSlashes>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn operation_specifier(&self) -> super::super::path_transformation::operation::OperationSpecifierOneof<'_> {
    match &self.operation_specifier_case() {
      super::super::path_transformation::operation::OperationSpecifierCase::NormalizePathRfc3986 =>
          super::super::path_transformation::operation::OperationSpecifierOneof::NormalizePathRfc3986(self.normalize_path_rfc_3986()),
      super::super::path_transformation::operation::OperationSpecifierCase::MergeSlashes =>
          super::super::path_transformation::operation::OperationSpecifierOneof::MergeSlashes(self.merge_slashes()),
      _ => super::super::path_transformation::operation::OperationSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn operation_specifier_case(&self) -> super::super::path_transformation::operation::OperationSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::path_transformation::operation::OperationSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `OperationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for OperationMut<'_> {}

// SAFETY:
// - `OperationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for OperationMut<'_> {}

impl<'msg> ::protobuf::AsView for OperationMut<'msg> {
  type Proxied = Operation;
  fn as_view(&self) -> ::protobuf::View<'_, Operation> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OperationMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Operation>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for OperationMut<'msg> {
  type MutProxied = Operation;
  fn as_mut(&mut self) -> OperationMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for OperationMut<'msg> {
  fn into_mut<'shorter>(self) -> OperationMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Operation {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Operation> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> OperationView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> OperationMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // normalize_path_rfc_3986: optional message envoy.type.http.v3.PathTransformation.Operation.NormalizePathRFC3986
  pub fn has_normalize_path_rfc_3986(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_normalize_path_rfc_3986(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn normalize_path_rfc_3986_opt(&self) -> ::std::option::Option<super::super::path_transformation::operation::NormalizePathRFC3986View<'_>> {
    self.has_normalize_path_rfc_3986().then(|| self.normalize_path_rfc_3986())
  }
  pub fn normalize_path_rfc_3986(&self) -> super::super::path_transformation::operation::NormalizePathRFC3986View<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::path_transformation::operation::NormalizePathRFC3986View::default())
  }
  pub fn normalize_path_rfc_3986_mut(&mut self) -> super::super::path_transformation::operation::NormalizePathRFC3986Mut<'_> {
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
  pub fn set_normalize_path_rfc_3986(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::path_transformation::operation::NormalizePathRFC3986>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // merge_slashes: optional message envoy.type.http.v3.PathTransformation.Operation.MergeSlashes
  pub fn has_merge_slashes(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_merge_slashes(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn merge_slashes_opt(&self) -> ::std::option::Option<super::super::path_transformation::operation::MergeSlashesView<'_>> {
    self.has_merge_slashes().then(|| self.merge_slashes())
  }
  pub fn merge_slashes(&self) -> super::super::path_transformation::operation::MergeSlashesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::path_transformation::operation::MergeSlashesView::default())
  }
  pub fn merge_slashes_mut(&mut self) -> super::super::path_transformation::operation::MergeSlashesMut<'_> {
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
  pub fn set_merge_slashes(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::path_transformation::operation::MergeSlashes>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn operation_specifier(&self) -> super::super::path_transformation::operation::OperationSpecifierOneof<'_> {
    match &self.operation_specifier_case() {
      super::super::path_transformation::operation::OperationSpecifierCase::NormalizePathRfc3986 =>
          super::super::path_transformation::operation::OperationSpecifierOneof::NormalizePathRfc3986(self.normalize_path_rfc_3986()),
      super::super::path_transformation::operation::OperationSpecifierCase::MergeSlashes =>
          super::super::path_transformation::operation::OperationSpecifierOneof::MergeSlashes(self.merge_slashes()),
      _ => super::super::path_transformation::operation::OperationSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn operation_specifier_case(&self) -> super::super::path_transformation::operation::OperationSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::path_transformation::operation::OperationSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl Operation

impl ::std::ops::Drop for Operation {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Operation {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Operation {
  type Proxied = Self;
  fn as_view(&self) -> OperationView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Operation {
  type MutProxied = Self;
  fn as_mut(&mut self) -> OperationMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Operation {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::path_transformation::envoy__type__http__v3__PathTransformation__Operation_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$a33^#|$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::path_transformation::envoy__type__http__v3__PathTransformation__Operation_msg_init.0, &[<super::super::path_transformation::operation::NormalizePathRFC3986 as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::path_transformation::operation::MergeSlashes as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::path_transformation::envoy__type__http__v3__PathTransformation__Operation_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Operation {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Operation {
  type Msg = Operation;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Operation> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Operation {
  type Msg = Operation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Operation> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for OperationMut<'_> {
  type Msg = Operation;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Operation> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OperationMut<'_> {
  type Msg = Operation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Operation> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OperationView<'_> {
  type Msg = Operation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Operation> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for OperationMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod operation {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__http__v3__PathTransformation__Operation__NormalizePathRFC3986_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct NormalizePathRFC3986 {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<NormalizePathRFC3986>
}

impl ::protobuf::Message for NormalizePathRFC3986 {
  type MessageView<'msg> = NormalizePathRFC3986View<'msg>;
  type MessageMut<'msg> = NormalizePathRFC3986Mut<'msg>;
}

impl ::std::default::Default for NormalizePathRFC3986 {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for NormalizePathRFC3986 {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `NormalizePathRFC3986` is `Sync` because it does not implement interior mutability.
//    Neither does `NormalizePathRFC3986Mut`.
unsafe impl ::std::marker::Sync for NormalizePathRFC3986 {}

// SAFETY:
// - `NormalizePathRFC3986` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for NormalizePathRFC3986 {}

impl ::protobuf::Proxied for NormalizePathRFC3986 {
  type View<'msg> = NormalizePathRFC3986View<'msg>;
}

impl ::protobuf::__internal::SealedInternal for NormalizePathRFC3986 {}

impl ::protobuf::MutProxied for NormalizePathRFC3986 {
  type Mut<'msg> = NormalizePathRFC3986Mut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct NormalizePathRFC3986View<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, NormalizePathRFC3986>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for NormalizePathRFC3986View<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for NormalizePathRFC3986View<'msg> {
  type Message = NormalizePathRFC3986;
}

impl ::std::fmt::Debug for NormalizePathRFC3986View<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for NormalizePathRFC3986View<'_> {
  fn default() -> NormalizePathRFC3986View<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, NormalizePathRFC3986>> for NormalizePathRFC3986View<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, NormalizePathRFC3986>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> NormalizePathRFC3986View<'msg> {

  pub fn to_owned(&self) -> NormalizePathRFC3986 {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `NormalizePathRFC3986View` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for NormalizePathRFC3986View<'_> {}

// SAFETY:
// - `NormalizePathRFC3986View` is `Send` because while its alive a `NormalizePathRFC3986Mut` cannot.
// - `NormalizePathRFC3986View` does not use thread-local data.
unsafe impl ::std::marker::Send for NormalizePathRFC3986View<'_> {}

impl<'msg> ::protobuf::AsView for NormalizePathRFC3986View<'msg> {
  type Proxied = NormalizePathRFC3986;
  fn as_view(&self) -> ::protobuf::View<'msg, NormalizePathRFC3986> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for NormalizePathRFC3986View<'msg> {
  fn into_view<'shorter>(self) -> NormalizePathRFC3986View<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<NormalizePathRFC3986> for NormalizePathRFC3986View<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> NormalizePathRFC3986 {
    let mut dst = NormalizePathRFC3986::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<NormalizePathRFC3986> for NormalizePathRFC3986Mut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> NormalizePathRFC3986 {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for NormalizePathRFC3986 {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for NormalizePathRFC3986View<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for NormalizePathRFC3986Mut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct NormalizePathRFC3986Mut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, NormalizePathRFC3986>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for NormalizePathRFC3986Mut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for NormalizePathRFC3986Mut<'msg> {
  type Message = NormalizePathRFC3986;
}

impl ::std::fmt::Debug for NormalizePathRFC3986Mut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, NormalizePathRFC3986>> for NormalizePathRFC3986Mut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, NormalizePathRFC3986>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> NormalizePathRFC3986Mut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, NormalizePathRFC3986> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> NormalizePathRFC3986 {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `NormalizePathRFC3986Mut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for NormalizePathRFC3986Mut<'_> {}

// SAFETY:
// - `NormalizePathRFC3986Mut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for NormalizePathRFC3986Mut<'_> {}

impl<'msg> ::protobuf::AsView for NormalizePathRFC3986Mut<'msg> {
  type Proxied = NormalizePathRFC3986;
  fn as_view(&self) -> ::protobuf::View<'_, NormalizePathRFC3986> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for NormalizePathRFC3986Mut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, NormalizePathRFC3986>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for NormalizePathRFC3986Mut<'msg> {
  type MutProxied = NormalizePathRFC3986;
  fn as_mut(&mut self) -> NormalizePathRFC3986Mut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for NormalizePathRFC3986Mut<'msg> {
  fn into_mut<'shorter>(self) -> NormalizePathRFC3986Mut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl NormalizePathRFC3986 {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, NormalizePathRFC3986> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> NormalizePathRFC3986View<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> NormalizePathRFC3986Mut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl NormalizePathRFC3986

impl ::std::ops::Drop for NormalizePathRFC3986 {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for NormalizePathRFC3986 {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for NormalizePathRFC3986 {
  type Proxied = Self;
  fn as_view(&self) -> NormalizePathRFC3986View<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for NormalizePathRFC3986 {
  type MutProxied = Self;
  fn as_mut(&mut self) -> NormalizePathRFC3986Mut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for NormalizePathRFC3986 {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::path_transformation::operation::envoy__type__http__v3__PathTransformation__Operation__NormalizePathRFC3986_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::path_transformation::operation::envoy__type__http__v3__PathTransformation__Operation__NormalizePathRFC3986_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::path_transformation::operation::envoy__type__http__v3__PathTransformation__Operation__NormalizePathRFC3986_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for NormalizePathRFC3986 {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for NormalizePathRFC3986 {
  type Msg = NormalizePathRFC3986;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NormalizePathRFC3986> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NormalizePathRFC3986 {
  type Msg = NormalizePathRFC3986;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NormalizePathRFC3986> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for NormalizePathRFC3986Mut<'_> {
  type Msg = NormalizePathRFC3986;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NormalizePathRFC3986> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NormalizePathRFC3986Mut<'_> {
  type Msg = NormalizePathRFC3986;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NormalizePathRFC3986> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NormalizePathRFC3986View<'_> {
  type Msg = NormalizePathRFC3986;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NormalizePathRFC3986> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for NormalizePathRFC3986Mut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__http__v3__PathTransformation__Operation__MergeSlashes_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MergeSlashes {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MergeSlashes>
}

impl ::protobuf::Message for MergeSlashes {
  type MessageView<'msg> = MergeSlashesView<'msg>;
  type MessageMut<'msg> = MergeSlashesMut<'msg>;
}

impl ::std::default::Default for MergeSlashes {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MergeSlashes {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MergeSlashes` is `Sync` because it does not implement interior mutability.
//    Neither does `MergeSlashesMut`.
unsafe impl ::std::marker::Sync for MergeSlashes {}

// SAFETY:
// - `MergeSlashes` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for MergeSlashes {}

impl ::protobuf::Proxied for MergeSlashes {
  type View<'msg> = MergeSlashesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MergeSlashes {}

impl ::protobuf::MutProxied for MergeSlashes {
  type Mut<'msg> = MergeSlashesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MergeSlashesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MergeSlashes>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MergeSlashesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MergeSlashesView<'msg> {
  type Message = MergeSlashes;
}

impl ::std::fmt::Debug for MergeSlashesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MergeSlashesView<'_> {
  fn default() -> MergeSlashesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MergeSlashes>> for MergeSlashesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MergeSlashes>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MergeSlashesView<'msg> {

  pub fn to_owned(&self) -> MergeSlashes {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `MergeSlashesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for MergeSlashesView<'_> {}

// SAFETY:
// - `MergeSlashesView` is `Send` because while its alive a `MergeSlashesMut` cannot.
// - `MergeSlashesView` does not use thread-local data.
unsafe impl ::std::marker::Send for MergeSlashesView<'_> {}

impl<'msg> ::protobuf::AsView for MergeSlashesView<'msg> {
  type Proxied = MergeSlashes;
  fn as_view(&self) -> ::protobuf::View<'msg, MergeSlashes> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MergeSlashesView<'msg> {
  fn into_view<'shorter>(self) -> MergeSlashesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MergeSlashes> for MergeSlashesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MergeSlashes {
    let mut dst = MergeSlashes::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MergeSlashes> for MergeSlashesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MergeSlashes {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for MergeSlashes {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MergeSlashesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MergeSlashesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MergeSlashesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MergeSlashes>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MergeSlashesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MergeSlashesMut<'msg> {
  type Message = MergeSlashes;
}

impl ::std::fmt::Debug for MergeSlashesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MergeSlashes>> for MergeSlashesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MergeSlashes>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MergeSlashesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MergeSlashes> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> MergeSlashes {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `MergeSlashesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for MergeSlashesMut<'_> {}

// SAFETY:
// - `MergeSlashesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for MergeSlashesMut<'_> {}

impl<'msg> ::protobuf::AsView for MergeSlashesMut<'msg> {
  type Proxied = MergeSlashes;
  fn as_view(&self) -> ::protobuf::View<'_, MergeSlashes> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MergeSlashesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MergeSlashes>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for MergeSlashesMut<'msg> {
  type MutProxied = MergeSlashes;
  fn as_mut(&mut self) -> MergeSlashesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MergeSlashesMut<'msg> {
  fn into_mut<'shorter>(self) -> MergeSlashesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MergeSlashes {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MergeSlashes> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MergeSlashesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MergeSlashesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl MergeSlashes

impl ::std::ops::Drop for MergeSlashes {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MergeSlashes {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MergeSlashes {
  type Proxied = Self;
  fn as_view(&self) -> MergeSlashesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MergeSlashes {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MergeSlashesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MergeSlashes {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::path_transformation::operation::envoy__type__http__v3__PathTransformation__Operation__MergeSlashes_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::path_transformation::operation::envoy__type__http__v3__PathTransformation__Operation__MergeSlashes_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::path_transformation::operation::envoy__type__http__v3__PathTransformation__Operation__MergeSlashes_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MergeSlashes {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MergeSlashes {
  type Msg = MergeSlashes;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MergeSlashes> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MergeSlashes {
  type Msg = MergeSlashes;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MergeSlashes> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MergeSlashesMut<'_> {
  type Msg = MergeSlashes;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MergeSlashes> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MergeSlashesMut<'_> {
  type Msg = MergeSlashes;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MergeSlashes> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MergeSlashesView<'_> {
  type Msg = MergeSlashes;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MergeSlashes> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MergeSlashesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum OperationSpecifierOneof<'msg> {
  NormalizePathRfc3986(::protobuf::View<'msg, super::super::super::path_transformation::operation::NormalizePathRFC3986>) = 2,
  MergeSlashes(::protobuf::View<'msg, super::super::super::path_transformation::operation::MergeSlashes>) = 3,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum OperationSpecifierCase {
  NormalizePathRfc3986 = 2,
  MergeSlashes = 3,

  not_set = 0
}

impl OperationSpecifierCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<OperationSpecifierCase> {
    match v {
      0 => Some(OperationSpecifierCase::not_set),
      2 => Some(OperationSpecifierCase::NormalizePathRfc3986),
      3 => Some(OperationSpecifierCase::MergeSlashes),
      _ => None
    }
  }
}
}  // pub mod operation


}  // pub mod path_transformation


