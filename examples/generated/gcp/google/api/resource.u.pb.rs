const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.34.0-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__api__ResourceDescriptor_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ResourceDescriptor {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ResourceDescriptor>
}

impl ::protobuf::Message for ResourceDescriptor {}

impl ::std::default::Default for ResourceDescriptor {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ResourceDescriptor {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ResourceDescriptor` is `Sync` because it does not implement interior mutability.
//    Neither does `ResourceDescriptorMut`.
unsafe impl Sync for ResourceDescriptor {}

// SAFETY:
// - `ResourceDescriptor` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ResourceDescriptor {}

impl ::protobuf::Proxied for ResourceDescriptor {
  type View<'msg> = ResourceDescriptorView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ResourceDescriptor {}

impl ::protobuf::MutProxied for ResourceDescriptor {
  type Mut<'msg> = ResourceDescriptorMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ResourceDescriptorView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ResourceDescriptor>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ResourceDescriptorView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ResourceDescriptorView<'msg> {
  type Message = ResourceDescriptor;
}

impl ::std::fmt::Debug for ResourceDescriptorView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ResourceDescriptorView<'_> {
  fn default() -> ResourceDescriptorView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ResourceDescriptor>> for ResourceDescriptorView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ResourceDescriptor>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ResourceDescriptorView<'msg> {

  pub fn to_owned(&self) -> ResourceDescriptor {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // type: optional string
  pub fn r#type(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // pattern: repeated string
  pub fn pattern(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
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

  // name_field: optional string
  pub fn name_field(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // history: optional enum google.api.ResourceDescriptor.History
  pub fn history(self) -> super::resource_descriptor::History {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (super::resource_descriptor::History::Unspecified).into()
      ).try_into().unwrap()
    }
  }

  // plural: optional string
  pub fn plural(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // singular: optional string
  pub fn singular(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // style: repeated enum google.api.ResourceDescriptor.Style
  pub fn style(self) -> ::protobuf::RepeatedView<'msg, super::resource_descriptor::Style> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::resource_descriptor::Style>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `ResourceDescriptorView` is `Sync` because it does not support mutation.
unsafe impl Sync for ResourceDescriptorView<'_> {}

// SAFETY:
// - `ResourceDescriptorView` is `Send` because while its alive a `ResourceDescriptorMut` cannot.
// - `ResourceDescriptorView` does not use thread-local data.
unsafe impl Send for ResourceDescriptorView<'_> {}

impl<'msg> ::protobuf::AsView for ResourceDescriptorView<'msg> {
  type Proxied = ResourceDescriptor;
  fn as_view(&self) -> ::protobuf::View<'msg, ResourceDescriptor> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ResourceDescriptorView<'msg> {
  fn into_view<'shorter>(self) -> ResourceDescriptorView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ResourceDescriptor> for ResourceDescriptorView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ResourceDescriptor {
    let mut dst = ResourceDescriptor::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ResourceDescriptor> for ResourceDescriptorMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ResourceDescriptor {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ResourceDescriptor {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ResourceDescriptorView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ResourceDescriptorMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ResourceDescriptorMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceDescriptor>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ResourceDescriptorMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ResourceDescriptorMut<'msg> {
  type Message = ResourceDescriptor;
}

impl ::std::fmt::Debug for ResourceDescriptorMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceDescriptor>> for ResourceDescriptorMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceDescriptor>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ResourceDescriptorMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceDescriptor> {
    self.inner
  }

  pub fn to_owned(&self) -> ResourceDescriptor {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // type: optional string
  pub fn r#type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // pattern: repeated string
  pub fn pattern(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn pattern_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_pattern(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // name_field: optional string
  pub fn name_field(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name_field(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // history: optional enum google.api.ResourceDescriptor.History
  pub fn history(&self) -> super::resource_descriptor::History {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (super::resource_descriptor::History::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_history(&mut self, val: super::resource_descriptor::History) {
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

  // plural: optional string
  pub fn plural(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_plural(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // singular: optional string
  pub fn singular(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_singular(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // style: repeated enum google.api.ResourceDescriptor.Style
  pub fn style(&self) -> ::protobuf::RepeatedView<'_, super::resource_descriptor::Style> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::resource_descriptor::Style>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn style_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::resource_descriptor::Style> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        6,
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
  pub fn set_style(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::resource_descriptor::Style>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

}

// SAFETY:
// - `ResourceDescriptorMut` does not perform any shared mutation.
unsafe impl Send for ResourceDescriptorMut<'_> {}

// SAFETY:
// - `ResourceDescriptorMut` does not perform any shared mutation.
unsafe impl Sync for ResourceDescriptorMut<'_> {}

impl<'msg> ::protobuf::AsView for ResourceDescriptorMut<'msg> {
  type Proxied = ResourceDescriptor;
  fn as_view(&self) -> ::protobuf::View<'_, ResourceDescriptor> {
    ResourceDescriptorView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ResourceDescriptorMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ResourceDescriptor>
  where
      'msg: 'shorter {
    ResourceDescriptorView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ResourceDescriptorMut<'msg> {
  type MutProxied = ResourceDescriptor;
  fn as_mut(&mut self) -> ResourceDescriptorMut<'msg> {
    ResourceDescriptorMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ResourceDescriptorMut<'msg> {
  fn into_mut<'shorter>(self) -> ResourceDescriptorMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ResourceDescriptor {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ResourceDescriptor> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ResourceDescriptorView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ResourceDescriptorMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // type: optional string
  pub fn r#type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // pattern: repeated string
  pub fn pattern(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn pattern_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_pattern(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // name_field: optional string
  pub fn name_field(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name_field(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // history: optional enum google.api.ResourceDescriptor.History
  pub fn history(&self) -> super::resource_descriptor::History {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (super::resource_descriptor::History::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_history(&mut self, val: super::resource_descriptor::History) {
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

  // plural: optional string
  pub fn plural(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_plural(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // singular: optional string
  pub fn singular(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_singular(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // style: repeated enum google.api.ResourceDescriptor.Style
  pub fn style(&self) -> ::protobuf::RepeatedView<'_, super::resource_descriptor::Style> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::resource_descriptor::Style>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn style_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::resource_descriptor::Style> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        6,
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
  pub fn set_style(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::resource_descriptor::Style>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

}  // impl ResourceDescriptor

impl ::std::ops::Drop for ResourceDescriptor {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ResourceDescriptor {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ResourceDescriptor {
  type Proxied = Self;
  fn as_view(&self) -> ResourceDescriptorView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ResourceDescriptor {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ResourceDescriptorMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ResourceDescriptor {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__api__ResourceDescriptor_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$N1XET1X.P1X1XcB");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__api__ResourceDescriptor_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__api__ResourceDescriptor_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ResourceDescriptor {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ResourceDescriptor {
  type Msg = ResourceDescriptor;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceDescriptor> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResourceDescriptor {
  type Msg = ResourceDescriptor;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceDescriptor> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ResourceDescriptorMut<'_> {
  type Msg = ResourceDescriptor;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceDescriptor> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResourceDescriptorMut<'_> {
  type Msg = ResourceDescriptor;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceDescriptor> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResourceDescriptorView<'_> {
  type Msg = ResourceDescriptor;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceDescriptor> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ResourceDescriptorMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod resource_descriptor {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct History(i32);

#[allow(non_upper_case_globals)]
impl History {
  pub const Unspecified: History = History(0);
  pub const OriginallySinglePattern: History = History(1);
  pub const FutureMultiPattern: History = History(2);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unspecified",
      1 => "OriginallySinglePattern",
      2 => "FutureMultiPattern",
      _ => return None
    })
  }
}

impl ::std::convert::From<History> for i32 {
  fn from(val: History) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for History {
  fn from(val: i32) -> History {
    Self(val)
  }
}

impl ::std::default::Default for History {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for History {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "History::{}", constant_name)
    } else {
      write!(f, "History::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for History {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for History {}

impl ::protobuf::Proxied for History {
  type View<'a> = History;
}

impl ::protobuf::AsView for History {
  type Proxied = History;

  fn as_view(&self) -> History {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for History {
  fn into_view<'shorter>(self) -> History where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for History {
  const NAME: &'static str = "History";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2)
  }
}

impl ::protobuf::__internal::runtime::EntityType for History {
    type Tag = ::protobuf::__internal::runtime::EnumTag;
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Style(i32);

#[allow(non_upper_case_globals)]
impl Style {
  pub const Unspecified: Style = Style(0);
  pub const DeclarativeFriendly: Style = Style(1);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unspecified",
      1 => "DeclarativeFriendly",
      _ => return None
    })
  }
}

impl ::std::convert::From<Style> for i32 {
  fn from(val: Style) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for Style {
  fn from(val: i32) -> Style {
    Self(val)
  }
}

impl ::std::default::Default for Style {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for Style {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "Style::{}", constant_name)
    } else {
      write!(f, "Style::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for Style {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for Style {}

impl ::protobuf::Proxied for Style {
  type View<'a> = Style;
}

impl ::protobuf::AsView for Style {
  type Proxied = Style;

  fn as_view(&self) -> Style {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Style {
  fn into_view<'shorter>(self) -> Style where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for Style {
  const NAME: &'static str = "Style";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Style {
    type Tag = ::protobuf::__internal::runtime::EnumTag;
}


}  // pub mod resource_descriptor


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__api__ResourceReference_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ResourceReference {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ResourceReference>
}

impl ::protobuf::Message for ResourceReference {}

impl ::std::default::Default for ResourceReference {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ResourceReference {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ResourceReference` is `Sync` because it does not implement interior mutability.
//    Neither does `ResourceReferenceMut`.
unsafe impl Sync for ResourceReference {}

// SAFETY:
// - `ResourceReference` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ResourceReference {}

impl ::protobuf::Proxied for ResourceReference {
  type View<'msg> = ResourceReferenceView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ResourceReference {}

impl ::protobuf::MutProxied for ResourceReference {
  type Mut<'msg> = ResourceReferenceMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ResourceReferenceView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ResourceReference>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ResourceReferenceView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ResourceReferenceView<'msg> {
  type Message = ResourceReference;
}

impl ::std::fmt::Debug for ResourceReferenceView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ResourceReferenceView<'_> {
  fn default() -> ResourceReferenceView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ResourceReference>> for ResourceReferenceView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ResourceReference>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ResourceReferenceView<'msg> {

  pub fn to_owned(&self) -> ResourceReference {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // type: optional string
  pub fn r#type(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // child_type: optional string
  pub fn child_type(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `ResourceReferenceView` is `Sync` because it does not support mutation.
unsafe impl Sync for ResourceReferenceView<'_> {}

// SAFETY:
// - `ResourceReferenceView` is `Send` because while its alive a `ResourceReferenceMut` cannot.
// - `ResourceReferenceView` does not use thread-local data.
unsafe impl Send for ResourceReferenceView<'_> {}

impl<'msg> ::protobuf::AsView for ResourceReferenceView<'msg> {
  type Proxied = ResourceReference;
  fn as_view(&self) -> ::protobuf::View<'msg, ResourceReference> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ResourceReferenceView<'msg> {
  fn into_view<'shorter>(self) -> ResourceReferenceView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ResourceReference> for ResourceReferenceView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ResourceReference {
    let mut dst = ResourceReference::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ResourceReference> for ResourceReferenceMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ResourceReference {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ResourceReference {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ResourceReferenceView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ResourceReferenceMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ResourceReferenceMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceReference>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ResourceReferenceMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ResourceReferenceMut<'msg> {
  type Message = ResourceReference;
}

impl ::std::fmt::Debug for ResourceReferenceMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceReference>> for ResourceReferenceMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceReference>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ResourceReferenceMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceReference> {
    self.inner
  }

  pub fn to_owned(&self) -> ResourceReference {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // type: optional string
  pub fn r#type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // child_type: optional string
  pub fn child_type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_child_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `ResourceReferenceMut` does not perform any shared mutation.
unsafe impl Send for ResourceReferenceMut<'_> {}

// SAFETY:
// - `ResourceReferenceMut` does not perform any shared mutation.
unsafe impl Sync for ResourceReferenceMut<'_> {}

impl<'msg> ::protobuf::AsView for ResourceReferenceMut<'msg> {
  type Proxied = ResourceReference;
  fn as_view(&self) -> ::protobuf::View<'_, ResourceReference> {
    ResourceReferenceView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ResourceReferenceMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ResourceReference>
  where
      'msg: 'shorter {
    ResourceReferenceView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ResourceReferenceMut<'msg> {
  type MutProxied = ResourceReference;
  fn as_mut(&mut self) -> ResourceReferenceMut<'msg> {
    ResourceReferenceMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ResourceReferenceMut<'msg> {
  fn into_mut<'shorter>(self) -> ResourceReferenceMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ResourceReference {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ResourceReference> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ResourceReferenceView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ResourceReferenceMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // type: optional string
  pub fn r#type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // child_type: optional string
  pub fn child_type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_child_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl ResourceReference

impl ::std::ops::Drop for ResourceReference {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ResourceReference {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ResourceReference {
  type Proxied = Self;
  fn as_view(&self) -> ResourceReferenceView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ResourceReference {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ResourceReferenceMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ResourceReference {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__api__ResourceReference_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__api__ResourceReference_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__api__ResourceReference_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ResourceReference {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ResourceReference {
  type Msg = ResourceReference;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceReference> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResourceReference {
  type Msg = ResourceReference;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceReference> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ResourceReferenceMut<'_> {
  type Msg = ResourceReference;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceReference> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResourceReferenceMut<'_> {
  type Msg = ResourceReference;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceReference> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResourceReferenceView<'_> {
  type Msg = ResourceReference;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceReference> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ResourceReferenceMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



