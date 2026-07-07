const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut xds__core__v3__CollectionEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CollectionEntry {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CollectionEntry>
}

impl ::protobuf::Message for CollectionEntry {
  type MessageView<'msg> = CollectionEntryView<'msg>;
  type MessageMut<'msg> = CollectionEntryMut<'msg>;
}

impl ::std::default::Default for CollectionEntry {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CollectionEntry {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CollectionEntry` is `Sync` because it does not implement interior mutability.
//    Neither does `CollectionEntryMut`.
unsafe impl ::std::marker::Sync for CollectionEntry {}

// SAFETY:
// - `CollectionEntry` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CollectionEntry {}

impl ::protobuf::Proxied for CollectionEntry {
  type View<'msg> = CollectionEntryView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CollectionEntry {}

impl ::protobuf::MutProxied for CollectionEntry {
  type Mut<'msg> = CollectionEntryMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CollectionEntryView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CollectionEntry>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CollectionEntryView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CollectionEntryView<'msg> {
  type Message = CollectionEntry;
}

impl ::std::fmt::Debug for CollectionEntryView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CollectionEntryView<'_> {
  fn default() -> CollectionEntryView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CollectionEntry>> for CollectionEntryView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CollectionEntry>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CollectionEntryView<'msg> {

  pub fn to_owned(&self) -> CollectionEntry {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // locator: optional message xds.core.v3.ResourceLocator
  pub fn has_locator(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn locator_opt(self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::resource_locator::ResourceLocatorView<'msg>> {
    self.has_locator().then(|| self.locator())
  }
  pub fn locator(self) -> crate::xds::generated::xds::core::v3::resource_locator::ResourceLocatorView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::resource_locator::ResourceLocatorView::default())
  }

  // inline_entry: optional message xds.core.v3.CollectionEntry.InlineEntry
  pub fn has_inline_entry(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn inline_entry_opt(self) -> ::std::option::Option<super::collection_entry::InlineEntryView<'msg>> {
    self.has_inline_entry().then(|| self.inline_entry())
  }
  pub fn inline_entry(self) -> super::collection_entry::InlineEntryView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::collection_entry::InlineEntryView::default())
  }

  pub fn resource_specifier(self) -> super::collection_entry::ResourceSpecifierOneof<'msg> {
    match self.resource_specifier_case() {
      super::collection_entry::ResourceSpecifierCase::Locator =>
          super::collection_entry::ResourceSpecifierOneof::Locator(self.locator()),
      super::collection_entry::ResourceSpecifierCase::InlineEntry =>
          super::collection_entry::ResourceSpecifierOneof::InlineEntry(self.inline_entry()),
      _ => super::collection_entry::ResourceSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn resource_specifier_case(self) -> super::collection_entry::ResourceSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::collection_entry::ResourceSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `CollectionEntryView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CollectionEntryView<'_> {}

// SAFETY:
// - `CollectionEntryView` is `Send` because while its alive a `CollectionEntryMut` cannot.
// - `CollectionEntryView` does not use thread-local data.
unsafe impl ::std::marker::Send for CollectionEntryView<'_> {}

impl<'msg> ::protobuf::AsView for CollectionEntryView<'msg> {
  type Proxied = CollectionEntry;
  fn as_view(&self) -> ::protobuf::View<'msg, CollectionEntry> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CollectionEntryView<'msg> {
  fn into_view<'shorter>(self) -> CollectionEntryView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CollectionEntry> for CollectionEntryView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CollectionEntry {
    let mut dst = CollectionEntry::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CollectionEntry> for CollectionEntryMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CollectionEntry {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CollectionEntry {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CollectionEntryView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CollectionEntryMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CollectionEntryMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CollectionEntry>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CollectionEntryMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CollectionEntryMut<'msg> {
  type Message = CollectionEntry;
}

impl ::std::fmt::Debug for CollectionEntryMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CollectionEntry>> for CollectionEntryMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CollectionEntry>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CollectionEntryMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CollectionEntry> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CollectionEntry {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // locator: optional message xds.core.v3.ResourceLocator
  pub fn has_locator(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_locator(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn locator_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::resource_locator::ResourceLocatorView<'_>> {
    self.has_locator().then(|| self.locator())
  }
  pub fn locator(&self) -> crate::xds::generated::xds::core::v3::resource_locator::ResourceLocatorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::resource_locator::ResourceLocatorView::default())
  }
  pub fn locator_mut(&mut self) -> crate::xds::generated::xds::core::v3::resource_locator::ResourceLocatorMut<'_> {
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
  pub fn set_locator(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::core::v3::resource_locator::ResourceLocator>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // inline_entry: optional message xds.core.v3.CollectionEntry.InlineEntry
  pub fn has_inline_entry(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_inline_entry(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn inline_entry_opt(&self) -> ::std::option::Option<super::collection_entry::InlineEntryView<'_>> {
    self.has_inline_entry().then(|| self.inline_entry())
  }
  pub fn inline_entry(&self) -> super::collection_entry::InlineEntryView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::collection_entry::InlineEntryView::default())
  }
  pub fn inline_entry_mut(&mut self) -> super::collection_entry::InlineEntryMut<'_> {
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
  pub fn set_inline_entry(&mut self,
    val: impl ::protobuf::IntoProxied<super::collection_entry::InlineEntry>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn resource_specifier(&self) -> super::collection_entry::ResourceSpecifierOneof<'_> {
    match &self.resource_specifier_case() {
      super::collection_entry::ResourceSpecifierCase::Locator =>
          super::collection_entry::ResourceSpecifierOneof::Locator(self.locator()),
      super::collection_entry::ResourceSpecifierCase::InlineEntry =>
          super::collection_entry::ResourceSpecifierOneof::InlineEntry(self.inline_entry()),
      _ => super::collection_entry::ResourceSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn resource_specifier_case(&self) -> super::collection_entry::ResourceSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::collection_entry::ResourceSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `CollectionEntryMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CollectionEntryMut<'_> {}

// SAFETY:
// - `CollectionEntryMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CollectionEntryMut<'_> {}

impl<'msg> ::protobuf::AsView for CollectionEntryMut<'msg> {
  type Proxied = CollectionEntry;
  fn as_view(&self) -> ::protobuf::View<'_, CollectionEntry> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CollectionEntryMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CollectionEntry>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CollectionEntryMut<'msg> {
  type MutProxied = CollectionEntry;
  fn as_mut(&mut self) -> CollectionEntryMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CollectionEntryMut<'msg> {
  fn into_mut<'shorter>(self) -> CollectionEntryMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CollectionEntry {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CollectionEntry> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CollectionEntryView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CollectionEntryMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // locator: optional message xds.core.v3.ResourceLocator
  pub fn has_locator(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_locator(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn locator_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::resource_locator::ResourceLocatorView<'_>> {
    self.has_locator().then(|| self.locator())
  }
  pub fn locator(&self) -> crate::xds::generated::xds::core::v3::resource_locator::ResourceLocatorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::resource_locator::ResourceLocatorView::default())
  }
  pub fn locator_mut(&mut self) -> crate::xds::generated::xds::core::v3::resource_locator::ResourceLocatorMut<'_> {
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
  pub fn set_locator(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::core::v3::resource_locator::ResourceLocator>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // inline_entry: optional message xds.core.v3.CollectionEntry.InlineEntry
  pub fn has_inline_entry(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_inline_entry(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn inline_entry_opt(&self) -> ::std::option::Option<super::collection_entry::InlineEntryView<'_>> {
    self.has_inline_entry().then(|| self.inline_entry())
  }
  pub fn inline_entry(&self) -> super::collection_entry::InlineEntryView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::collection_entry::InlineEntryView::default())
  }
  pub fn inline_entry_mut(&mut self) -> super::collection_entry::InlineEntryMut<'_> {
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
  pub fn set_inline_entry(&mut self,
    val: impl ::protobuf::IntoProxied<super::collection_entry::InlineEntry>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn resource_specifier(&self) -> super::collection_entry::ResourceSpecifierOneof<'_> {
    match &self.resource_specifier_case() {
      super::collection_entry::ResourceSpecifierCase::Locator =>
          super::collection_entry::ResourceSpecifierOneof::Locator(self.locator()),
      super::collection_entry::ResourceSpecifierCase::InlineEntry =>
          super::collection_entry::ResourceSpecifierOneof::InlineEntry(self.inline_entry()),
      _ => super::collection_entry::ResourceSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn resource_specifier_case(&self) -> super::collection_entry::ResourceSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::collection_entry::ResourceSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl CollectionEntry

impl ::std::ops::Drop for CollectionEntry {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CollectionEntry {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CollectionEntry {
  type Proxied = Self;
  fn as_view(&self) -> CollectionEntryView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CollectionEntry {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CollectionEntryMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CollectionEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::xds__core__v3__CollectionEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33^!|#");
        ::protobuf::__internal::runtime::link_mini_table(
            super::xds__core__v3__CollectionEntry_msg_init.0, &[<crate::xds::generated::xds::core::v3::resource_locator::ResourceLocator as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::collection_entry::InlineEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::xds__core__v3__CollectionEntry_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CollectionEntry {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CollectionEntry {
  type Msg = CollectionEntry;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CollectionEntry> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CollectionEntry {
  type Msg = CollectionEntry;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CollectionEntry> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CollectionEntryMut<'_> {
  type Msg = CollectionEntry;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CollectionEntry> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CollectionEntryMut<'_> {
  type Msg = CollectionEntry;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CollectionEntry> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CollectionEntryView<'_> {
  type Msg = CollectionEntry;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CollectionEntry> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CollectionEntryMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod collection_entry {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut xds__core__v3__CollectionEntry__InlineEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct InlineEntry {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<InlineEntry>
}

impl ::protobuf::Message for InlineEntry {
  type MessageView<'msg> = InlineEntryView<'msg>;
  type MessageMut<'msg> = InlineEntryMut<'msg>;
}

impl ::std::default::Default for InlineEntry {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for InlineEntry {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `InlineEntry` is `Sync` because it does not implement interior mutability.
//    Neither does `InlineEntryMut`.
unsafe impl ::std::marker::Sync for InlineEntry {}

// SAFETY:
// - `InlineEntry` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for InlineEntry {}

impl ::protobuf::Proxied for InlineEntry {
  type View<'msg> = InlineEntryView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for InlineEntry {}

impl ::protobuf::MutProxied for InlineEntry {
  type Mut<'msg> = InlineEntryMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct InlineEntryView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, InlineEntry>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for InlineEntryView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for InlineEntryView<'msg> {
  type Message = InlineEntry;
}

impl ::std::fmt::Debug for InlineEntryView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for InlineEntryView<'_> {
  fn default() -> InlineEntryView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, InlineEntry>> for InlineEntryView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, InlineEntry>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> InlineEntryView<'msg> {

  pub fn to_owned(&self) -> InlineEntry {
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

  // version: optional string
  pub fn version(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // resource: optional message google.protobuf.Any
  pub fn has_resource(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn resource_opt(self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'msg>> {
    self.has_resource().then(|| self.resource())
  }
  pub fn resource(self) -> ::protobuf_well_known_types::AnyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }

}

// SAFETY:
// - `InlineEntryView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for InlineEntryView<'_> {}

// SAFETY:
// - `InlineEntryView` is `Send` because while its alive a `InlineEntryMut` cannot.
// - `InlineEntryView` does not use thread-local data.
unsafe impl ::std::marker::Send for InlineEntryView<'_> {}

impl<'msg> ::protobuf::AsView for InlineEntryView<'msg> {
  type Proxied = InlineEntry;
  fn as_view(&self) -> ::protobuf::View<'msg, InlineEntry> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for InlineEntryView<'msg> {
  fn into_view<'shorter>(self) -> InlineEntryView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<InlineEntry> for InlineEntryView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> InlineEntry {
    let mut dst = InlineEntry::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<InlineEntry> for InlineEntryMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> InlineEntry {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for InlineEntry {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for InlineEntryView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for InlineEntryMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct InlineEntryMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, InlineEntry>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for InlineEntryMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for InlineEntryMut<'msg> {
  type Message = InlineEntry;
}

impl ::std::fmt::Debug for InlineEntryMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, InlineEntry>> for InlineEntryMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, InlineEntry>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> InlineEntryMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, InlineEntry> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> InlineEntry {
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

  // version: optional string
  pub fn version(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_version(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // resource: optional message google.protobuf.Any
  pub fn has_resource(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_resource(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn resource_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_resource().then(|| self.resource())
  }
  pub fn resource(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn resource_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_resource(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

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
// - `InlineEntryMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for InlineEntryMut<'_> {}

// SAFETY:
// - `InlineEntryMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for InlineEntryMut<'_> {}

impl<'msg> ::protobuf::AsView for InlineEntryMut<'msg> {
  type Proxied = InlineEntry;
  fn as_view(&self) -> ::protobuf::View<'_, InlineEntry> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for InlineEntryMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, InlineEntry>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for InlineEntryMut<'msg> {
  type MutProxied = InlineEntry;
  fn as_mut(&mut self) -> InlineEntryMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for InlineEntryMut<'msg> {
  fn into_mut<'shorter>(self) -> InlineEntryMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl InlineEntry {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, InlineEntry> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> InlineEntryView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> InlineEntryMut<'_> {
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

  // version: optional string
  pub fn version(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_version(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // resource: optional message google.protobuf.Any
  pub fn has_resource(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_resource(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn resource_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_resource().then(|| self.resource())
  }
  pub fn resource(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn resource_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_resource(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl InlineEntry

impl ::std::ops::Drop for InlineEntry {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for InlineEntry {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for InlineEntry {
  type Proxied = Self;
  fn as_view(&self) -> InlineEntryView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for InlineEntry {
  type MutProxied = Self;
  fn as_mut(&mut self) -> InlineEntryMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for InlineEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::collection_entry::xds__core__v3__CollectionEntry__InlineEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::collection_entry::xds__core__v3__CollectionEntry__InlineEntry_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::collection_entry::xds__core__v3__CollectionEntry__InlineEntry_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for InlineEntry {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for InlineEntry {
  type Msg = InlineEntry;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<InlineEntry> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for InlineEntry {
  type Msg = InlineEntry;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<InlineEntry> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for InlineEntryMut<'_> {
  type Msg = InlineEntry;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<InlineEntry> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for InlineEntryMut<'_> {
  type Msg = InlineEntry;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<InlineEntry> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for InlineEntryView<'_> {
  type Msg = InlineEntry;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<InlineEntry> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for InlineEntryMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ResourceSpecifierOneof<'msg> {
  Locator(::protobuf::View<'msg, crate::xds::generated::xds::core::v3::resource_locator::ResourceLocator>) = 1,
  InlineEntry(::protobuf::View<'msg, super::super::collection_entry::InlineEntry>) = 2,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ResourceSpecifierCase {
  Locator = 1,
  InlineEntry = 2,

  not_set = 0
}

impl ResourceSpecifierCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ResourceSpecifierCase> {
    match v {
      0 => Some(ResourceSpecifierCase::not_set),
      1 => Some(ResourceSpecifierCase::Locator),
      2 => Some(ResourceSpecifierCase::InlineEntry),
      _ => None
    }
  }
}
}  // pub mod collection_entry


