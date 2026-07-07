const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut xds__core__v3__ResourceLocator_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ResourceLocator {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ResourceLocator>
}

impl ::protobuf::Message for ResourceLocator {
  type MessageView<'msg> = ResourceLocatorView<'msg>;
  type MessageMut<'msg> = ResourceLocatorMut<'msg>;
}

impl ::std::default::Default for ResourceLocator {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ResourceLocator {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ResourceLocator` is `Sync` because it does not implement interior mutability.
//    Neither does `ResourceLocatorMut`.
unsafe impl ::std::marker::Sync for ResourceLocator {}

// SAFETY:
// - `ResourceLocator` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ResourceLocator {}

impl ::protobuf::Proxied for ResourceLocator {
  type View<'msg> = ResourceLocatorView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ResourceLocator {}

impl ::protobuf::MutProxied for ResourceLocator {
  type Mut<'msg> = ResourceLocatorMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ResourceLocatorView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ResourceLocator>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ResourceLocatorView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ResourceLocatorView<'msg> {
  type Message = ResourceLocator;
}

impl ::std::fmt::Debug for ResourceLocatorView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ResourceLocatorView<'_> {
  fn default() -> ResourceLocatorView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ResourceLocator>> for ResourceLocatorView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ResourceLocator>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ResourceLocatorView<'msg> {

  pub fn to_owned(&self) -> ResourceLocator {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // scheme: optional enum xds.core.v3.ResourceLocator.Scheme
  pub fn scheme(self) -> super::resource_locator::Scheme {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::resource_locator::Scheme::Xdstp).into()
      ).try_into().unwrap()
    }
  }

  // id: optional string
  pub fn id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // authority: optional string
  pub fn authority(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // resource_type: optional string
  pub fn resource_type(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // exact_context: optional message xds.core.v3.ContextParams
  pub fn has_exact_context(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn exact_context_opt(self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::context_params::ContextParamsView<'msg>> {
    self.has_exact_context().then(|| self.exact_context())
  }
  pub fn exact_context(self) -> crate::xds::generated::xds::core::v3::context_params::ContextParamsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::context_params::ContextParamsView::default())
  }

  // directives: repeated message xds.core.v3.ResourceLocator.Directive
  pub fn directives(self) -> ::protobuf::RepeatedView<'msg, super::resource_locator::Directive> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::resource_locator::Directive>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  pub fn context_param_specifier(self) -> super::resource_locator::ContextParamSpecifierOneof<'msg> {
    match self.context_param_specifier_case() {
      super::resource_locator::ContextParamSpecifierCase::ExactContext =>
          super::resource_locator::ContextParamSpecifierOneof::ExactContext(self.exact_context()),
      _ => super::resource_locator::ContextParamSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn context_param_specifier_case(self) -> super::resource_locator::ContextParamSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(4);
      super::resource_locator::ContextParamSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ResourceLocatorView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ResourceLocatorView<'_> {}

// SAFETY:
// - `ResourceLocatorView` is `Send` because while its alive a `ResourceLocatorMut` cannot.
// - `ResourceLocatorView` does not use thread-local data.
unsafe impl ::std::marker::Send for ResourceLocatorView<'_> {}

impl<'msg> ::protobuf::AsView for ResourceLocatorView<'msg> {
  type Proxied = ResourceLocator;
  fn as_view(&self) -> ::protobuf::View<'msg, ResourceLocator> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ResourceLocatorView<'msg> {
  fn into_view<'shorter>(self) -> ResourceLocatorView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ResourceLocator> for ResourceLocatorView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ResourceLocator {
    let mut dst = ResourceLocator::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ResourceLocator> for ResourceLocatorMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ResourceLocator {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ResourceLocator {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ResourceLocatorView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ResourceLocatorMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ResourceLocatorMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceLocator>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ResourceLocatorMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ResourceLocatorMut<'msg> {
  type Message = ResourceLocator;
}

impl ::std::fmt::Debug for ResourceLocatorMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceLocator>> for ResourceLocatorMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceLocator>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ResourceLocatorMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceLocator> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ResourceLocator {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // scheme: optional enum xds.core.v3.ResourceLocator.Scheme
  pub fn scheme(&self) -> super::resource_locator::Scheme {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::resource_locator::Scheme::Xdstp).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_scheme(&mut self, val: super::resource_locator::Scheme) {
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

  // id: optional string
  pub fn id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // authority: optional string
  pub fn authority(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_authority(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // resource_type: optional string
  pub fn resource_type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_resource_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // exact_context: optional message xds.core.v3.ContextParams
  pub fn has_exact_context(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_exact_context(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn exact_context_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::context_params::ContextParamsView<'_>> {
    self.has_exact_context().then(|| self.exact_context())
  }
  pub fn exact_context(&self) -> crate::xds::generated::xds::core::v3::context_params::ContextParamsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::context_params::ContextParamsView::default())
  }
  pub fn exact_context_mut(&mut self) -> crate::xds::generated::xds::core::v3::context_params::ContextParamsMut<'_> {
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
  pub fn set_exact_context(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::core::v3::context_params::ContextParams>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // directives: repeated message xds.core.v3.ResourceLocator.Directive
  pub fn directives(&self) -> ::protobuf::RepeatedView<'_, super::resource_locator::Directive> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::resource_locator::Directive>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn directives_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::resource_locator::Directive> {
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
  pub fn set_directives(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::resource_locator::Directive>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  pub fn context_param_specifier(&self) -> super::resource_locator::ContextParamSpecifierOneof<'_> {
    match &self.context_param_specifier_case() {
      super::resource_locator::ContextParamSpecifierCase::ExactContext =>
          super::resource_locator::ContextParamSpecifierOneof::ExactContext(self.exact_context()),
      _ => super::resource_locator::ContextParamSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn context_param_specifier_case(&self) -> super::resource_locator::ContextParamSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(4);
      super::resource_locator::ContextParamSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ResourceLocatorMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ResourceLocatorMut<'_> {}

// SAFETY:
// - `ResourceLocatorMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ResourceLocatorMut<'_> {}

impl<'msg> ::protobuf::AsView for ResourceLocatorMut<'msg> {
  type Proxied = ResourceLocator;
  fn as_view(&self) -> ::protobuf::View<'_, ResourceLocator> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ResourceLocatorMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ResourceLocator>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ResourceLocatorMut<'msg> {
  type MutProxied = ResourceLocator;
  fn as_mut(&mut self) -> ResourceLocatorMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ResourceLocatorMut<'msg> {
  fn into_mut<'shorter>(self) -> ResourceLocatorMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ResourceLocator {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ResourceLocator> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ResourceLocatorView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ResourceLocatorMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // scheme: optional enum xds.core.v3.ResourceLocator.Scheme
  pub fn scheme(&self) -> super::resource_locator::Scheme {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::resource_locator::Scheme::Xdstp).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_scheme(&mut self, val: super::resource_locator::Scheme) {
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

  // id: optional string
  pub fn id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // authority: optional string
  pub fn authority(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_authority(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // resource_type: optional string
  pub fn resource_type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_resource_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // exact_context: optional message xds.core.v3.ContextParams
  pub fn has_exact_context(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_exact_context(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn exact_context_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::context_params::ContextParamsView<'_>> {
    self.has_exact_context().then(|| self.exact_context())
  }
  pub fn exact_context(&self) -> crate::xds::generated::xds::core::v3::context_params::ContextParamsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::context_params::ContextParamsView::default())
  }
  pub fn exact_context_mut(&mut self) -> crate::xds::generated::xds::core::v3::context_params::ContextParamsMut<'_> {
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
  pub fn set_exact_context(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::core::v3::context_params::ContextParams>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // directives: repeated message xds.core.v3.ResourceLocator.Directive
  pub fn directives(&self) -> ::protobuf::RepeatedView<'_, super::resource_locator::Directive> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::resource_locator::Directive>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn directives_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::resource_locator::Directive> {
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
  pub fn set_directives(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::resource_locator::Directive>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  pub fn context_param_specifier(&self) -> super::resource_locator::ContextParamSpecifierOneof<'_> {
    match &self.context_param_specifier_case() {
      super::resource_locator::ContextParamSpecifierCase::ExactContext =>
          super::resource_locator::ContextParamSpecifierOneof::ExactContext(self.exact_context()),
      _ => super::resource_locator::ContextParamSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn context_param_specifier_case(&self) -> super::resource_locator::ContextParamSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(4);
      super::resource_locator::ContextParamSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl ResourceLocator

impl ::std::ops::Drop for ResourceLocator {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ResourceLocator {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ResourceLocator {
  type Proxied = Self;
  fn as_view(&self) -> ResourceLocatorView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ResourceLocator {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ResourceLocatorMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ResourceLocator {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::xds__core__v3__ResourceLocator_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.P1X1X1X3G^&");
        super::resource_locator::xds__core__v3__ResourceLocator__Directive_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$31T^!|#");
        ::protobuf::__internal::runtime::link_mini_table(
            super::xds__core__v3__ResourceLocator_msg_init.0, &[<crate::xds::generated::xds::core::v3::context_params::ContextParams as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            super::resource_locator::xds__core__v3__ResourceLocator__Directive_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::resource_locator::xds__core__v3__ResourceLocator__Directive_msg_init.0, &[super::xds__core__v3__ResourceLocator_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::xds__core__v3__ResourceLocator_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ResourceLocator {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ResourceLocator {
  type Msg = ResourceLocator;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceLocator> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResourceLocator {
  type Msg = ResourceLocator;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceLocator> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ResourceLocatorMut<'_> {
  type Msg = ResourceLocator;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceLocator> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResourceLocatorMut<'_> {
  type Msg = ResourceLocator;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceLocator> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResourceLocatorView<'_> {
  type Msg = ResourceLocator;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceLocator> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ResourceLocatorMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod resource_locator {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut xds__core__v3__ResourceLocator__Directive_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Directive {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Directive>
}

impl ::protobuf::Message for Directive {
  type MessageView<'msg> = DirectiveView<'msg>;
  type MessageMut<'msg> = DirectiveMut<'msg>;
}

impl ::std::default::Default for Directive {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Directive {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Directive` is `Sync` because it does not implement interior mutability.
//    Neither does `DirectiveMut`.
unsafe impl ::std::marker::Sync for Directive {}

// SAFETY:
// - `Directive` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Directive {}

impl ::protobuf::Proxied for Directive {
  type View<'msg> = DirectiveView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Directive {}

impl ::protobuf::MutProxied for Directive {
  type Mut<'msg> = DirectiveMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DirectiveView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Directive>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DirectiveView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DirectiveView<'msg> {
  type Message = Directive;
}

impl ::std::fmt::Debug for DirectiveView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DirectiveView<'_> {
  fn default() -> DirectiveView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Directive>> for DirectiveView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Directive>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DirectiveView<'msg> {

  pub fn to_owned(&self) -> Directive {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // alt: optional message xds.core.v3.ResourceLocator
  pub fn has_alt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn alt_opt(self) -> ::std::option::Option<super::super::ResourceLocatorView<'msg>> {
    self.has_alt().then(|| self.alt())
  }
  pub fn alt(self) -> super::super::ResourceLocatorView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::ResourceLocatorView::default())
  }

  // entry: optional string
  pub fn has_entry(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn entry_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_entry().then(|| self.entry())
  }
  pub fn entry(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  pub fn directive(self) -> super::super::resource_locator::directive::DirectiveOneof<'msg> {
    match self.directive_case() {
      super::super::resource_locator::directive::DirectiveCase::Alt =>
          super::super::resource_locator::directive::DirectiveOneof::Alt(self.alt()),
      super::super::resource_locator::directive::DirectiveCase::Entry =>
          super::super::resource_locator::directive::DirectiveOneof::Entry(self.entry()),
      _ => super::super::resource_locator::directive::DirectiveOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn directive_case(self) -> super::super::resource_locator::directive::DirectiveCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::resource_locator::directive::DirectiveCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `DirectiveView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DirectiveView<'_> {}

// SAFETY:
// - `DirectiveView` is `Send` because while its alive a `DirectiveMut` cannot.
// - `DirectiveView` does not use thread-local data.
unsafe impl ::std::marker::Send for DirectiveView<'_> {}

impl<'msg> ::protobuf::AsView for DirectiveView<'msg> {
  type Proxied = Directive;
  fn as_view(&self) -> ::protobuf::View<'msg, Directive> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DirectiveView<'msg> {
  fn into_view<'shorter>(self) -> DirectiveView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Directive> for DirectiveView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Directive {
    let mut dst = Directive::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Directive> for DirectiveMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Directive {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Directive {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DirectiveView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DirectiveMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DirectiveMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Directive>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DirectiveMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DirectiveMut<'msg> {
  type Message = Directive;
}

impl ::std::fmt::Debug for DirectiveMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Directive>> for DirectiveMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Directive>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DirectiveMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Directive> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Directive {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // alt: optional message xds.core.v3.ResourceLocator
  pub fn has_alt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_alt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn alt_opt(&self) -> ::std::option::Option<super::super::ResourceLocatorView<'_>> {
    self.has_alt().then(|| self.alt())
  }
  pub fn alt(&self) -> super::super::ResourceLocatorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::ResourceLocatorView::default())
  }
  pub fn alt_mut(&mut self) -> super::super::ResourceLocatorMut<'_> {
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
  pub fn set_alt(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::ResourceLocator>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // entry: optional string
  pub fn has_entry(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_entry(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn entry_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_entry().then(|| self.entry())
  }
  pub fn entry(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_entry(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  pub fn directive(&self) -> super::super::resource_locator::directive::DirectiveOneof<'_> {
    match &self.directive_case() {
      super::super::resource_locator::directive::DirectiveCase::Alt =>
          super::super::resource_locator::directive::DirectiveOneof::Alt(self.alt()),
      super::super::resource_locator::directive::DirectiveCase::Entry =>
          super::super::resource_locator::directive::DirectiveOneof::Entry(self.entry()),
      _ => super::super::resource_locator::directive::DirectiveOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn directive_case(&self) -> super::super::resource_locator::directive::DirectiveCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::resource_locator::directive::DirectiveCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `DirectiveMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DirectiveMut<'_> {}

// SAFETY:
// - `DirectiveMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DirectiveMut<'_> {}

impl<'msg> ::protobuf::AsView for DirectiveMut<'msg> {
  type Proxied = Directive;
  fn as_view(&self) -> ::protobuf::View<'_, Directive> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DirectiveMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Directive>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DirectiveMut<'msg> {
  type MutProxied = Directive;
  fn as_mut(&mut self) -> DirectiveMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DirectiveMut<'msg> {
  fn into_mut<'shorter>(self) -> DirectiveMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Directive {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Directive> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DirectiveView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DirectiveMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // alt: optional message xds.core.v3.ResourceLocator
  pub fn has_alt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_alt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn alt_opt(&self) -> ::std::option::Option<super::super::ResourceLocatorView<'_>> {
    self.has_alt().then(|| self.alt())
  }
  pub fn alt(&self) -> super::super::ResourceLocatorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::ResourceLocatorView::default())
  }
  pub fn alt_mut(&mut self) -> super::super::ResourceLocatorMut<'_> {
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
  pub fn set_alt(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::ResourceLocator>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // entry: optional string
  pub fn has_entry(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_entry(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn entry_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_entry().then(|| self.entry())
  }
  pub fn entry(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_entry(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  pub fn directive(&self) -> super::super::resource_locator::directive::DirectiveOneof<'_> {
    match &self.directive_case() {
      super::super::resource_locator::directive::DirectiveCase::Alt =>
          super::super::resource_locator::directive::DirectiveOneof::Alt(self.alt()),
      super::super::resource_locator::directive::DirectiveCase::Entry =>
          super::super::resource_locator::directive::DirectiveOneof::Entry(self.entry()),
      _ => super::super::resource_locator::directive::DirectiveOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn directive_case(&self) -> super::super::resource_locator::directive::DirectiveCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::resource_locator::directive::DirectiveCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl Directive

impl ::std::ops::Drop for Directive {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Directive {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Directive {
  type Proxied = Self;
  fn as_view(&self) -> DirectiveView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Directive {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DirectiveMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Directive {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::super::ResourceLocator as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::resource_locator::xds__core__v3__ResourceLocator__Directive_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Directive {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Directive {
  type Msg = Directive;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Directive> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Directive {
  type Msg = Directive;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Directive> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DirectiveMut<'_> {
  type Msg = Directive;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Directive> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DirectiveMut<'_> {
  type Msg = Directive;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Directive> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DirectiveView<'_> {
  type Msg = Directive;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Directive> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DirectiveMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod directive {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum DirectiveOneof<'msg> {
  Alt(::protobuf::View<'msg, super::super::super::ResourceLocator>) = 1,
  Entry(&'msg ::protobuf::ProtoStr) = 2,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum DirectiveCase {
  Alt = 1,
  Entry = 2,

  not_set = 0
}

impl DirectiveCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<DirectiveCase> {
    match v {
      0 => Some(DirectiveCase::not_set),
      1 => Some(DirectiveCase::Alt),
      2 => Some(DirectiveCase::Entry),
      _ => None
    }
  }
}
}  // pub mod directive

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Scheme(i32);

#[allow(non_upper_case_globals)]
impl Scheme {
  pub const Xdstp: Scheme = Scheme(0);
  pub const Http: Scheme = Scheme(1);
  pub const File: Scheme = Scheme(2);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Xdstp",
      1 => "Http",
      2 => "File",
      _ => return None
    })
  }
}

impl ::std::convert::From<Scheme> for i32 {
  fn from(val: Scheme) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for Scheme {
  fn from(val: i32) -> Scheme {
    Self(val)
  }
}

impl ::std::default::Default for Scheme {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for Scheme {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "Scheme::{}", constant_name)
    } else {
      write!(f, "Scheme::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for Scheme {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for Scheme {}

impl ::protobuf::Proxied for Scheme {
  type View<'a> = Scheme;
}

impl ::protobuf::AsView for Scheme {
  type Proxied = Scheme;

  fn as_view(&self) -> Scheme {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Scheme {
  fn into_view<'shorter>(self) -> Scheme where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for Scheme {
  const NAME: &'static str = "Scheme";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2)
  }
}

impl ::protobuf::__internal::EntityType for Scheme {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ContextParamSpecifierOneof<'msg> {
  ExactContext(::protobuf::View<'msg, crate::xds::generated::xds::core::v3::context_params::ContextParams>) = 5,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ContextParamSpecifierCase {
  ExactContext = 5,

  not_set = 0
}

impl ContextParamSpecifierCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ContextParamSpecifierCase> {
    match v {
      0 => Some(ContextParamSpecifierCase::not_set),
      5 => Some(ContextParamSpecifierCase::ExactContext),
      _ => None
    }
  }
}
}  // pub mod resource_locator


