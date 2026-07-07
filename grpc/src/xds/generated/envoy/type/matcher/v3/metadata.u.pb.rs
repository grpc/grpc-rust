const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__matcher__v3__MetadataMatcher_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MetadataMatcher {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MetadataMatcher>
}

impl ::protobuf::Message for MetadataMatcher {
  type MessageView<'msg> = MetadataMatcherView<'msg>;
  type MessageMut<'msg> = MetadataMatcherMut<'msg>;
}

impl ::std::default::Default for MetadataMatcher {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MetadataMatcher {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MetadataMatcher` is `Sync` because it does not implement interior mutability.
//    Neither does `MetadataMatcherMut`.
unsafe impl ::std::marker::Sync for MetadataMatcher {}

// SAFETY:
// - `MetadataMatcher` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for MetadataMatcher {}

impl ::protobuf::Proxied for MetadataMatcher {
  type View<'msg> = MetadataMatcherView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MetadataMatcher {}

impl ::protobuf::MutProxied for MetadataMatcher {
  type Mut<'msg> = MetadataMatcherMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MetadataMatcherView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MetadataMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MetadataMatcherView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MetadataMatcherView<'msg> {
  type Message = MetadataMatcher;
}

impl ::std::fmt::Debug for MetadataMatcherView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MetadataMatcherView<'_> {
  fn default() -> MetadataMatcherView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MetadataMatcher>> for MetadataMatcherView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MetadataMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MetadataMatcherView<'msg> {

  pub fn to_owned(&self) -> MetadataMatcher {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // filter: optional string
  pub fn filter(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // path: repeated message envoy.type.matcher.v3.MetadataMatcher.PathSegment
  pub fn path(self) -> ::protobuf::RepeatedView<'msg, super::metadata_matcher::PathSegment> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::metadata_matcher::PathSegment>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // value: optional message envoy.type.matcher.v3.ValueMatcher
  pub fn has_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn value_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::value::ValueMatcherView<'msg>> {
    self.has_value().then(|| self.value())
  }
  pub fn value(self) -> crate::xds::generated::envoy::r#type::matcher::v3::value::ValueMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::value::ValueMatcherView::default())
  }

  // invert: optional bool
  pub fn invert(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `MetadataMatcherView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for MetadataMatcherView<'_> {}

// SAFETY:
// - `MetadataMatcherView` is `Send` because while its alive a `MetadataMatcherMut` cannot.
// - `MetadataMatcherView` does not use thread-local data.
unsafe impl ::std::marker::Send for MetadataMatcherView<'_> {}

impl<'msg> ::protobuf::AsView for MetadataMatcherView<'msg> {
  type Proxied = MetadataMatcher;
  fn as_view(&self) -> ::protobuf::View<'msg, MetadataMatcher> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MetadataMatcherView<'msg> {
  fn into_view<'shorter>(self) -> MetadataMatcherView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MetadataMatcher> for MetadataMatcherView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MetadataMatcher {
    let mut dst = MetadataMatcher::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MetadataMatcher> for MetadataMatcherMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MetadataMatcher {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for MetadataMatcher {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MetadataMatcherView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MetadataMatcherMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MetadataMatcherMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MetadataMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MetadataMatcherMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MetadataMatcherMut<'msg> {
  type Message = MetadataMatcher;
}

impl ::std::fmt::Debug for MetadataMatcherMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MetadataMatcher>> for MetadataMatcherMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MetadataMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MetadataMatcherMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MetadataMatcher> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> MetadataMatcher {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // filter: optional string
  pub fn filter(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_filter(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // path: repeated message envoy.type.matcher.v3.MetadataMatcher.PathSegment
  pub fn path(&self) -> ::protobuf::RepeatedView<'_, super::metadata_matcher::PathSegment> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::metadata_matcher::PathSegment>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn path_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::metadata_matcher::PathSegment> {
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
  pub fn set_path(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::metadata_matcher::PathSegment>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // value: optional message envoy.type.matcher.v3.ValueMatcher
  pub fn has_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn value_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::value::ValueMatcherView<'_>> {
    self.has_value().then(|| self.value())
  }
  pub fn value(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::value::ValueMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::value::ValueMatcherView::default())
  }
  pub fn value_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::value::ValueMatcherMut<'_> {
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
  pub fn set_value(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::value::ValueMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // invert: optional bool
  pub fn invert(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_invert(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        3, val.into()
      )
    }
  }

}

// SAFETY:
// - `MetadataMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for MetadataMatcherMut<'_> {}

// SAFETY:
// - `MetadataMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for MetadataMatcherMut<'_> {}

impl<'msg> ::protobuf::AsView for MetadataMatcherMut<'msg> {
  type Proxied = MetadataMatcher;
  fn as_view(&self) -> ::protobuf::View<'_, MetadataMatcher> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MetadataMatcherMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MetadataMatcher>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for MetadataMatcherMut<'msg> {
  type MutProxied = MetadataMatcher;
  fn as_mut(&mut self) -> MetadataMatcherMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MetadataMatcherMut<'msg> {
  fn into_mut<'shorter>(self) -> MetadataMatcherMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MetadataMatcher {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MetadataMatcher> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MetadataMatcherView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MetadataMatcherMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // filter: optional string
  pub fn filter(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_filter(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // path: repeated message envoy.type.matcher.v3.MetadataMatcher.PathSegment
  pub fn path(&self) -> ::protobuf::RepeatedView<'_, super::metadata_matcher::PathSegment> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::metadata_matcher::PathSegment>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn path_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::metadata_matcher::PathSegment> {
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
  pub fn set_path(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::metadata_matcher::PathSegment>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // value: optional message envoy.type.matcher.v3.ValueMatcher
  pub fn has_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn value_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::value::ValueMatcherView<'_>> {
    self.has_value().then(|| self.value())
  }
  pub fn value(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::value::ValueMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::value::ValueMatcherView::default())
  }
  pub fn value_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::value::ValueMatcherMut<'_> {
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
  pub fn set_value(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::value::ValueMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // invert: optional bool
  pub fn invert(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_invert(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        3, val.into()
      )
    }
  }

}  // impl MetadataMatcher

impl ::std::ops::Drop for MetadataMatcher {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MetadataMatcher {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MetadataMatcher {
  type Proxied = Self;
  fn as_view(&self) -> MetadataMatcherView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MetadataMatcher {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MetadataMatcherMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MetadataMatcher {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__type__matcher__v3__MetadataMatcher_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1XG3/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__matcher__v3__MetadataMatcher_msg_init.0, &[<super::metadata_matcher::PathSegment as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::matcher::v3::value::ValueMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__matcher__v3__MetadataMatcher_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MetadataMatcher {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MetadataMatcher {
  type Msg = MetadataMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MetadataMatcher {
  type Msg = MetadataMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MetadataMatcherMut<'_> {
  type Msg = MetadataMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MetadataMatcherMut<'_> {
  type Msg = MetadataMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MetadataMatcherView<'_> {
  type Msg = MetadataMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataMatcher> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MetadataMatcherMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod metadata_matcher {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__matcher__v3__MetadataMatcher__PathSegment_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct PathSegment {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PathSegment>
}

impl ::protobuf::Message for PathSegment {
  type MessageView<'msg> = PathSegmentView<'msg>;
  type MessageMut<'msg> = PathSegmentMut<'msg>;
}

impl ::std::default::Default for PathSegment {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for PathSegment {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `PathSegment` is `Sync` because it does not implement interior mutability.
//    Neither does `PathSegmentMut`.
unsafe impl ::std::marker::Sync for PathSegment {}

// SAFETY:
// - `PathSegment` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for PathSegment {}

impl ::protobuf::Proxied for PathSegment {
  type View<'msg> = PathSegmentView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PathSegment {}

impl ::protobuf::MutProxied for PathSegment {
  type Mut<'msg> = PathSegmentMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PathSegmentView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PathSegment>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PathSegmentView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PathSegmentView<'msg> {
  type Message = PathSegment;
}

impl ::std::fmt::Debug for PathSegmentView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PathSegmentView<'_> {
  fn default() -> PathSegmentView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, PathSegment>> for PathSegmentView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PathSegment>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PathSegmentView<'msg> {

  pub fn to_owned(&self) -> PathSegment {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // key: optional string
  pub fn has_key(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn key_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_key().then(|| self.key())
  }
  pub fn key(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  pub fn segment(self) -> super::super::metadata_matcher::path_segment::SegmentOneof<'msg> {
    match self.segment_case() {
      super::super::metadata_matcher::path_segment::SegmentCase::Key =>
          super::super::metadata_matcher::path_segment::SegmentOneof::Key(self.key()),
      _ => super::super::metadata_matcher::path_segment::SegmentOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn segment_case(self) -> super::super::metadata_matcher::path_segment::SegmentCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::metadata_matcher::path_segment::SegmentCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `PathSegmentView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for PathSegmentView<'_> {}

// SAFETY:
// - `PathSegmentView` is `Send` because while its alive a `PathSegmentMut` cannot.
// - `PathSegmentView` does not use thread-local data.
unsafe impl ::std::marker::Send for PathSegmentView<'_> {}

impl<'msg> ::protobuf::AsView for PathSegmentView<'msg> {
  type Proxied = PathSegment;
  fn as_view(&self) -> ::protobuf::View<'msg, PathSegment> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PathSegmentView<'msg> {
  fn into_view<'shorter>(self) -> PathSegmentView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PathSegment> for PathSegmentView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PathSegment {
    let mut dst = PathSegment::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PathSegment> for PathSegmentMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PathSegment {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for PathSegment {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PathSegmentView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PathSegmentMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PathSegmentMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PathSegment>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PathSegmentMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PathSegmentMut<'msg> {
  type Message = PathSegment;
}

impl ::std::fmt::Debug for PathSegmentMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, PathSegment>> for PathSegmentMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PathSegment>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PathSegmentMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PathSegment> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> PathSegment {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // key: optional string
  pub fn has_key(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_key(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn key_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_key().then(|| self.key())
  }
  pub fn key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  pub fn segment(&self) -> super::super::metadata_matcher::path_segment::SegmentOneof<'_> {
    match &self.segment_case() {
      super::super::metadata_matcher::path_segment::SegmentCase::Key =>
          super::super::metadata_matcher::path_segment::SegmentOneof::Key(self.key()),
      _ => super::super::metadata_matcher::path_segment::SegmentOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn segment_case(&self) -> super::super::metadata_matcher::path_segment::SegmentCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::metadata_matcher::path_segment::SegmentCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `PathSegmentMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for PathSegmentMut<'_> {}

// SAFETY:
// - `PathSegmentMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for PathSegmentMut<'_> {}

impl<'msg> ::protobuf::AsView for PathSegmentMut<'msg> {
  type Proxied = PathSegment;
  fn as_view(&self) -> ::protobuf::View<'_, PathSegment> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PathSegmentMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PathSegment>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for PathSegmentMut<'msg> {
  type MutProxied = PathSegment;
  fn as_mut(&mut self) -> PathSegmentMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PathSegmentMut<'msg> {
  fn into_mut<'shorter>(self) -> PathSegmentMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PathSegment {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PathSegment> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PathSegmentView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PathSegmentMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // key: optional string
  pub fn has_key(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_key(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn key_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_key().then(|| self.key())
  }
  pub fn key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  pub fn segment(&self) -> super::super::metadata_matcher::path_segment::SegmentOneof<'_> {
    match &self.segment_case() {
      super::super::metadata_matcher::path_segment::SegmentCase::Key =>
          super::super::metadata_matcher::path_segment::SegmentOneof::Key(self.key()),
      _ => super::super::metadata_matcher::path_segment::SegmentOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn segment_case(&self) -> super::super::metadata_matcher::path_segment::SegmentCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::metadata_matcher::path_segment::SegmentCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl PathSegment

impl ::std::ops::Drop for PathSegment {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PathSegment {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PathSegment {
  type Proxied = Self;
  fn as_view(&self) -> PathSegmentView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PathSegment {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PathSegmentMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PathSegment {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::metadata_matcher::envoy__type__matcher__v3__MetadataMatcher__PathSegment_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1^!");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::metadata_matcher::envoy__type__matcher__v3__MetadataMatcher__PathSegment_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::metadata_matcher::envoy__type__matcher__v3__MetadataMatcher__PathSegment_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PathSegment {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PathSegment {
  type Msg = PathSegment;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PathSegment> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PathSegment {
  type Msg = PathSegment;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PathSegment> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PathSegmentMut<'_> {
  type Msg = PathSegment;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PathSegment> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PathSegmentMut<'_> {
  type Msg = PathSegment;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PathSegment> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PathSegmentView<'_> {
  type Msg = PathSegment;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PathSegment> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PathSegmentMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod path_segment {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum SegmentOneof<'msg> {
  Key(&'msg ::protobuf::ProtoStr) = 1,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum SegmentCase {
  Key = 1,

  not_set = 0
}

impl SegmentCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<SegmentCase> {
    match v {
      0 => Some(SegmentCase::not_set),
      1 => Some(SegmentCase::Key),
      _ => None
    }
  }
}
}  // pub mod path_segment


}  // pub mod metadata_matcher


