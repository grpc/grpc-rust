const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__v3__TokenBucket_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TokenBucket {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TokenBucket>
}

impl ::protobuf::Message for TokenBucket {
  type MessageView<'msg> = TokenBucketView<'msg>;
  type MessageMut<'msg> = TokenBucketMut<'msg>;
}

impl ::std::default::Default for TokenBucket {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TokenBucket {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TokenBucket` is `Sync` because it does not implement interior mutability.
//    Neither does `TokenBucketMut`.
unsafe impl ::std::marker::Sync for TokenBucket {}

// SAFETY:
// - `TokenBucket` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TokenBucket {}

impl ::protobuf::Proxied for TokenBucket {
  type View<'msg> = TokenBucketView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TokenBucket {}

impl ::protobuf::MutProxied for TokenBucket {
  type Mut<'msg> = TokenBucketMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TokenBucketView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TokenBucket>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TokenBucketView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TokenBucketView<'msg> {
  type Message = TokenBucket;
}

impl ::std::fmt::Debug for TokenBucketView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TokenBucketView<'_> {
  fn default() -> TokenBucketView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TokenBucket>> for TokenBucketView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TokenBucket>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TokenBucketView<'msg> {

  pub fn to_owned(&self) -> TokenBucket {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // max_tokens: optional uint32
  pub fn max_tokens(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // tokens_per_fill: optional message google.protobuf.UInt32Value
  pub fn has_tokens_per_fill(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn tokens_per_fill_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_tokens_per_fill().then(|| self.tokens_per_fill())
  }
  pub fn tokens_per_fill(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // fill_interval: optional message google.protobuf.Duration
  pub fn has_fill_interval(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn fill_interval_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_fill_interval().then(|| self.fill_interval())
  }
  pub fn fill_interval(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

}

// SAFETY:
// - `TokenBucketView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TokenBucketView<'_> {}

// SAFETY:
// - `TokenBucketView` is `Send` because while its alive a `TokenBucketMut` cannot.
// - `TokenBucketView` does not use thread-local data.
unsafe impl ::std::marker::Send for TokenBucketView<'_> {}

impl<'msg> ::protobuf::AsView for TokenBucketView<'msg> {
  type Proxied = TokenBucket;
  fn as_view(&self) -> ::protobuf::View<'msg, TokenBucket> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TokenBucketView<'msg> {
  fn into_view<'shorter>(self) -> TokenBucketView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TokenBucket> for TokenBucketView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TokenBucket {
    let mut dst = TokenBucket::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TokenBucket> for TokenBucketMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TokenBucket {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TokenBucket {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TokenBucketView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TokenBucketMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TokenBucketMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TokenBucket>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TokenBucketMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TokenBucketMut<'msg> {
  type Message = TokenBucket;
}

impl ::std::fmt::Debug for TokenBucketMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TokenBucket>> for TokenBucketMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TokenBucket>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TokenBucketMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TokenBucket> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TokenBucket {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // max_tokens: optional uint32
  pub fn max_tokens(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_max_tokens(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        0, val.into()
      )
    }
  }

  // tokens_per_fill: optional message google.protobuf.UInt32Value
  pub fn has_tokens_per_fill(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_tokens_per_fill(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn tokens_per_fill_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_tokens_per_fill().then(|| self.tokens_per_fill())
  }
  pub fn tokens_per_fill(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn tokens_per_fill_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_tokens_per_fill(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // fill_interval: optional message google.protobuf.Duration
  pub fn has_fill_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_fill_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn fill_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_fill_interval().then(|| self.fill_interval())
  }
  pub fn fill_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn fill_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_fill_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

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
// - `TokenBucketMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TokenBucketMut<'_> {}

// SAFETY:
// - `TokenBucketMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TokenBucketMut<'_> {}

impl<'msg> ::protobuf::AsView for TokenBucketMut<'msg> {
  type Proxied = TokenBucket;
  fn as_view(&self) -> ::protobuf::View<'_, TokenBucket> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TokenBucketMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TokenBucket>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TokenBucketMut<'msg> {
  type MutProxied = TokenBucket;
  fn as_mut(&mut self) -> TokenBucketMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TokenBucketMut<'msg> {
  fn into_mut<'shorter>(self) -> TokenBucketMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TokenBucket {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TokenBucket> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TokenBucketView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TokenBucketMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // max_tokens: optional uint32
  pub fn max_tokens(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_max_tokens(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        0, val.into()
      )
    }
  }

  // tokens_per_fill: optional message google.protobuf.UInt32Value
  pub fn has_tokens_per_fill(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_tokens_per_fill(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn tokens_per_fill_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_tokens_per_fill().then(|| self.tokens_per_fill())
  }
  pub fn tokens_per_fill(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn tokens_per_fill_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_tokens_per_fill(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // fill_interval: optional message google.protobuf.Duration
  pub fn has_fill_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_fill_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn fill_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_fill_interval().then(|| self.fill_interval())
  }
  pub fn fill_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn fill_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_fill_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl TokenBucket

impl ::std::ops::Drop for TokenBucket {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TokenBucket {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TokenBucket {
  type Proxied = Self;
  fn as_view(&self) -> TokenBucketView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TokenBucket {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TokenBucketMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TokenBucket {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__type__v3__TokenBucket_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$)P33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__v3__TokenBucket_msg_init.0, &[<::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__v3__TokenBucket_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TokenBucket {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TokenBucket {
  type Msg = TokenBucket;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TokenBucket> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TokenBucket {
  type Msg = TokenBucket;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TokenBucket> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TokenBucketMut<'_> {
  type Msg = TokenBucket;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TokenBucket> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TokenBucketMut<'_> {
  type Msg = TokenBucket;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TokenBucket> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TokenBucketView<'_> {
  type Msg = TokenBucket;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TokenBucket> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TokenBucketMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



