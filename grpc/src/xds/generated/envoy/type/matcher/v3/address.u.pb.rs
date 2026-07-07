const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__matcher__v3__AddressMatcher_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AddressMatcher {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AddressMatcher>
}

impl ::protobuf::Message for AddressMatcher {
  type MessageView<'msg> = AddressMatcherView<'msg>;
  type MessageMut<'msg> = AddressMatcherMut<'msg>;
}

impl ::std::default::Default for AddressMatcher {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AddressMatcher {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AddressMatcher` is `Sync` because it does not implement interior mutability.
//    Neither does `AddressMatcherMut`.
unsafe impl ::std::marker::Sync for AddressMatcher {}

// SAFETY:
// - `AddressMatcher` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for AddressMatcher {}

impl ::protobuf::Proxied for AddressMatcher {
  type View<'msg> = AddressMatcherView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AddressMatcher {}

impl ::protobuf::MutProxied for AddressMatcher {
  type Mut<'msg> = AddressMatcherMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AddressMatcherView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AddressMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AddressMatcherView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AddressMatcherView<'msg> {
  type Message = AddressMatcher;
}

impl ::std::fmt::Debug for AddressMatcherView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AddressMatcherView<'_> {
  fn default() -> AddressMatcherView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AddressMatcher>> for AddressMatcherView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AddressMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AddressMatcherView<'msg> {

  pub fn to_owned(&self) -> AddressMatcher {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // ranges: repeated message xds.core.v3.CidrRange
  pub fn ranges(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::xds::core::v3::cidr::CidrRange> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::xds::core::v3::cidr::CidrRange>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `AddressMatcherView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AddressMatcherView<'_> {}

// SAFETY:
// - `AddressMatcherView` is `Send` because while its alive a `AddressMatcherMut` cannot.
// - `AddressMatcherView` does not use thread-local data.
unsafe impl ::std::marker::Send for AddressMatcherView<'_> {}

impl<'msg> ::protobuf::AsView for AddressMatcherView<'msg> {
  type Proxied = AddressMatcher;
  fn as_view(&self) -> ::protobuf::View<'msg, AddressMatcher> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AddressMatcherView<'msg> {
  fn into_view<'shorter>(self) -> AddressMatcherView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AddressMatcher> for AddressMatcherView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AddressMatcher {
    let mut dst = AddressMatcher::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AddressMatcher> for AddressMatcherMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AddressMatcher {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for AddressMatcher {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AddressMatcherView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AddressMatcherMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AddressMatcherMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AddressMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AddressMatcherMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AddressMatcherMut<'msg> {
  type Message = AddressMatcher;
}

impl ::std::fmt::Debug for AddressMatcherMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AddressMatcher>> for AddressMatcherMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AddressMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AddressMatcherMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AddressMatcher> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> AddressMatcher {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // ranges: repeated message xds.core.v3.CidrRange
  pub fn ranges(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::xds::core::v3::cidr::CidrRange> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::xds::core::v3::cidr::CidrRange>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn ranges_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::xds::core::v3::cidr::CidrRange> {
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
  pub fn set_ranges(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::xds::core::v3::cidr::CidrRange>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `AddressMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AddressMatcherMut<'_> {}

// SAFETY:
// - `AddressMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AddressMatcherMut<'_> {}

impl<'msg> ::protobuf::AsView for AddressMatcherMut<'msg> {
  type Proxied = AddressMatcher;
  fn as_view(&self) -> ::protobuf::View<'_, AddressMatcher> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AddressMatcherMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AddressMatcher>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AddressMatcherMut<'msg> {
  type MutProxied = AddressMatcher;
  fn as_mut(&mut self) -> AddressMatcherMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AddressMatcherMut<'msg> {
  fn into_mut<'shorter>(self) -> AddressMatcherMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AddressMatcher {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AddressMatcher> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AddressMatcherView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AddressMatcherMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // ranges: repeated message xds.core.v3.CidrRange
  pub fn ranges(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::xds::core::v3::cidr::CidrRange> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::xds::core::v3::cidr::CidrRange>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn ranges_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::xds::core::v3::cidr::CidrRange> {
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
  pub fn set_ranges(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::xds::core::v3::cidr::CidrRange>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl AddressMatcher

impl ::std::ops::Drop for AddressMatcher {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AddressMatcher {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AddressMatcher {
  type Proxied = Self;
  fn as_view(&self) -> AddressMatcherView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AddressMatcher {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AddressMatcherMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AddressMatcher {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__type__matcher__v3__AddressMatcher_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__matcher__v3__AddressMatcher_msg_init.0, &[<crate::xds::generated::xds::core::v3::cidr::CidrRange as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__matcher__v3__AddressMatcher_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AddressMatcher {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AddressMatcher {
  type Msg = AddressMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AddressMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AddressMatcher {
  type Msg = AddressMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AddressMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AddressMatcherMut<'_> {
  type Msg = AddressMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AddressMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AddressMatcherMut<'_> {
  type Msg = AddressMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AddressMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AddressMatcherView<'_> {
  type Msg = AddressMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AddressMatcher> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AddressMatcherMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



