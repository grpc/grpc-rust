const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__admin__v3__UpdateFailureState_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct UpdateFailureState {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<UpdateFailureState>
}

impl ::protobuf::Message for UpdateFailureState {
  type MessageView<'msg> = UpdateFailureStateView<'msg>;
  type MessageMut<'msg> = UpdateFailureStateMut<'msg>;
}

impl ::std::default::Default for UpdateFailureState {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for UpdateFailureState {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `UpdateFailureState` is `Sync` because it does not implement interior mutability.
//    Neither does `UpdateFailureStateMut`.
unsafe impl ::std::marker::Sync for UpdateFailureState {}

// SAFETY:
// - `UpdateFailureState` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for UpdateFailureState {}

impl ::protobuf::Proxied for UpdateFailureState {
  type View<'msg> = UpdateFailureStateView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for UpdateFailureState {}

impl ::protobuf::MutProxied for UpdateFailureState {
  type Mut<'msg> = UpdateFailureStateMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UpdateFailureStateView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UpdateFailureState>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UpdateFailureStateView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for UpdateFailureStateView<'msg> {
  type Message = UpdateFailureState;
}

impl ::std::fmt::Debug for UpdateFailureStateView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for UpdateFailureStateView<'_> {
  fn default() -> UpdateFailureStateView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, UpdateFailureState>> for UpdateFailureStateView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UpdateFailureState>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UpdateFailureStateView<'msg> {

  pub fn to_owned(&self) -> UpdateFailureState {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // failed_configuration: optional message google.protobuf.Any
  pub fn has_failed_configuration(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn failed_configuration_opt(self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'msg>> {
    self.has_failed_configuration().then(|| self.failed_configuration())
  }
  pub fn failed_configuration(self) -> ::protobuf_well_known_types::AnyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }

  // last_update_attempt: optional message google.protobuf.Timestamp
  pub fn has_last_update_attempt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn last_update_attempt_opt(self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'msg>> {
    self.has_last_update_attempt().then(|| self.last_update_attempt())
  }
  pub fn last_update_attempt(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

  // details: optional string
  pub fn details(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // version_info: optional string
  pub fn version_info(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `UpdateFailureStateView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for UpdateFailureStateView<'_> {}

// SAFETY:
// - `UpdateFailureStateView` is `Send` because while its alive a `UpdateFailureStateMut` cannot.
// - `UpdateFailureStateView` does not use thread-local data.
unsafe impl ::std::marker::Send for UpdateFailureStateView<'_> {}

impl<'msg> ::protobuf::AsView for UpdateFailureStateView<'msg> {
  type Proxied = UpdateFailureState;
  fn as_view(&self) -> ::protobuf::View<'msg, UpdateFailureState> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UpdateFailureStateView<'msg> {
  fn into_view<'shorter>(self) -> UpdateFailureStateView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<UpdateFailureState> for UpdateFailureStateView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UpdateFailureState {
    let mut dst = UpdateFailureState::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<UpdateFailureState> for UpdateFailureStateMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UpdateFailureState {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for UpdateFailureState {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UpdateFailureStateView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UpdateFailureStateMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UpdateFailureStateMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UpdateFailureState>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UpdateFailureStateMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for UpdateFailureStateMut<'msg> {
  type Message = UpdateFailureState;
}

impl ::std::fmt::Debug for UpdateFailureStateMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, UpdateFailureState>> for UpdateFailureStateMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UpdateFailureState>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UpdateFailureStateMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, UpdateFailureState> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> UpdateFailureState {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // failed_configuration: optional message google.protobuf.Any
  pub fn has_failed_configuration(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_failed_configuration(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn failed_configuration_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_failed_configuration().then(|| self.failed_configuration())
  }
  pub fn failed_configuration(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn failed_configuration_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_failed_configuration(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // last_update_attempt: optional message google.protobuf.Timestamp
  pub fn has_last_update_attempt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_last_update_attempt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn last_update_attempt_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_update_attempt().then(|| self.last_update_attempt())
  }
  pub fn last_update_attempt(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_update_attempt_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_update_attempt(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // details: optional string
  pub fn details(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_details(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // version_info: optional string
  pub fn version_info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_version_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

}

// SAFETY:
// - `UpdateFailureStateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for UpdateFailureStateMut<'_> {}

// SAFETY:
// - `UpdateFailureStateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for UpdateFailureStateMut<'_> {}

impl<'msg> ::protobuf::AsView for UpdateFailureStateMut<'msg> {
  type Proxied = UpdateFailureState;
  fn as_view(&self) -> ::protobuf::View<'_, UpdateFailureState> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UpdateFailureStateMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, UpdateFailureState>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for UpdateFailureStateMut<'msg> {
  type MutProxied = UpdateFailureState;
  fn as_mut(&mut self) -> UpdateFailureStateMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for UpdateFailureStateMut<'msg> {
  fn into_mut<'shorter>(self) -> UpdateFailureStateMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl UpdateFailureState {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, UpdateFailureState> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> UpdateFailureStateView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> UpdateFailureStateMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // failed_configuration: optional message google.protobuf.Any
  pub fn has_failed_configuration(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_failed_configuration(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn failed_configuration_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_failed_configuration().then(|| self.failed_configuration())
  }
  pub fn failed_configuration(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn failed_configuration_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_failed_configuration(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // last_update_attempt: optional message google.protobuf.Timestamp
  pub fn has_last_update_attempt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_last_update_attempt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn last_update_attempt_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_update_attempt().then(|| self.last_update_attempt())
  }
  pub fn last_update_attempt(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_update_attempt_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_update_attempt(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // details: optional string
  pub fn details(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_details(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // version_info: optional string
  pub fn version_info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_version_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

}  // impl UpdateFailureState

impl ::std::ops::Drop for UpdateFailureState {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for UpdateFailureState {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for UpdateFailureState {
  type Proxied = Self;
  fn as_view(&self) -> UpdateFailureStateView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for UpdateFailureState {
  type MutProxied = Self;
  fn as_mut(&mut self) -> UpdateFailureStateMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for UpdateFailureState {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__admin__v3__UpdateFailureState_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$331X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__admin__v3__UpdateFailureState_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__admin__v3__UpdateFailureState_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UpdateFailureState {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UpdateFailureState {
  type Msg = UpdateFailureState;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpdateFailureState> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpdateFailureState {
  type Msg = UpdateFailureState;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpdateFailureState> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UpdateFailureStateMut<'_> {
  type Msg = UpdateFailureState;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpdateFailureState> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpdateFailureStateMut<'_> {
  type Msg = UpdateFailureState;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpdateFailureState> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpdateFailureStateView<'_> {
  type Msg = UpdateFailureState;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpdateFailureState> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UpdateFailureStateMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__admin__v3__ListenersConfigDump_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ListenersConfigDump {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ListenersConfigDump>
}

impl ::protobuf::Message for ListenersConfigDump {
  type MessageView<'msg> = ListenersConfigDumpView<'msg>;
  type MessageMut<'msg> = ListenersConfigDumpMut<'msg>;
}

impl ::std::default::Default for ListenersConfigDump {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ListenersConfigDump {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ListenersConfigDump` is `Sync` because it does not implement interior mutability.
//    Neither does `ListenersConfigDumpMut`.
unsafe impl ::std::marker::Sync for ListenersConfigDump {}

// SAFETY:
// - `ListenersConfigDump` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ListenersConfigDump {}

impl ::protobuf::Proxied for ListenersConfigDump {
  type View<'msg> = ListenersConfigDumpView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ListenersConfigDump {}

impl ::protobuf::MutProxied for ListenersConfigDump {
  type Mut<'msg> = ListenersConfigDumpMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ListenersConfigDumpView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListenersConfigDump>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListenersConfigDumpView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ListenersConfigDumpView<'msg> {
  type Message = ListenersConfigDump;
}

impl ::std::fmt::Debug for ListenersConfigDumpView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ListenersConfigDumpView<'_> {
  fn default() -> ListenersConfigDumpView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ListenersConfigDump>> for ListenersConfigDumpView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListenersConfigDump>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListenersConfigDumpView<'msg> {

  pub fn to_owned(&self) -> ListenersConfigDump {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // version_info: optional string
  pub fn version_info(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // static_listeners: repeated message envoy.admin.v3.ListenersConfigDump.StaticListener
  pub fn static_listeners(self) -> ::protobuf::RepeatedView<'msg, super::listeners_config_dump::StaticListener> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::listeners_config_dump::StaticListener>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // dynamic_listeners: repeated message envoy.admin.v3.ListenersConfigDump.DynamicListener
  pub fn dynamic_listeners(self) -> ::protobuf::RepeatedView<'msg, super::listeners_config_dump::DynamicListener> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::listeners_config_dump::DynamicListener>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `ListenersConfigDumpView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ListenersConfigDumpView<'_> {}

// SAFETY:
// - `ListenersConfigDumpView` is `Send` because while its alive a `ListenersConfigDumpMut` cannot.
// - `ListenersConfigDumpView` does not use thread-local data.
unsafe impl ::std::marker::Send for ListenersConfigDumpView<'_> {}

impl<'msg> ::protobuf::AsView for ListenersConfigDumpView<'msg> {
  type Proxied = ListenersConfigDump;
  fn as_view(&self) -> ::protobuf::View<'msg, ListenersConfigDump> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListenersConfigDumpView<'msg> {
  fn into_view<'shorter>(self) -> ListenersConfigDumpView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ListenersConfigDump> for ListenersConfigDumpView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListenersConfigDump {
    let mut dst = ListenersConfigDump::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ListenersConfigDump> for ListenersConfigDumpMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListenersConfigDump {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ListenersConfigDump {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ListenersConfigDumpView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ListenersConfigDumpMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ListenersConfigDumpMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListenersConfigDump>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListenersConfigDumpMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ListenersConfigDumpMut<'msg> {
  type Message = ListenersConfigDump;
}

impl ::std::fmt::Debug for ListenersConfigDumpMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ListenersConfigDump>> for ListenersConfigDumpMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListenersConfigDump>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListenersConfigDumpMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ListenersConfigDump> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ListenersConfigDump {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // version_info: optional string
  pub fn version_info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_version_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // static_listeners: repeated message envoy.admin.v3.ListenersConfigDump.StaticListener
  pub fn static_listeners(&self) -> ::protobuf::RepeatedView<'_, super::listeners_config_dump::StaticListener> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::listeners_config_dump::StaticListener>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn static_listeners_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::listeners_config_dump::StaticListener> {
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
  pub fn set_static_listeners(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::listeners_config_dump::StaticListener>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // dynamic_listeners: repeated message envoy.admin.v3.ListenersConfigDump.DynamicListener
  pub fn dynamic_listeners(&self) -> ::protobuf::RepeatedView<'_, super::listeners_config_dump::DynamicListener> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::listeners_config_dump::DynamicListener>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn dynamic_listeners_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::listeners_config_dump::DynamicListener> {
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
  pub fn set_dynamic_listeners(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::listeners_config_dump::DynamicListener>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}

// SAFETY:
// - `ListenersConfigDumpMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ListenersConfigDumpMut<'_> {}

// SAFETY:
// - `ListenersConfigDumpMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ListenersConfigDumpMut<'_> {}

impl<'msg> ::protobuf::AsView for ListenersConfigDumpMut<'msg> {
  type Proxied = ListenersConfigDump;
  fn as_view(&self) -> ::protobuf::View<'_, ListenersConfigDump> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListenersConfigDumpMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ListenersConfigDump>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ListenersConfigDumpMut<'msg> {
  type MutProxied = ListenersConfigDump;
  fn as_mut(&mut self) -> ListenersConfigDumpMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ListenersConfigDumpMut<'msg> {
  fn into_mut<'shorter>(self) -> ListenersConfigDumpMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ListenersConfigDump {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ListenersConfigDump> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ListenersConfigDumpView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ListenersConfigDumpMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // version_info: optional string
  pub fn version_info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_version_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // static_listeners: repeated message envoy.admin.v3.ListenersConfigDump.StaticListener
  pub fn static_listeners(&self) -> ::protobuf::RepeatedView<'_, super::listeners_config_dump::StaticListener> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::listeners_config_dump::StaticListener>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn static_listeners_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::listeners_config_dump::StaticListener> {
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
  pub fn set_static_listeners(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::listeners_config_dump::StaticListener>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // dynamic_listeners: repeated message envoy.admin.v3.ListenersConfigDump.DynamicListener
  pub fn dynamic_listeners(&self) -> ::protobuf::RepeatedView<'_, super::listeners_config_dump::DynamicListener> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::listeners_config_dump::DynamicListener>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn dynamic_listeners_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::listeners_config_dump::DynamicListener> {
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
  pub fn set_dynamic_listeners(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::listeners_config_dump::DynamicListener>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}  // impl ListenersConfigDump

impl ::std::ops::Drop for ListenersConfigDump {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ListenersConfigDump {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ListenersConfigDump {
  type Proxied = Self;
  fn as_view(&self) -> ListenersConfigDumpView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ListenersConfigDump {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ListenersConfigDumpMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ListenersConfigDump {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__admin__v3__ListenersConfigDump_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1XGG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__admin__v3__ListenersConfigDump_msg_init.0, &[<super::listeners_config_dump::StaticListener as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::listeners_config_dump::DynamicListener as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__admin__v3__ListenersConfigDump_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListenersConfigDump {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListenersConfigDump {
  type Msg = ListenersConfigDump;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListenersConfigDump> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListenersConfigDump {
  type Msg = ListenersConfigDump;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListenersConfigDump> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListenersConfigDumpMut<'_> {
  type Msg = ListenersConfigDump;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListenersConfigDump> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListenersConfigDumpMut<'_> {
  type Msg = ListenersConfigDump;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListenersConfigDump> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListenersConfigDumpView<'_> {
  type Msg = ListenersConfigDump;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListenersConfigDump> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListenersConfigDumpMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod listeners_config_dump {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__admin__v3__ListenersConfigDump__StaticListener_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct StaticListener {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<StaticListener>
}

impl ::protobuf::Message for StaticListener {
  type MessageView<'msg> = StaticListenerView<'msg>;
  type MessageMut<'msg> = StaticListenerMut<'msg>;
}

impl ::std::default::Default for StaticListener {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for StaticListener {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `StaticListener` is `Sync` because it does not implement interior mutability.
//    Neither does `StaticListenerMut`.
unsafe impl ::std::marker::Sync for StaticListener {}

// SAFETY:
// - `StaticListener` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for StaticListener {}

impl ::protobuf::Proxied for StaticListener {
  type View<'msg> = StaticListenerView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for StaticListener {}

impl ::protobuf::MutProxied for StaticListener {
  type Mut<'msg> = StaticListenerMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct StaticListenerView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StaticListener>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StaticListenerView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for StaticListenerView<'msg> {
  type Message = StaticListener;
}

impl ::std::fmt::Debug for StaticListenerView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for StaticListenerView<'_> {
  fn default() -> StaticListenerView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, StaticListener>> for StaticListenerView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StaticListener>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StaticListenerView<'msg> {

  pub fn to_owned(&self) -> StaticListener {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // listener: optional message google.protobuf.Any
  pub fn has_listener(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn listener_opt(self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'msg>> {
    self.has_listener().then(|| self.listener())
  }
  pub fn listener(self) -> ::protobuf_well_known_types::AnyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn last_updated_opt(self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'msg>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

}

// SAFETY:
// - `StaticListenerView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for StaticListenerView<'_> {}

// SAFETY:
// - `StaticListenerView` is `Send` because while its alive a `StaticListenerMut` cannot.
// - `StaticListenerView` does not use thread-local data.
unsafe impl ::std::marker::Send for StaticListenerView<'_> {}

impl<'msg> ::protobuf::AsView for StaticListenerView<'msg> {
  type Proxied = StaticListener;
  fn as_view(&self) -> ::protobuf::View<'msg, StaticListener> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StaticListenerView<'msg> {
  fn into_view<'shorter>(self) -> StaticListenerView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<StaticListener> for StaticListenerView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StaticListener {
    let mut dst = StaticListener::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<StaticListener> for StaticListenerMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StaticListener {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for StaticListener {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StaticListenerView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StaticListenerMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct StaticListenerMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StaticListener>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StaticListenerMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for StaticListenerMut<'msg> {
  type Message = StaticListener;
}

impl ::std::fmt::Debug for StaticListenerMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, StaticListener>> for StaticListenerMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StaticListener>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StaticListenerMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, StaticListener> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> StaticListener {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // listener: optional message google.protobuf.Any
  pub fn has_listener(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_listener(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn listener_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_listener().then(|| self.listener())
  }
  pub fn listener(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn listener_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_listener(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_last_updated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn last_updated_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_updated_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_updated(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

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
// - `StaticListenerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for StaticListenerMut<'_> {}

// SAFETY:
// - `StaticListenerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for StaticListenerMut<'_> {}

impl<'msg> ::protobuf::AsView for StaticListenerMut<'msg> {
  type Proxied = StaticListener;
  fn as_view(&self) -> ::protobuf::View<'_, StaticListener> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StaticListenerMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, StaticListener>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for StaticListenerMut<'msg> {
  type MutProxied = StaticListener;
  fn as_mut(&mut self) -> StaticListenerMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for StaticListenerMut<'msg> {
  fn into_mut<'shorter>(self) -> StaticListenerMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl StaticListener {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, StaticListener> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> StaticListenerView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> StaticListenerMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // listener: optional message google.protobuf.Any
  pub fn has_listener(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_listener(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn listener_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_listener().then(|| self.listener())
  }
  pub fn listener(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn listener_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_listener(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_last_updated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn last_updated_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_updated_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_updated(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl StaticListener

impl ::std::ops::Drop for StaticListener {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for StaticListener {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for StaticListener {
  type Proxied = Self;
  fn as_view(&self) -> StaticListenerView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for StaticListener {
  type MutProxied = Self;
  fn as_mut(&mut self) -> StaticListenerMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for StaticListener {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::listeners_config_dump::envoy__admin__v3__ListenersConfigDump__StaticListener_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::listeners_config_dump::envoy__admin__v3__ListenersConfigDump__StaticListener_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::listeners_config_dump::envoy__admin__v3__ListenersConfigDump__StaticListener_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StaticListener {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StaticListener {
  type Msg = StaticListener;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StaticListener> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StaticListener {
  type Msg = StaticListener;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StaticListener> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StaticListenerMut<'_> {
  type Msg = StaticListener;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StaticListener> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StaticListenerMut<'_> {
  type Msg = StaticListener;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StaticListener> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StaticListenerView<'_> {
  type Msg = StaticListener;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StaticListener> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StaticListenerMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__admin__v3__ListenersConfigDump__DynamicListenerState_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DynamicListenerState {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DynamicListenerState>
}

impl ::protobuf::Message for DynamicListenerState {
  type MessageView<'msg> = DynamicListenerStateView<'msg>;
  type MessageMut<'msg> = DynamicListenerStateMut<'msg>;
}

impl ::std::default::Default for DynamicListenerState {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DynamicListenerState {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DynamicListenerState` is `Sync` because it does not implement interior mutability.
//    Neither does `DynamicListenerStateMut`.
unsafe impl ::std::marker::Sync for DynamicListenerState {}

// SAFETY:
// - `DynamicListenerState` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DynamicListenerState {}

impl ::protobuf::Proxied for DynamicListenerState {
  type View<'msg> = DynamicListenerStateView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DynamicListenerState {}

impl ::protobuf::MutProxied for DynamicListenerState {
  type Mut<'msg> = DynamicListenerStateMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DynamicListenerStateView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicListenerState>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DynamicListenerStateView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DynamicListenerStateView<'msg> {
  type Message = DynamicListenerState;
}

impl ::std::fmt::Debug for DynamicListenerStateView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DynamicListenerStateView<'_> {
  fn default() -> DynamicListenerStateView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicListenerState>> for DynamicListenerStateView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicListenerState>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DynamicListenerStateView<'msg> {

  pub fn to_owned(&self) -> DynamicListenerState {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // version_info: optional string
  pub fn version_info(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // listener: optional message google.protobuf.Any
  pub fn has_listener(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn listener_opt(self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'msg>> {
    self.has_listener().then(|| self.listener())
  }
  pub fn listener(self) -> ::protobuf_well_known_types::AnyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn last_updated_opt(self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'msg>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

}

// SAFETY:
// - `DynamicListenerStateView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DynamicListenerStateView<'_> {}

// SAFETY:
// - `DynamicListenerStateView` is `Send` because while its alive a `DynamicListenerStateMut` cannot.
// - `DynamicListenerStateView` does not use thread-local data.
unsafe impl ::std::marker::Send for DynamicListenerStateView<'_> {}

impl<'msg> ::protobuf::AsView for DynamicListenerStateView<'msg> {
  type Proxied = DynamicListenerState;
  fn as_view(&self) -> ::protobuf::View<'msg, DynamicListenerState> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DynamicListenerStateView<'msg> {
  fn into_view<'shorter>(self) -> DynamicListenerStateView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DynamicListenerState> for DynamicListenerStateView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DynamicListenerState {
    let mut dst = DynamicListenerState::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DynamicListenerState> for DynamicListenerStateMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DynamicListenerState {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DynamicListenerState {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DynamicListenerStateView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DynamicListenerStateMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DynamicListenerStateMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicListenerState>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DynamicListenerStateMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DynamicListenerStateMut<'msg> {
  type Message = DynamicListenerState;
}

impl ::std::fmt::Debug for DynamicListenerStateMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicListenerState>> for DynamicListenerStateMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicListenerState>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DynamicListenerStateMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicListenerState> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DynamicListenerState {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // version_info: optional string
  pub fn version_info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_version_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // listener: optional message google.protobuf.Any
  pub fn has_listener(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_listener(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn listener_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_listener().then(|| self.listener())
  }
  pub fn listener(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn listener_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_listener(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_last_updated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn last_updated_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_updated_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_updated(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

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
// - `DynamicListenerStateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DynamicListenerStateMut<'_> {}

// SAFETY:
// - `DynamicListenerStateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DynamicListenerStateMut<'_> {}

impl<'msg> ::protobuf::AsView for DynamicListenerStateMut<'msg> {
  type Proxied = DynamicListenerState;
  fn as_view(&self) -> ::protobuf::View<'_, DynamicListenerState> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DynamicListenerStateMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DynamicListenerState>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DynamicListenerStateMut<'msg> {
  type MutProxied = DynamicListenerState;
  fn as_mut(&mut self) -> DynamicListenerStateMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DynamicListenerStateMut<'msg> {
  fn into_mut<'shorter>(self) -> DynamicListenerStateMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DynamicListenerState {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DynamicListenerState> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DynamicListenerStateView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DynamicListenerStateMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // version_info: optional string
  pub fn version_info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_version_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // listener: optional message google.protobuf.Any
  pub fn has_listener(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_listener(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn listener_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_listener().then(|| self.listener())
  }
  pub fn listener(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn listener_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_listener(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_last_updated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn last_updated_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_updated_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_updated(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl DynamicListenerState

impl ::std::ops::Drop for DynamicListenerState {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DynamicListenerState {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DynamicListenerState {
  type Proxied = Self;
  fn as_view(&self) -> DynamicListenerStateView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DynamicListenerState {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DynamicListenerStateMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DynamicListenerState {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::listeners_config_dump::envoy__admin__v3__ListenersConfigDump__DynamicListenerState_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::listeners_config_dump::envoy__admin__v3__ListenersConfigDump__DynamicListenerState_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::listeners_config_dump::envoy__admin__v3__ListenersConfigDump__DynamicListenerState_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DynamicListenerState {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DynamicListenerState {
  type Msg = DynamicListenerState;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicListenerState> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicListenerState {
  type Msg = DynamicListenerState;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicListenerState> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DynamicListenerStateMut<'_> {
  type Msg = DynamicListenerState;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicListenerState> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicListenerStateMut<'_> {
  type Msg = DynamicListenerState;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicListenerState> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicListenerStateView<'_> {
  type Msg = DynamicListenerState;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicListenerState> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DynamicListenerStateMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__admin__v3__ListenersConfigDump__DynamicListener_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DynamicListener {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DynamicListener>
}

impl ::protobuf::Message for DynamicListener {
  type MessageView<'msg> = DynamicListenerView<'msg>;
  type MessageMut<'msg> = DynamicListenerMut<'msg>;
}

impl ::std::default::Default for DynamicListener {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DynamicListener {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DynamicListener` is `Sync` because it does not implement interior mutability.
//    Neither does `DynamicListenerMut`.
unsafe impl ::std::marker::Sync for DynamicListener {}

// SAFETY:
// - `DynamicListener` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DynamicListener {}

impl ::protobuf::Proxied for DynamicListener {
  type View<'msg> = DynamicListenerView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DynamicListener {}

impl ::protobuf::MutProxied for DynamicListener {
  type Mut<'msg> = DynamicListenerMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DynamicListenerView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicListener>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DynamicListenerView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DynamicListenerView<'msg> {
  type Message = DynamicListener;
}

impl ::std::fmt::Debug for DynamicListenerView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DynamicListenerView<'_> {
  fn default() -> DynamicListenerView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicListener>> for DynamicListenerView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicListener>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DynamicListenerView<'msg> {

  pub fn to_owned(&self) -> DynamicListener {
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

  // active_state: optional message envoy.admin.v3.ListenersConfigDump.DynamicListenerState
  pub fn has_active_state(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn active_state_opt(self) -> ::std::option::Option<super::super::listeners_config_dump::DynamicListenerStateView<'msg>> {
    self.has_active_state().then(|| self.active_state())
  }
  pub fn active_state(self) -> super::super::listeners_config_dump::DynamicListenerStateView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::listeners_config_dump::DynamicListenerStateView::default())
  }

  // warming_state: optional message envoy.admin.v3.ListenersConfigDump.DynamicListenerState
  pub fn has_warming_state(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn warming_state_opt(self) -> ::std::option::Option<super::super::listeners_config_dump::DynamicListenerStateView<'msg>> {
    self.has_warming_state().then(|| self.warming_state())
  }
  pub fn warming_state(self) -> super::super::listeners_config_dump::DynamicListenerStateView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::listeners_config_dump::DynamicListenerStateView::default())
  }

  // draining_state: optional message envoy.admin.v3.ListenersConfigDump.DynamicListenerState
  pub fn has_draining_state(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn draining_state_opt(self) -> ::std::option::Option<super::super::listeners_config_dump::DynamicListenerStateView<'msg>> {
    self.has_draining_state().then(|| self.draining_state())
  }
  pub fn draining_state(self) -> super::super::listeners_config_dump::DynamicListenerStateView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::listeners_config_dump::DynamicListenerStateView::default())
  }

  // error_state: optional message envoy.admin.v3.UpdateFailureState
  pub fn has_error_state(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn error_state_opt(self) -> ::std::option::Option<super::super::UpdateFailureStateView<'msg>> {
    self.has_error_state().then(|| self.error_state())
  }
  pub fn error_state(self) -> super::super::UpdateFailureStateView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::UpdateFailureStateView::default())
  }

  // client_status: optional enum envoy.admin.v3.ClientResourceStatus
  pub fn client_status(self) -> super::super::ClientResourceStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::super::ClientResourceStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `DynamicListenerView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DynamicListenerView<'_> {}

// SAFETY:
// - `DynamicListenerView` is `Send` because while its alive a `DynamicListenerMut` cannot.
// - `DynamicListenerView` does not use thread-local data.
unsafe impl ::std::marker::Send for DynamicListenerView<'_> {}

impl<'msg> ::protobuf::AsView for DynamicListenerView<'msg> {
  type Proxied = DynamicListener;
  fn as_view(&self) -> ::protobuf::View<'msg, DynamicListener> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DynamicListenerView<'msg> {
  fn into_view<'shorter>(self) -> DynamicListenerView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DynamicListener> for DynamicListenerView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DynamicListener {
    let mut dst = DynamicListener::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DynamicListener> for DynamicListenerMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DynamicListener {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DynamicListener {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DynamicListenerView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DynamicListenerMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DynamicListenerMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicListener>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DynamicListenerMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DynamicListenerMut<'msg> {
  type Message = DynamicListener;
}

impl ::std::fmt::Debug for DynamicListenerMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicListener>> for DynamicListenerMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicListener>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DynamicListenerMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicListener> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DynamicListener {
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

  // active_state: optional message envoy.admin.v3.ListenersConfigDump.DynamicListenerState
  pub fn has_active_state(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_active_state(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn active_state_opt(&self) -> ::std::option::Option<super::super::listeners_config_dump::DynamicListenerStateView<'_>> {
    self.has_active_state().then(|| self.active_state())
  }
  pub fn active_state(&self) -> super::super::listeners_config_dump::DynamicListenerStateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::listeners_config_dump::DynamicListenerStateView::default())
  }
  pub fn active_state_mut(&mut self) -> super::super::listeners_config_dump::DynamicListenerStateMut<'_> {
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
  pub fn set_active_state(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::listeners_config_dump::DynamicListenerState>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // warming_state: optional message envoy.admin.v3.ListenersConfigDump.DynamicListenerState
  pub fn has_warming_state(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_warming_state(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn warming_state_opt(&self) -> ::std::option::Option<super::super::listeners_config_dump::DynamicListenerStateView<'_>> {
    self.has_warming_state().then(|| self.warming_state())
  }
  pub fn warming_state(&self) -> super::super::listeners_config_dump::DynamicListenerStateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::listeners_config_dump::DynamicListenerStateView::default())
  }
  pub fn warming_state_mut(&mut self) -> super::super::listeners_config_dump::DynamicListenerStateMut<'_> {
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
  pub fn set_warming_state(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::listeners_config_dump::DynamicListenerState>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // draining_state: optional message envoy.admin.v3.ListenersConfigDump.DynamicListenerState
  pub fn has_draining_state(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_draining_state(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn draining_state_opt(&self) -> ::std::option::Option<super::super::listeners_config_dump::DynamicListenerStateView<'_>> {
    self.has_draining_state().then(|| self.draining_state())
  }
  pub fn draining_state(&self) -> super::super::listeners_config_dump::DynamicListenerStateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::listeners_config_dump::DynamicListenerStateView::default())
  }
  pub fn draining_state_mut(&mut self) -> super::super::listeners_config_dump::DynamicListenerStateMut<'_> {
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
  pub fn set_draining_state(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::listeners_config_dump::DynamicListenerState>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // error_state: optional message envoy.admin.v3.UpdateFailureState
  pub fn has_error_state(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_error_state(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn error_state_opt(&self) -> ::std::option::Option<super::super::UpdateFailureStateView<'_>> {
    self.has_error_state().then(|| self.error_state())
  }
  pub fn error_state(&self) -> super::super::UpdateFailureStateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::UpdateFailureStateView::default())
  }
  pub fn error_state_mut(&mut self) -> super::super::UpdateFailureStateMut<'_> {
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
  pub fn set_error_state(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::UpdateFailureState>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // client_status: optional enum envoy.admin.v3.ClientResourceStatus
  pub fn client_status(&self) -> super::super::ClientResourceStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::super::ClientResourceStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_client_status(&mut self, val: super::super::ClientResourceStatus) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        5, val.into()
      )
    }
  }

}

// SAFETY:
// - `DynamicListenerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DynamicListenerMut<'_> {}

// SAFETY:
// - `DynamicListenerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DynamicListenerMut<'_> {}

impl<'msg> ::protobuf::AsView for DynamicListenerMut<'msg> {
  type Proxied = DynamicListener;
  fn as_view(&self) -> ::protobuf::View<'_, DynamicListener> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DynamicListenerMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DynamicListener>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DynamicListenerMut<'msg> {
  type MutProxied = DynamicListener;
  fn as_mut(&mut self) -> DynamicListenerMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DynamicListenerMut<'msg> {
  fn into_mut<'shorter>(self) -> DynamicListenerMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DynamicListener {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DynamicListener> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DynamicListenerView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DynamicListenerMut<'_> {
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

  // active_state: optional message envoy.admin.v3.ListenersConfigDump.DynamicListenerState
  pub fn has_active_state(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_active_state(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn active_state_opt(&self) -> ::std::option::Option<super::super::listeners_config_dump::DynamicListenerStateView<'_>> {
    self.has_active_state().then(|| self.active_state())
  }
  pub fn active_state(&self) -> super::super::listeners_config_dump::DynamicListenerStateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::listeners_config_dump::DynamicListenerStateView::default())
  }
  pub fn active_state_mut(&mut self) -> super::super::listeners_config_dump::DynamicListenerStateMut<'_> {
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
  pub fn set_active_state(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::listeners_config_dump::DynamicListenerState>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // warming_state: optional message envoy.admin.v3.ListenersConfigDump.DynamicListenerState
  pub fn has_warming_state(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_warming_state(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn warming_state_opt(&self) -> ::std::option::Option<super::super::listeners_config_dump::DynamicListenerStateView<'_>> {
    self.has_warming_state().then(|| self.warming_state())
  }
  pub fn warming_state(&self) -> super::super::listeners_config_dump::DynamicListenerStateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::listeners_config_dump::DynamicListenerStateView::default())
  }
  pub fn warming_state_mut(&mut self) -> super::super::listeners_config_dump::DynamicListenerStateMut<'_> {
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
  pub fn set_warming_state(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::listeners_config_dump::DynamicListenerState>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // draining_state: optional message envoy.admin.v3.ListenersConfigDump.DynamicListenerState
  pub fn has_draining_state(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_draining_state(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn draining_state_opt(&self) -> ::std::option::Option<super::super::listeners_config_dump::DynamicListenerStateView<'_>> {
    self.has_draining_state().then(|| self.draining_state())
  }
  pub fn draining_state(&self) -> super::super::listeners_config_dump::DynamicListenerStateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::listeners_config_dump::DynamicListenerStateView::default())
  }
  pub fn draining_state_mut(&mut self) -> super::super::listeners_config_dump::DynamicListenerStateMut<'_> {
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
  pub fn set_draining_state(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::listeners_config_dump::DynamicListenerState>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // error_state: optional message envoy.admin.v3.UpdateFailureState
  pub fn has_error_state(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_error_state(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn error_state_opt(&self) -> ::std::option::Option<super::super::UpdateFailureStateView<'_>> {
    self.has_error_state().then(|| self.error_state())
  }
  pub fn error_state(&self) -> super::super::UpdateFailureStateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::UpdateFailureStateView::default())
  }
  pub fn error_state_mut(&mut self) -> super::super::UpdateFailureStateMut<'_> {
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
  pub fn set_error_state(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::UpdateFailureState>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // client_status: optional enum envoy.admin.v3.ClientResourceStatus
  pub fn client_status(&self) -> super::super::ClientResourceStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::super::ClientResourceStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_client_status(&mut self, val: super::super::ClientResourceStatus) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        5, val.into()
      )
    }
  }

}  // impl DynamicListener

impl ::std::ops::Drop for DynamicListener {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DynamicListener {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DynamicListener {
  type Proxied = Self;
  fn as_view(&self) -> DynamicListenerView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DynamicListener {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DynamicListenerMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DynamicListener {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::listeners_config_dump::envoy__admin__v3__ListenersConfigDump__DynamicListener_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3333.P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::listeners_config_dump::envoy__admin__v3__ListenersConfigDump__DynamicListener_msg_init.0, &[<super::super::listeners_config_dump::DynamicListenerState as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::listeners_config_dump::DynamicListenerState as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::listeners_config_dump::DynamicListenerState as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::UpdateFailureState as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::listeners_config_dump::envoy__admin__v3__ListenersConfigDump__DynamicListener_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DynamicListener {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DynamicListener {
  type Msg = DynamicListener;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicListener> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicListener {
  type Msg = DynamicListener;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicListener> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DynamicListenerMut<'_> {
  type Msg = DynamicListener;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicListener> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicListenerMut<'_> {
  type Msg = DynamicListener;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicListener> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicListenerView<'_> {
  type Msg = DynamicListener;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicListener> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DynamicListenerMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod listeners_config_dump


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__admin__v3__ClustersConfigDump_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ClustersConfigDump {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ClustersConfigDump>
}

impl ::protobuf::Message for ClustersConfigDump {
  type MessageView<'msg> = ClustersConfigDumpView<'msg>;
  type MessageMut<'msg> = ClustersConfigDumpMut<'msg>;
}

impl ::std::default::Default for ClustersConfigDump {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ClustersConfigDump {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ClustersConfigDump` is `Sync` because it does not implement interior mutability.
//    Neither does `ClustersConfigDumpMut`.
unsafe impl ::std::marker::Sync for ClustersConfigDump {}

// SAFETY:
// - `ClustersConfigDump` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ClustersConfigDump {}

impl ::protobuf::Proxied for ClustersConfigDump {
  type View<'msg> = ClustersConfigDumpView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ClustersConfigDump {}

impl ::protobuf::MutProxied for ClustersConfigDump {
  type Mut<'msg> = ClustersConfigDumpMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ClustersConfigDumpView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ClustersConfigDump>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClustersConfigDumpView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ClustersConfigDumpView<'msg> {
  type Message = ClustersConfigDump;
}

impl ::std::fmt::Debug for ClustersConfigDumpView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ClustersConfigDumpView<'_> {
  fn default() -> ClustersConfigDumpView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ClustersConfigDump>> for ClustersConfigDumpView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ClustersConfigDump>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClustersConfigDumpView<'msg> {

  pub fn to_owned(&self) -> ClustersConfigDump {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // version_info: optional string
  pub fn version_info(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // static_clusters: repeated message envoy.admin.v3.ClustersConfigDump.StaticCluster
  pub fn static_clusters(self) -> ::protobuf::RepeatedView<'msg, super::clusters_config_dump::StaticCluster> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::clusters_config_dump::StaticCluster>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // dynamic_active_clusters: repeated message envoy.admin.v3.ClustersConfigDump.DynamicCluster
  pub fn dynamic_active_clusters(self) -> ::protobuf::RepeatedView<'msg, super::clusters_config_dump::DynamicCluster> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::clusters_config_dump::DynamicCluster>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // dynamic_warming_clusters: repeated message envoy.admin.v3.ClustersConfigDump.DynamicCluster
  pub fn dynamic_warming_clusters(self) -> ::protobuf::RepeatedView<'msg, super::clusters_config_dump::DynamicCluster> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::clusters_config_dump::DynamicCluster>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `ClustersConfigDumpView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ClustersConfigDumpView<'_> {}

// SAFETY:
// - `ClustersConfigDumpView` is `Send` because while its alive a `ClustersConfigDumpMut` cannot.
// - `ClustersConfigDumpView` does not use thread-local data.
unsafe impl ::std::marker::Send for ClustersConfigDumpView<'_> {}

impl<'msg> ::protobuf::AsView for ClustersConfigDumpView<'msg> {
  type Proxied = ClustersConfigDump;
  fn as_view(&self) -> ::protobuf::View<'msg, ClustersConfigDump> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClustersConfigDumpView<'msg> {
  fn into_view<'shorter>(self) -> ClustersConfigDumpView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ClustersConfigDump> for ClustersConfigDumpView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ClustersConfigDump {
    let mut dst = ClustersConfigDump::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ClustersConfigDump> for ClustersConfigDumpMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ClustersConfigDump {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ClustersConfigDump {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ClustersConfigDumpView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ClustersConfigDumpMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ClustersConfigDumpMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ClustersConfigDump>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClustersConfigDumpMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ClustersConfigDumpMut<'msg> {
  type Message = ClustersConfigDump;
}

impl ::std::fmt::Debug for ClustersConfigDumpMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ClustersConfigDump>> for ClustersConfigDumpMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ClustersConfigDump>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClustersConfigDumpMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ClustersConfigDump> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ClustersConfigDump {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // version_info: optional string
  pub fn version_info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_version_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // static_clusters: repeated message envoy.admin.v3.ClustersConfigDump.StaticCluster
  pub fn static_clusters(&self) -> ::protobuf::RepeatedView<'_, super::clusters_config_dump::StaticCluster> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::clusters_config_dump::StaticCluster>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn static_clusters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::clusters_config_dump::StaticCluster> {
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
  pub fn set_static_clusters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::clusters_config_dump::StaticCluster>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // dynamic_active_clusters: repeated message envoy.admin.v3.ClustersConfigDump.DynamicCluster
  pub fn dynamic_active_clusters(&self) -> ::protobuf::RepeatedView<'_, super::clusters_config_dump::DynamicCluster> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::clusters_config_dump::DynamicCluster>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn dynamic_active_clusters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::clusters_config_dump::DynamicCluster> {
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
  pub fn set_dynamic_active_clusters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::clusters_config_dump::DynamicCluster>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // dynamic_warming_clusters: repeated message envoy.admin.v3.ClustersConfigDump.DynamicCluster
  pub fn dynamic_warming_clusters(&self) -> ::protobuf::RepeatedView<'_, super::clusters_config_dump::DynamicCluster> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::clusters_config_dump::DynamicCluster>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn dynamic_warming_clusters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::clusters_config_dump::DynamicCluster> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        3,
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
  pub fn set_dynamic_warming_clusters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::clusters_config_dump::DynamicCluster>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

}

// SAFETY:
// - `ClustersConfigDumpMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ClustersConfigDumpMut<'_> {}

// SAFETY:
// - `ClustersConfigDumpMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ClustersConfigDumpMut<'_> {}

impl<'msg> ::protobuf::AsView for ClustersConfigDumpMut<'msg> {
  type Proxied = ClustersConfigDump;
  fn as_view(&self) -> ::protobuf::View<'_, ClustersConfigDump> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClustersConfigDumpMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ClustersConfigDump>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ClustersConfigDumpMut<'msg> {
  type MutProxied = ClustersConfigDump;
  fn as_mut(&mut self) -> ClustersConfigDumpMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ClustersConfigDumpMut<'msg> {
  fn into_mut<'shorter>(self) -> ClustersConfigDumpMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ClustersConfigDump {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ClustersConfigDump> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ClustersConfigDumpView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ClustersConfigDumpMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // version_info: optional string
  pub fn version_info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_version_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // static_clusters: repeated message envoy.admin.v3.ClustersConfigDump.StaticCluster
  pub fn static_clusters(&self) -> ::protobuf::RepeatedView<'_, super::clusters_config_dump::StaticCluster> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::clusters_config_dump::StaticCluster>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn static_clusters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::clusters_config_dump::StaticCluster> {
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
  pub fn set_static_clusters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::clusters_config_dump::StaticCluster>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // dynamic_active_clusters: repeated message envoy.admin.v3.ClustersConfigDump.DynamicCluster
  pub fn dynamic_active_clusters(&self) -> ::protobuf::RepeatedView<'_, super::clusters_config_dump::DynamicCluster> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::clusters_config_dump::DynamicCluster>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn dynamic_active_clusters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::clusters_config_dump::DynamicCluster> {
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
  pub fn set_dynamic_active_clusters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::clusters_config_dump::DynamicCluster>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // dynamic_warming_clusters: repeated message envoy.admin.v3.ClustersConfigDump.DynamicCluster
  pub fn dynamic_warming_clusters(&self) -> ::protobuf::RepeatedView<'_, super::clusters_config_dump::DynamicCluster> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::clusters_config_dump::DynamicCluster>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn dynamic_warming_clusters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::clusters_config_dump::DynamicCluster> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        3,
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
  pub fn set_dynamic_warming_clusters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::clusters_config_dump::DynamicCluster>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

}  // impl ClustersConfigDump

impl ::std::ops::Drop for ClustersConfigDump {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ClustersConfigDump {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ClustersConfigDump {
  type Proxied = Self;
  fn as_view(&self) -> ClustersConfigDumpView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ClustersConfigDump {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ClustersConfigDumpMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ClustersConfigDump {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__admin__v3__ClustersConfigDump_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1XGGG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__admin__v3__ClustersConfigDump_msg_init.0, &[<super::clusters_config_dump::StaticCluster as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::clusters_config_dump::DynamicCluster as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::clusters_config_dump::DynamicCluster as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__admin__v3__ClustersConfigDump_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ClustersConfigDump {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ClustersConfigDump {
  type Msg = ClustersConfigDump;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClustersConfigDump> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClustersConfigDump {
  type Msg = ClustersConfigDump;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClustersConfigDump> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ClustersConfigDumpMut<'_> {
  type Msg = ClustersConfigDump;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClustersConfigDump> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClustersConfigDumpMut<'_> {
  type Msg = ClustersConfigDump;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClustersConfigDump> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClustersConfigDumpView<'_> {
  type Msg = ClustersConfigDump;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClustersConfigDump> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ClustersConfigDumpMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod clusters_config_dump {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__admin__v3__ClustersConfigDump__StaticCluster_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct StaticCluster {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<StaticCluster>
}

impl ::protobuf::Message for StaticCluster {
  type MessageView<'msg> = StaticClusterView<'msg>;
  type MessageMut<'msg> = StaticClusterMut<'msg>;
}

impl ::std::default::Default for StaticCluster {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for StaticCluster {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `StaticCluster` is `Sync` because it does not implement interior mutability.
//    Neither does `StaticClusterMut`.
unsafe impl ::std::marker::Sync for StaticCluster {}

// SAFETY:
// - `StaticCluster` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for StaticCluster {}

impl ::protobuf::Proxied for StaticCluster {
  type View<'msg> = StaticClusterView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for StaticCluster {}

impl ::protobuf::MutProxied for StaticCluster {
  type Mut<'msg> = StaticClusterMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct StaticClusterView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StaticCluster>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StaticClusterView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for StaticClusterView<'msg> {
  type Message = StaticCluster;
}

impl ::std::fmt::Debug for StaticClusterView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for StaticClusterView<'_> {
  fn default() -> StaticClusterView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, StaticCluster>> for StaticClusterView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StaticCluster>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StaticClusterView<'msg> {

  pub fn to_owned(&self) -> StaticCluster {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // cluster: optional message google.protobuf.Any
  pub fn has_cluster(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn cluster_opt(self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'msg>> {
    self.has_cluster().then(|| self.cluster())
  }
  pub fn cluster(self) -> ::protobuf_well_known_types::AnyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn last_updated_opt(self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'msg>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

}

// SAFETY:
// - `StaticClusterView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for StaticClusterView<'_> {}

// SAFETY:
// - `StaticClusterView` is `Send` because while its alive a `StaticClusterMut` cannot.
// - `StaticClusterView` does not use thread-local data.
unsafe impl ::std::marker::Send for StaticClusterView<'_> {}

impl<'msg> ::protobuf::AsView for StaticClusterView<'msg> {
  type Proxied = StaticCluster;
  fn as_view(&self) -> ::protobuf::View<'msg, StaticCluster> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StaticClusterView<'msg> {
  fn into_view<'shorter>(self) -> StaticClusterView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<StaticCluster> for StaticClusterView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StaticCluster {
    let mut dst = StaticCluster::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<StaticCluster> for StaticClusterMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StaticCluster {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for StaticCluster {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StaticClusterView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StaticClusterMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct StaticClusterMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StaticCluster>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StaticClusterMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for StaticClusterMut<'msg> {
  type Message = StaticCluster;
}

impl ::std::fmt::Debug for StaticClusterMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, StaticCluster>> for StaticClusterMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StaticCluster>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StaticClusterMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, StaticCluster> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> StaticCluster {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // cluster: optional message google.protobuf.Any
  pub fn has_cluster(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_cluster(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn cluster_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_cluster().then(|| self.cluster())
  }
  pub fn cluster(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn cluster_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_cluster(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_last_updated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn last_updated_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_updated_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_updated(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

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
// - `StaticClusterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for StaticClusterMut<'_> {}

// SAFETY:
// - `StaticClusterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for StaticClusterMut<'_> {}

impl<'msg> ::protobuf::AsView for StaticClusterMut<'msg> {
  type Proxied = StaticCluster;
  fn as_view(&self) -> ::protobuf::View<'_, StaticCluster> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StaticClusterMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, StaticCluster>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for StaticClusterMut<'msg> {
  type MutProxied = StaticCluster;
  fn as_mut(&mut self) -> StaticClusterMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for StaticClusterMut<'msg> {
  fn into_mut<'shorter>(self) -> StaticClusterMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl StaticCluster {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, StaticCluster> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> StaticClusterView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> StaticClusterMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // cluster: optional message google.protobuf.Any
  pub fn has_cluster(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_cluster(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn cluster_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_cluster().then(|| self.cluster())
  }
  pub fn cluster(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn cluster_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_cluster(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_last_updated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn last_updated_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_updated_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_updated(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl StaticCluster

impl ::std::ops::Drop for StaticCluster {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for StaticCluster {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for StaticCluster {
  type Proxied = Self;
  fn as_view(&self) -> StaticClusterView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for StaticCluster {
  type MutProxied = Self;
  fn as_mut(&mut self) -> StaticClusterMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for StaticCluster {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::clusters_config_dump::envoy__admin__v3__ClustersConfigDump__StaticCluster_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::clusters_config_dump::envoy__admin__v3__ClustersConfigDump__StaticCluster_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::clusters_config_dump::envoy__admin__v3__ClustersConfigDump__StaticCluster_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StaticCluster {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StaticCluster {
  type Msg = StaticCluster;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StaticCluster> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StaticCluster {
  type Msg = StaticCluster;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StaticCluster> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StaticClusterMut<'_> {
  type Msg = StaticCluster;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StaticCluster> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StaticClusterMut<'_> {
  type Msg = StaticCluster;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StaticCluster> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StaticClusterView<'_> {
  type Msg = StaticCluster;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StaticCluster> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StaticClusterMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__admin__v3__ClustersConfigDump__DynamicCluster_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DynamicCluster {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DynamicCluster>
}

impl ::protobuf::Message for DynamicCluster {
  type MessageView<'msg> = DynamicClusterView<'msg>;
  type MessageMut<'msg> = DynamicClusterMut<'msg>;
}

impl ::std::default::Default for DynamicCluster {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DynamicCluster {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DynamicCluster` is `Sync` because it does not implement interior mutability.
//    Neither does `DynamicClusterMut`.
unsafe impl ::std::marker::Sync for DynamicCluster {}

// SAFETY:
// - `DynamicCluster` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DynamicCluster {}

impl ::protobuf::Proxied for DynamicCluster {
  type View<'msg> = DynamicClusterView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DynamicCluster {}

impl ::protobuf::MutProxied for DynamicCluster {
  type Mut<'msg> = DynamicClusterMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DynamicClusterView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicCluster>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DynamicClusterView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DynamicClusterView<'msg> {
  type Message = DynamicCluster;
}

impl ::std::fmt::Debug for DynamicClusterView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DynamicClusterView<'_> {
  fn default() -> DynamicClusterView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicCluster>> for DynamicClusterView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicCluster>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DynamicClusterView<'msg> {

  pub fn to_owned(&self) -> DynamicCluster {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // version_info: optional string
  pub fn version_info(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // cluster: optional message google.protobuf.Any
  pub fn has_cluster(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn cluster_opt(self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'msg>> {
    self.has_cluster().then(|| self.cluster())
  }
  pub fn cluster(self) -> ::protobuf_well_known_types::AnyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn last_updated_opt(self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'msg>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

  // error_state: optional message envoy.admin.v3.UpdateFailureState
  pub fn has_error_state(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn error_state_opt(self) -> ::std::option::Option<super::super::UpdateFailureStateView<'msg>> {
    self.has_error_state().then(|| self.error_state())
  }
  pub fn error_state(self) -> super::super::UpdateFailureStateView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::UpdateFailureStateView::default())
  }

  // client_status: optional enum envoy.admin.v3.ClientResourceStatus
  pub fn client_status(self) -> super::super::ClientResourceStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::super::ClientResourceStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `DynamicClusterView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DynamicClusterView<'_> {}

// SAFETY:
// - `DynamicClusterView` is `Send` because while its alive a `DynamicClusterMut` cannot.
// - `DynamicClusterView` does not use thread-local data.
unsafe impl ::std::marker::Send for DynamicClusterView<'_> {}

impl<'msg> ::protobuf::AsView for DynamicClusterView<'msg> {
  type Proxied = DynamicCluster;
  fn as_view(&self) -> ::protobuf::View<'msg, DynamicCluster> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DynamicClusterView<'msg> {
  fn into_view<'shorter>(self) -> DynamicClusterView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DynamicCluster> for DynamicClusterView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DynamicCluster {
    let mut dst = DynamicCluster::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DynamicCluster> for DynamicClusterMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DynamicCluster {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DynamicCluster {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DynamicClusterView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DynamicClusterMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DynamicClusterMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicCluster>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DynamicClusterMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DynamicClusterMut<'msg> {
  type Message = DynamicCluster;
}

impl ::std::fmt::Debug for DynamicClusterMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicCluster>> for DynamicClusterMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicCluster>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DynamicClusterMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicCluster> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DynamicCluster {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // version_info: optional string
  pub fn version_info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_version_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // cluster: optional message google.protobuf.Any
  pub fn has_cluster(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_cluster(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn cluster_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_cluster().then(|| self.cluster())
  }
  pub fn cluster(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn cluster_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_cluster(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_last_updated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn last_updated_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_updated_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_updated(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // error_state: optional message envoy.admin.v3.UpdateFailureState
  pub fn has_error_state(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_error_state(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn error_state_opt(&self) -> ::std::option::Option<super::super::UpdateFailureStateView<'_>> {
    self.has_error_state().then(|| self.error_state())
  }
  pub fn error_state(&self) -> super::super::UpdateFailureStateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::UpdateFailureStateView::default())
  }
  pub fn error_state_mut(&mut self) -> super::super::UpdateFailureStateMut<'_> {
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
  pub fn set_error_state(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::UpdateFailureState>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // client_status: optional enum envoy.admin.v3.ClientResourceStatus
  pub fn client_status(&self) -> super::super::ClientResourceStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::super::ClientResourceStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_client_status(&mut self, val: super::super::ClientResourceStatus) {
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

}

// SAFETY:
// - `DynamicClusterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DynamicClusterMut<'_> {}

// SAFETY:
// - `DynamicClusterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DynamicClusterMut<'_> {}

impl<'msg> ::protobuf::AsView for DynamicClusterMut<'msg> {
  type Proxied = DynamicCluster;
  fn as_view(&self) -> ::protobuf::View<'_, DynamicCluster> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DynamicClusterMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DynamicCluster>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DynamicClusterMut<'msg> {
  type MutProxied = DynamicCluster;
  fn as_mut(&mut self) -> DynamicClusterMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DynamicClusterMut<'msg> {
  fn into_mut<'shorter>(self) -> DynamicClusterMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DynamicCluster {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DynamicCluster> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DynamicClusterView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DynamicClusterMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // version_info: optional string
  pub fn version_info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_version_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // cluster: optional message google.protobuf.Any
  pub fn has_cluster(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_cluster(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn cluster_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_cluster().then(|| self.cluster())
  }
  pub fn cluster(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn cluster_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_cluster(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_last_updated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn last_updated_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_updated_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_updated(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // error_state: optional message envoy.admin.v3.UpdateFailureState
  pub fn has_error_state(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_error_state(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn error_state_opt(&self) -> ::std::option::Option<super::super::UpdateFailureStateView<'_>> {
    self.has_error_state().then(|| self.error_state())
  }
  pub fn error_state(&self) -> super::super::UpdateFailureStateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::UpdateFailureStateView::default())
  }
  pub fn error_state_mut(&mut self) -> super::super::UpdateFailureStateMut<'_> {
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
  pub fn set_error_state(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::UpdateFailureState>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // client_status: optional enum envoy.admin.v3.ClientResourceStatus
  pub fn client_status(&self) -> super::super::ClientResourceStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::super::ClientResourceStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_client_status(&mut self, val: super::super::ClientResourceStatus) {
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

}  // impl DynamicCluster

impl ::std::ops::Drop for DynamicCluster {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DynamicCluster {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DynamicCluster {
  type Proxied = Self;
  fn as_view(&self) -> DynamicClusterView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DynamicCluster {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DynamicClusterMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DynamicCluster {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::clusters_config_dump::envoy__admin__v3__ClustersConfigDump__DynamicCluster_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X333.P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::clusters_config_dump::envoy__admin__v3__ClustersConfigDump__DynamicCluster_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::UpdateFailureState as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::clusters_config_dump::envoy__admin__v3__ClustersConfigDump__DynamicCluster_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DynamicCluster {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DynamicCluster {
  type Msg = DynamicCluster;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicCluster> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicCluster {
  type Msg = DynamicCluster;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicCluster> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DynamicClusterMut<'_> {
  type Msg = DynamicCluster;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicCluster> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicClusterMut<'_> {
  type Msg = DynamicCluster;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicCluster> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicClusterView<'_> {
  type Msg = DynamicCluster;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicCluster> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DynamicClusterMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod clusters_config_dump


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__admin__v3__RoutesConfigDump_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RoutesConfigDump {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RoutesConfigDump>
}

impl ::protobuf::Message for RoutesConfigDump {
  type MessageView<'msg> = RoutesConfigDumpView<'msg>;
  type MessageMut<'msg> = RoutesConfigDumpMut<'msg>;
}

impl ::std::default::Default for RoutesConfigDump {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RoutesConfigDump {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RoutesConfigDump` is `Sync` because it does not implement interior mutability.
//    Neither does `RoutesConfigDumpMut`.
unsafe impl ::std::marker::Sync for RoutesConfigDump {}

// SAFETY:
// - `RoutesConfigDump` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RoutesConfigDump {}

impl ::protobuf::Proxied for RoutesConfigDump {
  type View<'msg> = RoutesConfigDumpView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RoutesConfigDump {}

impl ::protobuf::MutProxied for RoutesConfigDump {
  type Mut<'msg> = RoutesConfigDumpMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RoutesConfigDumpView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RoutesConfigDump>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RoutesConfigDumpView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RoutesConfigDumpView<'msg> {
  type Message = RoutesConfigDump;
}

impl ::std::fmt::Debug for RoutesConfigDumpView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RoutesConfigDumpView<'_> {
  fn default() -> RoutesConfigDumpView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RoutesConfigDump>> for RoutesConfigDumpView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RoutesConfigDump>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RoutesConfigDumpView<'msg> {

  pub fn to_owned(&self) -> RoutesConfigDump {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // static_route_configs: repeated message envoy.admin.v3.RoutesConfigDump.StaticRouteConfig
  pub fn static_route_configs(self) -> ::protobuf::RepeatedView<'msg, super::routes_config_dump::StaticRouteConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::routes_config_dump::StaticRouteConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // dynamic_route_configs: repeated message envoy.admin.v3.RoutesConfigDump.DynamicRouteConfig
  pub fn dynamic_route_configs(self) -> ::protobuf::RepeatedView<'msg, super::routes_config_dump::DynamicRouteConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::routes_config_dump::DynamicRouteConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `RoutesConfigDumpView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RoutesConfigDumpView<'_> {}

// SAFETY:
// - `RoutesConfigDumpView` is `Send` because while its alive a `RoutesConfigDumpMut` cannot.
// - `RoutesConfigDumpView` does not use thread-local data.
unsafe impl ::std::marker::Send for RoutesConfigDumpView<'_> {}

impl<'msg> ::protobuf::AsView for RoutesConfigDumpView<'msg> {
  type Proxied = RoutesConfigDump;
  fn as_view(&self) -> ::protobuf::View<'msg, RoutesConfigDump> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RoutesConfigDumpView<'msg> {
  fn into_view<'shorter>(self) -> RoutesConfigDumpView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RoutesConfigDump> for RoutesConfigDumpView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RoutesConfigDump {
    let mut dst = RoutesConfigDump::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RoutesConfigDump> for RoutesConfigDumpMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RoutesConfigDump {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RoutesConfigDump {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RoutesConfigDumpView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RoutesConfigDumpMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RoutesConfigDumpMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RoutesConfigDump>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RoutesConfigDumpMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RoutesConfigDumpMut<'msg> {
  type Message = RoutesConfigDump;
}

impl ::std::fmt::Debug for RoutesConfigDumpMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RoutesConfigDump>> for RoutesConfigDumpMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RoutesConfigDump>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RoutesConfigDumpMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RoutesConfigDump> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RoutesConfigDump {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // static_route_configs: repeated message envoy.admin.v3.RoutesConfigDump.StaticRouteConfig
  pub fn static_route_configs(&self) -> ::protobuf::RepeatedView<'_, super::routes_config_dump::StaticRouteConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::routes_config_dump::StaticRouteConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn static_route_configs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::routes_config_dump::StaticRouteConfig> {
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
  pub fn set_static_route_configs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::routes_config_dump::StaticRouteConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // dynamic_route_configs: repeated message envoy.admin.v3.RoutesConfigDump.DynamicRouteConfig
  pub fn dynamic_route_configs(&self) -> ::protobuf::RepeatedView<'_, super::routes_config_dump::DynamicRouteConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::routes_config_dump::DynamicRouteConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn dynamic_route_configs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::routes_config_dump::DynamicRouteConfig> {
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
  pub fn set_dynamic_route_configs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::routes_config_dump::DynamicRouteConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `RoutesConfigDumpMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RoutesConfigDumpMut<'_> {}

// SAFETY:
// - `RoutesConfigDumpMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RoutesConfigDumpMut<'_> {}

impl<'msg> ::protobuf::AsView for RoutesConfigDumpMut<'msg> {
  type Proxied = RoutesConfigDump;
  fn as_view(&self) -> ::protobuf::View<'_, RoutesConfigDump> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RoutesConfigDumpMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RoutesConfigDump>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RoutesConfigDumpMut<'msg> {
  type MutProxied = RoutesConfigDump;
  fn as_mut(&mut self) -> RoutesConfigDumpMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RoutesConfigDumpMut<'msg> {
  fn into_mut<'shorter>(self) -> RoutesConfigDumpMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RoutesConfigDump {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RoutesConfigDump> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RoutesConfigDumpView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RoutesConfigDumpMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // static_route_configs: repeated message envoy.admin.v3.RoutesConfigDump.StaticRouteConfig
  pub fn static_route_configs(&self) -> ::protobuf::RepeatedView<'_, super::routes_config_dump::StaticRouteConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::routes_config_dump::StaticRouteConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn static_route_configs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::routes_config_dump::StaticRouteConfig> {
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
  pub fn set_static_route_configs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::routes_config_dump::StaticRouteConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // dynamic_route_configs: repeated message envoy.admin.v3.RoutesConfigDump.DynamicRouteConfig
  pub fn dynamic_route_configs(&self) -> ::protobuf::RepeatedView<'_, super::routes_config_dump::DynamicRouteConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::routes_config_dump::DynamicRouteConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn dynamic_route_configs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::routes_config_dump::DynamicRouteConfig> {
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
  pub fn set_dynamic_route_configs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::routes_config_dump::DynamicRouteConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl RoutesConfigDump

impl ::std::ops::Drop for RoutesConfigDump {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RoutesConfigDump {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RoutesConfigDump {
  type Proxied = Self;
  fn as_view(&self) -> RoutesConfigDumpView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RoutesConfigDump {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RoutesConfigDumpMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RoutesConfigDump {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__admin__v3__RoutesConfigDump_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$aGG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__admin__v3__RoutesConfigDump_msg_init.0, &[<super::routes_config_dump::StaticRouteConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::routes_config_dump::DynamicRouteConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__admin__v3__RoutesConfigDump_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RoutesConfigDump {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RoutesConfigDump {
  type Msg = RoutesConfigDump;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RoutesConfigDump> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RoutesConfigDump {
  type Msg = RoutesConfigDump;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RoutesConfigDump> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RoutesConfigDumpMut<'_> {
  type Msg = RoutesConfigDump;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RoutesConfigDump> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RoutesConfigDumpMut<'_> {
  type Msg = RoutesConfigDump;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RoutesConfigDump> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RoutesConfigDumpView<'_> {
  type Msg = RoutesConfigDump;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RoutesConfigDump> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RoutesConfigDumpMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod routes_config_dump {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__admin__v3__RoutesConfigDump__StaticRouteConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct StaticRouteConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<StaticRouteConfig>
}

impl ::protobuf::Message for StaticRouteConfig {
  type MessageView<'msg> = StaticRouteConfigView<'msg>;
  type MessageMut<'msg> = StaticRouteConfigMut<'msg>;
}

impl ::std::default::Default for StaticRouteConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for StaticRouteConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `StaticRouteConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `StaticRouteConfigMut`.
unsafe impl ::std::marker::Sync for StaticRouteConfig {}

// SAFETY:
// - `StaticRouteConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for StaticRouteConfig {}

impl ::protobuf::Proxied for StaticRouteConfig {
  type View<'msg> = StaticRouteConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for StaticRouteConfig {}

impl ::protobuf::MutProxied for StaticRouteConfig {
  type Mut<'msg> = StaticRouteConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct StaticRouteConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StaticRouteConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StaticRouteConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for StaticRouteConfigView<'msg> {
  type Message = StaticRouteConfig;
}

impl ::std::fmt::Debug for StaticRouteConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for StaticRouteConfigView<'_> {
  fn default() -> StaticRouteConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, StaticRouteConfig>> for StaticRouteConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StaticRouteConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StaticRouteConfigView<'msg> {

  pub fn to_owned(&self) -> StaticRouteConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // route_config: optional message google.protobuf.Any
  pub fn has_route_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn route_config_opt(self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'msg>> {
    self.has_route_config().then(|| self.route_config())
  }
  pub fn route_config(self) -> ::protobuf_well_known_types::AnyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn last_updated_opt(self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'msg>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

}

// SAFETY:
// - `StaticRouteConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for StaticRouteConfigView<'_> {}

// SAFETY:
// - `StaticRouteConfigView` is `Send` because while its alive a `StaticRouteConfigMut` cannot.
// - `StaticRouteConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for StaticRouteConfigView<'_> {}

impl<'msg> ::protobuf::AsView for StaticRouteConfigView<'msg> {
  type Proxied = StaticRouteConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, StaticRouteConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StaticRouteConfigView<'msg> {
  fn into_view<'shorter>(self) -> StaticRouteConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<StaticRouteConfig> for StaticRouteConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StaticRouteConfig {
    let mut dst = StaticRouteConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<StaticRouteConfig> for StaticRouteConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StaticRouteConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for StaticRouteConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StaticRouteConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StaticRouteConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct StaticRouteConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StaticRouteConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StaticRouteConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for StaticRouteConfigMut<'msg> {
  type Message = StaticRouteConfig;
}

impl ::std::fmt::Debug for StaticRouteConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, StaticRouteConfig>> for StaticRouteConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StaticRouteConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StaticRouteConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, StaticRouteConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> StaticRouteConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // route_config: optional message google.protobuf.Any
  pub fn has_route_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_route_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn route_config_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_route_config().then(|| self.route_config())
  }
  pub fn route_config(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn route_config_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_route_config(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_last_updated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn last_updated_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_updated_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_updated(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

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
// - `StaticRouteConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for StaticRouteConfigMut<'_> {}

// SAFETY:
// - `StaticRouteConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for StaticRouteConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for StaticRouteConfigMut<'msg> {
  type Proxied = StaticRouteConfig;
  fn as_view(&self) -> ::protobuf::View<'_, StaticRouteConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StaticRouteConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, StaticRouteConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for StaticRouteConfigMut<'msg> {
  type MutProxied = StaticRouteConfig;
  fn as_mut(&mut self) -> StaticRouteConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for StaticRouteConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> StaticRouteConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl StaticRouteConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, StaticRouteConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> StaticRouteConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> StaticRouteConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // route_config: optional message google.protobuf.Any
  pub fn has_route_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_route_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn route_config_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_route_config().then(|| self.route_config())
  }
  pub fn route_config(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn route_config_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_route_config(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_last_updated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn last_updated_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_updated_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_updated(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl StaticRouteConfig

impl ::std::ops::Drop for StaticRouteConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for StaticRouteConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for StaticRouteConfig {
  type Proxied = Self;
  fn as_view(&self) -> StaticRouteConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for StaticRouteConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> StaticRouteConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for StaticRouteConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::routes_config_dump::envoy__admin__v3__RoutesConfigDump__StaticRouteConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::routes_config_dump::envoy__admin__v3__RoutesConfigDump__StaticRouteConfig_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::routes_config_dump::envoy__admin__v3__RoutesConfigDump__StaticRouteConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StaticRouteConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StaticRouteConfig {
  type Msg = StaticRouteConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StaticRouteConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StaticRouteConfig {
  type Msg = StaticRouteConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StaticRouteConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StaticRouteConfigMut<'_> {
  type Msg = StaticRouteConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StaticRouteConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StaticRouteConfigMut<'_> {
  type Msg = StaticRouteConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StaticRouteConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StaticRouteConfigView<'_> {
  type Msg = StaticRouteConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StaticRouteConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StaticRouteConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__admin__v3__RoutesConfigDump__DynamicRouteConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DynamicRouteConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DynamicRouteConfig>
}

impl ::protobuf::Message for DynamicRouteConfig {
  type MessageView<'msg> = DynamicRouteConfigView<'msg>;
  type MessageMut<'msg> = DynamicRouteConfigMut<'msg>;
}

impl ::std::default::Default for DynamicRouteConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DynamicRouteConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DynamicRouteConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `DynamicRouteConfigMut`.
unsafe impl ::std::marker::Sync for DynamicRouteConfig {}

// SAFETY:
// - `DynamicRouteConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DynamicRouteConfig {}

impl ::protobuf::Proxied for DynamicRouteConfig {
  type View<'msg> = DynamicRouteConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DynamicRouteConfig {}

impl ::protobuf::MutProxied for DynamicRouteConfig {
  type Mut<'msg> = DynamicRouteConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DynamicRouteConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicRouteConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DynamicRouteConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DynamicRouteConfigView<'msg> {
  type Message = DynamicRouteConfig;
}

impl ::std::fmt::Debug for DynamicRouteConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DynamicRouteConfigView<'_> {
  fn default() -> DynamicRouteConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicRouteConfig>> for DynamicRouteConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicRouteConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DynamicRouteConfigView<'msg> {

  pub fn to_owned(&self) -> DynamicRouteConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // version_info: optional string
  pub fn version_info(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // route_config: optional message google.protobuf.Any
  pub fn has_route_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn route_config_opt(self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'msg>> {
    self.has_route_config().then(|| self.route_config())
  }
  pub fn route_config(self) -> ::protobuf_well_known_types::AnyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn last_updated_opt(self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'msg>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

  // error_state: optional message envoy.admin.v3.UpdateFailureState
  pub fn has_error_state(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn error_state_opt(self) -> ::std::option::Option<super::super::UpdateFailureStateView<'msg>> {
    self.has_error_state().then(|| self.error_state())
  }
  pub fn error_state(self) -> super::super::UpdateFailureStateView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::UpdateFailureStateView::default())
  }

  // client_status: optional enum envoy.admin.v3.ClientResourceStatus
  pub fn client_status(self) -> super::super::ClientResourceStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::super::ClientResourceStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `DynamicRouteConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DynamicRouteConfigView<'_> {}

// SAFETY:
// - `DynamicRouteConfigView` is `Send` because while its alive a `DynamicRouteConfigMut` cannot.
// - `DynamicRouteConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for DynamicRouteConfigView<'_> {}

impl<'msg> ::protobuf::AsView for DynamicRouteConfigView<'msg> {
  type Proxied = DynamicRouteConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, DynamicRouteConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DynamicRouteConfigView<'msg> {
  fn into_view<'shorter>(self) -> DynamicRouteConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DynamicRouteConfig> for DynamicRouteConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DynamicRouteConfig {
    let mut dst = DynamicRouteConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DynamicRouteConfig> for DynamicRouteConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DynamicRouteConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DynamicRouteConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DynamicRouteConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DynamicRouteConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DynamicRouteConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicRouteConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DynamicRouteConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DynamicRouteConfigMut<'msg> {
  type Message = DynamicRouteConfig;
}

impl ::std::fmt::Debug for DynamicRouteConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicRouteConfig>> for DynamicRouteConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicRouteConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DynamicRouteConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicRouteConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DynamicRouteConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // version_info: optional string
  pub fn version_info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_version_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // route_config: optional message google.protobuf.Any
  pub fn has_route_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_route_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn route_config_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_route_config().then(|| self.route_config())
  }
  pub fn route_config(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn route_config_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_route_config(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_last_updated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn last_updated_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_updated_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_updated(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // error_state: optional message envoy.admin.v3.UpdateFailureState
  pub fn has_error_state(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_error_state(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn error_state_opt(&self) -> ::std::option::Option<super::super::UpdateFailureStateView<'_>> {
    self.has_error_state().then(|| self.error_state())
  }
  pub fn error_state(&self) -> super::super::UpdateFailureStateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::UpdateFailureStateView::default())
  }
  pub fn error_state_mut(&mut self) -> super::super::UpdateFailureStateMut<'_> {
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
  pub fn set_error_state(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::UpdateFailureState>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // client_status: optional enum envoy.admin.v3.ClientResourceStatus
  pub fn client_status(&self) -> super::super::ClientResourceStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::super::ClientResourceStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_client_status(&mut self, val: super::super::ClientResourceStatus) {
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

}

// SAFETY:
// - `DynamicRouteConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DynamicRouteConfigMut<'_> {}

// SAFETY:
// - `DynamicRouteConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DynamicRouteConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for DynamicRouteConfigMut<'msg> {
  type Proxied = DynamicRouteConfig;
  fn as_view(&self) -> ::protobuf::View<'_, DynamicRouteConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DynamicRouteConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DynamicRouteConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DynamicRouteConfigMut<'msg> {
  type MutProxied = DynamicRouteConfig;
  fn as_mut(&mut self) -> DynamicRouteConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DynamicRouteConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> DynamicRouteConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DynamicRouteConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DynamicRouteConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DynamicRouteConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DynamicRouteConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // version_info: optional string
  pub fn version_info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_version_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // route_config: optional message google.protobuf.Any
  pub fn has_route_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_route_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn route_config_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_route_config().then(|| self.route_config())
  }
  pub fn route_config(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn route_config_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_route_config(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_last_updated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn last_updated_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_updated_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_updated(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // error_state: optional message envoy.admin.v3.UpdateFailureState
  pub fn has_error_state(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_error_state(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn error_state_opt(&self) -> ::std::option::Option<super::super::UpdateFailureStateView<'_>> {
    self.has_error_state().then(|| self.error_state())
  }
  pub fn error_state(&self) -> super::super::UpdateFailureStateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::UpdateFailureStateView::default())
  }
  pub fn error_state_mut(&mut self) -> super::super::UpdateFailureStateMut<'_> {
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
  pub fn set_error_state(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::UpdateFailureState>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // client_status: optional enum envoy.admin.v3.ClientResourceStatus
  pub fn client_status(&self) -> super::super::ClientResourceStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::super::ClientResourceStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_client_status(&mut self, val: super::super::ClientResourceStatus) {
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

}  // impl DynamicRouteConfig

impl ::std::ops::Drop for DynamicRouteConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DynamicRouteConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DynamicRouteConfig {
  type Proxied = Self;
  fn as_view(&self) -> DynamicRouteConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DynamicRouteConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DynamicRouteConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DynamicRouteConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::routes_config_dump::envoy__admin__v3__RoutesConfigDump__DynamicRouteConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X333.P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::routes_config_dump::envoy__admin__v3__RoutesConfigDump__DynamicRouteConfig_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::UpdateFailureState as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::routes_config_dump::envoy__admin__v3__RoutesConfigDump__DynamicRouteConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DynamicRouteConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DynamicRouteConfig {
  type Msg = DynamicRouteConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicRouteConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicRouteConfig {
  type Msg = DynamicRouteConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicRouteConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DynamicRouteConfigMut<'_> {
  type Msg = DynamicRouteConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicRouteConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicRouteConfigMut<'_> {
  type Msg = DynamicRouteConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicRouteConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicRouteConfigView<'_> {
  type Msg = DynamicRouteConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicRouteConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DynamicRouteConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod routes_config_dump


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__admin__v3__ScopedRoutesConfigDump_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ScopedRoutesConfigDump {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ScopedRoutesConfigDump>
}

impl ::protobuf::Message for ScopedRoutesConfigDump {
  type MessageView<'msg> = ScopedRoutesConfigDumpView<'msg>;
  type MessageMut<'msg> = ScopedRoutesConfigDumpMut<'msg>;
}

impl ::std::default::Default for ScopedRoutesConfigDump {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ScopedRoutesConfigDump {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ScopedRoutesConfigDump` is `Sync` because it does not implement interior mutability.
//    Neither does `ScopedRoutesConfigDumpMut`.
unsafe impl ::std::marker::Sync for ScopedRoutesConfigDump {}

// SAFETY:
// - `ScopedRoutesConfigDump` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ScopedRoutesConfigDump {}

impl ::protobuf::Proxied for ScopedRoutesConfigDump {
  type View<'msg> = ScopedRoutesConfigDumpView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ScopedRoutesConfigDump {}

impl ::protobuf::MutProxied for ScopedRoutesConfigDump {
  type Mut<'msg> = ScopedRoutesConfigDumpMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ScopedRoutesConfigDumpView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ScopedRoutesConfigDump>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ScopedRoutesConfigDumpView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ScopedRoutesConfigDumpView<'msg> {
  type Message = ScopedRoutesConfigDump;
}

impl ::std::fmt::Debug for ScopedRoutesConfigDumpView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ScopedRoutesConfigDumpView<'_> {
  fn default() -> ScopedRoutesConfigDumpView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ScopedRoutesConfigDump>> for ScopedRoutesConfigDumpView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ScopedRoutesConfigDump>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ScopedRoutesConfigDumpView<'msg> {

  pub fn to_owned(&self) -> ScopedRoutesConfigDump {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // inline_scoped_route_configs: repeated message envoy.admin.v3.ScopedRoutesConfigDump.InlineScopedRouteConfigs
  pub fn inline_scoped_route_configs(self) -> ::protobuf::RepeatedView<'msg, super::scoped_routes_config_dump::InlineScopedRouteConfigs> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::scoped_routes_config_dump::InlineScopedRouteConfigs>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // dynamic_scoped_route_configs: repeated message envoy.admin.v3.ScopedRoutesConfigDump.DynamicScopedRouteConfigs
  pub fn dynamic_scoped_route_configs(self) -> ::protobuf::RepeatedView<'msg, super::scoped_routes_config_dump::DynamicScopedRouteConfigs> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::scoped_routes_config_dump::DynamicScopedRouteConfigs>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `ScopedRoutesConfigDumpView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ScopedRoutesConfigDumpView<'_> {}

// SAFETY:
// - `ScopedRoutesConfigDumpView` is `Send` because while its alive a `ScopedRoutesConfigDumpMut` cannot.
// - `ScopedRoutesConfigDumpView` does not use thread-local data.
unsafe impl ::std::marker::Send for ScopedRoutesConfigDumpView<'_> {}

impl<'msg> ::protobuf::AsView for ScopedRoutesConfigDumpView<'msg> {
  type Proxied = ScopedRoutesConfigDump;
  fn as_view(&self) -> ::protobuf::View<'msg, ScopedRoutesConfigDump> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ScopedRoutesConfigDumpView<'msg> {
  fn into_view<'shorter>(self) -> ScopedRoutesConfigDumpView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ScopedRoutesConfigDump> for ScopedRoutesConfigDumpView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ScopedRoutesConfigDump {
    let mut dst = ScopedRoutesConfigDump::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ScopedRoutesConfigDump> for ScopedRoutesConfigDumpMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ScopedRoutesConfigDump {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ScopedRoutesConfigDump {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ScopedRoutesConfigDumpView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ScopedRoutesConfigDumpMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ScopedRoutesConfigDumpMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ScopedRoutesConfigDump>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ScopedRoutesConfigDumpMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ScopedRoutesConfigDumpMut<'msg> {
  type Message = ScopedRoutesConfigDump;
}

impl ::std::fmt::Debug for ScopedRoutesConfigDumpMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ScopedRoutesConfigDump>> for ScopedRoutesConfigDumpMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ScopedRoutesConfigDump>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ScopedRoutesConfigDumpMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ScopedRoutesConfigDump> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ScopedRoutesConfigDump {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // inline_scoped_route_configs: repeated message envoy.admin.v3.ScopedRoutesConfigDump.InlineScopedRouteConfigs
  pub fn inline_scoped_route_configs(&self) -> ::protobuf::RepeatedView<'_, super::scoped_routes_config_dump::InlineScopedRouteConfigs> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::scoped_routes_config_dump::InlineScopedRouteConfigs>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn inline_scoped_route_configs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::scoped_routes_config_dump::InlineScopedRouteConfigs> {
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
  pub fn set_inline_scoped_route_configs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::scoped_routes_config_dump::InlineScopedRouteConfigs>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // dynamic_scoped_route_configs: repeated message envoy.admin.v3.ScopedRoutesConfigDump.DynamicScopedRouteConfigs
  pub fn dynamic_scoped_route_configs(&self) -> ::protobuf::RepeatedView<'_, super::scoped_routes_config_dump::DynamicScopedRouteConfigs> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::scoped_routes_config_dump::DynamicScopedRouteConfigs>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn dynamic_scoped_route_configs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::scoped_routes_config_dump::DynamicScopedRouteConfigs> {
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
  pub fn set_dynamic_scoped_route_configs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::scoped_routes_config_dump::DynamicScopedRouteConfigs>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `ScopedRoutesConfigDumpMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ScopedRoutesConfigDumpMut<'_> {}

// SAFETY:
// - `ScopedRoutesConfigDumpMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ScopedRoutesConfigDumpMut<'_> {}

impl<'msg> ::protobuf::AsView for ScopedRoutesConfigDumpMut<'msg> {
  type Proxied = ScopedRoutesConfigDump;
  fn as_view(&self) -> ::protobuf::View<'_, ScopedRoutesConfigDump> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ScopedRoutesConfigDumpMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ScopedRoutesConfigDump>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ScopedRoutesConfigDumpMut<'msg> {
  type MutProxied = ScopedRoutesConfigDump;
  fn as_mut(&mut self) -> ScopedRoutesConfigDumpMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ScopedRoutesConfigDumpMut<'msg> {
  fn into_mut<'shorter>(self) -> ScopedRoutesConfigDumpMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ScopedRoutesConfigDump {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ScopedRoutesConfigDump> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ScopedRoutesConfigDumpView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ScopedRoutesConfigDumpMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // inline_scoped_route_configs: repeated message envoy.admin.v3.ScopedRoutesConfigDump.InlineScopedRouteConfigs
  pub fn inline_scoped_route_configs(&self) -> ::protobuf::RepeatedView<'_, super::scoped_routes_config_dump::InlineScopedRouteConfigs> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::scoped_routes_config_dump::InlineScopedRouteConfigs>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn inline_scoped_route_configs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::scoped_routes_config_dump::InlineScopedRouteConfigs> {
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
  pub fn set_inline_scoped_route_configs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::scoped_routes_config_dump::InlineScopedRouteConfigs>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // dynamic_scoped_route_configs: repeated message envoy.admin.v3.ScopedRoutesConfigDump.DynamicScopedRouteConfigs
  pub fn dynamic_scoped_route_configs(&self) -> ::protobuf::RepeatedView<'_, super::scoped_routes_config_dump::DynamicScopedRouteConfigs> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::scoped_routes_config_dump::DynamicScopedRouteConfigs>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn dynamic_scoped_route_configs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::scoped_routes_config_dump::DynamicScopedRouteConfigs> {
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
  pub fn set_dynamic_scoped_route_configs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::scoped_routes_config_dump::DynamicScopedRouteConfigs>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl ScopedRoutesConfigDump

impl ::std::ops::Drop for ScopedRoutesConfigDump {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ScopedRoutesConfigDump {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ScopedRoutesConfigDump {
  type Proxied = Self;
  fn as_view(&self) -> ScopedRoutesConfigDumpView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ScopedRoutesConfigDump {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ScopedRoutesConfigDumpMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ScopedRoutesConfigDump {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__admin__v3__ScopedRoutesConfigDump_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$GG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__admin__v3__ScopedRoutesConfigDump_msg_init.0, &[<super::scoped_routes_config_dump::InlineScopedRouteConfigs as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::scoped_routes_config_dump::DynamicScopedRouteConfigs as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__admin__v3__ScopedRoutesConfigDump_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ScopedRoutesConfigDump {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ScopedRoutesConfigDump {
  type Msg = ScopedRoutesConfigDump;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScopedRoutesConfigDump> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ScopedRoutesConfigDump {
  type Msg = ScopedRoutesConfigDump;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScopedRoutesConfigDump> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ScopedRoutesConfigDumpMut<'_> {
  type Msg = ScopedRoutesConfigDump;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScopedRoutesConfigDump> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ScopedRoutesConfigDumpMut<'_> {
  type Msg = ScopedRoutesConfigDump;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScopedRoutesConfigDump> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ScopedRoutesConfigDumpView<'_> {
  type Msg = ScopedRoutesConfigDump;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScopedRoutesConfigDump> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ScopedRoutesConfigDumpMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod scoped_routes_config_dump {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__admin__v3__ScopedRoutesConfigDump__InlineScopedRouteConfigs_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct InlineScopedRouteConfigs {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<InlineScopedRouteConfigs>
}

impl ::protobuf::Message for InlineScopedRouteConfigs {
  type MessageView<'msg> = InlineScopedRouteConfigsView<'msg>;
  type MessageMut<'msg> = InlineScopedRouteConfigsMut<'msg>;
}

impl ::std::default::Default for InlineScopedRouteConfigs {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for InlineScopedRouteConfigs {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `InlineScopedRouteConfigs` is `Sync` because it does not implement interior mutability.
//    Neither does `InlineScopedRouteConfigsMut`.
unsafe impl ::std::marker::Sync for InlineScopedRouteConfigs {}

// SAFETY:
// - `InlineScopedRouteConfigs` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for InlineScopedRouteConfigs {}

impl ::protobuf::Proxied for InlineScopedRouteConfigs {
  type View<'msg> = InlineScopedRouteConfigsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for InlineScopedRouteConfigs {}

impl ::protobuf::MutProxied for InlineScopedRouteConfigs {
  type Mut<'msg> = InlineScopedRouteConfigsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct InlineScopedRouteConfigsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, InlineScopedRouteConfigs>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for InlineScopedRouteConfigsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for InlineScopedRouteConfigsView<'msg> {
  type Message = InlineScopedRouteConfigs;
}

impl ::std::fmt::Debug for InlineScopedRouteConfigsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for InlineScopedRouteConfigsView<'_> {
  fn default() -> InlineScopedRouteConfigsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, InlineScopedRouteConfigs>> for InlineScopedRouteConfigsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, InlineScopedRouteConfigs>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> InlineScopedRouteConfigsView<'msg> {

  pub fn to_owned(&self) -> InlineScopedRouteConfigs {
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

  // scoped_route_configs: repeated message google.protobuf.Any
  pub fn scoped_route_configs(self) -> ::protobuf::RepeatedView<'msg, ::protobuf_well_known_types::Any> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf_well_known_types::Any>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn last_updated_opt(self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'msg>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

}

// SAFETY:
// - `InlineScopedRouteConfigsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for InlineScopedRouteConfigsView<'_> {}

// SAFETY:
// - `InlineScopedRouteConfigsView` is `Send` because while its alive a `InlineScopedRouteConfigsMut` cannot.
// - `InlineScopedRouteConfigsView` does not use thread-local data.
unsafe impl ::std::marker::Send for InlineScopedRouteConfigsView<'_> {}

impl<'msg> ::protobuf::AsView for InlineScopedRouteConfigsView<'msg> {
  type Proxied = InlineScopedRouteConfigs;
  fn as_view(&self) -> ::protobuf::View<'msg, InlineScopedRouteConfigs> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for InlineScopedRouteConfigsView<'msg> {
  fn into_view<'shorter>(self) -> InlineScopedRouteConfigsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<InlineScopedRouteConfigs> for InlineScopedRouteConfigsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> InlineScopedRouteConfigs {
    let mut dst = InlineScopedRouteConfigs::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<InlineScopedRouteConfigs> for InlineScopedRouteConfigsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> InlineScopedRouteConfigs {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for InlineScopedRouteConfigs {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for InlineScopedRouteConfigsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for InlineScopedRouteConfigsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct InlineScopedRouteConfigsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, InlineScopedRouteConfigs>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for InlineScopedRouteConfigsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for InlineScopedRouteConfigsMut<'msg> {
  type Message = InlineScopedRouteConfigs;
}

impl ::std::fmt::Debug for InlineScopedRouteConfigsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, InlineScopedRouteConfigs>> for InlineScopedRouteConfigsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, InlineScopedRouteConfigs>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> InlineScopedRouteConfigsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, InlineScopedRouteConfigs> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> InlineScopedRouteConfigs {
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

  // scoped_route_configs: repeated message google.protobuf.Any
  pub fn scoped_route_configs(&self) -> ::protobuf::RepeatedView<'_, ::protobuf_well_known_types::Any> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf_well_known_types::Any>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn scoped_route_configs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf_well_known_types::Any> {
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
  pub fn set_scoped_route_configs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf_well_known_types::Any>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_last_updated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn last_updated_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_updated_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_updated(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

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
// - `InlineScopedRouteConfigsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for InlineScopedRouteConfigsMut<'_> {}

// SAFETY:
// - `InlineScopedRouteConfigsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for InlineScopedRouteConfigsMut<'_> {}

impl<'msg> ::protobuf::AsView for InlineScopedRouteConfigsMut<'msg> {
  type Proxied = InlineScopedRouteConfigs;
  fn as_view(&self) -> ::protobuf::View<'_, InlineScopedRouteConfigs> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for InlineScopedRouteConfigsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, InlineScopedRouteConfigs>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for InlineScopedRouteConfigsMut<'msg> {
  type MutProxied = InlineScopedRouteConfigs;
  fn as_mut(&mut self) -> InlineScopedRouteConfigsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for InlineScopedRouteConfigsMut<'msg> {
  fn into_mut<'shorter>(self) -> InlineScopedRouteConfigsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl InlineScopedRouteConfigs {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, InlineScopedRouteConfigs> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> InlineScopedRouteConfigsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> InlineScopedRouteConfigsMut<'_> {
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

  // scoped_route_configs: repeated message google.protobuf.Any
  pub fn scoped_route_configs(&self) -> ::protobuf::RepeatedView<'_, ::protobuf_well_known_types::Any> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf_well_known_types::Any>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn scoped_route_configs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf_well_known_types::Any> {
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
  pub fn set_scoped_route_configs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf_well_known_types::Any>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_last_updated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn last_updated_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_updated_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_updated(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl InlineScopedRouteConfigs

impl ::std::ops::Drop for InlineScopedRouteConfigs {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for InlineScopedRouteConfigs {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for InlineScopedRouteConfigs {
  type Proxied = Self;
  fn as_view(&self) -> InlineScopedRouteConfigsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for InlineScopedRouteConfigs {
  type MutProxied = Self;
  fn as_mut(&mut self) -> InlineScopedRouteConfigsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for InlineScopedRouteConfigs {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::scoped_routes_config_dump::envoy__admin__v3__ScopedRoutesConfigDump__InlineScopedRouteConfigs_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1XG3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::scoped_routes_config_dump::envoy__admin__v3__ScopedRoutesConfigDump__InlineScopedRouteConfigs_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::scoped_routes_config_dump::envoy__admin__v3__ScopedRoutesConfigDump__InlineScopedRouteConfigs_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for InlineScopedRouteConfigs {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for InlineScopedRouteConfigs {
  type Msg = InlineScopedRouteConfigs;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<InlineScopedRouteConfigs> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for InlineScopedRouteConfigs {
  type Msg = InlineScopedRouteConfigs;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<InlineScopedRouteConfigs> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for InlineScopedRouteConfigsMut<'_> {
  type Msg = InlineScopedRouteConfigs;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<InlineScopedRouteConfigs> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for InlineScopedRouteConfigsMut<'_> {
  type Msg = InlineScopedRouteConfigs;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<InlineScopedRouteConfigs> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for InlineScopedRouteConfigsView<'_> {
  type Msg = InlineScopedRouteConfigs;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<InlineScopedRouteConfigs> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for InlineScopedRouteConfigsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__admin__v3__ScopedRoutesConfigDump__DynamicScopedRouteConfigs_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DynamicScopedRouteConfigs {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DynamicScopedRouteConfigs>
}

impl ::protobuf::Message for DynamicScopedRouteConfigs {
  type MessageView<'msg> = DynamicScopedRouteConfigsView<'msg>;
  type MessageMut<'msg> = DynamicScopedRouteConfigsMut<'msg>;
}

impl ::std::default::Default for DynamicScopedRouteConfigs {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DynamicScopedRouteConfigs {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DynamicScopedRouteConfigs` is `Sync` because it does not implement interior mutability.
//    Neither does `DynamicScopedRouteConfigsMut`.
unsafe impl ::std::marker::Sync for DynamicScopedRouteConfigs {}

// SAFETY:
// - `DynamicScopedRouteConfigs` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DynamicScopedRouteConfigs {}

impl ::protobuf::Proxied for DynamicScopedRouteConfigs {
  type View<'msg> = DynamicScopedRouteConfigsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DynamicScopedRouteConfigs {}

impl ::protobuf::MutProxied for DynamicScopedRouteConfigs {
  type Mut<'msg> = DynamicScopedRouteConfigsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DynamicScopedRouteConfigsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicScopedRouteConfigs>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DynamicScopedRouteConfigsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DynamicScopedRouteConfigsView<'msg> {
  type Message = DynamicScopedRouteConfigs;
}

impl ::std::fmt::Debug for DynamicScopedRouteConfigsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DynamicScopedRouteConfigsView<'_> {
  fn default() -> DynamicScopedRouteConfigsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicScopedRouteConfigs>> for DynamicScopedRouteConfigsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicScopedRouteConfigs>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DynamicScopedRouteConfigsView<'msg> {

  pub fn to_owned(&self) -> DynamicScopedRouteConfigs {
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

  // version_info: optional string
  pub fn version_info(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // scoped_route_configs: repeated message google.protobuf.Any
  pub fn scoped_route_configs(self) -> ::protobuf::RepeatedView<'msg, ::protobuf_well_known_types::Any> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf_well_known_types::Any>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn last_updated_opt(self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'msg>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

  // error_state: optional message envoy.admin.v3.UpdateFailureState
  pub fn has_error_state(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn error_state_opt(self) -> ::std::option::Option<super::super::UpdateFailureStateView<'msg>> {
    self.has_error_state().then(|| self.error_state())
  }
  pub fn error_state(self) -> super::super::UpdateFailureStateView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::UpdateFailureStateView::default())
  }

  // client_status: optional enum envoy.admin.v3.ClientResourceStatus
  pub fn client_status(self) -> super::super::ClientResourceStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::super::ClientResourceStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `DynamicScopedRouteConfigsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DynamicScopedRouteConfigsView<'_> {}

// SAFETY:
// - `DynamicScopedRouteConfigsView` is `Send` because while its alive a `DynamicScopedRouteConfigsMut` cannot.
// - `DynamicScopedRouteConfigsView` does not use thread-local data.
unsafe impl ::std::marker::Send for DynamicScopedRouteConfigsView<'_> {}

impl<'msg> ::protobuf::AsView for DynamicScopedRouteConfigsView<'msg> {
  type Proxied = DynamicScopedRouteConfigs;
  fn as_view(&self) -> ::protobuf::View<'msg, DynamicScopedRouteConfigs> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DynamicScopedRouteConfigsView<'msg> {
  fn into_view<'shorter>(self) -> DynamicScopedRouteConfigsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DynamicScopedRouteConfigs> for DynamicScopedRouteConfigsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DynamicScopedRouteConfigs {
    let mut dst = DynamicScopedRouteConfigs::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DynamicScopedRouteConfigs> for DynamicScopedRouteConfigsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DynamicScopedRouteConfigs {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DynamicScopedRouteConfigs {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DynamicScopedRouteConfigsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DynamicScopedRouteConfigsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DynamicScopedRouteConfigsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicScopedRouteConfigs>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DynamicScopedRouteConfigsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DynamicScopedRouteConfigsMut<'msg> {
  type Message = DynamicScopedRouteConfigs;
}

impl ::std::fmt::Debug for DynamicScopedRouteConfigsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicScopedRouteConfigs>> for DynamicScopedRouteConfigsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicScopedRouteConfigs>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DynamicScopedRouteConfigsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicScopedRouteConfigs> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DynamicScopedRouteConfigs {
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

  // version_info: optional string
  pub fn version_info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_version_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // scoped_route_configs: repeated message google.protobuf.Any
  pub fn scoped_route_configs(&self) -> ::protobuf::RepeatedView<'_, ::protobuf_well_known_types::Any> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf_well_known_types::Any>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn scoped_route_configs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf_well_known_types::Any> {
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
  pub fn set_scoped_route_configs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf_well_known_types::Any>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_last_updated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn last_updated_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_updated_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_updated(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // error_state: optional message envoy.admin.v3.UpdateFailureState
  pub fn has_error_state(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_error_state(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn error_state_opt(&self) -> ::std::option::Option<super::super::UpdateFailureStateView<'_>> {
    self.has_error_state().then(|| self.error_state())
  }
  pub fn error_state(&self) -> super::super::UpdateFailureStateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::UpdateFailureStateView::default())
  }
  pub fn error_state_mut(&mut self) -> super::super::UpdateFailureStateMut<'_> {
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
  pub fn set_error_state(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::UpdateFailureState>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // client_status: optional enum envoy.admin.v3.ClientResourceStatus
  pub fn client_status(&self) -> super::super::ClientResourceStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::super::ClientResourceStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_client_status(&mut self, val: super::super::ClientResourceStatus) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        5, val.into()
      )
    }
  }

}

// SAFETY:
// - `DynamicScopedRouteConfigsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DynamicScopedRouteConfigsMut<'_> {}

// SAFETY:
// - `DynamicScopedRouteConfigsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DynamicScopedRouteConfigsMut<'_> {}

impl<'msg> ::protobuf::AsView for DynamicScopedRouteConfigsMut<'msg> {
  type Proxied = DynamicScopedRouteConfigs;
  fn as_view(&self) -> ::protobuf::View<'_, DynamicScopedRouteConfigs> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DynamicScopedRouteConfigsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DynamicScopedRouteConfigs>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DynamicScopedRouteConfigsMut<'msg> {
  type MutProxied = DynamicScopedRouteConfigs;
  fn as_mut(&mut self) -> DynamicScopedRouteConfigsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DynamicScopedRouteConfigsMut<'msg> {
  fn into_mut<'shorter>(self) -> DynamicScopedRouteConfigsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DynamicScopedRouteConfigs {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DynamicScopedRouteConfigs> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DynamicScopedRouteConfigsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DynamicScopedRouteConfigsMut<'_> {
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

  // version_info: optional string
  pub fn version_info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_version_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // scoped_route_configs: repeated message google.protobuf.Any
  pub fn scoped_route_configs(&self) -> ::protobuf::RepeatedView<'_, ::protobuf_well_known_types::Any> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf_well_known_types::Any>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn scoped_route_configs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf_well_known_types::Any> {
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
  pub fn set_scoped_route_configs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf_well_known_types::Any>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_last_updated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn last_updated_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_updated_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_updated(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // error_state: optional message envoy.admin.v3.UpdateFailureState
  pub fn has_error_state(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_error_state(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn error_state_opt(&self) -> ::std::option::Option<super::super::UpdateFailureStateView<'_>> {
    self.has_error_state().then(|| self.error_state())
  }
  pub fn error_state(&self) -> super::super::UpdateFailureStateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::UpdateFailureStateView::default())
  }
  pub fn error_state_mut(&mut self) -> super::super::UpdateFailureStateMut<'_> {
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
  pub fn set_error_state(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::UpdateFailureState>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // client_status: optional enum envoy.admin.v3.ClientResourceStatus
  pub fn client_status(&self) -> super::super::ClientResourceStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::super::ClientResourceStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_client_status(&mut self, val: super::super::ClientResourceStatus) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        5, val.into()
      )
    }
  }

}  // impl DynamicScopedRouteConfigs

impl ::std::ops::Drop for DynamicScopedRouteConfigs {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DynamicScopedRouteConfigs {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DynamicScopedRouteConfigs {
  type Proxied = Self;
  fn as_view(&self) -> DynamicScopedRouteConfigsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DynamicScopedRouteConfigs {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DynamicScopedRouteConfigsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DynamicScopedRouteConfigs {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::scoped_routes_config_dump::envoy__admin__v3__ScopedRoutesConfigDump__DynamicScopedRouteConfigs_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1XG33.P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::scoped_routes_config_dump::envoy__admin__v3__ScopedRoutesConfigDump__DynamicScopedRouteConfigs_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::UpdateFailureState as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::scoped_routes_config_dump::envoy__admin__v3__ScopedRoutesConfigDump__DynamicScopedRouteConfigs_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DynamicScopedRouteConfigs {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DynamicScopedRouteConfigs {
  type Msg = DynamicScopedRouteConfigs;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicScopedRouteConfigs> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicScopedRouteConfigs {
  type Msg = DynamicScopedRouteConfigs;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicScopedRouteConfigs> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DynamicScopedRouteConfigsMut<'_> {
  type Msg = DynamicScopedRouteConfigs;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicScopedRouteConfigs> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicScopedRouteConfigsMut<'_> {
  type Msg = DynamicScopedRouteConfigs;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicScopedRouteConfigs> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicScopedRouteConfigsView<'_> {
  type Msg = DynamicScopedRouteConfigs;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicScopedRouteConfigs> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DynamicScopedRouteConfigsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod scoped_routes_config_dump


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__admin__v3__EndpointsConfigDump_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct EndpointsConfigDump {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<EndpointsConfigDump>
}

impl ::protobuf::Message for EndpointsConfigDump {
  type MessageView<'msg> = EndpointsConfigDumpView<'msg>;
  type MessageMut<'msg> = EndpointsConfigDumpMut<'msg>;
}

impl ::std::default::Default for EndpointsConfigDump {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for EndpointsConfigDump {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `EndpointsConfigDump` is `Sync` because it does not implement interior mutability.
//    Neither does `EndpointsConfigDumpMut`.
unsafe impl ::std::marker::Sync for EndpointsConfigDump {}

// SAFETY:
// - `EndpointsConfigDump` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for EndpointsConfigDump {}

impl ::protobuf::Proxied for EndpointsConfigDump {
  type View<'msg> = EndpointsConfigDumpView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for EndpointsConfigDump {}

impl ::protobuf::MutProxied for EndpointsConfigDump {
  type Mut<'msg> = EndpointsConfigDumpMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct EndpointsConfigDumpView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EndpointsConfigDump>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EndpointsConfigDumpView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for EndpointsConfigDumpView<'msg> {
  type Message = EndpointsConfigDump;
}

impl ::std::fmt::Debug for EndpointsConfigDumpView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for EndpointsConfigDumpView<'_> {
  fn default() -> EndpointsConfigDumpView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, EndpointsConfigDump>> for EndpointsConfigDumpView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EndpointsConfigDump>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EndpointsConfigDumpView<'msg> {

  pub fn to_owned(&self) -> EndpointsConfigDump {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // static_endpoint_configs: repeated message envoy.admin.v3.EndpointsConfigDump.StaticEndpointConfig
  pub fn static_endpoint_configs(self) -> ::protobuf::RepeatedView<'msg, super::endpoints_config_dump::StaticEndpointConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::endpoints_config_dump::StaticEndpointConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // dynamic_endpoint_configs: repeated message envoy.admin.v3.EndpointsConfigDump.DynamicEndpointConfig
  pub fn dynamic_endpoint_configs(self) -> ::protobuf::RepeatedView<'msg, super::endpoints_config_dump::DynamicEndpointConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::endpoints_config_dump::DynamicEndpointConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `EndpointsConfigDumpView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for EndpointsConfigDumpView<'_> {}

// SAFETY:
// - `EndpointsConfigDumpView` is `Send` because while its alive a `EndpointsConfigDumpMut` cannot.
// - `EndpointsConfigDumpView` does not use thread-local data.
unsafe impl ::std::marker::Send for EndpointsConfigDumpView<'_> {}

impl<'msg> ::protobuf::AsView for EndpointsConfigDumpView<'msg> {
  type Proxied = EndpointsConfigDump;
  fn as_view(&self) -> ::protobuf::View<'msg, EndpointsConfigDump> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EndpointsConfigDumpView<'msg> {
  fn into_view<'shorter>(self) -> EndpointsConfigDumpView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<EndpointsConfigDump> for EndpointsConfigDumpView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EndpointsConfigDump {
    let mut dst = EndpointsConfigDump::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<EndpointsConfigDump> for EndpointsConfigDumpMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EndpointsConfigDump {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for EndpointsConfigDump {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for EndpointsConfigDumpView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for EndpointsConfigDumpMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct EndpointsConfigDumpMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EndpointsConfigDump>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EndpointsConfigDumpMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for EndpointsConfigDumpMut<'msg> {
  type Message = EndpointsConfigDump;
}

impl ::std::fmt::Debug for EndpointsConfigDumpMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, EndpointsConfigDump>> for EndpointsConfigDumpMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EndpointsConfigDump>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EndpointsConfigDumpMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, EndpointsConfigDump> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> EndpointsConfigDump {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // static_endpoint_configs: repeated message envoy.admin.v3.EndpointsConfigDump.StaticEndpointConfig
  pub fn static_endpoint_configs(&self) -> ::protobuf::RepeatedView<'_, super::endpoints_config_dump::StaticEndpointConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::endpoints_config_dump::StaticEndpointConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn static_endpoint_configs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::endpoints_config_dump::StaticEndpointConfig> {
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
  pub fn set_static_endpoint_configs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::endpoints_config_dump::StaticEndpointConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // dynamic_endpoint_configs: repeated message envoy.admin.v3.EndpointsConfigDump.DynamicEndpointConfig
  pub fn dynamic_endpoint_configs(&self) -> ::protobuf::RepeatedView<'_, super::endpoints_config_dump::DynamicEndpointConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::endpoints_config_dump::DynamicEndpointConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn dynamic_endpoint_configs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::endpoints_config_dump::DynamicEndpointConfig> {
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
  pub fn set_dynamic_endpoint_configs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::endpoints_config_dump::DynamicEndpointConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `EndpointsConfigDumpMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for EndpointsConfigDumpMut<'_> {}

// SAFETY:
// - `EndpointsConfigDumpMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for EndpointsConfigDumpMut<'_> {}

impl<'msg> ::protobuf::AsView for EndpointsConfigDumpMut<'msg> {
  type Proxied = EndpointsConfigDump;
  fn as_view(&self) -> ::protobuf::View<'_, EndpointsConfigDump> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EndpointsConfigDumpMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, EndpointsConfigDump>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for EndpointsConfigDumpMut<'msg> {
  type MutProxied = EndpointsConfigDump;
  fn as_mut(&mut self) -> EndpointsConfigDumpMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for EndpointsConfigDumpMut<'msg> {
  fn into_mut<'shorter>(self) -> EndpointsConfigDumpMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl EndpointsConfigDump {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, EndpointsConfigDump> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> EndpointsConfigDumpView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> EndpointsConfigDumpMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // static_endpoint_configs: repeated message envoy.admin.v3.EndpointsConfigDump.StaticEndpointConfig
  pub fn static_endpoint_configs(&self) -> ::protobuf::RepeatedView<'_, super::endpoints_config_dump::StaticEndpointConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::endpoints_config_dump::StaticEndpointConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn static_endpoint_configs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::endpoints_config_dump::StaticEndpointConfig> {
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
  pub fn set_static_endpoint_configs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::endpoints_config_dump::StaticEndpointConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // dynamic_endpoint_configs: repeated message envoy.admin.v3.EndpointsConfigDump.DynamicEndpointConfig
  pub fn dynamic_endpoint_configs(&self) -> ::protobuf::RepeatedView<'_, super::endpoints_config_dump::DynamicEndpointConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::endpoints_config_dump::DynamicEndpointConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn dynamic_endpoint_configs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::endpoints_config_dump::DynamicEndpointConfig> {
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
  pub fn set_dynamic_endpoint_configs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::endpoints_config_dump::DynamicEndpointConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl EndpointsConfigDump

impl ::std::ops::Drop for EndpointsConfigDump {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for EndpointsConfigDump {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for EndpointsConfigDump {
  type Proxied = Self;
  fn as_view(&self) -> EndpointsConfigDumpView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for EndpointsConfigDump {
  type MutProxied = Self;
  fn as_mut(&mut self) -> EndpointsConfigDumpMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for EndpointsConfigDump {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__admin__v3__EndpointsConfigDump_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$aGG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__admin__v3__EndpointsConfigDump_msg_init.0, &[<super::endpoints_config_dump::StaticEndpointConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::endpoints_config_dump::DynamicEndpointConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__admin__v3__EndpointsConfigDump_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EndpointsConfigDump {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EndpointsConfigDump {
  type Msg = EndpointsConfigDump;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EndpointsConfigDump> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EndpointsConfigDump {
  type Msg = EndpointsConfigDump;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EndpointsConfigDump> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EndpointsConfigDumpMut<'_> {
  type Msg = EndpointsConfigDump;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EndpointsConfigDump> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EndpointsConfigDumpMut<'_> {
  type Msg = EndpointsConfigDump;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EndpointsConfigDump> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EndpointsConfigDumpView<'_> {
  type Msg = EndpointsConfigDump;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EndpointsConfigDump> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EndpointsConfigDumpMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod endpoints_config_dump {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__admin__v3__EndpointsConfigDump__StaticEndpointConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct StaticEndpointConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<StaticEndpointConfig>
}

impl ::protobuf::Message for StaticEndpointConfig {
  type MessageView<'msg> = StaticEndpointConfigView<'msg>;
  type MessageMut<'msg> = StaticEndpointConfigMut<'msg>;
}

impl ::std::default::Default for StaticEndpointConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for StaticEndpointConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `StaticEndpointConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `StaticEndpointConfigMut`.
unsafe impl ::std::marker::Sync for StaticEndpointConfig {}

// SAFETY:
// - `StaticEndpointConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for StaticEndpointConfig {}

impl ::protobuf::Proxied for StaticEndpointConfig {
  type View<'msg> = StaticEndpointConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for StaticEndpointConfig {}

impl ::protobuf::MutProxied for StaticEndpointConfig {
  type Mut<'msg> = StaticEndpointConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct StaticEndpointConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StaticEndpointConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StaticEndpointConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for StaticEndpointConfigView<'msg> {
  type Message = StaticEndpointConfig;
}

impl ::std::fmt::Debug for StaticEndpointConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for StaticEndpointConfigView<'_> {
  fn default() -> StaticEndpointConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, StaticEndpointConfig>> for StaticEndpointConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StaticEndpointConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StaticEndpointConfigView<'msg> {

  pub fn to_owned(&self) -> StaticEndpointConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // endpoint_config: optional message google.protobuf.Any
  pub fn has_endpoint_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn endpoint_config_opt(self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'msg>> {
    self.has_endpoint_config().then(|| self.endpoint_config())
  }
  pub fn endpoint_config(self) -> ::protobuf_well_known_types::AnyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn last_updated_opt(self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'msg>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

}

// SAFETY:
// - `StaticEndpointConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for StaticEndpointConfigView<'_> {}

// SAFETY:
// - `StaticEndpointConfigView` is `Send` because while its alive a `StaticEndpointConfigMut` cannot.
// - `StaticEndpointConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for StaticEndpointConfigView<'_> {}

impl<'msg> ::protobuf::AsView for StaticEndpointConfigView<'msg> {
  type Proxied = StaticEndpointConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, StaticEndpointConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StaticEndpointConfigView<'msg> {
  fn into_view<'shorter>(self) -> StaticEndpointConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<StaticEndpointConfig> for StaticEndpointConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StaticEndpointConfig {
    let mut dst = StaticEndpointConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<StaticEndpointConfig> for StaticEndpointConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StaticEndpointConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for StaticEndpointConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StaticEndpointConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StaticEndpointConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct StaticEndpointConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StaticEndpointConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StaticEndpointConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for StaticEndpointConfigMut<'msg> {
  type Message = StaticEndpointConfig;
}

impl ::std::fmt::Debug for StaticEndpointConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, StaticEndpointConfig>> for StaticEndpointConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StaticEndpointConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StaticEndpointConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, StaticEndpointConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> StaticEndpointConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // endpoint_config: optional message google.protobuf.Any
  pub fn has_endpoint_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_endpoint_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn endpoint_config_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_endpoint_config().then(|| self.endpoint_config())
  }
  pub fn endpoint_config(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn endpoint_config_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_endpoint_config(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_last_updated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn last_updated_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_updated_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_updated(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

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
// - `StaticEndpointConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for StaticEndpointConfigMut<'_> {}

// SAFETY:
// - `StaticEndpointConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for StaticEndpointConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for StaticEndpointConfigMut<'msg> {
  type Proxied = StaticEndpointConfig;
  fn as_view(&self) -> ::protobuf::View<'_, StaticEndpointConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StaticEndpointConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, StaticEndpointConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for StaticEndpointConfigMut<'msg> {
  type MutProxied = StaticEndpointConfig;
  fn as_mut(&mut self) -> StaticEndpointConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for StaticEndpointConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> StaticEndpointConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl StaticEndpointConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, StaticEndpointConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> StaticEndpointConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> StaticEndpointConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // endpoint_config: optional message google.protobuf.Any
  pub fn has_endpoint_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_endpoint_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn endpoint_config_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_endpoint_config().then(|| self.endpoint_config())
  }
  pub fn endpoint_config(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn endpoint_config_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_endpoint_config(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_last_updated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn last_updated_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_updated_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_updated(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl StaticEndpointConfig

impl ::std::ops::Drop for StaticEndpointConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for StaticEndpointConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for StaticEndpointConfig {
  type Proxied = Self;
  fn as_view(&self) -> StaticEndpointConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for StaticEndpointConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> StaticEndpointConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for StaticEndpointConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::endpoints_config_dump::envoy__admin__v3__EndpointsConfigDump__StaticEndpointConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::endpoints_config_dump::envoy__admin__v3__EndpointsConfigDump__StaticEndpointConfig_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::endpoints_config_dump::envoy__admin__v3__EndpointsConfigDump__StaticEndpointConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StaticEndpointConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StaticEndpointConfig {
  type Msg = StaticEndpointConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StaticEndpointConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StaticEndpointConfig {
  type Msg = StaticEndpointConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StaticEndpointConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StaticEndpointConfigMut<'_> {
  type Msg = StaticEndpointConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StaticEndpointConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StaticEndpointConfigMut<'_> {
  type Msg = StaticEndpointConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StaticEndpointConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StaticEndpointConfigView<'_> {
  type Msg = StaticEndpointConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StaticEndpointConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StaticEndpointConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__admin__v3__EndpointsConfigDump__DynamicEndpointConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DynamicEndpointConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DynamicEndpointConfig>
}

impl ::protobuf::Message for DynamicEndpointConfig {
  type MessageView<'msg> = DynamicEndpointConfigView<'msg>;
  type MessageMut<'msg> = DynamicEndpointConfigMut<'msg>;
}

impl ::std::default::Default for DynamicEndpointConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DynamicEndpointConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DynamicEndpointConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `DynamicEndpointConfigMut`.
unsafe impl ::std::marker::Sync for DynamicEndpointConfig {}

// SAFETY:
// - `DynamicEndpointConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DynamicEndpointConfig {}

impl ::protobuf::Proxied for DynamicEndpointConfig {
  type View<'msg> = DynamicEndpointConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DynamicEndpointConfig {}

impl ::protobuf::MutProxied for DynamicEndpointConfig {
  type Mut<'msg> = DynamicEndpointConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DynamicEndpointConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicEndpointConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DynamicEndpointConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DynamicEndpointConfigView<'msg> {
  type Message = DynamicEndpointConfig;
}

impl ::std::fmt::Debug for DynamicEndpointConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DynamicEndpointConfigView<'_> {
  fn default() -> DynamicEndpointConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicEndpointConfig>> for DynamicEndpointConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicEndpointConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DynamicEndpointConfigView<'msg> {

  pub fn to_owned(&self) -> DynamicEndpointConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // version_info: optional string
  pub fn version_info(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // endpoint_config: optional message google.protobuf.Any
  pub fn has_endpoint_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn endpoint_config_opt(self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'msg>> {
    self.has_endpoint_config().then(|| self.endpoint_config())
  }
  pub fn endpoint_config(self) -> ::protobuf_well_known_types::AnyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn last_updated_opt(self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'msg>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

  // error_state: optional message envoy.admin.v3.UpdateFailureState
  pub fn has_error_state(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn error_state_opt(self) -> ::std::option::Option<super::super::UpdateFailureStateView<'msg>> {
    self.has_error_state().then(|| self.error_state())
  }
  pub fn error_state(self) -> super::super::UpdateFailureStateView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::UpdateFailureStateView::default())
  }

  // client_status: optional enum envoy.admin.v3.ClientResourceStatus
  pub fn client_status(self) -> super::super::ClientResourceStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::super::ClientResourceStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `DynamicEndpointConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DynamicEndpointConfigView<'_> {}

// SAFETY:
// - `DynamicEndpointConfigView` is `Send` because while its alive a `DynamicEndpointConfigMut` cannot.
// - `DynamicEndpointConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for DynamicEndpointConfigView<'_> {}

impl<'msg> ::protobuf::AsView for DynamicEndpointConfigView<'msg> {
  type Proxied = DynamicEndpointConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, DynamicEndpointConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DynamicEndpointConfigView<'msg> {
  fn into_view<'shorter>(self) -> DynamicEndpointConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DynamicEndpointConfig> for DynamicEndpointConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DynamicEndpointConfig {
    let mut dst = DynamicEndpointConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DynamicEndpointConfig> for DynamicEndpointConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DynamicEndpointConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DynamicEndpointConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DynamicEndpointConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DynamicEndpointConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DynamicEndpointConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicEndpointConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DynamicEndpointConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DynamicEndpointConfigMut<'msg> {
  type Message = DynamicEndpointConfig;
}

impl ::std::fmt::Debug for DynamicEndpointConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicEndpointConfig>> for DynamicEndpointConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicEndpointConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DynamicEndpointConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicEndpointConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DynamicEndpointConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // version_info: optional string
  pub fn version_info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_version_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // endpoint_config: optional message google.protobuf.Any
  pub fn has_endpoint_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_endpoint_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn endpoint_config_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_endpoint_config().then(|| self.endpoint_config())
  }
  pub fn endpoint_config(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn endpoint_config_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_endpoint_config(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_last_updated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn last_updated_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_updated_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_updated(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // error_state: optional message envoy.admin.v3.UpdateFailureState
  pub fn has_error_state(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_error_state(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn error_state_opt(&self) -> ::std::option::Option<super::super::UpdateFailureStateView<'_>> {
    self.has_error_state().then(|| self.error_state())
  }
  pub fn error_state(&self) -> super::super::UpdateFailureStateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::UpdateFailureStateView::default())
  }
  pub fn error_state_mut(&mut self) -> super::super::UpdateFailureStateMut<'_> {
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
  pub fn set_error_state(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::UpdateFailureState>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // client_status: optional enum envoy.admin.v3.ClientResourceStatus
  pub fn client_status(&self) -> super::super::ClientResourceStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::super::ClientResourceStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_client_status(&mut self, val: super::super::ClientResourceStatus) {
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

}

// SAFETY:
// - `DynamicEndpointConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DynamicEndpointConfigMut<'_> {}

// SAFETY:
// - `DynamicEndpointConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DynamicEndpointConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for DynamicEndpointConfigMut<'msg> {
  type Proxied = DynamicEndpointConfig;
  fn as_view(&self) -> ::protobuf::View<'_, DynamicEndpointConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DynamicEndpointConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DynamicEndpointConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DynamicEndpointConfigMut<'msg> {
  type MutProxied = DynamicEndpointConfig;
  fn as_mut(&mut self) -> DynamicEndpointConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DynamicEndpointConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> DynamicEndpointConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DynamicEndpointConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DynamicEndpointConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DynamicEndpointConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DynamicEndpointConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // version_info: optional string
  pub fn version_info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_version_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // endpoint_config: optional message google.protobuf.Any
  pub fn has_endpoint_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_endpoint_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn endpoint_config_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_endpoint_config().then(|| self.endpoint_config())
  }
  pub fn endpoint_config(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn endpoint_config_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_endpoint_config(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_last_updated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn last_updated_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_updated_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_updated(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // error_state: optional message envoy.admin.v3.UpdateFailureState
  pub fn has_error_state(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_error_state(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn error_state_opt(&self) -> ::std::option::Option<super::super::UpdateFailureStateView<'_>> {
    self.has_error_state().then(|| self.error_state())
  }
  pub fn error_state(&self) -> super::super::UpdateFailureStateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::UpdateFailureStateView::default())
  }
  pub fn error_state_mut(&mut self) -> super::super::UpdateFailureStateMut<'_> {
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
  pub fn set_error_state(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::UpdateFailureState>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // client_status: optional enum envoy.admin.v3.ClientResourceStatus
  pub fn client_status(&self) -> super::super::ClientResourceStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::super::ClientResourceStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_client_status(&mut self, val: super::super::ClientResourceStatus) {
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

}  // impl DynamicEndpointConfig

impl ::std::ops::Drop for DynamicEndpointConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DynamicEndpointConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DynamicEndpointConfig {
  type Proxied = Self;
  fn as_view(&self) -> DynamicEndpointConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DynamicEndpointConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DynamicEndpointConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DynamicEndpointConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::endpoints_config_dump::envoy__admin__v3__EndpointsConfigDump__DynamicEndpointConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X333.P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::endpoints_config_dump::envoy__admin__v3__EndpointsConfigDump__DynamicEndpointConfig_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::UpdateFailureState as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::endpoints_config_dump::envoy__admin__v3__EndpointsConfigDump__DynamicEndpointConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DynamicEndpointConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DynamicEndpointConfig {
  type Msg = DynamicEndpointConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicEndpointConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicEndpointConfig {
  type Msg = DynamicEndpointConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicEndpointConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DynamicEndpointConfigMut<'_> {
  type Msg = DynamicEndpointConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicEndpointConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicEndpointConfigMut<'_> {
  type Msg = DynamicEndpointConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicEndpointConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicEndpointConfigView<'_> {
  type Msg = DynamicEndpointConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicEndpointConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DynamicEndpointConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod endpoints_config_dump


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__admin__v3__EcdsConfigDump_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct EcdsConfigDump {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<EcdsConfigDump>
}

impl ::protobuf::Message for EcdsConfigDump {
  type MessageView<'msg> = EcdsConfigDumpView<'msg>;
  type MessageMut<'msg> = EcdsConfigDumpMut<'msg>;
}

impl ::std::default::Default for EcdsConfigDump {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for EcdsConfigDump {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `EcdsConfigDump` is `Sync` because it does not implement interior mutability.
//    Neither does `EcdsConfigDumpMut`.
unsafe impl ::std::marker::Sync for EcdsConfigDump {}

// SAFETY:
// - `EcdsConfigDump` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for EcdsConfigDump {}

impl ::protobuf::Proxied for EcdsConfigDump {
  type View<'msg> = EcdsConfigDumpView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for EcdsConfigDump {}

impl ::protobuf::MutProxied for EcdsConfigDump {
  type Mut<'msg> = EcdsConfigDumpMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct EcdsConfigDumpView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EcdsConfigDump>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EcdsConfigDumpView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for EcdsConfigDumpView<'msg> {
  type Message = EcdsConfigDump;
}

impl ::std::fmt::Debug for EcdsConfigDumpView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for EcdsConfigDumpView<'_> {
  fn default() -> EcdsConfigDumpView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, EcdsConfigDump>> for EcdsConfigDumpView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EcdsConfigDump>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EcdsConfigDumpView<'msg> {

  pub fn to_owned(&self) -> EcdsConfigDump {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // ecds_filters: repeated message envoy.admin.v3.EcdsConfigDump.EcdsFilterConfig
  pub fn ecds_filters(self) -> ::protobuf::RepeatedView<'msg, super::ecds_config_dump::EcdsFilterConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ecds_config_dump::EcdsFilterConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `EcdsConfigDumpView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for EcdsConfigDumpView<'_> {}

// SAFETY:
// - `EcdsConfigDumpView` is `Send` because while its alive a `EcdsConfigDumpMut` cannot.
// - `EcdsConfigDumpView` does not use thread-local data.
unsafe impl ::std::marker::Send for EcdsConfigDumpView<'_> {}

impl<'msg> ::protobuf::AsView for EcdsConfigDumpView<'msg> {
  type Proxied = EcdsConfigDump;
  fn as_view(&self) -> ::protobuf::View<'msg, EcdsConfigDump> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EcdsConfigDumpView<'msg> {
  fn into_view<'shorter>(self) -> EcdsConfigDumpView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<EcdsConfigDump> for EcdsConfigDumpView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EcdsConfigDump {
    let mut dst = EcdsConfigDump::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<EcdsConfigDump> for EcdsConfigDumpMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EcdsConfigDump {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for EcdsConfigDump {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for EcdsConfigDumpView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for EcdsConfigDumpMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct EcdsConfigDumpMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EcdsConfigDump>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EcdsConfigDumpMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for EcdsConfigDumpMut<'msg> {
  type Message = EcdsConfigDump;
}

impl ::std::fmt::Debug for EcdsConfigDumpMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, EcdsConfigDump>> for EcdsConfigDumpMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EcdsConfigDump>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EcdsConfigDumpMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, EcdsConfigDump> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> EcdsConfigDump {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // ecds_filters: repeated message envoy.admin.v3.EcdsConfigDump.EcdsFilterConfig
  pub fn ecds_filters(&self) -> ::protobuf::RepeatedView<'_, super::ecds_config_dump::EcdsFilterConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ecds_config_dump::EcdsFilterConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn ecds_filters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ecds_config_dump::EcdsFilterConfig> {
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
  pub fn set_ecds_filters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ecds_config_dump::EcdsFilterConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `EcdsConfigDumpMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for EcdsConfigDumpMut<'_> {}

// SAFETY:
// - `EcdsConfigDumpMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for EcdsConfigDumpMut<'_> {}

impl<'msg> ::protobuf::AsView for EcdsConfigDumpMut<'msg> {
  type Proxied = EcdsConfigDump;
  fn as_view(&self) -> ::protobuf::View<'_, EcdsConfigDump> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EcdsConfigDumpMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, EcdsConfigDump>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for EcdsConfigDumpMut<'msg> {
  type MutProxied = EcdsConfigDump;
  fn as_mut(&mut self) -> EcdsConfigDumpMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for EcdsConfigDumpMut<'msg> {
  fn into_mut<'shorter>(self) -> EcdsConfigDumpMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl EcdsConfigDump {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, EcdsConfigDump> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> EcdsConfigDumpView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> EcdsConfigDumpMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // ecds_filters: repeated message envoy.admin.v3.EcdsConfigDump.EcdsFilterConfig
  pub fn ecds_filters(&self) -> ::protobuf::RepeatedView<'_, super::ecds_config_dump::EcdsFilterConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ecds_config_dump::EcdsFilterConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn ecds_filters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ecds_config_dump::EcdsFilterConfig> {
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
  pub fn set_ecds_filters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ecds_config_dump::EcdsFilterConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl EcdsConfigDump

impl ::std::ops::Drop for EcdsConfigDump {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for EcdsConfigDump {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for EcdsConfigDump {
  type Proxied = Self;
  fn as_view(&self) -> EcdsConfigDumpView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for EcdsConfigDump {
  type MutProxied = Self;
  fn as_mut(&mut self) -> EcdsConfigDumpMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for EcdsConfigDump {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__admin__v3__EcdsConfigDump_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__admin__v3__EcdsConfigDump_msg_init.0, &[<super::ecds_config_dump::EcdsFilterConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__admin__v3__EcdsConfigDump_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EcdsConfigDump {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EcdsConfigDump {
  type Msg = EcdsConfigDump;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EcdsConfigDump> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EcdsConfigDump {
  type Msg = EcdsConfigDump;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EcdsConfigDump> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EcdsConfigDumpMut<'_> {
  type Msg = EcdsConfigDump;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EcdsConfigDump> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EcdsConfigDumpMut<'_> {
  type Msg = EcdsConfigDump;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EcdsConfigDump> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EcdsConfigDumpView<'_> {
  type Msg = EcdsConfigDump;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EcdsConfigDump> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EcdsConfigDumpMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod ecds_config_dump {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__admin__v3__EcdsConfigDump__EcdsFilterConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct EcdsFilterConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<EcdsFilterConfig>
}

impl ::protobuf::Message for EcdsFilterConfig {
  type MessageView<'msg> = EcdsFilterConfigView<'msg>;
  type MessageMut<'msg> = EcdsFilterConfigMut<'msg>;
}

impl ::std::default::Default for EcdsFilterConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for EcdsFilterConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `EcdsFilterConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `EcdsFilterConfigMut`.
unsafe impl ::std::marker::Sync for EcdsFilterConfig {}

// SAFETY:
// - `EcdsFilterConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for EcdsFilterConfig {}

impl ::protobuf::Proxied for EcdsFilterConfig {
  type View<'msg> = EcdsFilterConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for EcdsFilterConfig {}

impl ::protobuf::MutProxied for EcdsFilterConfig {
  type Mut<'msg> = EcdsFilterConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct EcdsFilterConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EcdsFilterConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EcdsFilterConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for EcdsFilterConfigView<'msg> {
  type Message = EcdsFilterConfig;
}

impl ::std::fmt::Debug for EcdsFilterConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for EcdsFilterConfigView<'_> {
  fn default() -> EcdsFilterConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, EcdsFilterConfig>> for EcdsFilterConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EcdsFilterConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EcdsFilterConfigView<'msg> {

  pub fn to_owned(&self) -> EcdsFilterConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // version_info: optional string
  pub fn version_info(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // ecds_filter: optional message google.protobuf.Any
  pub fn has_ecds_filter(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn ecds_filter_opt(self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'msg>> {
    self.has_ecds_filter().then(|| self.ecds_filter())
  }
  pub fn ecds_filter(self) -> ::protobuf_well_known_types::AnyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn last_updated_opt(self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'msg>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

  // error_state: optional message envoy.admin.v3.UpdateFailureState
  pub fn has_error_state(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn error_state_opt(self) -> ::std::option::Option<super::super::UpdateFailureStateView<'msg>> {
    self.has_error_state().then(|| self.error_state())
  }
  pub fn error_state(self) -> super::super::UpdateFailureStateView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::UpdateFailureStateView::default())
  }

  // client_status: optional enum envoy.admin.v3.ClientResourceStatus
  pub fn client_status(self) -> super::super::ClientResourceStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::super::ClientResourceStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `EcdsFilterConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for EcdsFilterConfigView<'_> {}

// SAFETY:
// - `EcdsFilterConfigView` is `Send` because while its alive a `EcdsFilterConfigMut` cannot.
// - `EcdsFilterConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for EcdsFilterConfigView<'_> {}

impl<'msg> ::protobuf::AsView for EcdsFilterConfigView<'msg> {
  type Proxied = EcdsFilterConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, EcdsFilterConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EcdsFilterConfigView<'msg> {
  fn into_view<'shorter>(self) -> EcdsFilterConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<EcdsFilterConfig> for EcdsFilterConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EcdsFilterConfig {
    let mut dst = EcdsFilterConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<EcdsFilterConfig> for EcdsFilterConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EcdsFilterConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for EcdsFilterConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for EcdsFilterConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for EcdsFilterConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct EcdsFilterConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EcdsFilterConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EcdsFilterConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for EcdsFilterConfigMut<'msg> {
  type Message = EcdsFilterConfig;
}

impl ::std::fmt::Debug for EcdsFilterConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, EcdsFilterConfig>> for EcdsFilterConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EcdsFilterConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EcdsFilterConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, EcdsFilterConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> EcdsFilterConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // version_info: optional string
  pub fn version_info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_version_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // ecds_filter: optional message google.protobuf.Any
  pub fn has_ecds_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_ecds_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn ecds_filter_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_ecds_filter().then(|| self.ecds_filter())
  }
  pub fn ecds_filter(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn ecds_filter_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_ecds_filter(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_last_updated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn last_updated_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_updated_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_updated(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // error_state: optional message envoy.admin.v3.UpdateFailureState
  pub fn has_error_state(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_error_state(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn error_state_opt(&self) -> ::std::option::Option<super::super::UpdateFailureStateView<'_>> {
    self.has_error_state().then(|| self.error_state())
  }
  pub fn error_state(&self) -> super::super::UpdateFailureStateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::UpdateFailureStateView::default())
  }
  pub fn error_state_mut(&mut self) -> super::super::UpdateFailureStateMut<'_> {
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
  pub fn set_error_state(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::UpdateFailureState>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // client_status: optional enum envoy.admin.v3.ClientResourceStatus
  pub fn client_status(&self) -> super::super::ClientResourceStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::super::ClientResourceStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_client_status(&mut self, val: super::super::ClientResourceStatus) {
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

}

// SAFETY:
// - `EcdsFilterConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for EcdsFilterConfigMut<'_> {}

// SAFETY:
// - `EcdsFilterConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for EcdsFilterConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for EcdsFilterConfigMut<'msg> {
  type Proxied = EcdsFilterConfig;
  fn as_view(&self) -> ::protobuf::View<'_, EcdsFilterConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EcdsFilterConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, EcdsFilterConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for EcdsFilterConfigMut<'msg> {
  type MutProxied = EcdsFilterConfig;
  fn as_mut(&mut self) -> EcdsFilterConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for EcdsFilterConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> EcdsFilterConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl EcdsFilterConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, EcdsFilterConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> EcdsFilterConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> EcdsFilterConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // version_info: optional string
  pub fn version_info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_version_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // ecds_filter: optional message google.protobuf.Any
  pub fn has_ecds_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_ecds_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn ecds_filter_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_ecds_filter().then(|| self.ecds_filter())
  }
  pub fn ecds_filter(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn ecds_filter_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_ecds_filter(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_last_updated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn last_updated_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_updated_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_updated(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // error_state: optional message envoy.admin.v3.UpdateFailureState
  pub fn has_error_state(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_error_state(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn error_state_opt(&self) -> ::std::option::Option<super::super::UpdateFailureStateView<'_>> {
    self.has_error_state().then(|| self.error_state())
  }
  pub fn error_state(&self) -> super::super::UpdateFailureStateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::UpdateFailureStateView::default())
  }
  pub fn error_state_mut(&mut self) -> super::super::UpdateFailureStateMut<'_> {
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
  pub fn set_error_state(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::UpdateFailureState>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // client_status: optional enum envoy.admin.v3.ClientResourceStatus
  pub fn client_status(&self) -> super::super::ClientResourceStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::super::ClientResourceStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_client_status(&mut self, val: super::super::ClientResourceStatus) {
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

}  // impl EcdsFilterConfig

impl ::std::ops::Drop for EcdsFilterConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for EcdsFilterConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for EcdsFilterConfig {
  type Proxied = Self;
  fn as_view(&self) -> EcdsFilterConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for EcdsFilterConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> EcdsFilterConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for EcdsFilterConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::ecds_config_dump::envoy__admin__v3__EcdsConfigDump__EcdsFilterConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X333.P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::ecds_config_dump::envoy__admin__v3__EcdsConfigDump__EcdsFilterConfig_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::UpdateFailureState as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::ecds_config_dump::envoy__admin__v3__EcdsConfigDump__EcdsFilterConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EcdsFilterConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EcdsFilterConfig {
  type Msg = EcdsFilterConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EcdsFilterConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EcdsFilterConfig {
  type Msg = EcdsFilterConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EcdsFilterConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EcdsFilterConfigMut<'_> {
  type Msg = EcdsFilterConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EcdsFilterConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EcdsFilterConfigMut<'_> {
  type Msg = EcdsFilterConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EcdsFilterConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EcdsFilterConfigView<'_> {
  type Msg = EcdsFilterConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EcdsFilterConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EcdsFilterConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod ecds_config_dump


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClientResourceStatus(i32);

#[allow(non_upper_case_globals)]
impl ClientResourceStatus {
  pub const Unknown: ClientResourceStatus = ClientResourceStatus(0);
  pub const Requested: ClientResourceStatus = ClientResourceStatus(1);
  pub const DoesNotExist: ClientResourceStatus = ClientResourceStatus(2);
  pub const Acked: ClientResourceStatus = ClientResourceStatus(3);
  pub const Nacked: ClientResourceStatus = ClientResourceStatus(4);
  pub const ReceivedError: ClientResourceStatus = ClientResourceStatus(5);
  pub const Timeout: ClientResourceStatus = ClientResourceStatus(6);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unknown",
      1 => "Requested",
      2 => "DoesNotExist",
      3 => "Acked",
      4 => "Nacked",
      5 => "ReceivedError",
      6 => "Timeout",
      _ => return None
    })
  }
}

impl ::std::convert::From<ClientResourceStatus> for i32 {
  fn from(val: ClientResourceStatus) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for ClientResourceStatus {
  fn from(val: i32) -> ClientResourceStatus {
    Self(val)
  }
}

impl ::std::default::Default for ClientResourceStatus {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for ClientResourceStatus {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "ClientResourceStatus::{}", constant_name)
    } else {
      write!(f, "ClientResourceStatus::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for ClientResourceStatus {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for ClientResourceStatus {}

impl ::protobuf::Proxied for ClientResourceStatus {
  type View<'a> = ClientResourceStatus;
}

impl ::protobuf::AsView for ClientResourceStatus {
  type Proxied = ClientResourceStatus;

  fn as_view(&self) -> ClientResourceStatus {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClientResourceStatus {
  fn into_view<'shorter>(self) -> ClientResourceStatus where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for ClientResourceStatus {
  const NAME: &'static str = "ClientResourceStatus";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4|5|6)
  }
}

impl ::protobuf::__internal::EntityType for ClientResourceStatus {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


