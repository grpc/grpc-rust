const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__rbac__v3__RBAC_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RBAC {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RBAC>
}

impl ::protobuf::Message for RBAC {
  type MessageView<'msg> = RBACView<'msg>;
  type MessageMut<'msg> = RBACMut<'msg>;
}

impl ::std::default::Default for RBAC {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RBAC {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RBAC` is `Sync` because it does not implement interior mutability.
//    Neither does `RBACMut`.
unsafe impl ::std::marker::Sync for RBAC {}

// SAFETY:
// - `RBAC` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RBAC {}

impl ::protobuf::Proxied for RBAC {
  type View<'msg> = RBACView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RBAC {}

impl ::protobuf::MutProxied for RBAC {
  type Mut<'msg> = RBACMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RBACView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RBAC>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RBACView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RBACView<'msg> {
  type Message = RBAC;
}

impl ::std::fmt::Debug for RBACView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RBACView<'_> {
  fn default() -> RBACView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RBAC>> for RBACView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RBAC>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RBACView<'msg> {

  pub fn to_owned(&self) -> RBAC {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // action: optional enum envoy.config.rbac.v3.RBAC.Action
  pub fn action(self) -> super::r_b_a_c::Action {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::r_b_a_c::Action::Allow).into()
      ).try_into().unwrap()
    }
  }

  // policies: repeated message envoy.config.rbac.v3.RBAC.PoliciesEntry
  pub fn policies(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, super::Policy> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(1)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, super::Policy>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // audit_logging_options: optional message envoy.config.rbac.v3.RBAC.AuditLoggingOptions
  pub fn has_audit_logging_options(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn audit_logging_options_opt(self) -> ::std::option::Option<super::r_b_a_c::AuditLoggingOptionsView<'msg>> {
    self.has_audit_logging_options().then(|| self.audit_logging_options())
  }
  pub fn audit_logging_options(self) -> super::r_b_a_c::AuditLoggingOptionsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::r_b_a_c::AuditLoggingOptionsView::default())
  }

}

// SAFETY:
// - `RBACView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RBACView<'_> {}

// SAFETY:
// - `RBACView` is `Send` because while its alive a `RBACMut` cannot.
// - `RBACView` does not use thread-local data.
unsafe impl ::std::marker::Send for RBACView<'_> {}

impl<'msg> ::protobuf::AsView for RBACView<'msg> {
  type Proxied = RBAC;
  fn as_view(&self) -> ::protobuf::View<'msg, RBAC> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RBACView<'msg> {
  fn into_view<'shorter>(self) -> RBACView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RBAC> for RBACView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RBAC {
    let mut dst = RBAC::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RBAC> for RBACMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RBAC {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RBAC {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RBACView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RBACMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RBACMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RBAC>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RBACMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RBACMut<'msg> {
  type Message = RBAC;
}

impl ::std::fmt::Debug for RBACMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RBAC>> for RBACMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RBAC>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RBACMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RBAC> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RBAC {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // action: optional enum envoy.config.rbac.v3.RBAC.Action
  pub fn action(&self) -> super::r_b_a_c::Action {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::r_b_a_c::Action::Allow).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_action(&mut self, val: super::r_b_a_c::Action) {
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

  // policies: repeated message envoy.config.rbac.v3.RBAC.PoliciesEntry
  pub fn policies(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, super::Policy> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(1)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, super::Policy>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn policies_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, super::Policy> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          1, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_policies(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, super::Policy>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // audit_logging_options: optional message envoy.config.rbac.v3.RBAC.AuditLoggingOptions
  pub fn has_audit_logging_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_audit_logging_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn audit_logging_options_opt(&self) -> ::std::option::Option<super::r_b_a_c::AuditLoggingOptionsView<'_>> {
    self.has_audit_logging_options().then(|| self.audit_logging_options())
  }
  pub fn audit_logging_options(&self) -> super::r_b_a_c::AuditLoggingOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::r_b_a_c::AuditLoggingOptionsView::default())
  }
  pub fn audit_logging_options_mut(&mut self) -> super::r_b_a_c::AuditLoggingOptionsMut<'_> {
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
  pub fn set_audit_logging_options(&mut self,
    val: impl ::protobuf::IntoProxied<super::r_b_a_c::AuditLoggingOptions>) {

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
// - `RBACMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RBACMut<'_> {}

// SAFETY:
// - `RBACMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RBACMut<'_> {}

impl<'msg> ::protobuf::AsView for RBACMut<'msg> {
  type Proxied = RBAC;
  fn as_view(&self) -> ::protobuf::View<'_, RBAC> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RBACMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RBAC>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RBACMut<'msg> {
  type MutProxied = RBAC;
  fn as_mut(&mut self) -> RBACMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RBACMut<'msg> {
  fn into_mut<'shorter>(self) -> RBACMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RBAC {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RBAC> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RBACView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RBACMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // action: optional enum envoy.config.rbac.v3.RBAC.Action
  pub fn action(&self) -> super::r_b_a_c::Action {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::r_b_a_c::Action::Allow).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_action(&mut self, val: super::r_b_a_c::Action) {
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

  // policies: repeated message envoy.config.rbac.v3.RBAC.PoliciesEntry
  pub fn policies(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, super::Policy> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(1)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, super::Policy>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn policies_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, super::Policy> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          1, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_policies(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, super::Policy>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // audit_logging_options: optional message envoy.config.rbac.v3.RBAC.AuditLoggingOptions
  pub fn has_audit_logging_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_audit_logging_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn audit_logging_options_opt(&self) -> ::std::option::Option<super::r_b_a_c::AuditLoggingOptionsView<'_>> {
    self.has_audit_logging_options().then(|| self.audit_logging_options())
  }
  pub fn audit_logging_options(&self) -> super::r_b_a_c::AuditLoggingOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::r_b_a_c::AuditLoggingOptionsView::default())
  }
  pub fn audit_logging_options_mut(&mut self) -> super::r_b_a_c::AuditLoggingOptionsMut<'_> {
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
  pub fn set_audit_logging_options(&mut self,
    val: impl ::protobuf::IntoProxied<super::r_b_a_c::AuditLoggingOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl RBAC

impl ::std::ops::Drop for RBAC {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RBAC {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RBAC {
  type Proxied = Self;
  fn as_view(&self) -> RBACView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RBAC {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RBACMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RBAC {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__rbac__v3__RBAC_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.PG3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__rbac__v3__RBAC_msg_init.0, &[<super::r_b_a_c::PoliciesEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::r_b_a_c::AuditLoggingOptions as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__rbac__v3__RBAC_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RBAC {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RBAC {
  type Msg = RBAC;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RBAC> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RBAC {
  type Msg = RBAC;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RBAC> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RBACMut<'_> {
  type Msg = RBAC;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RBAC> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RBACMut<'_> {
  type Msg = RBAC;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RBAC> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RBACView<'_> {
  type Msg = RBAC;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RBAC> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RBACMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod r_b_a_c {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__rbac__v3__RBAC__AuditLoggingOptions_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AuditLoggingOptions {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AuditLoggingOptions>
}

impl ::protobuf::Message for AuditLoggingOptions {
  type MessageView<'msg> = AuditLoggingOptionsView<'msg>;
  type MessageMut<'msg> = AuditLoggingOptionsMut<'msg>;
}

impl ::std::default::Default for AuditLoggingOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AuditLoggingOptions {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AuditLoggingOptions` is `Sync` because it does not implement interior mutability.
//    Neither does `AuditLoggingOptionsMut`.
unsafe impl ::std::marker::Sync for AuditLoggingOptions {}

// SAFETY:
// - `AuditLoggingOptions` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for AuditLoggingOptions {}

impl ::protobuf::Proxied for AuditLoggingOptions {
  type View<'msg> = AuditLoggingOptionsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AuditLoggingOptions {}

impl ::protobuf::MutProxied for AuditLoggingOptions {
  type Mut<'msg> = AuditLoggingOptionsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AuditLoggingOptionsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AuditLoggingOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AuditLoggingOptionsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AuditLoggingOptionsView<'msg> {
  type Message = AuditLoggingOptions;
}

impl ::std::fmt::Debug for AuditLoggingOptionsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AuditLoggingOptionsView<'_> {
  fn default() -> AuditLoggingOptionsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AuditLoggingOptions>> for AuditLoggingOptionsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AuditLoggingOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AuditLoggingOptionsView<'msg> {

  pub fn to_owned(&self) -> AuditLoggingOptions {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // audit_condition: optional enum envoy.config.rbac.v3.RBAC.AuditLoggingOptions.AuditCondition
  pub fn audit_condition(self) -> super::super::r_b_a_c::audit_logging_options::AuditCondition {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::super::r_b_a_c::audit_logging_options::AuditCondition::None).into()
      ).try_into().unwrap()
    }
  }

  // logger_configs: repeated message envoy.config.rbac.v3.RBAC.AuditLoggingOptions.AuditLoggerConfig
  pub fn logger_configs(self) -> ::protobuf::RepeatedView<'msg, super::super::r_b_a_c::audit_logging_options::AuditLoggerConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::r_b_a_c::audit_logging_options::AuditLoggerConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `AuditLoggingOptionsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AuditLoggingOptionsView<'_> {}

// SAFETY:
// - `AuditLoggingOptionsView` is `Send` because while its alive a `AuditLoggingOptionsMut` cannot.
// - `AuditLoggingOptionsView` does not use thread-local data.
unsafe impl ::std::marker::Send for AuditLoggingOptionsView<'_> {}

impl<'msg> ::protobuf::AsView for AuditLoggingOptionsView<'msg> {
  type Proxied = AuditLoggingOptions;
  fn as_view(&self) -> ::protobuf::View<'msg, AuditLoggingOptions> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AuditLoggingOptionsView<'msg> {
  fn into_view<'shorter>(self) -> AuditLoggingOptionsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AuditLoggingOptions> for AuditLoggingOptionsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AuditLoggingOptions {
    let mut dst = AuditLoggingOptions::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AuditLoggingOptions> for AuditLoggingOptionsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AuditLoggingOptions {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for AuditLoggingOptions {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AuditLoggingOptionsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AuditLoggingOptionsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AuditLoggingOptionsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AuditLoggingOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AuditLoggingOptionsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AuditLoggingOptionsMut<'msg> {
  type Message = AuditLoggingOptions;
}

impl ::std::fmt::Debug for AuditLoggingOptionsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AuditLoggingOptions>> for AuditLoggingOptionsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AuditLoggingOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AuditLoggingOptionsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AuditLoggingOptions> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> AuditLoggingOptions {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // audit_condition: optional enum envoy.config.rbac.v3.RBAC.AuditLoggingOptions.AuditCondition
  pub fn audit_condition(&self) -> super::super::r_b_a_c::audit_logging_options::AuditCondition {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::super::r_b_a_c::audit_logging_options::AuditCondition::None).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_audit_condition(&mut self, val: super::super::r_b_a_c::audit_logging_options::AuditCondition) {
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

  // logger_configs: repeated message envoy.config.rbac.v3.RBAC.AuditLoggingOptions.AuditLoggerConfig
  pub fn logger_configs(&self) -> ::protobuf::RepeatedView<'_, super::super::r_b_a_c::audit_logging_options::AuditLoggerConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::r_b_a_c::audit_logging_options::AuditLoggerConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn logger_configs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::r_b_a_c::audit_logging_options::AuditLoggerConfig> {
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
  pub fn set_logger_configs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::r_b_a_c::audit_logging_options::AuditLoggerConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `AuditLoggingOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AuditLoggingOptionsMut<'_> {}

// SAFETY:
// - `AuditLoggingOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AuditLoggingOptionsMut<'_> {}

impl<'msg> ::protobuf::AsView for AuditLoggingOptionsMut<'msg> {
  type Proxied = AuditLoggingOptions;
  fn as_view(&self) -> ::protobuf::View<'_, AuditLoggingOptions> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AuditLoggingOptionsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AuditLoggingOptions>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AuditLoggingOptionsMut<'msg> {
  type MutProxied = AuditLoggingOptions;
  fn as_mut(&mut self) -> AuditLoggingOptionsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AuditLoggingOptionsMut<'msg> {
  fn into_mut<'shorter>(self) -> AuditLoggingOptionsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AuditLoggingOptions {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AuditLoggingOptions> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AuditLoggingOptionsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AuditLoggingOptionsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // audit_condition: optional enum envoy.config.rbac.v3.RBAC.AuditLoggingOptions.AuditCondition
  pub fn audit_condition(&self) -> super::super::r_b_a_c::audit_logging_options::AuditCondition {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::super::r_b_a_c::audit_logging_options::AuditCondition::None).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_audit_condition(&mut self, val: super::super::r_b_a_c::audit_logging_options::AuditCondition) {
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

  // logger_configs: repeated message envoy.config.rbac.v3.RBAC.AuditLoggingOptions.AuditLoggerConfig
  pub fn logger_configs(&self) -> ::protobuf::RepeatedView<'_, super::super::r_b_a_c::audit_logging_options::AuditLoggerConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::r_b_a_c::audit_logging_options::AuditLoggerConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn logger_configs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::r_b_a_c::audit_logging_options::AuditLoggerConfig> {
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
  pub fn set_logger_configs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::r_b_a_c::audit_logging_options::AuditLoggerConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl AuditLoggingOptions

impl ::std::ops::Drop for AuditLoggingOptions {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AuditLoggingOptions {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AuditLoggingOptions {
  type Proxied = Self;
  fn as_view(&self) -> AuditLoggingOptionsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AuditLoggingOptions {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AuditLoggingOptionsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AuditLoggingOptions {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::r_b_a_c::envoy__config__rbac__v3__RBAC__AuditLoggingOptions_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.PG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::r_b_a_c::envoy__config__rbac__v3__RBAC__AuditLoggingOptions_msg_init.0, &[<super::super::r_b_a_c::audit_logging_options::AuditLoggerConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::r_b_a_c::envoy__config__rbac__v3__RBAC__AuditLoggingOptions_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AuditLoggingOptions {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AuditLoggingOptions {
  type Msg = AuditLoggingOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AuditLoggingOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AuditLoggingOptions {
  type Msg = AuditLoggingOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AuditLoggingOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AuditLoggingOptionsMut<'_> {
  type Msg = AuditLoggingOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AuditLoggingOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AuditLoggingOptionsMut<'_> {
  type Msg = AuditLoggingOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AuditLoggingOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AuditLoggingOptionsView<'_> {
  type Msg = AuditLoggingOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AuditLoggingOptions> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AuditLoggingOptionsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod audit_logging_options {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__rbac__v3__RBAC__AuditLoggingOptions__AuditLoggerConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AuditLoggerConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AuditLoggerConfig>
}

impl ::protobuf::Message for AuditLoggerConfig {
  type MessageView<'msg> = AuditLoggerConfigView<'msg>;
  type MessageMut<'msg> = AuditLoggerConfigMut<'msg>;
}

impl ::std::default::Default for AuditLoggerConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AuditLoggerConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AuditLoggerConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `AuditLoggerConfigMut`.
unsafe impl ::std::marker::Sync for AuditLoggerConfig {}

// SAFETY:
// - `AuditLoggerConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for AuditLoggerConfig {}

impl ::protobuf::Proxied for AuditLoggerConfig {
  type View<'msg> = AuditLoggerConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AuditLoggerConfig {}

impl ::protobuf::MutProxied for AuditLoggerConfig {
  type Mut<'msg> = AuditLoggerConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AuditLoggerConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AuditLoggerConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AuditLoggerConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AuditLoggerConfigView<'msg> {
  type Message = AuditLoggerConfig;
}

impl ::std::fmt::Debug for AuditLoggerConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AuditLoggerConfigView<'_> {
  fn default() -> AuditLoggerConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AuditLoggerConfig>> for AuditLoggerConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AuditLoggerConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AuditLoggerConfigView<'msg> {

  pub fn to_owned(&self) -> AuditLoggerConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // audit_logger: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_audit_logger(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn audit_logger_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_audit_logger().then(|| self.audit_logger())
  }
  pub fn audit_logger(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  // is_optional: optional bool
  pub fn is_optional(self) -> bool {
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

}

// SAFETY:
// - `AuditLoggerConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AuditLoggerConfigView<'_> {}

// SAFETY:
// - `AuditLoggerConfigView` is `Send` because while its alive a `AuditLoggerConfigMut` cannot.
// - `AuditLoggerConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for AuditLoggerConfigView<'_> {}

impl<'msg> ::protobuf::AsView for AuditLoggerConfigView<'msg> {
  type Proxied = AuditLoggerConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, AuditLoggerConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AuditLoggerConfigView<'msg> {
  fn into_view<'shorter>(self) -> AuditLoggerConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AuditLoggerConfig> for AuditLoggerConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AuditLoggerConfig {
    let mut dst = AuditLoggerConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AuditLoggerConfig> for AuditLoggerConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AuditLoggerConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for AuditLoggerConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AuditLoggerConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AuditLoggerConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AuditLoggerConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AuditLoggerConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AuditLoggerConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AuditLoggerConfigMut<'msg> {
  type Message = AuditLoggerConfig;
}

impl ::std::fmt::Debug for AuditLoggerConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AuditLoggerConfig>> for AuditLoggerConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AuditLoggerConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AuditLoggerConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AuditLoggerConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> AuditLoggerConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // audit_logger: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_audit_logger(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_audit_logger(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn audit_logger_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_audit_logger().then(|| self.audit_logger())
  }
  pub fn audit_logger(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn audit_logger_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_audit_logger(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // is_optional: optional bool
  pub fn is_optional(&self) -> bool {
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
  pub fn set_is_optional(&mut self, val: bool) {
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

}

// SAFETY:
// - `AuditLoggerConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AuditLoggerConfigMut<'_> {}

// SAFETY:
// - `AuditLoggerConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AuditLoggerConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for AuditLoggerConfigMut<'msg> {
  type Proxied = AuditLoggerConfig;
  fn as_view(&self) -> ::protobuf::View<'_, AuditLoggerConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AuditLoggerConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AuditLoggerConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AuditLoggerConfigMut<'msg> {
  type MutProxied = AuditLoggerConfig;
  fn as_mut(&mut self) -> AuditLoggerConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AuditLoggerConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> AuditLoggerConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AuditLoggerConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AuditLoggerConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AuditLoggerConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AuditLoggerConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // audit_logger: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_audit_logger(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_audit_logger(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn audit_logger_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_audit_logger().then(|| self.audit_logger())
  }
  pub fn audit_logger(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn audit_logger_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_audit_logger(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // is_optional: optional bool
  pub fn is_optional(&self) -> bool {
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
  pub fn set_is_optional(&mut self, val: bool) {
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

}  // impl AuditLoggerConfig

impl ::std::ops::Drop for AuditLoggerConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AuditLoggerConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AuditLoggerConfig {
  type Proxied = Self;
  fn as_view(&self) -> AuditLoggerConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AuditLoggerConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AuditLoggerConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AuditLoggerConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::r_b_a_c::audit_logging_options::envoy__config__rbac__v3__RBAC__AuditLoggingOptions__AuditLoggerConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::r_b_a_c::audit_logging_options::envoy__config__rbac__v3__RBAC__AuditLoggingOptions__AuditLoggerConfig_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::r_b_a_c::audit_logging_options::envoy__config__rbac__v3__RBAC__AuditLoggingOptions__AuditLoggerConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AuditLoggerConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AuditLoggerConfig {
  type Msg = AuditLoggerConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AuditLoggerConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AuditLoggerConfig {
  type Msg = AuditLoggerConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AuditLoggerConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AuditLoggerConfigMut<'_> {
  type Msg = AuditLoggerConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AuditLoggerConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AuditLoggerConfigMut<'_> {
  type Msg = AuditLoggerConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AuditLoggerConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AuditLoggerConfigView<'_> {
  type Msg = AuditLoggerConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AuditLoggerConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AuditLoggerConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AuditCondition(i32);

#[allow(non_upper_case_globals)]
impl AuditCondition {
  pub const None: AuditCondition = AuditCondition(0);
  pub const OnDeny: AuditCondition = AuditCondition(1);
  pub const OnAllow: AuditCondition = AuditCondition(2);
  pub const OnDenyAndAllow: AuditCondition = AuditCondition(3);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "None",
      1 => "OnDeny",
      2 => "OnAllow",
      3 => "OnDenyAndAllow",
      _ => return None
    })
  }
}

impl ::std::convert::From<AuditCondition> for i32 {
  fn from(val: AuditCondition) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for AuditCondition {
  fn from(val: i32) -> AuditCondition {
    Self(val)
  }
}

impl ::std::default::Default for AuditCondition {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for AuditCondition {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "AuditCondition::{}", constant_name)
    } else {
      write!(f, "AuditCondition::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for AuditCondition {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for AuditCondition {}

impl ::protobuf::Proxied for AuditCondition {
  type View<'a> = AuditCondition;
}

impl ::protobuf::AsView for AuditCondition {
  type Proxied = AuditCondition;

  fn as_view(&self) -> AuditCondition {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AuditCondition {
  fn into_view<'shorter>(self) -> AuditCondition where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for AuditCondition {
  const NAME: &'static str = "AuditCondition";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3)
  }
}

impl ::protobuf::__internal::EntityType for AuditCondition {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod audit_logging_options

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__rbac__v3__RBAC__PoliciesEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct PoliciesEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PoliciesEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::r_b_a_c::envoy__config__rbac__v3__RBAC__PoliciesEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::r_b_a_c::envoy__config__rbac__v3__RBAC__PoliciesEntry_msg_init.0, &[<super::super::Policy as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::r_b_a_c::envoy__config__rbac__v3__RBAC__PoliciesEntry_msg_init.0)
      }).0
    }
  }
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Action(i32);

#[allow(non_upper_case_globals)]
impl Action {
  pub const Allow: Action = Action(0);
  pub const Deny: Action = Action(1);
  pub const Log: Action = Action(2);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Allow",
      1 => "Deny",
      2 => "Log",
      _ => return None
    })
  }
}

impl ::std::convert::From<Action> for i32 {
  fn from(val: Action) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for Action {
  fn from(val: i32) -> Action {
    Self(val)
  }
}

impl ::std::default::Default for Action {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for Action {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "Action::{}", constant_name)
    } else {
      write!(f, "Action::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for Action {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for Action {}

impl ::protobuf::Proxied for Action {
  type View<'a> = Action;
}

impl ::protobuf::AsView for Action {
  type Proxied = Action;

  fn as_view(&self) -> Action {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Action {
  fn into_view<'shorter>(self) -> Action where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for Action {
  const NAME: &'static str = "Action";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2)
  }
}

impl ::protobuf::__internal::EntityType for Action {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod r_b_a_c


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__rbac__v3__Policy_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Policy {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Policy>
}

impl ::protobuf::Message for Policy {
  type MessageView<'msg> = PolicyView<'msg>;
  type MessageMut<'msg> = PolicyMut<'msg>;
}

impl ::std::default::Default for Policy {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Policy {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Policy` is `Sync` because it does not implement interior mutability.
//    Neither does `PolicyMut`.
unsafe impl ::std::marker::Sync for Policy {}

// SAFETY:
// - `Policy` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Policy {}

impl ::protobuf::Proxied for Policy {
  type View<'msg> = PolicyView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Policy {}

impl ::protobuf::MutProxied for Policy {
  type Mut<'msg> = PolicyMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PolicyView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Policy>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PolicyView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PolicyView<'msg> {
  type Message = Policy;
}

impl ::std::fmt::Debug for PolicyView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PolicyView<'_> {
  fn default() -> PolicyView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Policy>> for PolicyView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Policy>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PolicyView<'msg> {

  pub fn to_owned(&self) -> Policy {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // permissions: repeated message envoy.config.rbac.v3.Permission
  pub fn permissions(self) -> ::protobuf::RepeatedView<'msg, super::Permission> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Permission>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // principals: repeated message envoy.config.rbac.v3.Principal
  pub fn principals(self) -> ::protobuf::RepeatedView<'msg, super::Principal> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Principal>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // condition: optional message google.api.expr.v1alpha1.Expr
  pub fn has_condition(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn condition_opt(self) -> ::std::option::Option<crate::xds::generated::google::api::expr::v1alpha1::syntax::ExprView<'msg>> {
    self.has_condition().then(|| self.condition())
  }
  pub fn condition(self) -> crate::xds::generated::google::api::expr::v1alpha1::syntax::ExprView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::google::api::expr::v1alpha1::syntax::ExprView::default())
  }

  // checked_condition: optional message google.api.expr.v1alpha1.CheckedExpr
  pub fn has_checked_condition(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn checked_condition_opt(self) -> ::std::option::Option<crate::xds::generated::google::api::expr::v1alpha1::checked::CheckedExprView<'msg>> {
    self.has_checked_condition().then(|| self.checked_condition())
  }
  pub fn checked_condition(self) -> crate::xds::generated::google::api::expr::v1alpha1::checked::CheckedExprView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::google::api::expr::v1alpha1::checked::CheckedExprView::default())
  }

  // cel_config: optional message envoy.config.core.v3.CelExpressionConfig
  pub fn has_cel_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn cel_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::cel::CelExpressionConfigView<'msg>> {
    self.has_cel_config().then(|| self.cel_config())
  }
  pub fn cel_config(self) -> crate::xds::generated::envoy::config::core::v3::cel::CelExpressionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::cel::CelExpressionConfigView::default())
  }

}

// SAFETY:
// - `PolicyView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for PolicyView<'_> {}

// SAFETY:
// - `PolicyView` is `Send` because while its alive a `PolicyMut` cannot.
// - `PolicyView` does not use thread-local data.
unsafe impl ::std::marker::Send for PolicyView<'_> {}

impl<'msg> ::protobuf::AsView for PolicyView<'msg> {
  type Proxied = Policy;
  fn as_view(&self) -> ::protobuf::View<'msg, Policy> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PolicyView<'msg> {
  fn into_view<'shorter>(self) -> PolicyView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Policy> for PolicyView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Policy {
    let mut dst = Policy::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Policy> for PolicyMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Policy {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Policy {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PolicyView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PolicyMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PolicyMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Policy>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PolicyMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PolicyMut<'msg> {
  type Message = Policy;
}

impl ::std::fmt::Debug for PolicyMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Policy>> for PolicyMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Policy>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PolicyMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Policy> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Policy {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // permissions: repeated message envoy.config.rbac.v3.Permission
  pub fn permissions(&self) -> ::protobuf::RepeatedView<'_, super::Permission> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Permission>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn permissions_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Permission> {
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
  pub fn set_permissions(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Permission>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // principals: repeated message envoy.config.rbac.v3.Principal
  pub fn principals(&self) -> ::protobuf::RepeatedView<'_, super::Principal> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Principal>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn principals_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Principal> {
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
  pub fn set_principals(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Principal>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // condition: optional message google.api.expr.v1alpha1.Expr
  pub fn has_condition(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_condition(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn condition_opt(&self) -> ::std::option::Option<crate::xds::generated::google::api::expr::v1alpha1::syntax::ExprView<'_>> {
    self.has_condition().then(|| self.condition())
  }
  pub fn condition(&self) -> crate::xds::generated::google::api::expr::v1alpha1::syntax::ExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::google::api::expr::v1alpha1::syntax::ExprView::default())
  }
  pub fn condition_mut(&mut self) -> crate::xds::generated::google::api::expr::v1alpha1::syntax::ExprMut<'_> {
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
  pub fn set_condition(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::google::api::expr::v1alpha1::syntax::Expr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // checked_condition: optional message google.api.expr.v1alpha1.CheckedExpr
  pub fn has_checked_condition(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_checked_condition(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn checked_condition_opt(&self) -> ::std::option::Option<crate::xds::generated::google::api::expr::v1alpha1::checked::CheckedExprView<'_>> {
    self.has_checked_condition().then(|| self.checked_condition())
  }
  pub fn checked_condition(&self) -> crate::xds::generated::google::api::expr::v1alpha1::checked::CheckedExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::google::api::expr::v1alpha1::checked::CheckedExprView::default())
  }
  pub fn checked_condition_mut(&mut self) -> crate::xds::generated::google::api::expr::v1alpha1::checked::CheckedExprMut<'_> {
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
  pub fn set_checked_condition(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::google::api::expr::v1alpha1::checked::CheckedExpr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // cel_config: optional message envoy.config.core.v3.CelExpressionConfig
  pub fn has_cel_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_cel_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn cel_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::cel::CelExpressionConfigView<'_>> {
    self.has_cel_config().then(|| self.cel_config())
  }
  pub fn cel_config(&self) -> crate::xds::generated::envoy::config::core::v3::cel::CelExpressionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::cel::CelExpressionConfigView::default())
  }
  pub fn cel_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::cel::CelExpressionConfigMut<'_> {
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
  pub fn set_cel_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::cel::CelExpressionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

}

// SAFETY:
// - `PolicyMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for PolicyMut<'_> {}

// SAFETY:
// - `PolicyMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for PolicyMut<'_> {}

impl<'msg> ::protobuf::AsView for PolicyMut<'msg> {
  type Proxied = Policy;
  fn as_view(&self) -> ::protobuf::View<'_, Policy> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PolicyMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Policy>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for PolicyMut<'msg> {
  type MutProxied = Policy;
  fn as_mut(&mut self) -> PolicyMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PolicyMut<'msg> {
  fn into_mut<'shorter>(self) -> PolicyMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Policy {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Policy> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PolicyView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PolicyMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // permissions: repeated message envoy.config.rbac.v3.Permission
  pub fn permissions(&self) -> ::protobuf::RepeatedView<'_, super::Permission> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Permission>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn permissions_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Permission> {
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
  pub fn set_permissions(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Permission>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // principals: repeated message envoy.config.rbac.v3.Principal
  pub fn principals(&self) -> ::protobuf::RepeatedView<'_, super::Principal> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Principal>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn principals_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Principal> {
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
  pub fn set_principals(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Principal>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // condition: optional message google.api.expr.v1alpha1.Expr
  pub fn has_condition(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_condition(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn condition_opt(&self) -> ::std::option::Option<crate::xds::generated::google::api::expr::v1alpha1::syntax::ExprView<'_>> {
    self.has_condition().then(|| self.condition())
  }
  pub fn condition(&self) -> crate::xds::generated::google::api::expr::v1alpha1::syntax::ExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::google::api::expr::v1alpha1::syntax::ExprView::default())
  }
  pub fn condition_mut(&mut self) -> crate::xds::generated::google::api::expr::v1alpha1::syntax::ExprMut<'_> {
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
  pub fn set_condition(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::google::api::expr::v1alpha1::syntax::Expr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // checked_condition: optional message google.api.expr.v1alpha1.CheckedExpr
  pub fn has_checked_condition(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_checked_condition(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn checked_condition_opt(&self) -> ::std::option::Option<crate::xds::generated::google::api::expr::v1alpha1::checked::CheckedExprView<'_>> {
    self.has_checked_condition().then(|| self.checked_condition())
  }
  pub fn checked_condition(&self) -> crate::xds::generated::google::api::expr::v1alpha1::checked::CheckedExprView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::google::api::expr::v1alpha1::checked::CheckedExprView::default())
  }
  pub fn checked_condition_mut(&mut self) -> crate::xds::generated::google::api::expr::v1alpha1::checked::CheckedExprMut<'_> {
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
  pub fn set_checked_condition(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::google::api::expr::v1alpha1::checked::CheckedExpr>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // cel_config: optional message envoy.config.core.v3.CelExpressionConfig
  pub fn has_cel_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_cel_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn cel_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::cel::CelExpressionConfigView<'_>> {
    self.has_cel_config().then(|| self.cel_config())
  }
  pub fn cel_config(&self) -> crate::xds::generated::envoy::config::core::v3::cel::CelExpressionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::cel::CelExpressionConfigView::default())
  }
  pub fn cel_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::cel::CelExpressionConfigMut<'_> {
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
  pub fn set_cel_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::cel::CelExpressionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

}  // impl Policy

impl ::std::ops::Drop for Policy {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Policy {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Policy {
  type Proxied = Self;
  fn as_view(&self) -> PolicyView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Policy {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PolicyMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Policy {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__rbac__v3__Policy_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$GG333");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__rbac__v3__Policy_msg_init.0, &[<super::Permission as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Principal as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::google::api::expr::v1alpha1::syntax::Expr as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::google::api::expr::v1alpha1::checked::CheckedExpr as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::cel::CelExpressionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__rbac__v3__Policy_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Policy {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Policy {
  type Msg = Policy;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Policy> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Policy {
  type Msg = Policy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Policy> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PolicyMut<'_> {
  type Msg = Policy;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Policy> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PolicyMut<'_> {
  type Msg = Policy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Policy> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PolicyView<'_> {
  type Msg = Policy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Policy> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PolicyMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__rbac__v3__SourcedMetadata_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SourcedMetadata {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SourcedMetadata>
}

impl ::protobuf::Message for SourcedMetadata {
  type MessageView<'msg> = SourcedMetadataView<'msg>;
  type MessageMut<'msg> = SourcedMetadataMut<'msg>;
}

impl ::std::default::Default for SourcedMetadata {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SourcedMetadata {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SourcedMetadata` is `Sync` because it does not implement interior mutability.
//    Neither does `SourcedMetadataMut`.
unsafe impl ::std::marker::Sync for SourcedMetadata {}

// SAFETY:
// - `SourcedMetadata` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SourcedMetadata {}

impl ::protobuf::Proxied for SourcedMetadata {
  type View<'msg> = SourcedMetadataView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SourcedMetadata {}

impl ::protobuf::MutProxied for SourcedMetadata {
  type Mut<'msg> = SourcedMetadataMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SourcedMetadataView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SourcedMetadata>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SourcedMetadataView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SourcedMetadataView<'msg> {
  type Message = SourcedMetadata;
}

impl ::std::fmt::Debug for SourcedMetadataView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SourcedMetadataView<'_> {
  fn default() -> SourcedMetadataView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SourcedMetadata>> for SourcedMetadataView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SourcedMetadata>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SourcedMetadataView<'msg> {

  pub fn to_owned(&self) -> SourcedMetadata {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // metadata_matcher: optional message envoy.type.matcher.v3.MetadataMatcher
  pub fn has_metadata_matcher(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn metadata_matcher_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView<'msg>> {
    self.has_metadata_matcher().then(|| self.metadata_matcher())
  }
  pub fn metadata_matcher(self) -> crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView::default())
  }

  // metadata_source: optional enum envoy.config.rbac.v3.MetadataSource
  pub fn metadata_source(self) -> super::MetadataSource {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::MetadataSource::Dynamic).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `SourcedMetadataView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SourcedMetadataView<'_> {}

// SAFETY:
// - `SourcedMetadataView` is `Send` because while its alive a `SourcedMetadataMut` cannot.
// - `SourcedMetadataView` does not use thread-local data.
unsafe impl ::std::marker::Send for SourcedMetadataView<'_> {}

impl<'msg> ::protobuf::AsView for SourcedMetadataView<'msg> {
  type Proxied = SourcedMetadata;
  fn as_view(&self) -> ::protobuf::View<'msg, SourcedMetadata> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SourcedMetadataView<'msg> {
  fn into_view<'shorter>(self) -> SourcedMetadataView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SourcedMetadata> for SourcedMetadataView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SourcedMetadata {
    let mut dst = SourcedMetadata::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SourcedMetadata> for SourcedMetadataMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SourcedMetadata {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SourcedMetadata {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SourcedMetadataView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SourcedMetadataMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SourcedMetadataMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SourcedMetadata>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SourcedMetadataMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SourcedMetadataMut<'msg> {
  type Message = SourcedMetadata;
}

impl ::std::fmt::Debug for SourcedMetadataMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SourcedMetadata>> for SourcedMetadataMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SourcedMetadata>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SourcedMetadataMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SourcedMetadata> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SourcedMetadata {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // metadata_matcher: optional message envoy.type.matcher.v3.MetadataMatcher
  pub fn has_metadata_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_metadata_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn metadata_matcher_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView<'_>> {
    self.has_metadata_matcher().then(|| self.metadata_matcher())
  }
  pub fn metadata_matcher(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView::default())
  }
  pub fn metadata_matcher_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherMut<'_> {
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
  pub fn set_metadata_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // metadata_source: optional enum envoy.config.rbac.v3.MetadataSource
  pub fn metadata_source(&self) -> super::MetadataSource {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::MetadataSource::Dynamic).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_metadata_source(&mut self, val: super::MetadataSource) {
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

}

// SAFETY:
// - `SourcedMetadataMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SourcedMetadataMut<'_> {}

// SAFETY:
// - `SourcedMetadataMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SourcedMetadataMut<'_> {}

impl<'msg> ::protobuf::AsView for SourcedMetadataMut<'msg> {
  type Proxied = SourcedMetadata;
  fn as_view(&self) -> ::protobuf::View<'_, SourcedMetadata> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SourcedMetadataMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SourcedMetadata>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SourcedMetadataMut<'msg> {
  type MutProxied = SourcedMetadata;
  fn as_mut(&mut self) -> SourcedMetadataMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SourcedMetadataMut<'msg> {
  fn into_mut<'shorter>(self) -> SourcedMetadataMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SourcedMetadata {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SourcedMetadata> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SourcedMetadataView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SourcedMetadataMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // metadata_matcher: optional message envoy.type.matcher.v3.MetadataMatcher
  pub fn has_metadata_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_metadata_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn metadata_matcher_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView<'_>> {
    self.has_metadata_matcher().then(|| self.metadata_matcher())
  }
  pub fn metadata_matcher(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView::default())
  }
  pub fn metadata_matcher_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherMut<'_> {
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
  pub fn set_metadata_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // metadata_source: optional enum envoy.config.rbac.v3.MetadataSource
  pub fn metadata_source(&self) -> super::MetadataSource {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::MetadataSource::Dynamic).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_metadata_source(&mut self, val: super::MetadataSource) {
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

}  // impl SourcedMetadata

impl ::std::ops::Drop for SourcedMetadata {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SourcedMetadata {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SourcedMetadata {
  type Proxied = Self;
  fn as_view(&self) -> SourcedMetadataView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SourcedMetadata {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SourcedMetadataMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SourcedMetadata {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__rbac__v3__SourcedMetadata_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3.P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__rbac__v3__SourcedMetadata_msg_init.0, &[<crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__rbac__v3__SourcedMetadata_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SourcedMetadata {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SourcedMetadata {
  type Msg = SourcedMetadata;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SourcedMetadata> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SourcedMetadata {
  type Msg = SourcedMetadata;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SourcedMetadata> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SourcedMetadataMut<'_> {
  type Msg = SourcedMetadata;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SourcedMetadata> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SourcedMetadataMut<'_> {
  type Msg = SourcedMetadata;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SourcedMetadata> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SourcedMetadataView<'_> {
  type Msg = SourcedMetadata;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SourcedMetadata> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SourcedMetadataMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__rbac__v3__Permission_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Permission {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Permission>
}

impl ::protobuf::Message for Permission {
  type MessageView<'msg> = PermissionView<'msg>;
  type MessageMut<'msg> = PermissionMut<'msg>;
}

impl ::std::default::Default for Permission {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Permission {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Permission` is `Sync` because it does not implement interior mutability.
//    Neither does `PermissionMut`.
unsafe impl ::std::marker::Sync for Permission {}

// SAFETY:
// - `Permission` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Permission {}

impl ::protobuf::Proxied for Permission {
  type View<'msg> = PermissionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Permission {}

impl ::protobuf::MutProxied for Permission {
  type Mut<'msg> = PermissionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PermissionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Permission>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PermissionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PermissionView<'msg> {
  type Message = Permission;
}

impl ::std::fmt::Debug for PermissionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PermissionView<'_> {
  fn default() -> PermissionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Permission>> for PermissionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Permission>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PermissionView<'msg> {

  pub fn to_owned(&self) -> Permission {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // and_rules: optional message envoy.config.rbac.v3.Permission.Set
  pub fn has_and_rules(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn and_rules_opt(self) -> ::std::option::Option<super::permission::SetView<'msg>> {
    self.has_and_rules().then(|| self.and_rules())
  }
  pub fn and_rules(self) -> super::permission::SetView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::permission::SetView::default())
  }

  // or_rules: optional message envoy.config.rbac.v3.Permission.Set
  pub fn has_or_rules(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn or_rules_opt(self) -> ::std::option::Option<super::permission::SetView<'msg>> {
    self.has_or_rules().then(|| self.or_rules())
  }
  pub fn or_rules(self) -> super::permission::SetView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::permission::SetView::default())
  }

  // any: optional bool
  pub fn has_any(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn any_opt(self) -> ::std::option::Option<bool> {
    self.has_any().then(|| self.any())
  }
  pub fn any(self) -> bool {
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

  // header: optional message envoy.config.route.v3.HeaderMatcher
  pub fn has_header(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn header_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherView<'msg>> {
    self.has_header().then(|| self.header())
  }
  pub fn header(self) -> crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherView::default())
  }

  // url_path: optional message envoy.type.matcher.v3.PathMatcher
  pub fn has_url_path(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn url_path_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::path::PathMatcherView<'msg>> {
    self.has_url_path().then(|| self.url_path())
  }
  pub fn url_path(self) -> crate::xds::generated::envoy::r#type::matcher::v3::path::PathMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::path::PathMatcherView::default())
  }

  // destination_ip: optional message envoy.config.core.v3.CidrRange
  pub fn has_destination_ip(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn destination_ip_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::CidrRangeView<'msg>> {
    self.has_destination_ip().then(|| self.destination_ip())
  }
  pub fn destination_ip(self) -> crate::xds::generated::envoy::config::core::v3::address::CidrRangeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::CidrRangeView::default())
  }

  // destination_port: optional uint32
  pub fn has_destination_port(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn destination_port_opt(self) -> ::std::option::Option<u32> {
    self.has_destination_port().then(|| self.destination_port())
  }
  pub fn destination_port(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        5, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // destination_port_range: optional message envoy.type.v3.Int32Range
  pub fn has_destination_port_range(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn destination_port_range_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::range::Int32RangeView<'msg>> {
    self.has_destination_port_range().then(|| self.destination_port_range())
  }
  pub fn destination_port_range(self) -> crate::xds::generated::envoy::r#type::v3::range::Int32RangeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::range::Int32RangeView::default())
  }

  // metadata: optional message envoy.type.matcher.v3.MetadataMatcher
  pub fn has_metadata(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn metadata_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView<'msg>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(self) -> crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView::default())
  }

  // not_rule: optional message envoy.config.rbac.v3.Permission
  pub fn has_not_rule(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn not_rule_opt(self) -> ::std::option::Option<super::PermissionView<'msg>> {
    self.has_not_rule().then(|| self.not_rule())
  }
  pub fn not_rule(self) -> super::PermissionView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PermissionView::default())
  }

  // requested_server_name: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_requested_server_name(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn requested_server_name_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'msg>> {
    self.has_requested_server_name().then(|| self.requested_server_name())
  }
  pub fn requested_server_name(self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }

  // matcher: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_matcher(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn matcher_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_matcher().then(|| self.matcher())
  }
  pub fn matcher(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  // uri_template: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_uri_template(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn uri_template_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_uri_template().then(|| self.uri_template())
  }
  pub fn uri_template(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  // sourced_metadata: optional message envoy.config.rbac.v3.SourcedMetadata
  pub fn has_sourced_metadata(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn sourced_metadata_opt(self) -> ::std::option::Option<super::SourcedMetadataView<'msg>> {
    self.has_sourced_metadata().then(|| self.sourced_metadata())
  }
  pub fn sourced_metadata(self) -> super::SourcedMetadataView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SourcedMetadataView::default())
  }

  pub fn rule(self) -> super::permission::RuleOneof<'msg> {
    match self.rule_case() {
      super::permission::RuleCase::AndRules =>
          super::permission::RuleOneof::AndRules(self.and_rules()),
      super::permission::RuleCase::OrRules =>
          super::permission::RuleOneof::OrRules(self.or_rules()),
      super::permission::RuleCase::Any =>
          super::permission::RuleOneof::Any(self.any()),
      super::permission::RuleCase::Header =>
          super::permission::RuleOneof::Header(self.header()),
      super::permission::RuleCase::UrlPath =>
          super::permission::RuleOneof::UrlPath(self.url_path()),
      super::permission::RuleCase::DestinationIp =>
          super::permission::RuleOneof::DestinationIp(self.destination_ip()),
      super::permission::RuleCase::DestinationPort =>
          super::permission::RuleOneof::DestinationPort(self.destination_port()),
      super::permission::RuleCase::DestinationPortRange =>
          super::permission::RuleOneof::DestinationPortRange(self.destination_port_range()),
      super::permission::RuleCase::Metadata =>
          super::permission::RuleOneof::Metadata(self.metadata()),
      super::permission::RuleCase::NotRule =>
          super::permission::RuleOneof::NotRule(self.not_rule()),
      super::permission::RuleCase::RequestedServerName =>
          super::permission::RuleOneof::RequestedServerName(self.requested_server_name()),
      super::permission::RuleCase::Matcher =>
          super::permission::RuleOneof::Matcher(self.matcher()),
      super::permission::RuleCase::UriTemplate =>
          super::permission::RuleOneof::UriTemplate(self.uri_template()),
      super::permission::RuleCase::SourcedMetadata =>
          super::permission::RuleOneof::SourcedMetadata(self.sourced_metadata()),
      _ => super::permission::RuleOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn rule_case(self) -> super::permission::RuleCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::permission::RuleCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `PermissionView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for PermissionView<'_> {}

// SAFETY:
// - `PermissionView` is `Send` because while its alive a `PermissionMut` cannot.
// - `PermissionView` does not use thread-local data.
unsafe impl ::std::marker::Send for PermissionView<'_> {}

impl<'msg> ::protobuf::AsView for PermissionView<'msg> {
  type Proxied = Permission;
  fn as_view(&self) -> ::protobuf::View<'msg, Permission> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PermissionView<'msg> {
  fn into_view<'shorter>(self) -> PermissionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Permission> for PermissionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Permission {
    let mut dst = Permission::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Permission> for PermissionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Permission {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Permission {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PermissionView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PermissionMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PermissionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Permission>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PermissionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PermissionMut<'msg> {
  type Message = Permission;
}

impl ::std::fmt::Debug for PermissionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Permission>> for PermissionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Permission>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PermissionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Permission> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Permission {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // and_rules: optional message envoy.config.rbac.v3.Permission.Set
  pub fn has_and_rules(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_and_rules(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn and_rules_opt(&self) -> ::std::option::Option<super::permission::SetView<'_>> {
    self.has_and_rules().then(|| self.and_rules())
  }
  pub fn and_rules(&self) -> super::permission::SetView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::permission::SetView::default())
  }
  pub fn and_rules_mut(&mut self) -> super::permission::SetMut<'_> {
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
  pub fn set_and_rules(&mut self,
    val: impl ::protobuf::IntoProxied<super::permission::Set>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // or_rules: optional message envoy.config.rbac.v3.Permission.Set
  pub fn has_or_rules(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_or_rules(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn or_rules_opt(&self) -> ::std::option::Option<super::permission::SetView<'_>> {
    self.has_or_rules().then(|| self.or_rules())
  }
  pub fn or_rules(&self) -> super::permission::SetView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::permission::SetView::default())
  }
  pub fn or_rules_mut(&mut self) -> super::permission::SetMut<'_> {
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
  pub fn set_or_rules(&mut self,
    val: impl ::protobuf::IntoProxied<super::permission::Set>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // any: optional bool
  pub fn has_any(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_any(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn any_opt(&self) -> ::std::option::Option<bool> {
    self.has_any().then(|| self.any())
  }
  pub fn any(&self) -> bool {
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
  pub fn set_any(&mut self, val: bool) {
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

  // header: optional message envoy.config.route.v3.HeaderMatcher
  pub fn has_header(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_header(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn header_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherView<'_>> {
    self.has_header().then(|| self.header())
  }
  pub fn header(&self) -> crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherView::default())
  }
  pub fn header_mut(&mut self) -> crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherMut<'_> {
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
  pub fn set_header(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // url_path: optional message envoy.type.matcher.v3.PathMatcher
  pub fn has_url_path(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_url_path(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn url_path_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::path::PathMatcherView<'_>> {
    self.has_url_path().then(|| self.url_path())
  }
  pub fn url_path(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::path::PathMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::path::PathMatcherView::default())
  }
  pub fn url_path_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::path::PathMatcherMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         9, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_url_path(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::path::PathMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  // destination_ip: optional message envoy.config.core.v3.CidrRange
  pub fn has_destination_ip(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_destination_ip(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn destination_ip_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::CidrRangeView<'_>> {
    self.has_destination_ip().then(|| self.destination_ip())
  }
  pub fn destination_ip(&self) -> crate::xds::generated::envoy::config::core::v3::address::CidrRangeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::CidrRangeView::default())
  }
  pub fn destination_ip_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::CidrRangeMut<'_> {
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
  pub fn set_destination_ip(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::CidrRange>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // destination_port: optional uint32
  pub fn has_destination_port(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_destination_port(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn destination_port_opt(&self) -> ::std::option::Option<u32> {
    self.has_destination_port().then(|| self.destination_port())
  }
  pub fn destination_port(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        5, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_destination_port(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        5, val.into()
      )
    }
  }

  // destination_port_range: optional message envoy.type.v3.Int32Range
  pub fn has_destination_port_range(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_destination_port_range(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn destination_port_range_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::range::Int32RangeView<'_>> {
    self.has_destination_port_range().then(|| self.destination_port_range())
  }
  pub fn destination_port_range(&self) -> crate::xds::generated::envoy::r#type::v3::range::Int32RangeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::range::Int32RangeView::default())
  }
  pub fn destination_port_range_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::range::Int32RangeMut<'_> {
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
  pub fn set_destination_port_range(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::range::Int32Range>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // metadata: optional message envoy.type.matcher.v3.MetadataMatcher
  pub fn has_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn metadata_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView<'_>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView::default())
  }
  pub fn metadata_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherMut<'_> {
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
  pub fn set_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // not_rule: optional message envoy.config.rbac.v3.Permission
  pub fn has_not_rule(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_not_rule(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn not_rule_opt(&self) -> ::std::option::Option<super::PermissionView<'_>> {
    self.has_not_rule().then(|| self.not_rule())
  }
  pub fn not_rule(&self) -> super::PermissionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PermissionView::default())
  }
  pub fn not_rule_mut(&mut self) -> super::PermissionMut<'_> {
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
  pub fn set_not_rule(&mut self,
    val: impl ::protobuf::IntoProxied<super::Permission>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // requested_server_name: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_requested_server_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_requested_server_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn requested_server_name_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_>> {
    self.has_requested_server_name().then(|| self.requested_server_name())
  }
  pub fn requested_server_name(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }
  pub fn requested_server_name_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherMut<'_> {
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
  pub fn set_requested_server_name(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // matcher: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn matcher_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_matcher().then(|| self.matcher())
  }
  pub fn matcher(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn matcher_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // uri_template: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_uri_template(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_uri_template(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn uri_template_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_uri_template().then(|| self.uri_template())
  }
  pub fn uri_template(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn uri_template_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_uri_template(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val
      );
    }
  }

  // sourced_metadata: optional message envoy.config.rbac.v3.SourcedMetadata
  pub fn has_sourced_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_sourced_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn sourced_metadata_opt(&self) -> ::std::option::Option<super::SourcedMetadataView<'_>> {
    self.has_sourced_metadata().then(|| self.sourced_metadata())
  }
  pub fn sourced_metadata(&self) -> super::SourcedMetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SourcedMetadataView::default())
  }
  pub fn sourced_metadata_mut(&mut self) -> super::SourcedMetadataMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         13, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_sourced_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<super::SourcedMetadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val
      );
    }
  }

  pub fn rule(&self) -> super::permission::RuleOneof<'_> {
    match &self.rule_case() {
      super::permission::RuleCase::AndRules =>
          super::permission::RuleOneof::AndRules(self.and_rules()),
      super::permission::RuleCase::OrRules =>
          super::permission::RuleOneof::OrRules(self.or_rules()),
      super::permission::RuleCase::Any =>
          super::permission::RuleOneof::Any(self.any()),
      super::permission::RuleCase::Header =>
          super::permission::RuleOneof::Header(self.header()),
      super::permission::RuleCase::UrlPath =>
          super::permission::RuleOneof::UrlPath(self.url_path()),
      super::permission::RuleCase::DestinationIp =>
          super::permission::RuleOneof::DestinationIp(self.destination_ip()),
      super::permission::RuleCase::DestinationPort =>
          super::permission::RuleOneof::DestinationPort(self.destination_port()),
      super::permission::RuleCase::DestinationPortRange =>
          super::permission::RuleOneof::DestinationPortRange(self.destination_port_range()),
      super::permission::RuleCase::Metadata =>
          super::permission::RuleOneof::Metadata(self.metadata()),
      super::permission::RuleCase::NotRule =>
          super::permission::RuleOneof::NotRule(self.not_rule()),
      super::permission::RuleCase::RequestedServerName =>
          super::permission::RuleOneof::RequestedServerName(self.requested_server_name()),
      super::permission::RuleCase::Matcher =>
          super::permission::RuleOneof::Matcher(self.matcher()),
      super::permission::RuleCase::UriTemplate =>
          super::permission::RuleOneof::UriTemplate(self.uri_template()),
      super::permission::RuleCase::SourcedMetadata =>
          super::permission::RuleOneof::SourcedMetadata(self.sourced_metadata()),
      _ => super::permission::RuleOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn rule_case(&self) -> super::permission::RuleCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::permission::RuleCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `PermissionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for PermissionMut<'_> {}

// SAFETY:
// - `PermissionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for PermissionMut<'_> {}

impl<'msg> ::protobuf::AsView for PermissionMut<'msg> {
  type Proxied = Permission;
  fn as_view(&self) -> ::protobuf::View<'_, Permission> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PermissionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Permission>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for PermissionMut<'msg> {
  type MutProxied = Permission;
  fn as_mut(&mut self) -> PermissionMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PermissionMut<'msg> {
  fn into_mut<'shorter>(self) -> PermissionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Permission {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Permission> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PermissionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PermissionMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // and_rules: optional message envoy.config.rbac.v3.Permission.Set
  pub fn has_and_rules(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_and_rules(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn and_rules_opt(&self) -> ::std::option::Option<super::permission::SetView<'_>> {
    self.has_and_rules().then(|| self.and_rules())
  }
  pub fn and_rules(&self) -> super::permission::SetView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::permission::SetView::default())
  }
  pub fn and_rules_mut(&mut self) -> super::permission::SetMut<'_> {
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
  pub fn set_and_rules(&mut self,
    val: impl ::protobuf::IntoProxied<super::permission::Set>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // or_rules: optional message envoy.config.rbac.v3.Permission.Set
  pub fn has_or_rules(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_or_rules(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn or_rules_opt(&self) -> ::std::option::Option<super::permission::SetView<'_>> {
    self.has_or_rules().then(|| self.or_rules())
  }
  pub fn or_rules(&self) -> super::permission::SetView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::permission::SetView::default())
  }
  pub fn or_rules_mut(&mut self) -> super::permission::SetMut<'_> {
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
  pub fn set_or_rules(&mut self,
    val: impl ::protobuf::IntoProxied<super::permission::Set>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // any: optional bool
  pub fn has_any(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_any(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn any_opt(&self) -> ::std::option::Option<bool> {
    self.has_any().then(|| self.any())
  }
  pub fn any(&self) -> bool {
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
  pub fn set_any(&mut self, val: bool) {
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

  // header: optional message envoy.config.route.v3.HeaderMatcher
  pub fn has_header(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_header(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn header_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherView<'_>> {
    self.has_header().then(|| self.header())
  }
  pub fn header(&self) -> crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherView::default())
  }
  pub fn header_mut(&mut self) -> crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherMut<'_> {
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
  pub fn set_header(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // url_path: optional message envoy.type.matcher.v3.PathMatcher
  pub fn has_url_path(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_url_path(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn url_path_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::path::PathMatcherView<'_>> {
    self.has_url_path().then(|| self.url_path())
  }
  pub fn url_path(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::path::PathMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::path::PathMatcherView::default())
  }
  pub fn url_path_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::path::PathMatcherMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         9, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_url_path(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::path::PathMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  // destination_ip: optional message envoy.config.core.v3.CidrRange
  pub fn has_destination_ip(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_destination_ip(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn destination_ip_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::CidrRangeView<'_>> {
    self.has_destination_ip().then(|| self.destination_ip())
  }
  pub fn destination_ip(&self) -> crate::xds::generated::envoy::config::core::v3::address::CidrRangeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::CidrRangeView::default())
  }
  pub fn destination_ip_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::CidrRangeMut<'_> {
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
  pub fn set_destination_ip(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::CidrRange>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // destination_port: optional uint32
  pub fn has_destination_port(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_destination_port(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn destination_port_opt(&self) -> ::std::option::Option<u32> {
    self.has_destination_port().then(|| self.destination_port())
  }
  pub fn destination_port(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        5, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_destination_port(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        5, val.into()
      )
    }
  }

  // destination_port_range: optional message envoy.type.v3.Int32Range
  pub fn has_destination_port_range(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_destination_port_range(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn destination_port_range_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::range::Int32RangeView<'_>> {
    self.has_destination_port_range().then(|| self.destination_port_range())
  }
  pub fn destination_port_range(&self) -> crate::xds::generated::envoy::r#type::v3::range::Int32RangeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::range::Int32RangeView::default())
  }
  pub fn destination_port_range_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::range::Int32RangeMut<'_> {
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
  pub fn set_destination_port_range(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::range::Int32Range>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // metadata: optional message envoy.type.matcher.v3.MetadataMatcher
  pub fn has_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn metadata_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView<'_>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView::default())
  }
  pub fn metadata_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherMut<'_> {
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
  pub fn set_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // not_rule: optional message envoy.config.rbac.v3.Permission
  pub fn has_not_rule(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_not_rule(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn not_rule_opt(&self) -> ::std::option::Option<super::PermissionView<'_>> {
    self.has_not_rule().then(|| self.not_rule())
  }
  pub fn not_rule(&self) -> super::PermissionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PermissionView::default())
  }
  pub fn not_rule_mut(&mut self) -> super::PermissionMut<'_> {
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
  pub fn set_not_rule(&mut self,
    val: impl ::protobuf::IntoProxied<super::Permission>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // requested_server_name: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_requested_server_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_requested_server_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn requested_server_name_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_>> {
    self.has_requested_server_name().then(|| self.requested_server_name())
  }
  pub fn requested_server_name(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }
  pub fn requested_server_name_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherMut<'_> {
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
  pub fn set_requested_server_name(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // matcher: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn matcher_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_matcher().then(|| self.matcher())
  }
  pub fn matcher(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn matcher_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // uri_template: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_uri_template(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_uri_template(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn uri_template_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_uri_template().then(|| self.uri_template())
  }
  pub fn uri_template(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn uri_template_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_uri_template(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val
      );
    }
  }

  // sourced_metadata: optional message envoy.config.rbac.v3.SourcedMetadata
  pub fn has_sourced_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_sourced_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn sourced_metadata_opt(&self) -> ::std::option::Option<super::SourcedMetadataView<'_>> {
    self.has_sourced_metadata().then(|| self.sourced_metadata())
  }
  pub fn sourced_metadata(&self) -> super::SourcedMetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SourcedMetadataView::default())
  }
  pub fn sourced_metadata_mut(&mut self) -> super::SourcedMetadataMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         13, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_sourced_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<super::SourcedMetadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val
      );
    }
  }

  pub fn rule(&self) -> super::permission::RuleOneof<'_> {
    match &self.rule_case() {
      super::permission::RuleCase::AndRules =>
          super::permission::RuleOneof::AndRules(self.and_rules()),
      super::permission::RuleCase::OrRules =>
          super::permission::RuleOneof::OrRules(self.or_rules()),
      super::permission::RuleCase::Any =>
          super::permission::RuleOneof::Any(self.any()),
      super::permission::RuleCase::Header =>
          super::permission::RuleOneof::Header(self.header()),
      super::permission::RuleCase::UrlPath =>
          super::permission::RuleOneof::UrlPath(self.url_path()),
      super::permission::RuleCase::DestinationIp =>
          super::permission::RuleOneof::DestinationIp(self.destination_ip()),
      super::permission::RuleCase::DestinationPort =>
          super::permission::RuleOneof::DestinationPort(self.destination_port()),
      super::permission::RuleCase::DestinationPortRange =>
          super::permission::RuleOneof::DestinationPortRange(self.destination_port_range()),
      super::permission::RuleCase::Metadata =>
          super::permission::RuleOneof::Metadata(self.metadata()),
      super::permission::RuleCase::NotRule =>
          super::permission::RuleOneof::NotRule(self.not_rule()),
      super::permission::RuleCase::RequestedServerName =>
          super::permission::RuleOneof::RequestedServerName(self.requested_server_name()),
      super::permission::RuleCase::Matcher =>
          super::permission::RuleOneof::Matcher(self.matcher()),
      super::permission::RuleCase::UriTemplate =>
          super::permission::RuleOneof::UriTemplate(self.uri_template()),
      super::permission::RuleCase::SourcedMetadata =>
          super::permission::RuleOneof::SourcedMetadata(self.sourced_metadata()),
      _ => super::permission::RuleOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn rule_case(&self) -> super::permission::RuleCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::permission::RuleCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl Permission

impl ::std::ops::Drop for Permission {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Permission {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Permission {
  type Proxied = Self;
  fn as_view(&self) -> PermissionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Permission {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PermissionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Permission {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__rbac__v3__Permission_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33/33)33333333^!|#|$|%|,|&|(|-|)|*|+|.|/|0");
        super::permission::envoy__config__rbac__v3__Permission__Set_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__rbac__v3__Permission_msg_init.0, &[super::permission::envoy__config__rbac__v3__Permission__Set_msg_init.0,
            super::permission::envoy__config__rbac__v3__Permission__Set_msg_init.0,
            <crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::address::CidrRange as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            super::envoy__config__rbac__v3__Permission_msg_init.0,
            <crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::matcher::v3::path::PathMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::v3::range::Int32Range as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::SourcedMetadata as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::permission::envoy__config__rbac__v3__Permission__Set_msg_init.0, &[super::envoy__config__rbac__v3__Permission_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__rbac__v3__Permission_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Permission {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Permission {
  type Msg = Permission;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Permission> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Permission {
  type Msg = Permission;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Permission> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PermissionMut<'_> {
  type Msg = Permission;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Permission> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PermissionMut<'_> {
  type Msg = Permission;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Permission> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PermissionView<'_> {
  type Msg = Permission;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Permission> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PermissionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod permission {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__rbac__v3__Permission__Set_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Set {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Set>
}

impl ::protobuf::Message for Set {
  type MessageView<'msg> = SetView<'msg>;
  type MessageMut<'msg> = SetMut<'msg>;
}

impl ::std::default::Default for Set {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Set {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Set` is `Sync` because it does not implement interior mutability.
//    Neither does `SetMut`.
unsafe impl ::std::marker::Sync for Set {}

// SAFETY:
// - `Set` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Set {}

impl ::protobuf::Proxied for Set {
  type View<'msg> = SetView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Set {}

impl ::protobuf::MutProxied for Set {
  type Mut<'msg> = SetMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SetView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Set>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SetView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SetView<'msg> {
  type Message = Set;
}

impl ::std::fmt::Debug for SetView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SetView<'_> {
  fn default() -> SetView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Set>> for SetView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Set>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SetView<'msg> {

  pub fn to_owned(&self) -> Set {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // rules: repeated message envoy.config.rbac.v3.Permission
  pub fn rules(self) -> ::protobuf::RepeatedView<'msg, super::super::Permission> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::Permission>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `SetView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SetView<'_> {}

// SAFETY:
// - `SetView` is `Send` because while its alive a `SetMut` cannot.
// - `SetView` does not use thread-local data.
unsafe impl ::std::marker::Send for SetView<'_> {}

impl<'msg> ::protobuf::AsView for SetView<'msg> {
  type Proxied = Set;
  fn as_view(&self) -> ::protobuf::View<'msg, Set> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SetView<'msg> {
  fn into_view<'shorter>(self) -> SetView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Set> for SetView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Set {
    let mut dst = Set::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Set> for SetMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Set {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Set {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SetView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SetMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SetMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Set>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SetMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SetMut<'msg> {
  type Message = Set;
}

impl ::std::fmt::Debug for SetMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Set>> for SetMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Set>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SetMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Set> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Set {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // rules: repeated message envoy.config.rbac.v3.Permission
  pub fn rules(&self) -> ::protobuf::RepeatedView<'_, super::super::Permission> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::Permission>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn rules_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::Permission> {
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
  pub fn set_rules(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::Permission>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `SetMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SetMut<'_> {}

// SAFETY:
// - `SetMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SetMut<'_> {}

impl<'msg> ::protobuf::AsView for SetMut<'msg> {
  type Proxied = Set;
  fn as_view(&self) -> ::protobuf::View<'_, Set> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SetMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Set>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SetMut<'msg> {
  type MutProxied = Set;
  fn as_mut(&mut self) -> SetMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SetMut<'msg> {
  fn into_mut<'shorter>(self) -> SetMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Set {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Set> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SetView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SetMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // rules: repeated message envoy.config.rbac.v3.Permission
  pub fn rules(&self) -> ::protobuf::RepeatedView<'_, super::super::Permission> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::Permission>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn rules_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::Permission> {
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
  pub fn set_rules(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::Permission>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl Set

impl ::std::ops::Drop for Set {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Set {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Set {
  type Proxied = Self;
  fn as_view(&self) -> SetView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Set {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SetMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Set {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::super::Permission as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::permission::envoy__config__rbac__v3__Permission__Set_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Set {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Set {
  type Msg = Set;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Set> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Set {
  type Msg = Set;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Set> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SetMut<'_> {
  type Msg = Set;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Set> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SetMut<'_> {
  type Msg = Set;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Set> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SetView<'_> {
  type Msg = Set;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Set> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SetMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum RuleOneof<'msg> {
  AndRules(::protobuf::View<'msg, super::super::permission::Set>) = 1,
  OrRules(::protobuf::View<'msg, super::super::permission::Set>) = 2,
  Any(bool) = 3,
  Header(::protobuf::View<'msg, crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher>) = 4,
  UrlPath(::protobuf::View<'msg, crate::xds::generated::envoy::r#type::matcher::v3::path::PathMatcher>) = 10,
  DestinationIp(::protobuf::View<'msg, crate::xds::generated::envoy::config::core::v3::address::CidrRange>) = 5,
  DestinationPort(u32) = 6,
  DestinationPortRange(::protobuf::View<'msg, crate::xds::generated::envoy::r#type::v3::range::Int32Range>) = 11,
  Metadata(::protobuf::View<'msg, crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcher>) = 7,
  NotRule(::protobuf::View<'msg, super::super::Permission>) = 8,
  RequestedServerName(::protobuf::View<'msg, crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>) = 9,
  Matcher(::protobuf::View<'msg, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) = 12,
  UriTemplate(::protobuf::View<'msg, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) = 13,
  SourcedMetadata(::protobuf::View<'msg, super::super::SourcedMetadata>) = 14,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum RuleCase {
  AndRules = 1,
  OrRules = 2,
  Any = 3,
  Header = 4,
  UrlPath = 10,
  DestinationIp = 5,
  DestinationPort = 6,
  DestinationPortRange = 11,
  Metadata = 7,
  NotRule = 8,
  RequestedServerName = 9,
  Matcher = 12,
  UriTemplate = 13,
  SourcedMetadata = 14,

  not_set = 0
}

impl RuleCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<RuleCase> {
    match v {
      0 => Some(RuleCase::not_set),
      1 => Some(RuleCase::AndRules),
      2 => Some(RuleCase::OrRules),
      3 => Some(RuleCase::Any),
      4 => Some(RuleCase::Header),
      10 => Some(RuleCase::UrlPath),
      5 => Some(RuleCase::DestinationIp),
      6 => Some(RuleCase::DestinationPort),
      11 => Some(RuleCase::DestinationPortRange),
      7 => Some(RuleCase::Metadata),
      8 => Some(RuleCase::NotRule),
      9 => Some(RuleCase::RequestedServerName),
      12 => Some(RuleCase::Matcher),
      13 => Some(RuleCase::UriTemplate),
      14 => Some(RuleCase::SourcedMetadata),
      _ => None
    }
  }
}
}  // pub mod permission


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__rbac__v3__Principal_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Principal {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Principal>
}

impl ::protobuf::Message for Principal {
  type MessageView<'msg> = PrincipalView<'msg>;
  type MessageMut<'msg> = PrincipalMut<'msg>;
}

impl ::std::default::Default for Principal {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Principal {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Principal` is `Sync` because it does not implement interior mutability.
//    Neither does `PrincipalMut`.
unsafe impl ::std::marker::Sync for Principal {}

// SAFETY:
// - `Principal` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Principal {}

impl ::protobuf::Proxied for Principal {
  type View<'msg> = PrincipalView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Principal {}

impl ::protobuf::MutProxied for Principal {
  type Mut<'msg> = PrincipalMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PrincipalView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Principal>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PrincipalView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PrincipalView<'msg> {
  type Message = Principal;
}

impl ::std::fmt::Debug for PrincipalView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PrincipalView<'_> {
  fn default() -> PrincipalView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Principal>> for PrincipalView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Principal>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PrincipalView<'msg> {

  pub fn to_owned(&self) -> Principal {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // and_ids: optional message envoy.config.rbac.v3.Principal.Set
  pub fn has_and_ids(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn and_ids_opt(self) -> ::std::option::Option<super::principal::SetView<'msg>> {
    self.has_and_ids().then(|| self.and_ids())
  }
  pub fn and_ids(self) -> super::principal::SetView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::principal::SetView::default())
  }

  // or_ids: optional message envoy.config.rbac.v3.Principal.Set
  pub fn has_or_ids(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn or_ids_opt(self) -> ::std::option::Option<super::principal::SetView<'msg>> {
    self.has_or_ids().then(|| self.or_ids())
  }
  pub fn or_ids(self) -> super::principal::SetView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::principal::SetView::default())
  }

  // any: optional bool
  pub fn has_any(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn any_opt(self) -> ::std::option::Option<bool> {
    self.has_any().then(|| self.any())
  }
  pub fn any(self) -> bool {
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

  // authenticated: optional message envoy.config.rbac.v3.Principal.Authenticated
  pub fn has_authenticated(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn authenticated_opt(self) -> ::std::option::Option<super::principal::AuthenticatedView<'msg>> {
    self.has_authenticated().then(|| self.authenticated())
  }
  pub fn authenticated(self) -> super::principal::AuthenticatedView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::principal::AuthenticatedView::default())
  }

  // source_ip: optional message envoy.config.core.v3.CidrRange
  pub fn has_source_ip(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn source_ip_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::CidrRangeView<'msg>> {
    self.has_source_ip().then(|| self.source_ip())
  }
  pub fn source_ip(self) -> crate::xds::generated::envoy::config::core::v3::address::CidrRangeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::CidrRangeView::default())
  }

  // direct_remote_ip: optional message envoy.config.core.v3.CidrRange
  pub fn has_direct_remote_ip(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn direct_remote_ip_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::CidrRangeView<'msg>> {
    self.has_direct_remote_ip().then(|| self.direct_remote_ip())
  }
  pub fn direct_remote_ip(self) -> crate::xds::generated::envoy::config::core::v3::address::CidrRangeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::CidrRangeView::default())
  }

  // remote_ip: optional message envoy.config.core.v3.CidrRange
  pub fn has_remote_ip(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn remote_ip_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::CidrRangeView<'msg>> {
    self.has_remote_ip().then(|| self.remote_ip())
  }
  pub fn remote_ip(self) -> crate::xds::generated::envoy::config::core::v3::address::CidrRangeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::CidrRangeView::default())
  }

  // header: optional message envoy.config.route.v3.HeaderMatcher
  pub fn has_header(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn header_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherView<'msg>> {
    self.has_header().then(|| self.header())
  }
  pub fn header(self) -> crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherView::default())
  }

  // url_path: optional message envoy.type.matcher.v3.PathMatcher
  pub fn has_url_path(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn url_path_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::path::PathMatcherView<'msg>> {
    self.has_url_path().then(|| self.url_path())
  }
  pub fn url_path(self) -> crate::xds::generated::envoy::r#type::matcher::v3::path::PathMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::path::PathMatcherView::default())
  }

  // metadata: optional message envoy.type.matcher.v3.MetadataMatcher
  pub fn has_metadata(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn metadata_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView<'msg>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(self) -> crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView::default())
  }

  // filter_state: optional message envoy.type.matcher.v3.FilterStateMatcher
  pub fn has_filter_state(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn filter_state_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::filter_state::FilterStateMatcherView<'msg>> {
    self.has_filter_state().then(|| self.filter_state())
  }
  pub fn filter_state(self) -> crate::xds::generated::envoy::r#type::matcher::v3::filter_state::FilterStateMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::filter_state::FilterStateMatcherView::default())
  }

  // not_id: optional message envoy.config.rbac.v3.Principal
  pub fn has_not_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn not_id_opt(self) -> ::std::option::Option<super::PrincipalView<'msg>> {
    self.has_not_id().then(|| self.not_id())
  }
  pub fn not_id(self) -> super::PrincipalView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PrincipalView::default())
  }

  // sourced_metadata: optional message envoy.config.rbac.v3.SourcedMetadata
  pub fn has_sourced_metadata(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn sourced_metadata_opt(self) -> ::std::option::Option<super::SourcedMetadataView<'msg>> {
    self.has_sourced_metadata().then(|| self.sourced_metadata())
  }
  pub fn sourced_metadata(self) -> super::SourcedMetadataView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SourcedMetadataView::default())
  }

  // custom: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_custom(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn custom_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_custom().then(|| self.custom())
  }
  pub fn custom(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  pub fn identifier(self) -> super::principal::IdentifierOneof<'msg> {
    match self.identifier_case() {
      super::principal::IdentifierCase::AndIds =>
          super::principal::IdentifierOneof::AndIds(self.and_ids()),
      super::principal::IdentifierCase::OrIds =>
          super::principal::IdentifierOneof::OrIds(self.or_ids()),
      super::principal::IdentifierCase::Any =>
          super::principal::IdentifierOneof::Any(self.any()),
      super::principal::IdentifierCase::Authenticated =>
          super::principal::IdentifierOneof::Authenticated(self.authenticated()),
      super::principal::IdentifierCase::SourceIp =>
          super::principal::IdentifierOneof::SourceIp(self.source_ip()),
      super::principal::IdentifierCase::DirectRemoteIp =>
          super::principal::IdentifierOneof::DirectRemoteIp(self.direct_remote_ip()),
      super::principal::IdentifierCase::RemoteIp =>
          super::principal::IdentifierOneof::RemoteIp(self.remote_ip()),
      super::principal::IdentifierCase::Header =>
          super::principal::IdentifierOneof::Header(self.header()),
      super::principal::IdentifierCase::UrlPath =>
          super::principal::IdentifierOneof::UrlPath(self.url_path()),
      super::principal::IdentifierCase::Metadata =>
          super::principal::IdentifierOneof::Metadata(self.metadata()),
      super::principal::IdentifierCase::FilterState =>
          super::principal::IdentifierOneof::FilterState(self.filter_state()),
      super::principal::IdentifierCase::NotId =>
          super::principal::IdentifierOneof::NotId(self.not_id()),
      super::principal::IdentifierCase::SourcedMetadata =>
          super::principal::IdentifierOneof::SourcedMetadata(self.sourced_metadata()),
      super::principal::IdentifierCase::Custom =>
          super::principal::IdentifierOneof::Custom(self.custom()),
      _ => super::principal::IdentifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn identifier_case(self) -> super::principal::IdentifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::principal::IdentifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `PrincipalView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for PrincipalView<'_> {}

// SAFETY:
// - `PrincipalView` is `Send` because while its alive a `PrincipalMut` cannot.
// - `PrincipalView` does not use thread-local data.
unsafe impl ::std::marker::Send for PrincipalView<'_> {}

impl<'msg> ::protobuf::AsView for PrincipalView<'msg> {
  type Proxied = Principal;
  fn as_view(&self) -> ::protobuf::View<'msg, Principal> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PrincipalView<'msg> {
  fn into_view<'shorter>(self) -> PrincipalView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Principal> for PrincipalView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Principal {
    let mut dst = Principal::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Principal> for PrincipalMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Principal {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Principal {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PrincipalView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PrincipalMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PrincipalMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Principal>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PrincipalMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PrincipalMut<'msg> {
  type Message = Principal;
}

impl ::std::fmt::Debug for PrincipalMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Principal>> for PrincipalMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Principal>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PrincipalMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Principal> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Principal {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // and_ids: optional message envoy.config.rbac.v3.Principal.Set
  pub fn has_and_ids(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_and_ids(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn and_ids_opt(&self) -> ::std::option::Option<super::principal::SetView<'_>> {
    self.has_and_ids().then(|| self.and_ids())
  }
  pub fn and_ids(&self) -> super::principal::SetView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::principal::SetView::default())
  }
  pub fn and_ids_mut(&mut self) -> super::principal::SetMut<'_> {
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
  pub fn set_and_ids(&mut self,
    val: impl ::protobuf::IntoProxied<super::principal::Set>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // or_ids: optional message envoy.config.rbac.v3.Principal.Set
  pub fn has_or_ids(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_or_ids(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn or_ids_opt(&self) -> ::std::option::Option<super::principal::SetView<'_>> {
    self.has_or_ids().then(|| self.or_ids())
  }
  pub fn or_ids(&self) -> super::principal::SetView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::principal::SetView::default())
  }
  pub fn or_ids_mut(&mut self) -> super::principal::SetMut<'_> {
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
  pub fn set_or_ids(&mut self,
    val: impl ::protobuf::IntoProxied<super::principal::Set>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // any: optional bool
  pub fn has_any(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_any(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn any_opt(&self) -> ::std::option::Option<bool> {
    self.has_any().then(|| self.any())
  }
  pub fn any(&self) -> bool {
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
  pub fn set_any(&mut self, val: bool) {
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

  // authenticated: optional message envoy.config.rbac.v3.Principal.Authenticated
  pub fn has_authenticated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_authenticated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn authenticated_opt(&self) -> ::std::option::Option<super::principal::AuthenticatedView<'_>> {
    self.has_authenticated().then(|| self.authenticated())
  }
  pub fn authenticated(&self) -> super::principal::AuthenticatedView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::principal::AuthenticatedView::default())
  }
  pub fn authenticated_mut(&mut self) -> super::principal::AuthenticatedMut<'_> {
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
  pub fn set_authenticated(&mut self,
    val: impl ::protobuf::IntoProxied<super::principal::Authenticated>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // source_ip: optional message envoy.config.core.v3.CidrRange
  pub fn has_source_ip(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_source_ip(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn source_ip_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::CidrRangeView<'_>> {
    self.has_source_ip().then(|| self.source_ip())
  }
  pub fn source_ip(&self) -> crate::xds::generated::envoy::config::core::v3::address::CidrRangeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::CidrRangeView::default())
  }
  pub fn source_ip_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::CidrRangeMut<'_> {
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
  pub fn set_source_ip(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::CidrRange>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // direct_remote_ip: optional message envoy.config.core.v3.CidrRange
  pub fn has_direct_remote_ip(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_direct_remote_ip(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn direct_remote_ip_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::CidrRangeView<'_>> {
    self.has_direct_remote_ip().then(|| self.direct_remote_ip())
  }
  pub fn direct_remote_ip(&self) -> crate::xds::generated::envoy::config::core::v3::address::CidrRangeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::CidrRangeView::default())
  }
  pub fn direct_remote_ip_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::CidrRangeMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         9, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_direct_remote_ip(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::CidrRange>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  // remote_ip: optional message envoy.config.core.v3.CidrRange
  pub fn has_remote_ip(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_remote_ip(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn remote_ip_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::CidrRangeView<'_>> {
    self.has_remote_ip().then(|| self.remote_ip())
  }
  pub fn remote_ip(&self) -> crate::xds::generated::envoy::config::core::v3::address::CidrRangeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::CidrRangeView::default())
  }
  pub fn remote_ip_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::CidrRangeMut<'_> {
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
  pub fn set_remote_ip(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::CidrRange>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // header: optional message envoy.config.route.v3.HeaderMatcher
  pub fn has_header(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_header(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn header_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherView<'_>> {
    self.has_header().then(|| self.header())
  }
  pub fn header(&self) -> crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherView::default())
  }
  pub fn header_mut(&mut self) -> crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherMut<'_> {
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
  pub fn set_header(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // url_path: optional message envoy.type.matcher.v3.PathMatcher
  pub fn has_url_path(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_url_path(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn url_path_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::path::PathMatcherView<'_>> {
    self.has_url_path().then(|| self.url_path())
  }
  pub fn url_path(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::path::PathMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::path::PathMatcherView::default())
  }
  pub fn url_path_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::path::PathMatcherMut<'_> {
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
  pub fn set_url_path(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::path::PathMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // metadata: optional message envoy.type.matcher.v3.MetadataMatcher
  pub fn has_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn metadata_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView<'_>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView::default())
  }
  pub fn metadata_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherMut<'_> {
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
  pub fn set_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // filter_state: optional message envoy.type.matcher.v3.FilterStateMatcher
  pub fn has_filter_state(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_filter_state(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn filter_state_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::filter_state::FilterStateMatcherView<'_>> {
    self.has_filter_state().then(|| self.filter_state())
  }
  pub fn filter_state(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::filter_state::FilterStateMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::filter_state::FilterStateMatcherView::default())
  }
  pub fn filter_state_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::filter_state::FilterStateMatcherMut<'_> {
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
  pub fn set_filter_state(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::filter_state::FilterStateMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // not_id: optional message envoy.config.rbac.v3.Principal
  pub fn has_not_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_not_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn not_id_opt(&self) -> ::std::option::Option<super::PrincipalView<'_>> {
    self.has_not_id().then(|| self.not_id())
  }
  pub fn not_id(&self) -> super::PrincipalView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PrincipalView::default())
  }
  pub fn not_id_mut(&mut self) -> super::PrincipalMut<'_> {
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
  pub fn set_not_id(&mut self,
    val: impl ::protobuf::IntoProxied<super::Principal>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // sourced_metadata: optional message envoy.config.rbac.v3.SourcedMetadata
  pub fn has_sourced_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_sourced_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn sourced_metadata_opt(&self) -> ::std::option::Option<super::SourcedMetadataView<'_>> {
    self.has_sourced_metadata().then(|| self.sourced_metadata())
  }
  pub fn sourced_metadata(&self) -> super::SourcedMetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SourcedMetadataView::default())
  }
  pub fn sourced_metadata_mut(&mut self) -> super::SourcedMetadataMut<'_> {
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
  pub fn set_sourced_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<super::SourcedMetadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val
      );
    }
  }

  // custom: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_custom(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_custom(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn custom_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_custom().then(|| self.custom())
  }
  pub fn custom(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn custom_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         13, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_custom(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val
      );
    }
  }

  pub fn identifier(&self) -> super::principal::IdentifierOneof<'_> {
    match &self.identifier_case() {
      super::principal::IdentifierCase::AndIds =>
          super::principal::IdentifierOneof::AndIds(self.and_ids()),
      super::principal::IdentifierCase::OrIds =>
          super::principal::IdentifierOneof::OrIds(self.or_ids()),
      super::principal::IdentifierCase::Any =>
          super::principal::IdentifierOneof::Any(self.any()),
      super::principal::IdentifierCase::Authenticated =>
          super::principal::IdentifierOneof::Authenticated(self.authenticated()),
      super::principal::IdentifierCase::SourceIp =>
          super::principal::IdentifierOneof::SourceIp(self.source_ip()),
      super::principal::IdentifierCase::DirectRemoteIp =>
          super::principal::IdentifierOneof::DirectRemoteIp(self.direct_remote_ip()),
      super::principal::IdentifierCase::RemoteIp =>
          super::principal::IdentifierOneof::RemoteIp(self.remote_ip()),
      super::principal::IdentifierCase::Header =>
          super::principal::IdentifierOneof::Header(self.header()),
      super::principal::IdentifierCase::UrlPath =>
          super::principal::IdentifierOneof::UrlPath(self.url_path()),
      super::principal::IdentifierCase::Metadata =>
          super::principal::IdentifierOneof::Metadata(self.metadata()),
      super::principal::IdentifierCase::FilterState =>
          super::principal::IdentifierOneof::FilterState(self.filter_state()),
      super::principal::IdentifierCase::NotId =>
          super::principal::IdentifierOneof::NotId(self.not_id()),
      super::principal::IdentifierCase::SourcedMetadata =>
          super::principal::IdentifierOneof::SourcedMetadata(self.sourced_metadata()),
      super::principal::IdentifierCase::Custom =>
          super::principal::IdentifierOneof::Custom(self.custom()),
      _ => super::principal::IdentifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn identifier_case(&self) -> super::principal::IdentifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::principal::IdentifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `PrincipalMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for PrincipalMut<'_> {}

// SAFETY:
// - `PrincipalMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for PrincipalMut<'_> {}

impl<'msg> ::protobuf::AsView for PrincipalMut<'msg> {
  type Proxied = Principal;
  fn as_view(&self) -> ::protobuf::View<'_, Principal> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PrincipalMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Principal>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for PrincipalMut<'msg> {
  type MutProxied = Principal;
  fn as_mut(&mut self) -> PrincipalMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PrincipalMut<'msg> {
  fn into_mut<'shorter>(self) -> PrincipalMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Principal {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Principal> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PrincipalView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PrincipalMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // and_ids: optional message envoy.config.rbac.v3.Principal.Set
  pub fn has_and_ids(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_and_ids(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn and_ids_opt(&self) -> ::std::option::Option<super::principal::SetView<'_>> {
    self.has_and_ids().then(|| self.and_ids())
  }
  pub fn and_ids(&self) -> super::principal::SetView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::principal::SetView::default())
  }
  pub fn and_ids_mut(&mut self) -> super::principal::SetMut<'_> {
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
  pub fn set_and_ids(&mut self,
    val: impl ::protobuf::IntoProxied<super::principal::Set>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // or_ids: optional message envoy.config.rbac.v3.Principal.Set
  pub fn has_or_ids(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_or_ids(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn or_ids_opt(&self) -> ::std::option::Option<super::principal::SetView<'_>> {
    self.has_or_ids().then(|| self.or_ids())
  }
  pub fn or_ids(&self) -> super::principal::SetView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::principal::SetView::default())
  }
  pub fn or_ids_mut(&mut self) -> super::principal::SetMut<'_> {
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
  pub fn set_or_ids(&mut self,
    val: impl ::protobuf::IntoProxied<super::principal::Set>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // any: optional bool
  pub fn has_any(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_any(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn any_opt(&self) -> ::std::option::Option<bool> {
    self.has_any().then(|| self.any())
  }
  pub fn any(&self) -> bool {
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
  pub fn set_any(&mut self, val: bool) {
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

  // authenticated: optional message envoy.config.rbac.v3.Principal.Authenticated
  pub fn has_authenticated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_authenticated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn authenticated_opt(&self) -> ::std::option::Option<super::principal::AuthenticatedView<'_>> {
    self.has_authenticated().then(|| self.authenticated())
  }
  pub fn authenticated(&self) -> super::principal::AuthenticatedView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::principal::AuthenticatedView::default())
  }
  pub fn authenticated_mut(&mut self) -> super::principal::AuthenticatedMut<'_> {
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
  pub fn set_authenticated(&mut self,
    val: impl ::protobuf::IntoProxied<super::principal::Authenticated>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // source_ip: optional message envoy.config.core.v3.CidrRange
  pub fn has_source_ip(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_source_ip(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn source_ip_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::CidrRangeView<'_>> {
    self.has_source_ip().then(|| self.source_ip())
  }
  pub fn source_ip(&self) -> crate::xds::generated::envoy::config::core::v3::address::CidrRangeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::CidrRangeView::default())
  }
  pub fn source_ip_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::CidrRangeMut<'_> {
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
  pub fn set_source_ip(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::CidrRange>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // direct_remote_ip: optional message envoy.config.core.v3.CidrRange
  pub fn has_direct_remote_ip(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_direct_remote_ip(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn direct_remote_ip_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::CidrRangeView<'_>> {
    self.has_direct_remote_ip().then(|| self.direct_remote_ip())
  }
  pub fn direct_remote_ip(&self) -> crate::xds::generated::envoy::config::core::v3::address::CidrRangeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::CidrRangeView::default())
  }
  pub fn direct_remote_ip_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::CidrRangeMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         9, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_direct_remote_ip(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::CidrRange>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  // remote_ip: optional message envoy.config.core.v3.CidrRange
  pub fn has_remote_ip(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_remote_ip(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn remote_ip_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::CidrRangeView<'_>> {
    self.has_remote_ip().then(|| self.remote_ip())
  }
  pub fn remote_ip(&self) -> crate::xds::generated::envoy::config::core::v3::address::CidrRangeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::CidrRangeView::default())
  }
  pub fn remote_ip_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::CidrRangeMut<'_> {
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
  pub fn set_remote_ip(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::CidrRange>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // header: optional message envoy.config.route.v3.HeaderMatcher
  pub fn has_header(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_header(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn header_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherView<'_>> {
    self.has_header().then(|| self.header())
  }
  pub fn header(&self) -> crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherView::default())
  }
  pub fn header_mut(&mut self) -> crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherMut<'_> {
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
  pub fn set_header(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // url_path: optional message envoy.type.matcher.v3.PathMatcher
  pub fn has_url_path(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_url_path(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn url_path_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::path::PathMatcherView<'_>> {
    self.has_url_path().then(|| self.url_path())
  }
  pub fn url_path(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::path::PathMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::path::PathMatcherView::default())
  }
  pub fn url_path_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::path::PathMatcherMut<'_> {
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
  pub fn set_url_path(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::path::PathMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // metadata: optional message envoy.type.matcher.v3.MetadataMatcher
  pub fn has_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn metadata_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView<'_>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView::default())
  }
  pub fn metadata_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherMut<'_> {
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
  pub fn set_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // filter_state: optional message envoy.type.matcher.v3.FilterStateMatcher
  pub fn has_filter_state(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_filter_state(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn filter_state_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::filter_state::FilterStateMatcherView<'_>> {
    self.has_filter_state().then(|| self.filter_state())
  }
  pub fn filter_state(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::filter_state::FilterStateMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::filter_state::FilterStateMatcherView::default())
  }
  pub fn filter_state_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::filter_state::FilterStateMatcherMut<'_> {
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
  pub fn set_filter_state(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::filter_state::FilterStateMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // not_id: optional message envoy.config.rbac.v3.Principal
  pub fn has_not_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_not_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn not_id_opt(&self) -> ::std::option::Option<super::PrincipalView<'_>> {
    self.has_not_id().then(|| self.not_id())
  }
  pub fn not_id(&self) -> super::PrincipalView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PrincipalView::default())
  }
  pub fn not_id_mut(&mut self) -> super::PrincipalMut<'_> {
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
  pub fn set_not_id(&mut self,
    val: impl ::protobuf::IntoProxied<super::Principal>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // sourced_metadata: optional message envoy.config.rbac.v3.SourcedMetadata
  pub fn has_sourced_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_sourced_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn sourced_metadata_opt(&self) -> ::std::option::Option<super::SourcedMetadataView<'_>> {
    self.has_sourced_metadata().then(|| self.sourced_metadata())
  }
  pub fn sourced_metadata(&self) -> super::SourcedMetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SourcedMetadataView::default())
  }
  pub fn sourced_metadata_mut(&mut self) -> super::SourcedMetadataMut<'_> {
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
  pub fn set_sourced_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<super::SourcedMetadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val
      );
    }
  }

  // custom: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_custom(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_custom(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn custom_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_custom().then(|| self.custom())
  }
  pub fn custom(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn custom_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         13, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_custom(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val
      );
    }
  }

  pub fn identifier(&self) -> super::principal::IdentifierOneof<'_> {
    match &self.identifier_case() {
      super::principal::IdentifierCase::AndIds =>
          super::principal::IdentifierOneof::AndIds(self.and_ids()),
      super::principal::IdentifierCase::OrIds =>
          super::principal::IdentifierOneof::OrIds(self.or_ids()),
      super::principal::IdentifierCase::Any =>
          super::principal::IdentifierOneof::Any(self.any()),
      super::principal::IdentifierCase::Authenticated =>
          super::principal::IdentifierOneof::Authenticated(self.authenticated()),
      super::principal::IdentifierCase::SourceIp =>
          super::principal::IdentifierOneof::SourceIp(self.source_ip()),
      super::principal::IdentifierCase::DirectRemoteIp =>
          super::principal::IdentifierOneof::DirectRemoteIp(self.direct_remote_ip()),
      super::principal::IdentifierCase::RemoteIp =>
          super::principal::IdentifierOneof::RemoteIp(self.remote_ip()),
      super::principal::IdentifierCase::Header =>
          super::principal::IdentifierOneof::Header(self.header()),
      super::principal::IdentifierCase::UrlPath =>
          super::principal::IdentifierOneof::UrlPath(self.url_path()),
      super::principal::IdentifierCase::Metadata =>
          super::principal::IdentifierOneof::Metadata(self.metadata()),
      super::principal::IdentifierCase::FilterState =>
          super::principal::IdentifierOneof::FilterState(self.filter_state()),
      super::principal::IdentifierCase::NotId =>
          super::principal::IdentifierOneof::NotId(self.not_id()),
      super::principal::IdentifierCase::SourcedMetadata =>
          super::principal::IdentifierOneof::SourcedMetadata(self.sourced_metadata()),
      super::principal::IdentifierCase::Custom =>
          super::principal::IdentifierOneof::Custom(self.custom()),
      _ => super::principal::IdentifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn identifier_case(&self) -> super::principal::IdentifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::principal::IdentifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl Principal

impl ::std::ops::Drop for Principal {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Principal {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Principal {
  type Proxied = Self;
  fn as_view(&self) -> PrincipalView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Principal {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PrincipalMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Principal {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__rbac__v3__Principal_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33/33333333333^!|#|$|%|&|,|-|(|+|)|.|*|/|0");
        super::principal::envoy__config__rbac__v3__Principal__Set_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__rbac__v3__Principal_msg_init.0, &[super::principal::envoy__config__rbac__v3__Principal__Set_msg_init.0,
            super::principal::envoy__config__rbac__v3__Principal__Set_msg_init.0,
            <super::principal::Authenticated as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::address::CidrRange as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            super::envoy__config__rbac__v3__Principal_msg_init.0,
            <crate::xds::generated::envoy::r#type::matcher::v3::path::PathMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::address::CidrRange as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::address::CidrRange as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::matcher::v3::filter_state::FilterStateMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::SourcedMetadata as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::principal::envoy__config__rbac__v3__Principal__Set_msg_init.0, &[super::envoy__config__rbac__v3__Principal_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__rbac__v3__Principal_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Principal {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Principal {
  type Msg = Principal;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Principal> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Principal {
  type Msg = Principal;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Principal> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PrincipalMut<'_> {
  type Msg = Principal;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Principal> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PrincipalMut<'_> {
  type Msg = Principal;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Principal> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PrincipalView<'_> {
  type Msg = Principal;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Principal> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PrincipalMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod principal {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__rbac__v3__Principal__Set_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Set {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Set>
}

impl ::protobuf::Message for Set {
  type MessageView<'msg> = SetView<'msg>;
  type MessageMut<'msg> = SetMut<'msg>;
}

impl ::std::default::Default for Set {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Set {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Set` is `Sync` because it does not implement interior mutability.
//    Neither does `SetMut`.
unsafe impl ::std::marker::Sync for Set {}

// SAFETY:
// - `Set` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Set {}

impl ::protobuf::Proxied for Set {
  type View<'msg> = SetView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Set {}

impl ::protobuf::MutProxied for Set {
  type Mut<'msg> = SetMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SetView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Set>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SetView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SetView<'msg> {
  type Message = Set;
}

impl ::std::fmt::Debug for SetView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SetView<'_> {
  fn default() -> SetView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Set>> for SetView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Set>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SetView<'msg> {

  pub fn to_owned(&self) -> Set {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // ids: repeated message envoy.config.rbac.v3.Principal
  pub fn ids(self) -> ::protobuf::RepeatedView<'msg, super::super::Principal> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::Principal>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `SetView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SetView<'_> {}

// SAFETY:
// - `SetView` is `Send` because while its alive a `SetMut` cannot.
// - `SetView` does not use thread-local data.
unsafe impl ::std::marker::Send for SetView<'_> {}

impl<'msg> ::protobuf::AsView for SetView<'msg> {
  type Proxied = Set;
  fn as_view(&self) -> ::protobuf::View<'msg, Set> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SetView<'msg> {
  fn into_view<'shorter>(self) -> SetView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Set> for SetView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Set {
    let mut dst = Set::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Set> for SetMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Set {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Set {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SetView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SetMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SetMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Set>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SetMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SetMut<'msg> {
  type Message = Set;
}

impl ::std::fmt::Debug for SetMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Set>> for SetMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Set>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SetMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Set> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Set {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // ids: repeated message envoy.config.rbac.v3.Principal
  pub fn ids(&self) -> ::protobuf::RepeatedView<'_, super::super::Principal> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::Principal>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn ids_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::Principal> {
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
  pub fn set_ids(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::Principal>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `SetMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SetMut<'_> {}

// SAFETY:
// - `SetMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SetMut<'_> {}

impl<'msg> ::protobuf::AsView for SetMut<'msg> {
  type Proxied = Set;
  fn as_view(&self) -> ::protobuf::View<'_, Set> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SetMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Set>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SetMut<'msg> {
  type MutProxied = Set;
  fn as_mut(&mut self) -> SetMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SetMut<'msg> {
  fn into_mut<'shorter>(self) -> SetMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Set {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Set> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SetView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SetMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // ids: repeated message envoy.config.rbac.v3.Principal
  pub fn ids(&self) -> ::protobuf::RepeatedView<'_, super::super::Principal> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::Principal>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn ids_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::Principal> {
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
  pub fn set_ids(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::Principal>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl Set

impl ::std::ops::Drop for Set {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Set {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Set {
  type Proxied = Self;
  fn as_view(&self) -> SetView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Set {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SetMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Set {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::super::Principal as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::principal::envoy__config__rbac__v3__Principal__Set_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Set {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Set {
  type Msg = Set;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Set> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Set {
  type Msg = Set;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Set> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SetMut<'_> {
  type Msg = Set;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Set> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SetMut<'_> {
  type Msg = Set;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Set> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SetView<'_> {
  type Msg = Set;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Set> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SetMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__rbac__v3__Principal__Authenticated_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Authenticated {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Authenticated>
}

impl ::protobuf::Message for Authenticated {
  type MessageView<'msg> = AuthenticatedView<'msg>;
  type MessageMut<'msg> = AuthenticatedMut<'msg>;
}

impl ::std::default::Default for Authenticated {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Authenticated {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Authenticated` is `Sync` because it does not implement interior mutability.
//    Neither does `AuthenticatedMut`.
unsafe impl ::std::marker::Sync for Authenticated {}

// SAFETY:
// - `Authenticated` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Authenticated {}

impl ::protobuf::Proxied for Authenticated {
  type View<'msg> = AuthenticatedView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Authenticated {}

impl ::protobuf::MutProxied for Authenticated {
  type Mut<'msg> = AuthenticatedMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AuthenticatedView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Authenticated>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AuthenticatedView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AuthenticatedView<'msg> {
  type Message = Authenticated;
}

impl ::std::fmt::Debug for AuthenticatedView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AuthenticatedView<'_> {
  fn default() -> AuthenticatedView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Authenticated>> for AuthenticatedView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Authenticated>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AuthenticatedView<'msg> {

  pub fn to_owned(&self) -> Authenticated {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // principal_name: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_principal_name(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn principal_name_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'msg>> {
    self.has_principal_name().then(|| self.principal_name())
  }
  pub fn principal_name(self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }

}

// SAFETY:
// - `AuthenticatedView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AuthenticatedView<'_> {}

// SAFETY:
// - `AuthenticatedView` is `Send` because while its alive a `AuthenticatedMut` cannot.
// - `AuthenticatedView` does not use thread-local data.
unsafe impl ::std::marker::Send for AuthenticatedView<'_> {}

impl<'msg> ::protobuf::AsView for AuthenticatedView<'msg> {
  type Proxied = Authenticated;
  fn as_view(&self) -> ::protobuf::View<'msg, Authenticated> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AuthenticatedView<'msg> {
  fn into_view<'shorter>(self) -> AuthenticatedView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Authenticated> for AuthenticatedView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Authenticated {
    let mut dst = Authenticated::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Authenticated> for AuthenticatedMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Authenticated {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Authenticated {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AuthenticatedView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AuthenticatedMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AuthenticatedMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Authenticated>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AuthenticatedMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AuthenticatedMut<'msg> {
  type Message = Authenticated;
}

impl ::std::fmt::Debug for AuthenticatedMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Authenticated>> for AuthenticatedMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Authenticated>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AuthenticatedMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Authenticated> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Authenticated {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // principal_name: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_principal_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_principal_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn principal_name_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_>> {
    self.has_principal_name().then(|| self.principal_name())
  }
  pub fn principal_name(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }
  pub fn principal_name_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherMut<'_> {
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
  pub fn set_principal_name(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>) {

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
// - `AuthenticatedMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AuthenticatedMut<'_> {}

// SAFETY:
// - `AuthenticatedMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AuthenticatedMut<'_> {}

impl<'msg> ::protobuf::AsView for AuthenticatedMut<'msg> {
  type Proxied = Authenticated;
  fn as_view(&self) -> ::protobuf::View<'_, Authenticated> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AuthenticatedMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Authenticated>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AuthenticatedMut<'msg> {
  type MutProxied = Authenticated;
  fn as_mut(&mut self) -> AuthenticatedMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AuthenticatedMut<'msg> {
  fn into_mut<'shorter>(self) -> AuthenticatedMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Authenticated {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Authenticated> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AuthenticatedView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AuthenticatedMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // principal_name: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_principal_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_principal_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn principal_name_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_>> {
    self.has_principal_name().then(|| self.principal_name())
  }
  pub fn principal_name(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }
  pub fn principal_name_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherMut<'_> {
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
  pub fn set_principal_name(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl Authenticated

impl ::std::ops::Drop for Authenticated {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Authenticated {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Authenticated {
  type Proxied = Self;
  fn as_view(&self) -> AuthenticatedView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Authenticated {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AuthenticatedMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Authenticated {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::principal::envoy__config__rbac__v3__Principal__Authenticated_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$a3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::principal::envoy__config__rbac__v3__Principal__Authenticated_msg_init.0, &[<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::principal::envoy__config__rbac__v3__Principal__Authenticated_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Authenticated {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Authenticated {
  type Msg = Authenticated;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Authenticated> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Authenticated {
  type Msg = Authenticated;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Authenticated> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AuthenticatedMut<'_> {
  type Msg = Authenticated;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Authenticated> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AuthenticatedMut<'_> {
  type Msg = Authenticated;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Authenticated> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AuthenticatedView<'_> {
  type Msg = Authenticated;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Authenticated> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AuthenticatedMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum IdentifierOneof<'msg> {
  AndIds(::protobuf::View<'msg, super::super::principal::Set>) = 1,
  OrIds(::protobuf::View<'msg, super::super::principal::Set>) = 2,
  Any(bool) = 3,
  Authenticated(::protobuf::View<'msg, super::super::principal::Authenticated>) = 4,
  SourceIp(::protobuf::View<'msg, crate::xds::generated::envoy::config::core::v3::address::CidrRange>) = 5,
  DirectRemoteIp(::protobuf::View<'msg, crate::xds::generated::envoy::config::core::v3::address::CidrRange>) = 10,
  RemoteIp(::protobuf::View<'msg, crate::xds::generated::envoy::config::core::v3::address::CidrRange>) = 11,
  Header(::protobuf::View<'msg, crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher>) = 6,
  UrlPath(::protobuf::View<'msg, crate::xds::generated::envoy::r#type::matcher::v3::path::PathMatcher>) = 9,
  Metadata(::protobuf::View<'msg, crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcher>) = 7,
  FilterState(::protobuf::View<'msg, crate::xds::generated::envoy::r#type::matcher::v3::filter_state::FilterStateMatcher>) = 12,
  NotId(::protobuf::View<'msg, super::super::Principal>) = 8,
  SourcedMetadata(::protobuf::View<'msg, super::super::SourcedMetadata>) = 13,
  Custom(::protobuf::View<'msg, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) = 14,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum IdentifierCase {
  AndIds = 1,
  OrIds = 2,
  Any = 3,
  Authenticated = 4,
  SourceIp = 5,
  DirectRemoteIp = 10,
  RemoteIp = 11,
  Header = 6,
  UrlPath = 9,
  Metadata = 7,
  FilterState = 12,
  NotId = 8,
  SourcedMetadata = 13,
  Custom = 14,

  not_set = 0
}

impl IdentifierCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<IdentifierCase> {
    match v {
      0 => Some(IdentifierCase::not_set),
      1 => Some(IdentifierCase::AndIds),
      2 => Some(IdentifierCase::OrIds),
      3 => Some(IdentifierCase::Any),
      4 => Some(IdentifierCase::Authenticated),
      5 => Some(IdentifierCase::SourceIp),
      10 => Some(IdentifierCase::DirectRemoteIp),
      11 => Some(IdentifierCase::RemoteIp),
      6 => Some(IdentifierCase::Header),
      9 => Some(IdentifierCase::UrlPath),
      7 => Some(IdentifierCase::Metadata),
      12 => Some(IdentifierCase::FilterState),
      8 => Some(IdentifierCase::NotId),
      13 => Some(IdentifierCase::SourcedMetadata),
      14 => Some(IdentifierCase::Custom),
      _ => None
    }
  }
}
}  // pub mod principal


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__rbac__v3__Action_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Action {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Action>
}

impl ::protobuf::Message for Action {
  type MessageView<'msg> = ActionView<'msg>;
  type MessageMut<'msg> = ActionMut<'msg>;
}

impl ::std::default::Default for Action {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Action {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Action` is `Sync` because it does not implement interior mutability.
//    Neither does `ActionMut`.
unsafe impl ::std::marker::Sync for Action {}

// SAFETY:
// - `Action` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Action {}

impl ::protobuf::Proxied for Action {
  type View<'msg> = ActionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Action {}

impl ::protobuf::MutProxied for Action {
  type Mut<'msg> = ActionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ActionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Action>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ActionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ActionView<'msg> {
  type Message = Action;
}

impl ::std::fmt::Debug for ActionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ActionView<'_> {
  fn default() -> ActionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Action>> for ActionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Action>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ActionView<'msg> {

  pub fn to_owned(&self) -> Action {
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

  // action: optional enum envoy.config.rbac.v3.RBAC.Action
  pub fn action(self) -> super::r_b_a_c::Action {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::r_b_a_c::Action::Allow).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `ActionView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ActionView<'_> {}

// SAFETY:
// - `ActionView` is `Send` because while its alive a `ActionMut` cannot.
// - `ActionView` does not use thread-local data.
unsafe impl ::std::marker::Send for ActionView<'_> {}

impl<'msg> ::protobuf::AsView for ActionView<'msg> {
  type Proxied = Action;
  fn as_view(&self) -> ::protobuf::View<'msg, Action> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ActionView<'msg> {
  fn into_view<'shorter>(self) -> ActionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Action> for ActionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Action {
    let mut dst = Action::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Action> for ActionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Action {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Action {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ActionView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ActionMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ActionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Action>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ActionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ActionMut<'msg> {
  type Message = Action;
}

impl ::std::fmt::Debug for ActionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Action>> for ActionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Action>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ActionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Action> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Action {
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

  // action: optional enum envoy.config.rbac.v3.RBAC.Action
  pub fn action(&self) -> super::r_b_a_c::Action {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::r_b_a_c::Action::Allow).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_action(&mut self, val: super::r_b_a_c::Action) {
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

}

// SAFETY:
// - `ActionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ActionMut<'_> {}

// SAFETY:
// - `ActionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ActionMut<'_> {}

impl<'msg> ::protobuf::AsView for ActionMut<'msg> {
  type Proxied = Action;
  fn as_view(&self) -> ::protobuf::View<'_, Action> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ActionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Action>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ActionMut<'msg> {
  type MutProxied = Action;
  fn as_mut(&mut self) -> ActionMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ActionMut<'msg> {
  fn into_mut<'shorter>(self) -> ActionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Action {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Action> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ActionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ActionMut<'_> {
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

  // action: optional enum envoy.config.rbac.v3.RBAC.Action
  pub fn action(&self) -> super::r_b_a_c::Action {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::r_b_a_c::Action::Allow).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_action(&mut self, val: super::r_b_a_c::Action) {
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

}  // impl Action

impl ::std::ops::Drop for Action {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Action {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Action {
  type Proxied = Self;
  fn as_view(&self) -> ActionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Action {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ActionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Action {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__rbac__v3__Action_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X.P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__rbac__v3__Action_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__rbac__v3__Action_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Action {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Action {
  type Msg = Action;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Action> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Action {
  type Msg = Action;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Action> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ActionMut<'_> {
  type Msg = Action;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Action> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ActionMut<'_> {
  type Msg = Action;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Action> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ActionView<'_> {
  type Msg = Action;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Action> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ActionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MetadataSource(i32);

#[allow(non_upper_case_globals)]
impl MetadataSource {
  pub const Dynamic: MetadataSource = MetadataSource(0);
  pub const Route: MetadataSource = MetadataSource(1);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Dynamic",
      1 => "Route",
      _ => return None
    })
  }
}

impl ::std::convert::From<MetadataSource> for i32 {
  fn from(val: MetadataSource) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for MetadataSource {
  fn from(val: i32) -> MetadataSource {
    Self(val)
  }
}

impl ::std::default::Default for MetadataSource {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for MetadataSource {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "MetadataSource::{}", constant_name)
    } else {
      write!(f, "MetadataSource::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for MetadataSource {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for MetadataSource {}

impl ::protobuf::Proxied for MetadataSource {
  type View<'a> = MetadataSource;
}

impl ::protobuf::AsView for MetadataSource {
  type Proxied = MetadataSource;

  fn as_view(&self) -> MetadataSource {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MetadataSource {
  fn into_view<'shorter>(self) -> MetadataSource where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for MetadataSource {
  const NAME: &'static str = "MetadataSource";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1)
  }
}

impl ::protobuf::__internal::EntityType for MetadataSource {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


