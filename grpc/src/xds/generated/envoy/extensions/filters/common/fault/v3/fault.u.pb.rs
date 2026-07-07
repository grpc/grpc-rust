const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__common__fault__v3__FaultDelay_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct FaultDelay {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<FaultDelay>
}

impl ::protobuf::Message for FaultDelay {
  type MessageView<'msg> = FaultDelayView<'msg>;
  type MessageMut<'msg> = FaultDelayMut<'msg>;
}

impl ::std::default::Default for FaultDelay {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for FaultDelay {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `FaultDelay` is `Sync` because it does not implement interior mutability.
//    Neither does `FaultDelayMut`.
unsafe impl ::std::marker::Sync for FaultDelay {}

// SAFETY:
// - `FaultDelay` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for FaultDelay {}

impl ::protobuf::Proxied for FaultDelay {
  type View<'msg> = FaultDelayView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for FaultDelay {}

impl ::protobuf::MutProxied for FaultDelay {
  type Mut<'msg> = FaultDelayMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FaultDelayView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FaultDelay>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FaultDelayView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FaultDelayView<'msg> {
  type Message = FaultDelay;
}

impl ::std::fmt::Debug for FaultDelayView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FaultDelayView<'_> {
  fn default() -> FaultDelayView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, FaultDelay>> for FaultDelayView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FaultDelay>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FaultDelayView<'msg> {

  pub fn to_owned(&self) -> FaultDelay {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // fixed_delay: optional message google.protobuf.Duration
  pub fn has_fixed_delay(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn fixed_delay_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_fixed_delay().then(|| self.fixed_delay())
  }
  pub fn fixed_delay(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // header_delay: optional message envoy.extensions.filters.common.fault.v3.FaultDelay.HeaderDelay
  pub fn has_header_delay(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn header_delay_opt(self) -> ::std::option::Option<super::fault_delay::HeaderDelayView<'msg>> {
    self.has_header_delay().then(|| self.header_delay())
  }
  pub fn header_delay(self) -> super::fault_delay::HeaderDelayView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::fault_delay::HeaderDelayView::default())
  }

  // percentage: optional message envoy.type.v3.FractionalPercent
  pub fn has_percentage(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn percentage_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'msg>> {
    self.has_percentage().then(|| self.percentage())
  }
  pub fn percentage(self) -> crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView::default())
  }

  pub fn fault_delay_secifier(self) -> super::fault_delay::FaultDelaySecifierOneof<'msg> {
    match self.fault_delay_secifier_case() {
      super::fault_delay::FaultDelaySecifierCase::FixedDelay =>
          super::fault_delay::FaultDelaySecifierOneof::FixedDelay(self.fixed_delay()),
      super::fault_delay::FaultDelaySecifierCase::HeaderDelay =>
          super::fault_delay::FaultDelaySecifierOneof::HeaderDelay(self.header_delay()),
      _ => super::fault_delay::FaultDelaySecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn fault_delay_secifier_case(self) -> super::fault_delay::FaultDelaySecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::fault_delay::FaultDelaySecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `FaultDelayView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for FaultDelayView<'_> {}

// SAFETY:
// - `FaultDelayView` is `Send` because while its alive a `FaultDelayMut` cannot.
// - `FaultDelayView` does not use thread-local data.
unsafe impl ::std::marker::Send for FaultDelayView<'_> {}

impl<'msg> ::protobuf::AsView for FaultDelayView<'msg> {
  type Proxied = FaultDelay;
  fn as_view(&self) -> ::protobuf::View<'msg, FaultDelay> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FaultDelayView<'msg> {
  fn into_view<'shorter>(self) -> FaultDelayView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<FaultDelay> for FaultDelayView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FaultDelay {
    let mut dst = FaultDelay::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<FaultDelay> for FaultDelayMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FaultDelay {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for FaultDelay {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FaultDelayView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FaultDelayMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FaultDelayMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FaultDelay>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FaultDelayMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FaultDelayMut<'msg> {
  type Message = FaultDelay;
}

impl ::std::fmt::Debug for FaultDelayMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, FaultDelay>> for FaultDelayMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FaultDelay>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FaultDelayMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, FaultDelay> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> FaultDelay {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // fixed_delay: optional message google.protobuf.Duration
  pub fn has_fixed_delay(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_fixed_delay(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn fixed_delay_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_fixed_delay().then(|| self.fixed_delay())
  }
  pub fn fixed_delay(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn fixed_delay_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_fixed_delay(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // header_delay: optional message envoy.extensions.filters.common.fault.v3.FaultDelay.HeaderDelay
  pub fn has_header_delay(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_header_delay(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn header_delay_opt(&self) -> ::std::option::Option<super::fault_delay::HeaderDelayView<'_>> {
    self.has_header_delay().then(|| self.header_delay())
  }
  pub fn header_delay(&self) -> super::fault_delay::HeaderDelayView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::fault_delay::HeaderDelayView::default())
  }
  pub fn header_delay_mut(&mut self) -> super::fault_delay::HeaderDelayMut<'_> {
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
  pub fn set_header_delay(&mut self,
    val: impl ::protobuf::IntoProxied<super::fault_delay::HeaderDelay>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // percentage: optional message envoy.type.v3.FractionalPercent
  pub fn has_percentage(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_percentage(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn percentage_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'_>> {
    self.has_percentage().then(|| self.percentage())
  }
  pub fn percentage(&self) -> crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView::default())
  }
  pub fn percentage_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentMut<'_> {
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
  pub fn set_percentage(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn fault_delay_secifier(&self) -> super::fault_delay::FaultDelaySecifierOneof<'_> {
    match &self.fault_delay_secifier_case() {
      super::fault_delay::FaultDelaySecifierCase::FixedDelay =>
          super::fault_delay::FaultDelaySecifierOneof::FixedDelay(self.fixed_delay()),
      super::fault_delay::FaultDelaySecifierCase::HeaderDelay =>
          super::fault_delay::FaultDelaySecifierOneof::HeaderDelay(self.header_delay()),
      _ => super::fault_delay::FaultDelaySecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn fault_delay_secifier_case(&self) -> super::fault_delay::FaultDelaySecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::fault_delay::FaultDelaySecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `FaultDelayMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for FaultDelayMut<'_> {}

// SAFETY:
// - `FaultDelayMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for FaultDelayMut<'_> {}

impl<'msg> ::protobuf::AsView for FaultDelayMut<'msg> {
  type Proxied = FaultDelay;
  fn as_view(&self) -> ::protobuf::View<'_, FaultDelay> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FaultDelayMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, FaultDelay>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for FaultDelayMut<'msg> {
  type MutProxied = FaultDelay;
  fn as_mut(&mut self) -> FaultDelayMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FaultDelayMut<'msg> {
  fn into_mut<'shorter>(self) -> FaultDelayMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl FaultDelay {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, FaultDelay> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FaultDelayView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FaultDelayMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // fixed_delay: optional message google.protobuf.Duration
  pub fn has_fixed_delay(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_fixed_delay(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn fixed_delay_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_fixed_delay().then(|| self.fixed_delay())
  }
  pub fn fixed_delay(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn fixed_delay_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_fixed_delay(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // header_delay: optional message envoy.extensions.filters.common.fault.v3.FaultDelay.HeaderDelay
  pub fn has_header_delay(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_header_delay(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn header_delay_opt(&self) -> ::std::option::Option<super::fault_delay::HeaderDelayView<'_>> {
    self.has_header_delay().then(|| self.header_delay())
  }
  pub fn header_delay(&self) -> super::fault_delay::HeaderDelayView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::fault_delay::HeaderDelayView::default())
  }
  pub fn header_delay_mut(&mut self) -> super::fault_delay::HeaderDelayMut<'_> {
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
  pub fn set_header_delay(&mut self,
    val: impl ::protobuf::IntoProxied<super::fault_delay::HeaderDelay>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // percentage: optional message envoy.type.v3.FractionalPercent
  pub fn has_percentage(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_percentage(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn percentage_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'_>> {
    self.has_percentage().then(|| self.percentage())
  }
  pub fn percentage(&self) -> crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView::default())
  }
  pub fn percentage_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentMut<'_> {
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
  pub fn set_percentage(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn fault_delay_secifier(&self) -> super::fault_delay::FaultDelaySecifierOneof<'_> {
    match &self.fault_delay_secifier_case() {
      super::fault_delay::FaultDelaySecifierCase::FixedDelay =>
          super::fault_delay::FaultDelaySecifierOneof::FixedDelay(self.fixed_delay()),
      super::fault_delay::FaultDelaySecifierCase::HeaderDelay =>
          super::fault_delay::FaultDelaySecifierOneof::HeaderDelay(self.header_delay()),
      _ => super::fault_delay::FaultDelaySecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn fault_delay_secifier_case(&self) -> super::fault_delay::FaultDelaySecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::fault_delay::FaultDelaySecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl FaultDelay

impl ::std::ops::Drop for FaultDelay {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for FaultDelay {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for FaultDelay {
  type Proxied = Self;
  fn as_view(&self) -> FaultDelayView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for FaultDelay {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FaultDelayMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FaultDelay {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__common__fault__v3__FaultDelay_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$b333^$|&");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__common__fault__v3__FaultDelay_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::v3::percent::FractionalPercent as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::fault_delay::HeaderDelay as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__common__fault__v3__FaultDelay_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FaultDelay {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FaultDelay {
  type Msg = FaultDelay;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FaultDelay> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FaultDelay {
  type Msg = FaultDelay;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FaultDelay> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FaultDelayMut<'_> {
  type Msg = FaultDelay;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FaultDelay> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FaultDelayMut<'_> {
  type Msg = FaultDelay;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FaultDelay> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FaultDelayView<'_> {
  type Msg = FaultDelay;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FaultDelay> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FaultDelayMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod fault_delay {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__common__fault__v3__FaultDelay__HeaderDelay_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HeaderDelay {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HeaderDelay>
}

impl ::protobuf::Message for HeaderDelay {
  type MessageView<'msg> = HeaderDelayView<'msg>;
  type MessageMut<'msg> = HeaderDelayMut<'msg>;
}

impl ::std::default::Default for HeaderDelay {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HeaderDelay {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HeaderDelay` is `Sync` because it does not implement interior mutability.
//    Neither does `HeaderDelayMut`.
unsafe impl ::std::marker::Sync for HeaderDelay {}

// SAFETY:
// - `HeaderDelay` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HeaderDelay {}

impl ::protobuf::Proxied for HeaderDelay {
  type View<'msg> = HeaderDelayView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HeaderDelay {}

impl ::protobuf::MutProxied for HeaderDelay {
  type Mut<'msg> = HeaderDelayMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HeaderDelayView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderDelay>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeaderDelayView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HeaderDelayView<'msg> {
  type Message = HeaderDelay;
}

impl ::std::fmt::Debug for HeaderDelayView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HeaderDelayView<'_> {
  fn default() -> HeaderDelayView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderDelay>> for HeaderDelayView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderDelay>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HeaderDelayView<'msg> {

  pub fn to_owned(&self) -> HeaderDelay {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `HeaderDelayView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HeaderDelayView<'_> {}

// SAFETY:
// - `HeaderDelayView` is `Send` because while its alive a `HeaderDelayMut` cannot.
// - `HeaderDelayView` does not use thread-local data.
unsafe impl ::std::marker::Send for HeaderDelayView<'_> {}

impl<'msg> ::protobuf::AsView for HeaderDelayView<'msg> {
  type Proxied = HeaderDelay;
  fn as_view(&self) -> ::protobuf::View<'msg, HeaderDelay> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeaderDelayView<'msg> {
  fn into_view<'shorter>(self) -> HeaderDelayView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HeaderDelay> for HeaderDelayView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HeaderDelay {
    let mut dst = HeaderDelay::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HeaderDelay> for HeaderDelayMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HeaderDelay {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HeaderDelay {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HeaderDelayView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HeaderDelayMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HeaderDelayMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderDelay>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeaderDelayMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HeaderDelayMut<'msg> {
  type Message = HeaderDelay;
}

impl ::std::fmt::Debug for HeaderDelayMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderDelay>> for HeaderDelayMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderDelay>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HeaderDelayMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderDelay> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HeaderDelay {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `HeaderDelayMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HeaderDelayMut<'_> {}

// SAFETY:
// - `HeaderDelayMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HeaderDelayMut<'_> {}

impl<'msg> ::protobuf::AsView for HeaderDelayMut<'msg> {
  type Proxied = HeaderDelay;
  fn as_view(&self) -> ::protobuf::View<'_, HeaderDelay> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeaderDelayMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HeaderDelay>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HeaderDelayMut<'msg> {
  type MutProxied = HeaderDelay;
  fn as_mut(&mut self) -> HeaderDelayMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HeaderDelayMut<'msg> {
  fn into_mut<'shorter>(self) -> HeaderDelayMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HeaderDelay {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HeaderDelay> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HeaderDelayView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HeaderDelayMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl HeaderDelay

impl ::std::ops::Drop for HeaderDelay {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HeaderDelay {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HeaderDelay {
  type Proxied = Self;
  fn as_view(&self) -> HeaderDelayView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HeaderDelay {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HeaderDelayMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HeaderDelay {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::fault_delay::envoy__extensions__filters__common__fault__v3__FaultDelay__HeaderDelay_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::fault_delay::envoy__extensions__filters__common__fault__v3__FaultDelay__HeaderDelay_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::fault_delay::envoy__extensions__filters__common__fault__v3__FaultDelay__HeaderDelay_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HeaderDelay {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HeaderDelay {
  type Msg = HeaderDelay;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderDelay> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderDelay {
  type Msg = HeaderDelay;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderDelay> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HeaderDelayMut<'_> {
  type Msg = HeaderDelay;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderDelay> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderDelayMut<'_> {
  type Msg = HeaderDelay;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderDelay> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderDelayView<'_> {
  type Msg = HeaderDelay;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderDelay> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HeaderDelayMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FaultDelayType(i32);

#[allow(non_upper_case_globals)]
impl FaultDelayType {
  pub const Fixed: FaultDelayType = FaultDelayType(0);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Fixed",
      _ => return None
    })
  }
}

impl ::std::convert::From<FaultDelayType> for i32 {
  fn from(val: FaultDelayType) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for FaultDelayType {
  fn from(val: i32) -> FaultDelayType {
    Self(val)
  }
}

impl ::std::default::Default for FaultDelayType {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for FaultDelayType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "FaultDelayType::{}", constant_name)
    } else {
      write!(f, "FaultDelayType::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for FaultDelayType {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for FaultDelayType {}

impl ::protobuf::Proxied for FaultDelayType {
  type View<'a> = FaultDelayType;
}

impl ::protobuf::AsView for FaultDelayType {
  type Proxied = FaultDelayType;

  fn as_view(&self) -> FaultDelayType {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FaultDelayType {
  fn into_view<'shorter>(self) -> FaultDelayType where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for FaultDelayType {
  const NAME: &'static str = "FaultDelayType";

  fn is_known(value: i32) -> bool {
    matches!(value, 0)
  }
}

impl ::protobuf::__internal::EntityType for FaultDelayType {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum FaultDelaySecifierOneof<'msg> {
  FixedDelay(::protobuf::View<'msg, ::protobuf_well_known_types::Duration>) = 3,
  HeaderDelay(::protobuf::View<'msg, super::super::fault_delay::HeaderDelay>) = 5,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum FaultDelaySecifierCase {
  FixedDelay = 3,
  HeaderDelay = 5,

  not_set = 0
}

impl FaultDelaySecifierCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<FaultDelaySecifierCase> {
    match v {
      0 => Some(FaultDelaySecifierCase::not_set),
      3 => Some(FaultDelaySecifierCase::FixedDelay),
      5 => Some(FaultDelaySecifierCase::HeaderDelay),
      _ => None
    }
  }
}
}  // pub mod fault_delay


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__common__fault__v3__FaultRateLimit_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct FaultRateLimit {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<FaultRateLimit>
}

impl ::protobuf::Message for FaultRateLimit {
  type MessageView<'msg> = FaultRateLimitView<'msg>;
  type MessageMut<'msg> = FaultRateLimitMut<'msg>;
}

impl ::std::default::Default for FaultRateLimit {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for FaultRateLimit {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `FaultRateLimit` is `Sync` because it does not implement interior mutability.
//    Neither does `FaultRateLimitMut`.
unsafe impl ::std::marker::Sync for FaultRateLimit {}

// SAFETY:
// - `FaultRateLimit` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for FaultRateLimit {}

impl ::protobuf::Proxied for FaultRateLimit {
  type View<'msg> = FaultRateLimitView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for FaultRateLimit {}

impl ::protobuf::MutProxied for FaultRateLimit {
  type Mut<'msg> = FaultRateLimitMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FaultRateLimitView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FaultRateLimit>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FaultRateLimitView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FaultRateLimitView<'msg> {
  type Message = FaultRateLimit;
}

impl ::std::fmt::Debug for FaultRateLimitView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FaultRateLimitView<'_> {
  fn default() -> FaultRateLimitView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, FaultRateLimit>> for FaultRateLimitView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FaultRateLimit>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FaultRateLimitView<'msg> {

  pub fn to_owned(&self) -> FaultRateLimit {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // fixed_limit: optional message envoy.extensions.filters.common.fault.v3.FaultRateLimit.FixedLimit
  pub fn has_fixed_limit(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn fixed_limit_opt(self) -> ::std::option::Option<super::fault_rate_limit::FixedLimitView<'msg>> {
    self.has_fixed_limit().then(|| self.fixed_limit())
  }
  pub fn fixed_limit(self) -> super::fault_rate_limit::FixedLimitView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::fault_rate_limit::FixedLimitView::default())
  }

  // header_limit: optional message envoy.extensions.filters.common.fault.v3.FaultRateLimit.HeaderLimit
  pub fn has_header_limit(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn header_limit_opt(self) -> ::std::option::Option<super::fault_rate_limit::HeaderLimitView<'msg>> {
    self.has_header_limit().then(|| self.header_limit())
  }
  pub fn header_limit(self) -> super::fault_rate_limit::HeaderLimitView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::fault_rate_limit::HeaderLimitView::default())
  }

  // percentage: optional message envoy.type.v3.FractionalPercent
  pub fn has_percentage(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn percentage_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'msg>> {
    self.has_percentage().then(|| self.percentage())
  }
  pub fn percentage(self) -> crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView::default())
  }

  pub fn limit_type(self) -> super::fault_rate_limit::LimitTypeOneof<'msg> {
    match self.limit_type_case() {
      super::fault_rate_limit::LimitTypeCase::FixedLimit =>
          super::fault_rate_limit::LimitTypeOneof::FixedLimit(self.fixed_limit()),
      super::fault_rate_limit::LimitTypeCase::HeaderLimit =>
          super::fault_rate_limit::LimitTypeOneof::HeaderLimit(self.header_limit()),
      _ => super::fault_rate_limit::LimitTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn limit_type_case(self) -> super::fault_rate_limit::LimitTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::fault_rate_limit::LimitTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `FaultRateLimitView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for FaultRateLimitView<'_> {}

// SAFETY:
// - `FaultRateLimitView` is `Send` because while its alive a `FaultRateLimitMut` cannot.
// - `FaultRateLimitView` does not use thread-local data.
unsafe impl ::std::marker::Send for FaultRateLimitView<'_> {}

impl<'msg> ::protobuf::AsView for FaultRateLimitView<'msg> {
  type Proxied = FaultRateLimit;
  fn as_view(&self) -> ::protobuf::View<'msg, FaultRateLimit> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FaultRateLimitView<'msg> {
  fn into_view<'shorter>(self) -> FaultRateLimitView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<FaultRateLimit> for FaultRateLimitView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FaultRateLimit {
    let mut dst = FaultRateLimit::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<FaultRateLimit> for FaultRateLimitMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FaultRateLimit {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for FaultRateLimit {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FaultRateLimitView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FaultRateLimitMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FaultRateLimitMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FaultRateLimit>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FaultRateLimitMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FaultRateLimitMut<'msg> {
  type Message = FaultRateLimit;
}

impl ::std::fmt::Debug for FaultRateLimitMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, FaultRateLimit>> for FaultRateLimitMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FaultRateLimit>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FaultRateLimitMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, FaultRateLimit> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> FaultRateLimit {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // fixed_limit: optional message envoy.extensions.filters.common.fault.v3.FaultRateLimit.FixedLimit
  pub fn has_fixed_limit(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_fixed_limit(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn fixed_limit_opt(&self) -> ::std::option::Option<super::fault_rate_limit::FixedLimitView<'_>> {
    self.has_fixed_limit().then(|| self.fixed_limit())
  }
  pub fn fixed_limit(&self) -> super::fault_rate_limit::FixedLimitView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::fault_rate_limit::FixedLimitView::default())
  }
  pub fn fixed_limit_mut(&mut self) -> super::fault_rate_limit::FixedLimitMut<'_> {
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
  pub fn set_fixed_limit(&mut self,
    val: impl ::protobuf::IntoProxied<super::fault_rate_limit::FixedLimit>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // header_limit: optional message envoy.extensions.filters.common.fault.v3.FaultRateLimit.HeaderLimit
  pub fn has_header_limit(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_header_limit(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn header_limit_opt(&self) -> ::std::option::Option<super::fault_rate_limit::HeaderLimitView<'_>> {
    self.has_header_limit().then(|| self.header_limit())
  }
  pub fn header_limit(&self) -> super::fault_rate_limit::HeaderLimitView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::fault_rate_limit::HeaderLimitView::default())
  }
  pub fn header_limit_mut(&mut self) -> super::fault_rate_limit::HeaderLimitMut<'_> {
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
  pub fn set_header_limit(&mut self,
    val: impl ::protobuf::IntoProxied<super::fault_rate_limit::HeaderLimit>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // percentage: optional message envoy.type.v3.FractionalPercent
  pub fn has_percentage(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_percentage(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn percentage_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'_>> {
    self.has_percentage().then(|| self.percentage())
  }
  pub fn percentage(&self) -> crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView::default())
  }
  pub fn percentage_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentMut<'_> {
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
  pub fn set_percentage(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn limit_type(&self) -> super::fault_rate_limit::LimitTypeOneof<'_> {
    match &self.limit_type_case() {
      super::fault_rate_limit::LimitTypeCase::FixedLimit =>
          super::fault_rate_limit::LimitTypeOneof::FixedLimit(self.fixed_limit()),
      super::fault_rate_limit::LimitTypeCase::HeaderLimit =>
          super::fault_rate_limit::LimitTypeOneof::HeaderLimit(self.header_limit()),
      _ => super::fault_rate_limit::LimitTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn limit_type_case(&self) -> super::fault_rate_limit::LimitTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::fault_rate_limit::LimitTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `FaultRateLimitMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for FaultRateLimitMut<'_> {}

// SAFETY:
// - `FaultRateLimitMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for FaultRateLimitMut<'_> {}

impl<'msg> ::protobuf::AsView for FaultRateLimitMut<'msg> {
  type Proxied = FaultRateLimit;
  fn as_view(&self) -> ::protobuf::View<'_, FaultRateLimit> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FaultRateLimitMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, FaultRateLimit>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for FaultRateLimitMut<'msg> {
  type MutProxied = FaultRateLimit;
  fn as_mut(&mut self) -> FaultRateLimitMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FaultRateLimitMut<'msg> {
  fn into_mut<'shorter>(self) -> FaultRateLimitMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl FaultRateLimit {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, FaultRateLimit> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FaultRateLimitView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FaultRateLimitMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // fixed_limit: optional message envoy.extensions.filters.common.fault.v3.FaultRateLimit.FixedLimit
  pub fn has_fixed_limit(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_fixed_limit(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn fixed_limit_opt(&self) -> ::std::option::Option<super::fault_rate_limit::FixedLimitView<'_>> {
    self.has_fixed_limit().then(|| self.fixed_limit())
  }
  pub fn fixed_limit(&self) -> super::fault_rate_limit::FixedLimitView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::fault_rate_limit::FixedLimitView::default())
  }
  pub fn fixed_limit_mut(&mut self) -> super::fault_rate_limit::FixedLimitMut<'_> {
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
  pub fn set_fixed_limit(&mut self,
    val: impl ::protobuf::IntoProxied<super::fault_rate_limit::FixedLimit>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // header_limit: optional message envoy.extensions.filters.common.fault.v3.FaultRateLimit.HeaderLimit
  pub fn has_header_limit(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_header_limit(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn header_limit_opt(&self) -> ::std::option::Option<super::fault_rate_limit::HeaderLimitView<'_>> {
    self.has_header_limit().then(|| self.header_limit())
  }
  pub fn header_limit(&self) -> super::fault_rate_limit::HeaderLimitView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::fault_rate_limit::HeaderLimitView::default())
  }
  pub fn header_limit_mut(&mut self) -> super::fault_rate_limit::HeaderLimitMut<'_> {
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
  pub fn set_header_limit(&mut self,
    val: impl ::protobuf::IntoProxied<super::fault_rate_limit::HeaderLimit>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // percentage: optional message envoy.type.v3.FractionalPercent
  pub fn has_percentage(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_percentage(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn percentage_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'_>> {
    self.has_percentage().then(|| self.percentage())
  }
  pub fn percentage(&self) -> crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView::default())
  }
  pub fn percentage_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentMut<'_> {
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
  pub fn set_percentage(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn limit_type(&self) -> super::fault_rate_limit::LimitTypeOneof<'_> {
    match &self.limit_type_case() {
      super::fault_rate_limit::LimitTypeCase::FixedLimit =>
          super::fault_rate_limit::LimitTypeOneof::FixedLimit(self.fixed_limit()),
      super::fault_rate_limit::LimitTypeCase::HeaderLimit =>
          super::fault_rate_limit::LimitTypeOneof::HeaderLimit(self.header_limit()),
      _ => super::fault_rate_limit::LimitTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn limit_type_case(&self) -> super::fault_rate_limit::LimitTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::fault_rate_limit::LimitTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl FaultRateLimit

impl ::std::ops::Drop for FaultRateLimit {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for FaultRateLimit {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for FaultRateLimit {
  type Proxied = Self;
  fn as_view(&self) -> FaultRateLimitView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for FaultRateLimit {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FaultRateLimitMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FaultRateLimit {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__common__fault__v3__FaultRateLimit_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333^!|$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__common__fault__v3__FaultRateLimit_msg_init.0, &[<super::fault_rate_limit::FixedLimit as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::v3::percent::FractionalPercent as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::fault_rate_limit::HeaderLimit as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__common__fault__v3__FaultRateLimit_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FaultRateLimit {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FaultRateLimit {
  type Msg = FaultRateLimit;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FaultRateLimit> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FaultRateLimit {
  type Msg = FaultRateLimit;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FaultRateLimit> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FaultRateLimitMut<'_> {
  type Msg = FaultRateLimit;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FaultRateLimit> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FaultRateLimitMut<'_> {
  type Msg = FaultRateLimit;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FaultRateLimit> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FaultRateLimitView<'_> {
  type Msg = FaultRateLimit;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FaultRateLimit> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FaultRateLimitMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod fault_rate_limit {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__common__fault__v3__FaultRateLimit__FixedLimit_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct FixedLimit {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<FixedLimit>
}

impl ::protobuf::Message for FixedLimit {
  type MessageView<'msg> = FixedLimitView<'msg>;
  type MessageMut<'msg> = FixedLimitMut<'msg>;
}

impl ::std::default::Default for FixedLimit {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for FixedLimit {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `FixedLimit` is `Sync` because it does not implement interior mutability.
//    Neither does `FixedLimitMut`.
unsafe impl ::std::marker::Sync for FixedLimit {}

// SAFETY:
// - `FixedLimit` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for FixedLimit {}

impl ::protobuf::Proxied for FixedLimit {
  type View<'msg> = FixedLimitView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for FixedLimit {}

impl ::protobuf::MutProxied for FixedLimit {
  type Mut<'msg> = FixedLimitMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FixedLimitView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FixedLimit>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FixedLimitView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FixedLimitView<'msg> {
  type Message = FixedLimit;
}

impl ::std::fmt::Debug for FixedLimitView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FixedLimitView<'_> {
  fn default() -> FixedLimitView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, FixedLimit>> for FixedLimitView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FixedLimit>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FixedLimitView<'msg> {

  pub fn to_owned(&self) -> FixedLimit {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // limit_kbps: optional uint64
  pub fn limit_kbps(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `FixedLimitView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for FixedLimitView<'_> {}

// SAFETY:
// - `FixedLimitView` is `Send` because while its alive a `FixedLimitMut` cannot.
// - `FixedLimitView` does not use thread-local data.
unsafe impl ::std::marker::Send for FixedLimitView<'_> {}

impl<'msg> ::protobuf::AsView for FixedLimitView<'msg> {
  type Proxied = FixedLimit;
  fn as_view(&self) -> ::protobuf::View<'msg, FixedLimit> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FixedLimitView<'msg> {
  fn into_view<'shorter>(self) -> FixedLimitView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<FixedLimit> for FixedLimitView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FixedLimit {
    let mut dst = FixedLimit::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<FixedLimit> for FixedLimitMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FixedLimit {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for FixedLimit {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FixedLimitView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FixedLimitMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FixedLimitMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FixedLimit>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FixedLimitMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FixedLimitMut<'msg> {
  type Message = FixedLimit;
}

impl ::std::fmt::Debug for FixedLimitMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, FixedLimit>> for FixedLimitMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FixedLimit>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FixedLimitMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, FixedLimit> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> FixedLimit {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // limit_kbps: optional uint64
  pub fn limit_kbps(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_limit_kbps(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        0, val.into()
      )
    }
  }

}

// SAFETY:
// - `FixedLimitMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for FixedLimitMut<'_> {}

// SAFETY:
// - `FixedLimitMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for FixedLimitMut<'_> {}

impl<'msg> ::protobuf::AsView for FixedLimitMut<'msg> {
  type Proxied = FixedLimit;
  fn as_view(&self) -> ::protobuf::View<'_, FixedLimit> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FixedLimitMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, FixedLimit>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for FixedLimitMut<'msg> {
  type MutProxied = FixedLimit;
  fn as_mut(&mut self) -> FixedLimitMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FixedLimitMut<'msg> {
  fn into_mut<'shorter>(self) -> FixedLimitMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl FixedLimit {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, FixedLimit> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FixedLimitView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FixedLimitMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // limit_kbps: optional uint64
  pub fn limit_kbps(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_limit_kbps(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        0, val.into()
      )
    }
  }

}  // impl FixedLimit

impl ::std::ops::Drop for FixedLimit {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for FixedLimit {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for FixedLimit {
  type Proxied = Self;
  fn as_view(&self) -> FixedLimitView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for FixedLimit {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FixedLimitMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FixedLimit {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::fault_rate_limit::envoy__extensions__filters__common__fault__v3__FaultRateLimit__FixedLimit_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$,P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::fault_rate_limit::envoy__extensions__filters__common__fault__v3__FaultRateLimit__FixedLimit_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::fault_rate_limit::envoy__extensions__filters__common__fault__v3__FaultRateLimit__FixedLimit_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FixedLimit {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FixedLimit {
  type Msg = FixedLimit;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FixedLimit> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FixedLimit {
  type Msg = FixedLimit;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FixedLimit> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FixedLimitMut<'_> {
  type Msg = FixedLimit;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FixedLimit> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FixedLimitMut<'_> {
  type Msg = FixedLimit;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FixedLimit> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FixedLimitView<'_> {
  type Msg = FixedLimit;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FixedLimit> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FixedLimitMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__common__fault__v3__FaultRateLimit__HeaderLimit_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HeaderLimit {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HeaderLimit>
}

impl ::protobuf::Message for HeaderLimit {
  type MessageView<'msg> = HeaderLimitView<'msg>;
  type MessageMut<'msg> = HeaderLimitMut<'msg>;
}

impl ::std::default::Default for HeaderLimit {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HeaderLimit {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HeaderLimit` is `Sync` because it does not implement interior mutability.
//    Neither does `HeaderLimitMut`.
unsafe impl ::std::marker::Sync for HeaderLimit {}

// SAFETY:
// - `HeaderLimit` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HeaderLimit {}

impl ::protobuf::Proxied for HeaderLimit {
  type View<'msg> = HeaderLimitView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HeaderLimit {}

impl ::protobuf::MutProxied for HeaderLimit {
  type Mut<'msg> = HeaderLimitMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HeaderLimitView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderLimit>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeaderLimitView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HeaderLimitView<'msg> {
  type Message = HeaderLimit;
}

impl ::std::fmt::Debug for HeaderLimitView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HeaderLimitView<'_> {
  fn default() -> HeaderLimitView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderLimit>> for HeaderLimitView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderLimit>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HeaderLimitView<'msg> {

  pub fn to_owned(&self) -> HeaderLimit {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `HeaderLimitView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HeaderLimitView<'_> {}

// SAFETY:
// - `HeaderLimitView` is `Send` because while its alive a `HeaderLimitMut` cannot.
// - `HeaderLimitView` does not use thread-local data.
unsafe impl ::std::marker::Send for HeaderLimitView<'_> {}

impl<'msg> ::protobuf::AsView for HeaderLimitView<'msg> {
  type Proxied = HeaderLimit;
  fn as_view(&self) -> ::protobuf::View<'msg, HeaderLimit> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeaderLimitView<'msg> {
  fn into_view<'shorter>(self) -> HeaderLimitView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HeaderLimit> for HeaderLimitView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HeaderLimit {
    let mut dst = HeaderLimit::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HeaderLimit> for HeaderLimitMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HeaderLimit {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HeaderLimit {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HeaderLimitView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HeaderLimitMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HeaderLimitMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderLimit>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeaderLimitMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HeaderLimitMut<'msg> {
  type Message = HeaderLimit;
}

impl ::std::fmt::Debug for HeaderLimitMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderLimit>> for HeaderLimitMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderLimit>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HeaderLimitMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderLimit> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HeaderLimit {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `HeaderLimitMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HeaderLimitMut<'_> {}

// SAFETY:
// - `HeaderLimitMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HeaderLimitMut<'_> {}

impl<'msg> ::protobuf::AsView for HeaderLimitMut<'msg> {
  type Proxied = HeaderLimit;
  fn as_view(&self) -> ::protobuf::View<'_, HeaderLimit> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeaderLimitMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HeaderLimit>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HeaderLimitMut<'msg> {
  type MutProxied = HeaderLimit;
  fn as_mut(&mut self) -> HeaderLimitMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HeaderLimitMut<'msg> {
  fn into_mut<'shorter>(self) -> HeaderLimitMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HeaderLimit {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HeaderLimit> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HeaderLimitView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HeaderLimitMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl HeaderLimit

impl ::std::ops::Drop for HeaderLimit {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HeaderLimit {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HeaderLimit {
  type Proxied = Self;
  fn as_view(&self) -> HeaderLimitView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HeaderLimit {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HeaderLimitMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HeaderLimit {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::fault_rate_limit::envoy__extensions__filters__common__fault__v3__FaultRateLimit__HeaderLimit_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::fault_rate_limit::envoy__extensions__filters__common__fault__v3__FaultRateLimit__HeaderLimit_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::fault_rate_limit::envoy__extensions__filters__common__fault__v3__FaultRateLimit__HeaderLimit_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HeaderLimit {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HeaderLimit {
  type Msg = HeaderLimit;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderLimit> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderLimit {
  type Msg = HeaderLimit;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderLimit> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HeaderLimitMut<'_> {
  type Msg = HeaderLimit;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderLimit> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderLimitMut<'_> {
  type Msg = HeaderLimit;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderLimit> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderLimitView<'_> {
  type Msg = HeaderLimit;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderLimit> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HeaderLimitMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum LimitTypeOneof<'msg> {
  FixedLimit(::protobuf::View<'msg, super::super::fault_rate_limit::FixedLimit>) = 1,
  HeaderLimit(::protobuf::View<'msg, super::super::fault_rate_limit::HeaderLimit>) = 3,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum LimitTypeCase {
  FixedLimit = 1,
  HeaderLimit = 3,

  not_set = 0
}

impl LimitTypeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<LimitTypeCase> {
    match v {
      0 => Some(LimitTypeCase::not_set),
      1 => Some(LimitTypeCase::FixedLimit),
      3 => Some(LimitTypeCase::HeaderLimit),
      _ => None
    }
  }
}
}  // pub mod fault_rate_limit


