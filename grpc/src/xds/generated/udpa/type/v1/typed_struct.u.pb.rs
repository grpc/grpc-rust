const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut udpa__type__v1__TypedStruct_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TypedStruct {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TypedStruct>
}

impl ::protobuf::Message for TypedStruct {
  type MessageView<'msg> = TypedStructView<'msg>;
  type MessageMut<'msg> = TypedStructMut<'msg>;
}

impl ::std::default::Default for TypedStruct {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TypedStruct {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TypedStruct` is `Sync` because it does not implement interior mutability.
//    Neither does `TypedStructMut`.
unsafe impl ::std::marker::Sync for TypedStruct {}

// SAFETY:
// - `TypedStruct` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TypedStruct {}

impl ::protobuf::Proxied for TypedStruct {
  type View<'msg> = TypedStructView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TypedStruct {}

impl ::protobuf::MutProxied for TypedStruct {
  type Mut<'msg> = TypedStructMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TypedStructView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TypedStruct>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TypedStructView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TypedStructView<'msg> {
  type Message = TypedStruct;
}

impl ::std::fmt::Debug for TypedStructView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TypedStructView<'_> {
  fn default() -> TypedStructView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TypedStruct>> for TypedStructView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TypedStruct>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TypedStructView<'msg> {

  pub fn to_owned(&self) -> TypedStruct {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // type_url: optional string
  pub fn type_url(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // value: optional message google.protobuf.Struct
  pub fn has_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn value_opt(self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'msg>> {
    self.has_value().then(|| self.value())
  }
  pub fn value(self) -> ::protobuf_well_known_types::StructView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }

}

// SAFETY:
// - `TypedStructView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TypedStructView<'_> {}

// SAFETY:
// - `TypedStructView` is `Send` because while its alive a `TypedStructMut` cannot.
// - `TypedStructView` does not use thread-local data.
unsafe impl ::std::marker::Send for TypedStructView<'_> {}

impl<'msg> ::protobuf::AsView for TypedStructView<'msg> {
  type Proxied = TypedStruct;
  fn as_view(&self) -> ::protobuf::View<'msg, TypedStruct> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TypedStructView<'msg> {
  fn into_view<'shorter>(self) -> TypedStructView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TypedStruct> for TypedStructView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TypedStruct {
    let mut dst = TypedStruct::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TypedStruct> for TypedStructMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TypedStruct {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TypedStruct {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TypedStructView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TypedStructMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TypedStructMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TypedStruct>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TypedStructMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TypedStructMut<'msg> {
  type Message = TypedStruct;
}

impl ::std::fmt::Debug for TypedStructMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TypedStruct>> for TypedStructMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TypedStruct>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TypedStructMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TypedStruct> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TypedStruct {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // type_url: optional string
  pub fn type_url(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_type_url(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // value: optional message google.protobuf.Struct
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
  pub fn value_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_value().then(|| self.value())
  }
  pub fn value(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn value_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

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
// - `TypedStructMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TypedStructMut<'_> {}

// SAFETY:
// - `TypedStructMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TypedStructMut<'_> {}

impl<'msg> ::protobuf::AsView for TypedStructMut<'msg> {
  type Proxied = TypedStruct;
  fn as_view(&self) -> ::protobuf::View<'_, TypedStruct> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TypedStructMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TypedStruct>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TypedStructMut<'msg> {
  type MutProxied = TypedStruct;
  fn as_mut(&mut self) -> TypedStructMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TypedStructMut<'msg> {
  fn into_mut<'shorter>(self) -> TypedStructMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TypedStruct {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TypedStruct> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TypedStructView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TypedStructMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // type_url: optional string
  pub fn type_url(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_type_url(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // value: optional message google.protobuf.Struct
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
  pub fn value_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_value().then(|| self.value())
  }
  pub fn value(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn value_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl TypedStruct

impl ::std::ops::Drop for TypedStruct {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TypedStruct {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TypedStruct {
  type Proxied = Self;
  fn as_view(&self) -> TypedStructView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TypedStruct {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TypedStructMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TypedStruct {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::udpa__type__v1__TypedStruct_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::udpa__type__v1__TypedStruct_msg_init.0, &[<::protobuf_well_known_types::Struct as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::udpa__type__v1__TypedStruct_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TypedStruct {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TypedStruct {
  type Msg = TypedStruct;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TypedStruct> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TypedStruct {
  type Msg = TypedStruct;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TypedStruct> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TypedStructMut<'_> {
  type Msg = TypedStruct;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TypedStruct> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TypedStructMut<'_> {
  type Msg = TypedStruct;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TypedStruct> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TypedStructView<'_> {
  type Msg = TypedStruct;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TypedStruct> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TypedStructMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



