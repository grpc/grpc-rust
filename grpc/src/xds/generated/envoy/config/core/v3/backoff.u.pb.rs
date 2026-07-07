const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__BackoffStrategy_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct BackoffStrategy {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<BackoffStrategy>
}

impl ::protobuf::Message for BackoffStrategy {
  type MessageView<'msg> = BackoffStrategyView<'msg>;
  type MessageMut<'msg> = BackoffStrategyMut<'msg>;
}

impl ::std::default::Default for BackoffStrategy {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for BackoffStrategy {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `BackoffStrategy` is `Sync` because it does not implement interior mutability.
//    Neither does `BackoffStrategyMut`.
unsafe impl ::std::marker::Sync for BackoffStrategy {}

// SAFETY:
// - `BackoffStrategy` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for BackoffStrategy {}

impl ::protobuf::Proxied for BackoffStrategy {
  type View<'msg> = BackoffStrategyView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for BackoffStrategy {}

impl ::protobuf::MutProxied for BackoffStrategy {
  type Mut<'msg> = BackoffStrategyMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct BackoffStrategyView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BackoffStrategy>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BackoffStrategyView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for BackoffStrategyView<'msg> {
  type Message = BackoffStrategy;
}

impl ::std::fmt::Debug for BackoffStrategyView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for BackoffStrategyView<'_> {
  fn default() -> BackoffStrategyView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, BackoffStrategy>> for BackoffStrategyView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BackoffStrategy>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BackoffStrategyView<'msg> {

  pub fn to_owned(&self) -> BackoffStrategy {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // base_interval: optional message google.protobuf.Duration
  pub fn has_base_interval(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn base_interval_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_base_interval().then(|| self.base_interval())
  }
  pub fn base_interval(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // max_interval: optional message google.protobuf.Duration
  pub fn has_max_interval(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn max_interval_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_max_interval().then(|| self.max_interval())
  }
  pub fn max_interval(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

}

// SAFETY:
// - `BackoffStrategyView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for BackoffStrategyView<'_> {}

// SAFETY:
// - `BackoffStrategyView` is `Send` because while its alive a `BackoffStrategyMut` cannot.
// - `BackoffStrategyView` does not use thread-local data.
unsafe impl ::std::marker::Send for BackoffStrategyView<'_> {}

impl<'msg> ::protobuf::AsView for BackoffStrategyView<'msg> {
  type Proxied = BackoffStrategy;
  fn as_view(&self) -> ::protobuf::View<'msg, BackoffStrategy> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BackoffStrategyView<'msg> {
  fn into_view<'shorter>(self) -> BackoffStrategyView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<BackoffStrategy> for BackoffStrategyView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BackoffStrategy {
    let mut dst = BackoffStrategy::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<BackoffStrategy> for BackoffStrategyMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BackoffStrategy {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for BackoffStrategy {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BackoffStrategyView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BackoffStrategyMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct BackoffStrategyMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BackoffStrategy>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BackoffStrategyMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for BackoffStrategyMut<'msg> {
  type Message = BackoffStrategy;
}

impl ::std::fmt::Debug for BackoffStrategyMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, BackoffStrategy>> for BackoffStrategyMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BackoffStrategy>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BackoffStrategyMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, BackoffStrategy> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> BackoffStrategy {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // base_interval: optional message google.protobuf.Duration
  pub fn has_base_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_base_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn base_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_base_interval().then(|| self.base_interval())
  }
  pub fn base_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn base_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_base_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // max_interval: optional message google.protobuf.Duration
  pub fn has_max_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_max_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn max_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_max_interval().then(|| self.max_interval())
  }
  pub fn max_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn max_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_max_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

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
// - `BackoffStrategyMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for BackoffStrategyMut<'_> {}

// SAFETY:
// - `BackoffStrategyMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for BackoffStrategyMut<'_> {}

impl<'msg> ::protobuf::AsView for BackoffStrategyMut<'msg> {
  type Proxied = BackoffStrategy;
  fn as_view(&self) -> ::protobuf::View<'_, BackoffStrategy> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BackoffStrategyMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, BackoffStrategy>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for BackoffStrategyMut<'msg> {
  type MutProxied = BackoffStrategy;
  fn as_mut(&mut self) -> BackoffStrategyMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for BackoffStrategyMut<'msg> {
  fn into_mut<'shorter>(self) -> BackoffStrategyMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl BackoffStrategy {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, BackoffStrategy> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> BackoffStrategyView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> BackoffStrategyMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // base_interval: optional message google.protobuf.Duration
  pub fn has_base_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_base_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn base_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_base_interval().then(|| self.base_interval())
  }
  pub fn base_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn base_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_base_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // max_interval: optional message google.protobuf.Duration
  pub fn has_max_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_max_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn max_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_max_interval().then(|| self.max_interval())
  }
  pub fn max_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn max_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_max_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl BackoffStrategy

impl ::std::ops::Drop for BackoffStrategy {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for BackoffStrategy {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for BackoffStrategy {
  type Proxied = Self;
  fn as_view(&self) -> BackoffStrategyView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for BackoffStrategy {
  type MutProxied = Self;
  fn as_mut(&mut self) -> BackoffStrategyMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for BackoffStrategy {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__BackoffStrategy_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__BackoffStrategy_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__BackoffStrategy_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BackoffStrategy {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BackoffStrategy {
  type Msg = BackoffStrategy;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BackoffStrategy> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BackoffStrategy {
  type Msg = BackoffStrategy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BackoffStrategy> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BackoffStrategyMut<'_> {
  type Msg = BackoffStrategy;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BackoffStrategy> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BackoffStrategyMut<'_> {
  type Msg = BackoffStrategy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BackoffStrategy> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BackoffStrategyView<'_> {
  type Msg = BackoffStrategy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BackoffStrategy> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BackoffStrategyMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



