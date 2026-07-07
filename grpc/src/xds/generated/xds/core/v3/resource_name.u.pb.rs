const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut xds__core__v3__ResourceName_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ResourceName {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ResourceName>
}

impl ::protobuf::Message for ResourceName {
  type MessageView<'msg> = ResourceNameView<'msg>;
  type MessageMut<'msg> = ResourceNameMut<'msg>;
}

impl ::std::default::Default for ResourceName {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ResourceName {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ResourceName` is `Sync` because it does not implement interior mutability.
//    Neither does `ResourceNameMut`.
unsafe impl ::std::marker::Sync for ResourceName {}

// SAFETY:
// - `ResourceName` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ResourceName {}

impl ::protobuf::Proxied for ResourceName {
  type View<'msg> = ResourceNameView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ResourceName {}

impl ::protobuf::MutProxied for ResourceName {
  type Mut<'msg> = ResourceNameMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ResourceNameView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ResourceName>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ResourceNameView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ResourceNameView<'msg> {
  type Message = ResourceName;
}

impl ::std::fmt::Debug for ResourceNameView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ResourceNameView<'_> {
  fn default() -> ResourceNameView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ResourceName>> for ResourceNameView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ResourceName>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ResourceNameView<'msg> {

  pub fn to_owned(&self) -> ResourceName {
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

  // authority: optional string
  pub fn authority(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // resource_type: optional string
  pub fn resource_type(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // context: optional message xds.core.v3.ContextParams
  pub fn has_context(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn context_opt(self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::context_params::ContextParamsView<'msg>> {
    self.has_context().then(|| self.context())
  }
  pub fn context(self) -> crate::xds::generated::xds::core::v3::context_params::ContextParamsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::context_params::ContextParamsView::default())
  }

}

// SAFETY:
// - `ResourceNameView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ResourceNameView<'_> {}

// SAFETY:
// - `ResourceNameView` is `Send` because while its alive a `ResourceNameMut` cannot.
// - `ResourceNameView` does not use thread-local data.
unsafe impl ::std::marker::Send for ResourceNameView<'_> {}

impl<'msg> ::protobuf::AsView for ResourceNameView<'msg> {
  type Proxied = ResourceName;
  fn as_view(&self) -> ::protobuf::View<'msg, ResourceName> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ResourceNameView<'msg> {
  fn into_view<'shorter>(self) -> ResourceNameView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ResourceName> for ResourceNameView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ResourceName {
    let mut dst = ResourceName::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ResourceName> for ResourceNameMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ResourceName {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ResourceName {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ResourceNameView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ResourceNameMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ResourceNameMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceName>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ResourceNameMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ResourceNameMut<'msg> {
  type Message = ResourceName;
}

impl ::std::fmt::Debug for ResourceNameMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceName>> for ResourceNameMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceName>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ResourceNameMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceName> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ResourceName {
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

  // authority: optional string
  pub fn authority(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_authority(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // resource_type: optional string
  pub fn resource_type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_resource_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // context: optional message xds.core.v3.ContextParams
  pub fn has_context(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_context(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn context_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::context_params::ContextParamsView<'_>> {
    self.has_context().then(|| self.context())
  }
  pub fn context(&self) -> crate::xds::generated::xds::core::v3::context_params::ContextParamsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::context_params::ContextParamsView::default())
  }
  pub fn context_mut(&mut self) -> crate::xds::generated::xds::core::v3::context_params::ContextParamsMut<'_> {
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
  pub fn set_context(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::core::v3::context_params::ContextParams>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

}

// SAFETY:
// - `ResourceNameMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ResourceNameMut<'_> {}

// SAFETY:
// - `ResourceNameMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ResourceNameMut<'_> {}

impl<'msg> ::protobuf::AsView for ResourceNameMut<'msg> {
  type Proxied = ResourceName;
  fn as_view(&self) -> ::protobuf::View<'_, ResourceName> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ResourceNameMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ResourceName>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ResourceNameMut<'msg> {
  type MutProxied = ResourceName;
  fn as_mut(&mut self) -> ResourceNameMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ResourceNameMut<'msg> {
  fn into_mut<'shorter>(self) -> ResourceNameMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ResourceName {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ResourceName> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ResourceNameView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ResourceNameMut<'_> {
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

  // authority: optional string
  pub fn authority(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_authority(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // resource_type: optional string
  pub fn resource_type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_resource_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // context: optional message xds.core.v3.ContextParams
  pub fn has_context(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_context(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn context_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::context_params::ContextParamsView<'_>> {
    self.has_context().then(|| self.context())
  }
  pub fn context(&self) -> crate::xds::generated::xds::core::v3::context_params::ContextParamsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::context_params::ContextParamsView::default())
  }
  pub fn context_mut(&mut self) -> crate::xds::generated::xds::core::v3::context_params::ContextParamsMut<'_> {
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
  pub fn set_context(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::core::v3::context_params::ContextParams>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

}  // impl ResourceName

impl ::std::ops::Drop for ResourceName {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ResourceName {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ResourceName {
  type Proxied = Self;
  fn as_view(&self) -> ResourceNameView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ResourceName {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ResourceNameMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ResourceName {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::xds__core__v3__ResourceName_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::xds__core__v3__ResourceName_msg_init.0, &[<crate::xds::generated::xds::core::v3::context_params::ContextParams as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::xds__core__v3__ResourceName_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ResourceName {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ResourceName {
  type Msg = ResourceName;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceName> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResourceName {
  type Msg = ResourceName;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceName> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ResourceNameMut<'_> {
  type Msg = ResourceName;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceName> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResourceNameMut<'_> {
  type Msg = ResourceName;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceName> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResourceNameView<'_> {
  type Msg = ResourceName;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceName> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ResourceNameMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



