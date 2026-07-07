const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__tracing__v3__CustomTag_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CustomTag {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CustomTag>
}

impl ::protobuf::Message for CustomTag {
  type MessageView<'msg> = CustomTagView<'msg>;
  type MessageMut<'msg> = CustomTagMut<'msg>;
}

impl ::std::default::Default for CustomTag {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CustomTag {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CustomTag` is `Sync` because it does not implement interior mutability.
//    Neither does `CustomTagMut`.
unsafe impl ::std::marker::Sync for CustomTag {}

// SAFETY:
// - `CustomTag` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CustomTag {}

impl ::protobuf::Proxied for CustomTag {
  type View<'msg> = CustomTagView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CustomTag {}

impl ::protobuf::MutProxied for CustomTag {
  type Mut<'msg> = CustomTagMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CustomTagView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CustomTag>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CustomTagView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CustomTagView<'msg> {
  type Message = CustomTag;
}

impl ::std::fmt::Debug for CustomTagView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CustomTagView<'_> {
  fn default() -> CustomTagView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CustomTag>> for CustomTagView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CustomTag>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CustomTagView<'msg> {

  pub fn to_owned(&self) -> CustomTag {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // tag: optional string
  pub fn tag(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // literal: optional message envoy.type.tracing.v3.CustomTag.Literal
  pub fn has_literal(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn literal_opt(self) -> ::std::option::Option<super::custom_tag::LiteralView<'msg>> {
    self.has_literal().then(|| self.literal())
  }
  pub fn literal(self) -> super::custom_tag::LiteralView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::custom_tag::LiteralView::default())
  }

  // environment: optional message envoy.type.tracing.v3.CustomTag.Environment
  pub fn has_environment(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn environment_opt(self) -> ::std::option::Option<super::custom_tag::EnvironmentView<'msg>> {
    self.has_environment().then(|| self.environment())
  }
  pub fn environment(self) -> super::custom_tag::EnvironmentView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::custom_tag::EnvironmentView::default())
  }

  // request_header: optional message envoy.type.tracing.v3.CustomTag.Header
  pub fn has_request_header(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn request_header_opt(self) -> ::std::option::Option<super::custom_tag::HeaderView<'msg>> {
    self.has_request_header().then(|| self.request_header())
  }
  pub fn request_header(self) -> super::custom_tag::HeaderView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::custom_tag::HeaderView::default())
  }

  // metadata: optional message envoy.type.tracing.v3.CustomTag.Metadata
  pub fn has_metadata(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn metadata_opt(self) -> ::std::option::Option<super::custom_tag::MetadataView<'msg>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(self) -> super::custom_tag::MetadataView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::custom_tag::MetadataView::default())
  }

  // value: optional string
  pub fn has_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn value_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_value().then(|| self.value())
  }
  pub fn value(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  pub fn r#type(self) -> super::custom_tag::TypeOneof<'msg> {
    match self.r#type_case() {
      super::custom_tag::TypeCase::Literal =>
          super::custom_tag::TypeOneof::Literal(self.literal()),
      super::custom_tag::TypeCase::Environment =>
          super::custom_tag::TypeOneof::Environment(self.environment()),
      super::custom_tag::TypeCase::RequestHeader =>
          super::custom_tag::TypeOneof::RequestHeader(self.request_header()),
      super::custom_tag::TypeCase::Metadata =>
          super::custom_tag::TypeOneof::Metadata(self.metadata()),
      super::custom_tag::TypeCase::Value =>
          super::custom_tag::TypeOneof::Value(self.value()),
      _ => super::custom_tag::TypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn r#type_case(self) -> super::custom_tag::TypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::custom_tag::TypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `CustomTagView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CustomTagView<'_> {}

// SAFETY:
// - `CustomTagView` is `Send` because while its alive a `CustomTagMut` cannot.
// - `CustomTagView` does not use thread-local data.
unsafe impl ::std::marker::Send for CustomTagView<'_> {}

impl<'msg> ::protobuf::AsView for CustomTagView<'msg> {
  type Proxied = CustomTag;
  fn as_view(&self) -> ::protobuf::View<'msg, CustomTag> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CustomTagView<'msg> {
  fn into_view<'shorter>(self) -> CustomTagView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CustomTag> for CustomTagView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CustomTag {
    let mut dst = CustomTag::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CustomTag> for CustomTagMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CustomTag {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CustomTag {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CustomTagView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CustomTagMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CustomTagMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CustomTag>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CustomTagMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CustomTagMut<'msg> {
  type Message = CustomTag;
}

impl ::std::fmt::Debug for CustomTagMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CustomTag>> for CustomTagMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CustomTag>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CustomTagMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CustomTag> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CustomTag {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // tag: optional string
  pub fn tag(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_tag(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // literal: optional message envoy.type.tracing.v3.CustomTag.Literal
  pub fn has_literal(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_literal(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn literal_opt(&self) -> ::std::option::Option<super::custom_tag::LiteralView<'_>> {
    self.has_literal().then(|| self.literal())
  }
  pub fn literal(&self) -> super::custom_tag::LiteralView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::custom_tag::LiteralView::default())
  }
  pub fn literal_mut(&mut self) -> super::custom_tag::LiteralMut<'_> {
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
  pub fn set_literal(&mut self,
    val: impl ::protobuf::IntoProxied<super::custom_tag::Literal>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // environment: optional message envoy.type.tracing.v3.CustomTag.Environment
  pub fn has_environment(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_environment(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn environment_opt(&self) -> ::std::option::Option<super::custom_tag::EnvironmentView<'_>> {
    self.has_environment().then(|| self.environment())
  }
  pub fn environment(&self) -> super::custom_tag::EnvironmentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::custom_tag::EnvironmentView::default())
  }
  pub fn environment_mut(&mut self) -> super::custom_tag::EnvironmentMut<'_> {
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
  pub fn set_environment(&mut self,
    val: impl ::protobuf::IntoProxied<super::custom_tag::Environment>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // request_header: optional message envoy.type.tracing.v3.CustomTag.Header
  pub fn has_request_header(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_request_header(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn request_header_opt(&self) -> ::std::option::Option<super::custom_tag::HeaderView<'_>> {
    self.has_request_header().then(|| self.request_header())
  }
  pub fn request_header(&self) -> super::custom_tag::HeaderView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::custom_tag::HeaderView::default())
  }
  pub fn request_header_mut(&mut self) -> super::custom_tag::HeaderMut<'_> {
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
  pub fn set_request_header(&mut self,
    val: impl ::protobuf::IntoProxied<super::custom_tag::Header>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // metadata: optional message envoy.type.tracing.v3.CustomTag.Metadata
  pub fn has_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn metadata_opt(&self) -> ::std::option::Option<super::custom_tag::MetadataView<'_>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(&self) -> super::custom_tag::MetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::custom_tag::MetadataView::default())
  }
  pub fn metadata_mut(&mut self) -> super::custom_tag::MetadataMut<'_> {
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
  pub fn set_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<super::custom_tag::Metadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // value: optional string
  pub fn has_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn value_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_value().then(|| self.value())
  }
  pub fn value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  pub fn r#type(&self) -> super::custom_tag::TypeOneof<'_> {
    match &self.r#type_case() {
      super::custom_tag::TypeCase::Literal =>
          super::custom_tag::TypeOneof::Literal(self.literal()),
      super::custom_tag::TypeCase::Environment =>
          super::custom_tag::TypeOneof::Environment(self.environment()),
      super::custom_tag::TypeCase::RequestHeader =>
          super::custom_tag::TypeOneof::RequestHeader(self.request_header()),
      super::custom_tag::TypeCase::Metadata =>
          super::custom_tag::TypeOneof::Metadata(self.metadata()),
      super::custom_tag::TypeCase::Value =>
          super::custom_tag::TypeOneof::Value(self.value()),
      _ => super::custom_tag::TypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn r#type_case(&self) -> super::custom_tag::TypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::custom_tag::TypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `CustomTagMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CustomTagMut<'_> {}

// SAFETY:
// - `CustomTagMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CustomTagMut<'_> {}

impl<'msg> ::protobuf::AsView for CustomTagMut<'msg> {
  type Proxied = CustomTag;
  fn as_view(&self) -> ::protobuf::View<'_, CustomTag> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CustomTagMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CustomTag>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CustomTagMut<'msg> {
  type MutProxied = CustomTag;
  fn as_mut(&mut self) -> CustomTagMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CustomTagMut<'msg> {
  fn into_mut<'shorter>(self) -> CustomTagMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CustomTag {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CustomTag> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CustomTagView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CustomTagMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // tag: optional string
  pub fn tag(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_tag(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // literal: optional message envoy.type.tracing.v3.CustomTag.Literal
  pub fn has_literal(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_literal(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn literal_opt(&self) -> ::std::option::Option<super::custom_tag::LiteralView<'_>> {
    self.has_literal().then(|| self.literal())
  }
  pub fn literal(&self) -> super::custom_tag::LiteralView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::custom_tag::LiteralView::default())
  }
  pub fn literal_mut(&mut self) -> super::custom_tag::LiteralMut<'_> {
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
  pub fn set_literal(&mut self,
    val: impl ::protobuf::IntoProxied<super::custom_tag::Literal>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // environment: optional message envoy.type.tracing.v3.CustomTag.Environment
  pub fn has_environment(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_environment(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn environment_opt(&self) -> ::std::option::Option<super::custom_tag::EnvironmentView<'_>> {
    self.has_environment().then(|| self.environment())
  }
  pub fn environment(&self) -> super::custom_tag::EnvironmentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::custom_tag::EnvironmentView::default())
  }
  pub fn environment_mut(&mut self) -> super::custom_tag::EnvironmentMut<'_> {
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
  pub fn set_environment(&mut self,
    val: impl ::protobuf::IntoProxied<super::custom_tag::Environment>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // request_header: optional message envoy.type.tracing.v3.CustomTag.Header
  pub fn has_request_header(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_request_header(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn request_header_opt(&self) -> ::std::option::Option<super::custom_tag::HeaderView<'_>> {
    self.has_request_header().then(|| self.request_header())
  }
  pub fn request_header(&self) -> super::custom_tag::HeaderView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::custom_tag::HeaderView::default())
  }
  pub fn request_header_mut(&mut self) -> super::custom_tag::HeaderMut<'_> {
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
  pub fn set_request_header(&mut self,
    val: impl ::protobuf::IntoProxied<super::custom_tag::Header>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // metadata: optional message envoy.type.tracing.v3.CustomTag.Metadata
  pub fn has_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn metadata_opt(&self) -> ::std::option::Option<super::custom_tag::MetadataView<'_>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(&self) -> super::custom_tag::MetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::custom_tag::MetadataView::default())
  }
  pub fn metadata_mut(&mut self) -> super::custom_tag::MetadataMut<'_> {
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
  pub fn set_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<super::custom_tag::Metadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // value: optional string
  pub fn has_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn value_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_value().then(|| self.value())
  }
  pub fn value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  pub fn r#type(&self) -> super::custom_tag::TypeOneof<'_> {
    match &self.r#type_case() {
      super::custom_tag::TypeCase::Literal =>
          super::custom_tag::TypeOneof::Literal(self.literal()),
      super::custom_tag::TypeCase::Environment =>
          super::custom_tag::TypeOneof::Environment(self.environment()),
      super::custom_tag::TypeCase::RequestHeader =>
          super::custom_tag::TypeOneof::RequestHeader(self.request_header()),
      super::custom_tag::TypeCase::Metadata =>
          super::custom_tag::TypeOneof::Metadata(self.metadata()),
      super::custom_tag::TypeCase::Value =>
          super::custom_tag::TypeOneof::Value(self.value()),
      _ => super::custom_tag::TypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn r#type_case(&self) -> super::custom_tag::TypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::custom_tag::TypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl CustomTag

impl ::std::ops::Drop for CustomTag {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CustomTag {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CustomTag {
  type Proxied = Self;
  fn as_view(&self) -> CustomTagView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CustomTag {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CustomTagMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CustomTag {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__type__tracing__v3__CustomTag_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X33331T^#|$|%|&|(");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__tracing__v3__CustomTag_msg_init.0, &[<super::custom_tag::Literal as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::custom_tag::Environment as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::custom_tag::Header as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::custom_tag::Metadata as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__tracing__v3__CustomTag_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CustomTag {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CustomTag {
  type Msg = CustomTag;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CustomTag> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CustomTag {
  type Msg = CustomTag;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CustomTag> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CustomTagMut<'_> {
  type Msg = CustomTag;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CustomTag> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CustomTagMut<'_> {
  type Msg = CustomTag;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CustomTag> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CustomTagView<'_> {
  type Msg = CustomTag;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CustomTag> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CustomTagMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod custom_tag {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__tracing__v3__CustomTag__Literal_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Literal {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Literal>
}

impl ::protobuf::Message for Literal {
  type MessageView<'msg> = LiteralView<'msg>;
  type MessageMut<'msg> = LiteralMut<'msg>;
}

impl ::std::default::Default for Literal {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Literal {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Literal` is `Sync` because it does not implement interior mutability.
//    Neither does `LiteralMut`.
unsafe impl ::std::marker::Sync for Literal {}

// SAFETY:
// - `Literal` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Literal {}

impl ::protobuf::Proxied for Literal {
  type View<'msg> = LiteralView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Literal {}

impl ::protobuf::MutProxied for Literal {
  type Mut<'msg> = LiteralMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LiteralView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Literal>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LiteralView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LiteralView<'msg> {
  type Message = Literal;
}

impl ::std::fmt::Debug for LiteralView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LiteralView<'_> {
  fn default() -> LiteralView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Literal>> for LiteralView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Literal>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LiteralView<'msg> {

  pub fn to_owned(&self) -> Literal {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // value: optional string
  pub fn value(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `LiteralView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for LiteralView<'_> {}

// SAFETY:
// - `LiteralView` is `Send` because while its alive a `LiteralMut` cannot.
// - `LiteralView` does not use thread-local data.
unsafe impl ::std::marker::Send for LiteralView<'_> {}

impl<'msg> ::protobuf::AsView for LiteralView<'msg> {
  type Proxied = Literal;
  fn as_view(&self) -> ::protobuf::View<'msg, Literal> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LiteralView<'msg> {
  fn into_view<'shorter>(self) -> LiteralView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Literal> for LiteralView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Literal {
    let mut dst = Literal::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Literal> for LiteralMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Literal {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Literal {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LiteralView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LiteralMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LiteralMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Literal>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LiteralMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LiteralMut<'msg> {
  type Message = Literal;
}

impl ::std::fmt::Debug for LiteralMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Literal>> for LiteralMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Literal>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LiteralMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Literal> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Literal {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // value: optional string
  pub fn value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `LiteralMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for LiteralMut<'_> {}

// SAFETY:
// - `LiteralMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for LiteralMut<'_> {}

impl<'msg> ::protobuf::AsView for LiteralMut<'msg> {
  type Proxied = Literal;
  fn as_view(&self) -> ::protobuf::View<'_, Literal> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LiteralMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Literal>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for LiteralMut<'msg> {
  type MutProxied = Literal;
  fn as_mut(&mut self) -> LiteralMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LiteralMut<'msg> {
  fn into_mut<'shorter>(self) -> LiteralMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Literal {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Literal> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LiteralView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LiteralMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // value: optional string
  pub fn value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl Literal

impl ::std::ops::Drop for Literal {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Literal {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Literal {
  type Proxied = Self;
  fn as_view(&self) -> LiteralView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Literal {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LiteralMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Literal {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::custom_tag::envoy__type__tracing__v3__CustomTag__Literal_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::custom_tag::envoy__type__tracing__v3__CustomTag__Literal_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::custom_tag::envoy__type__tracing__v3__CustomTag__Literal_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Literal {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Literal {
  type Msg = Literal;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Literal> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Literal {
  type Msg = Literal;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Literal> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LiteralMut<'_> {
  type Msg = Literal;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Literal> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LiteralMut<'_> {
  type Msg = Literal;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Literal> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LiteralView<'_> {
  type Msg = Literal;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Literal> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LiteralMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__tracing__v3__CustomTag__Environment_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Environment {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Environment>
}

impl ::protobuf::Message for Environment {
  type MessageView<'msg> = EnvironmentView<'msg>;
  type MessageMut<'msg> = EnvironmentMut<'msg>;
}

impl ::std::default::Default for Environment {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Environment {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Environment` is `Sync` because it does not implement interior mutability.
//    Neither does `EnvironmentMut`.
unsafe impl ::std::marker::Sync for Environment {}

// SAFETY:
// - `Environment` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Environment {}

impl ::protobuf::Proxied for Environment {
  type View<'msg> = EnvironmentView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Environment {}

impl ::protobuf::MutProxied for Environment {
  type Mut<'msg> = EnvironmentMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct EnvironmentView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Environment>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EnvironmentView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for EnvironmentView<'msg> {
  type Message = Environment;
}

impl ::std::fmt::Debug for EnvironmentView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for EnvironmentView<'_> {
  fn default() -> EnvironmentView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Environment>> for EnvironmentView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Environment>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EnvironmentView<'msg> {

  pub fn to_owned(&self) -> Environment {
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

  // default_value: optional string
  pub fn default_value(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `EnvironmentView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for EnvironmentView<'_> {}

// SAFETY:
// - `EnvironmentView` is `Send` because while its alive a `EnvironmentMut` cannot.
// - `EnvironmentView` does not use thread-local data.
unsafe impl ::std::marker::Send for EnvironmentView<'_> {}

impl<'msg> ::protobuf::AsView for EnvironmentView<'msg> {
  type Proxied = Environment;
  fn as_view(&self) -> ::protobuf::View<'msg, Environment> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EnvironmentView<'msg> {
  fn into_view<'shorter>(self) -> EnvironmentView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Environment> for EnvironmentView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Environment {
    let mut dst = Environment::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Environment> for EnvironmentMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Environment {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Environment {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for EnvironmentView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for EnvironmentMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct EnvironmentMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Environment>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EnvironmentMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for EnvironmentMut<'msg> {
  type Message = Environment;
}

impl ::std::fmt::Debug for EnvironmentMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Environment>> for EnvironmentMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Environment>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EnvironmentMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Environment> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Environment {
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

  // default_value: optional string
  pub fn default_value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_default_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `EnvironmentMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for EnvironmentMut<'_> {}

// SAFETY:
// - `EnvironmentMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for EnvironmentMut<'_> {}

impl<'msg> ::protobuf::AsView for EnvironmentMut<'msg> {
  type Proxied = Environment;
  fn as_view(&self) -> ::protobuf::View<'_, Environment> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EnvironmentMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Environment>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for EnvironmentMut<'msg> {
  type MutProxied = Environment;
  fn as_mut(&mut self) -> EnvironmentMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for EnvironmentMut<'msg> {
  fn into_mut<'shorter>(self) -> EnvironmentMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Environment {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Environment> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> EnvironmentView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> EnvironmentMut<'_> {
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

  // default_value: optional string
  pub fn default_value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_default_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl Environment

impl ::std::ops::Drop for Environment {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Environment {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Environment {
  type Proxied = Self;
  fn as_view(&self) -> EnvironmentView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Environment {
  type MutProxied = Self;
  fn as_mut(&mut self) -> EnvironmentMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Environment {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::custom_tag::envoy__type__tracing__v3__CustomTag__Environment_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::custom_tag::envoy__type__tracing__v3__CustomTag__Environment_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::custom_tag::envoy__type__tracing__v3__CustomTag__Environment_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Environment {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Environment {
  type Msg = Environment;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Environment> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Environment {
  type Msg = Environment;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Environment> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EnvironmentMut<'_> {
  type Msg = Environment;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Environment> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EnvironmentMut<'_> {
  type Msg = Environment;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Environment> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EnvironmentView<'_> {
  type Msg = Environment;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Environment> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EnvironmentMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__tracing__v3__CustomTag__Header_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Header {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Header>
}

impl ::protobuf::Message for Header {
  type MessageView<'msg> = HeaderView<'msg>;
  type MessageMut<'msg> = HeaderMut<'msg>;
}

impl ::std::default::Default for Header {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Header {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Header` is `Sync` because it does not implement interior mutability.
//    Neither does `HeaderMut`.
unsafe impl ::std::marker::Sync for Header {}

// SAFETY:
// - `Header` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Header {}

impl ::protobuf::Proxied for Header {
  type View<'msg> = HeaderView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Header {}

impl ::protobuf::MutProxied for Header {
  type Mut<'msg> = HeaderMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HeaderView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Header>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeaderView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HeaderView<'msg> {
  type Message = Header;
}

impl ::std::fmt::Debug for HeaderView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HeaderView<'_> {
  fn default() -> HeaderView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Header>> for HeaderView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Header>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HeaderView<'msg> {

  pub fn to_owned(&self) -> Header {
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

  // default_value: optional string
  pub fn default_value(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `HeaderView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HeaderView<'_> {}

// SAFETY:
// - `HeaderView` is `Send` because while its alive a `HeaderMut` cannot.
// - `HeaderView` does not use thread-local data.
unsafe impl ::std::marker::Send for HeaderView<'_> {}

impl<'msg> ::protobuf::AsView for HeaderView<'msg> {
  type Proxied = Header;
  fn as_view(&self) -> ::protobuf::View<'msg, Header> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeaderView<'msg> {
  fn into_view<'shorter>(self) -> HeaderView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Header> for HeaderView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Header {
    let mut dst = Header::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Header> for HeaderMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Header {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Header {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HeaderView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HeaderMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HeaderMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Header>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeaderMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HeaderMut<'msg> {
  type Message = Header;
}

impl ::std::fmt::Debug for HeaderMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Header>> for HeaderMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Header>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HeaderMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Header> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Header {
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

  // default_value: optional string
  pub fn default_value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_default_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `HeaderMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HeaderMut<'_> {}

// SAFETY:
// - `HeaderMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HeaderMut<'_> {}

impl<'msg> ::protobuf::AsView for HeaderMut<'msg> {
  type Proxied = Header;
  fn as_view(&self) -> ::protobuf::View<'_, Header> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeaderMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Header>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HeaderMut<'msg> {
  type MutProxied = Header;
  fn as_mut(&mut self) -> HeaderMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HeaderMut<'msg> {
  fn into_mut<'shorter>(self) -> HeaderMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Header {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Header> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HeaderView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HeaderMut<'_> {
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

  // default_value: optional string
  pub fn default_value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_default_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl Header

impl ::std::ops::Drop for Header {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Header {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Header {
  type Proxied = Self;
  fn as_view(&self) -> HeaderView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Header {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HeaderMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Header {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::custom_tag::envoy__type__tracing__v3__CustomTag__Header_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::custom_tag::envoy__type__tracing__v3__CustomTag__Header_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::custom_tag::envoy__type__tracing__v3__CustomTag__Header_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Header {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Header {
  type Msg = Header;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Header> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Header {
  type Msg = Header;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Header> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HeaderMut<'_> {
  type Msg = Header;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Header> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderMut<'_> {
  type Msg = Header;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Header> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderView<'_> {
  type Msg = Header;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Header> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HeaderMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__tracing__v3__CustomTag__Metadata_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Metadata {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Metadata>
}

impl ::protobuf::Message for Metadata {
  type MessageView<'msg> = MetadataView<'msg>;
  type MessageMut<'msg> = MetadataMut<'msg>;
}

impl ::std::default::Default for Metadata {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Metadata {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Metadata` is `Sync` because it does not implement interior mutability.
//    Neither does `MetadataMut`.
unsafe impl ::std::marker::Sync for Metadata {}

// SAFETY:
// - `Metadata` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Metadata {}

impl ::protobuf::Proxied for Metadata {
  type View<'msg> = MetadataView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Metadata {}

impl ::protobuf::MutProxied for Metadata {
  type Mut<'msg> = MetadataMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MetadataView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Metadata>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MetadataView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MetadataView<'msg> {
  type Message = Metadata;
}

impl ::std::fmt::Debug for MetadataView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MetadataView<'_> {
  fn default() -> MetadataView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Metadata>> for MetadataView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Metadata>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MetadataView<'msg> {

  pub fn to_owned(&self) -> Metadata {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // kind: optional message envoy.type.metadata.v3.MetadataKind
  pub fn has_kind(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn kind_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKindView<'msg>> {
    self.has_kind().then(|| self.kind())
  }
  pub fn kind(self) -> crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKindView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKindView::default())
  }

  // metadata_key: optional message envoy.type.metadata.v3.MetadataKey
  pub fn has_metadata_key(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn metadata_key_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKeyView<'msg>> {
    self.has_metadata_key().then(|| self.metadata_key())
  }
  pub fn metadata_key(self) -> crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKeyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKeyView::default())
  }

  // default_value: optional string
  pub fn default_value(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `MetadataView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for MetadataView<'_> {}

// SAFETY:
// - `MetadataView` is `Send` because while its alive a `MetadataMut` cannot.
// - `MetadataView` does not use thread-local data.
unsafe impl ::std::marker::Send for MetadataView<'_> {}

impl<'msg> ::protobuf::AsView for MetadataView<'msg> {
  type Proxied = Metadata;
  fn as_view(&self) -> ::protobuf::View<'msg, Metadata> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MetadataView<'msg> {
  fn into_view<'shorter>(self) -> MetadataView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Metadata> for MetadataView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Metadata {
    let mut dst = Metadata::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Metadata> for MetadataMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Metadata {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Metadata {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MetadataView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MetadataMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MetadataMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Metadata>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MetadataMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MetadataMut<'msg> {
  type Message = Metadata;
}

impl ::std::fmt::Debug for MetadataMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Metadata>> for MetadataMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Metadata>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MetadataMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Metadata> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Metadata {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // kind: optional message envoy.type.metadata.v3.MetadataKind
  pub fn has_kind(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_kind(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn kind_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKindView<'_>> {
    self.has_kind().then(|| self.kind())
  }
  pub fn kind(&self) -> crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKindView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKindView::default())
  }
  pub fn kind_mut(&mut self) -> crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKindMut<'_> {
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
  pub fn set_kind(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKind>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // metadata_key: optional message envoy.type.metadata.v3.MetadataKey
  pub fn has_metadata_key(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_metadata_key(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn metadata_key_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKeyView<'_>> {
    self.has_metadata_key().then(|| self.metadata_key())
  }
  pub fn metadata_key(&self) -> crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKeyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKeyView::default())
  }
  pub fn metadata_key_mut(&mut self) -> crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKeyMut<'_> {
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
  pub fn set_metadata_key(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKey>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // default_value: optional string
  pub fn default_value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_default_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}

// SAFETY:
// - `MetadataMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for MetadataMut<'_> {}

// SAFETY:
// - `MetadataMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for MetadataMut<'_> {}

impl<'msg> ::protobuf::AsView for MetadataMut<'msg> {
  type Proxied = Metadata;
  fn as_view(&self) -> ::protobuf::View<'_, Metadata> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MetadataMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Metadata>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for MetadataMut<'msg> {
  type MutProxied = Metadata;
  fn as_mut(&mut self) -> MetadataMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MetadataMut<'msg> {
  fn into_mut<'shorter>(self) -> MetadataMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Metadata {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Metadata> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MetadataView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MetadataMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // kind: optional message envoy.type.metadata.v3.MetadataKind
  pub fn has_kind(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_kind(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn kind_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKindView<'_>> {
    self.has_kind().then(|| self.kind())
  }
  pub fn kind(&self) -> crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKindView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKindView::default())
  }
  pub fn kind_mut(&mut self) -> crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKindMut<'_> {
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
  pub fn set_kind(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKind>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // metadata_key: optional message envoy.type.metadata.v3.MetadataKey
  pub fn has_metadata_key(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_metadata_key(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn metadata_key_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKeyView<'_>> {
    self.has_metadata_key().then(|| self.metadata_key())
  }
  pub fn metadata_key(&self) -> crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKeyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKeyView::default())
  }
  pub fn metadata_key_mut(&mut self) -> crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKeyMut<'_> {
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
  pub fn set_metadata_key(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKey>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // default_value: optional string
  pub fn default_value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_default_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}  // impl Metadata

impl ::std::ops::Drop for Metadata {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Metadata {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Metadata {
  type Proxied = Self;
  fn as_view(&self) -> MetadataView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Metadata {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MetadataMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Metadata {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::custom_tag::envoy__type__tracing__v3__CustomTag__Metadata_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$331X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::custom_tag::envoy__type__tracing__v3__CustomTag__Metadata_msg_init.0, &[<crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKind as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKey as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::custom_tag::envoy__type__tracing__v3__CustomTag__Metadata_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Metadata {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Metadata {
  type Msg = Metadata;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Metadata> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Metadata {
  type Msg = Metadata;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Metadata> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MetadataMut<'_> {
  type Msg = Metadata;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Metadata> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MetadataMut<'_> {
  type Msg = Metadata;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Metadata> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MetadataView<'_> {
  type Msg = Metadata;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Metadata> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MetadataMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum TypeOneof<'msg> {
  Literal(::protobuf::View<'msg, super::super::custom_tag::Literal>) = 2,
  Environment(::protobuf::View<'msg, super::super::custom_tag::Environment>) = 3,
  RequestHeader(::protobuf::View<'msg, super::super::custom_tag::Header>) = 4,
  Metadata(::protobuf::View<'msg, super::super::custom_tag::Metadata>) = 5,
  Value(&'msg ::protobuf::ProtoStr) = 6,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum TypeCase {
  Literal = 2,
  Environment = 3,
  RequestHeader = 4,
  Metadata = 5,
  Value = 6,

  not_set = 0
}

impl TypeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<TypeCase> {
    match v {
      0 => Some(TypeCase::not_set),
      2 => Some(TypeCase::Literal),
      3 => Some(TypeCase::Environment),
      4 => Some(TypeCase::RequestHeader),
      5 => Some(TypeCase::Metadata),
      6 => Some(TypeCase::Value),
      _ => None
    }
  }
}
}  // pub mod custom_tag


