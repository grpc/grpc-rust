const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut validate__FieldRules_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct FieldRules {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<FieldRules>
}

impl ::protobuf::Message for FieldRules {
  type MessageView<'msg> = FieldRulesView<'msg>;
  type MessageMut<'msg> = FieldRulesMut<'msg>;
}

impl ::std::default::Default for FieldRules {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for FieldRules {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `FieldRules` is `Sync` because it does not implement interior mutability.
//    Neither does `FieldRulesMut`.
unsafe impl ::std::marker::Sync for FieldRules {}

// SAFETY:
// - `FieldRules` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for FieldRules {}

impl ::protobuf::Proxied for FieldRules {
  type View<'msg> = FieldRulesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for FieldRules {}

impl ::protobuf::MutProxied for FieldRules {
  type Mut<'msg> = FieldRulesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FieldRulesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FieldRules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FieldRulesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FieldRulesView<'msg> {
  type Message = FieldRules;
}

impl ::std::fmt::Debug for FieldRulesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FieldRulesView<'_> {
  fn default() -> FieldRulesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, FieldRules>> for FieldRulesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FieldRules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FieldRulesView<'msg> {

  pub fn to_owned(&self) -> FieldRules {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // message: optional message validate.MessageRules
  pub fn has_message(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(16)
    }
  }
  pub fn message_opt(self) -> ::std::option::Option<super::MessageRulesView<'msg>> {
    self.has_message().then(|| self.message())
  }
  pub fn message(self) -> super::MessageRulesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(16)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MessageRulesView::default())
  }

  // float: optional message validate.FloatRules
  pub fn has_float(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn float_opt(self) -> ::std::option::Option<super::FloatRulesView<'msg>> {
    self.has_float().then(|| self.float())
  }
  pub fn float(self) -> super::FloatRulesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FloatRulesView::default())
  }

  // double: optional message validate.DoubleRules
  pub fn has_double(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn double_opt(self) -> ::std::option::Option<super::DoubleRulesView<'msg>> {
    self.has_double().then(|| self.double())
  }
  pub fn double(self) -> super::DoubleRulesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DoubleRulesView::default())
  }

  // int32: optional message validate.Int32Rules
  pub fn has_int32(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn int32_opt(self) -> ::std::option::Option<super::Int32RulesView<'msg>> {
    self.has_int32().then(|| self.int32())
  }
  pub fn int32(self) -> super::Int32RulesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Int32RulesView::default())
  }

  // int64: optional message validate.Int64Rules
  pub fn has_int64(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn int64_opt(self) -> ::std::option::Option<super::Int64RulesView<'msg>> {
    self.has_int64().then(|| self.int64())
  }
  pub fn int64(self) -> super::Int64RulesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Int64RulesView::default())
  }

  // uint32: optional message validate.UInt32Rules
  pub fn has_uint32(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn uint32_opt(self) -> ::std::option::Option<super::UInt32RulesView<'msg>> {
    self.has_uint32().then(|| self.uint32())
  }
  pub fn uint32(self) -> super::UInt32RulesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::UInt32RulesView::default())
  }

  // uint64: optional message validate.UInt64Rules
  pub fn has_uint64(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn uint64_opt(self) -> ::std::option::Option<super::UInt64RulesView<'msg>> {
    self.has_uint64().then(|| self.uint64())
  }
  pub fn uint64(self) -> super::UInt64RulesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::UInt64RulesView::default())
  }

  // sint32: optional message validate.SInt32Rules
  pub fn has_sint32(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn sint32_opt(self) -> ::std::option::Option<super::SInt32RulesView<'msg>> {
    self.has_sint32().then(|| self.sint32())
  }
  pub fn sint32(self) -> super::SInt32RulesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SInt32RulesView::default())
  }

  // sint64: optional message validate.SInt64Rules
  pub fn has_sint64(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn sint64_opt(self) -> ::std::option::Option<super::SInt64RulesView<'msg>> {
    self.has_sint64().then(|| self.sint64())
  }
  pub fn sint64(self) -> super::SInt64RulesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SInt64RulesView::default())
  }

  // fixed32: optional message validate.Fixed32Rules
  pub fn has_fixed32(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn fixed32_opt(self) -> ::std::option::Option<super::Fixed32RulesView<'msg>> {
    self.has_fixed32().then(|| self.fixed32())
  }
  pub fn fixed32(self) -> super::Fixed32RulesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Fixed32RulesView::default())
  }

  // fixed64: optional message validate.Fixed64Rules
  pub fn has_fixed64(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn fixed64_opt(self) -> ::std::option::Option<super::Fixed64RulesView<'msg>> {
    self.has_fixed64().then(|| self.fixed64())
  }
  pub fn fixed64(self) -> super::Fixed64RulesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Fixed64RulesView::default())
  }

  // sfixed32: optional message validate.SFixed32Rules
  pub fn has_sfixed32(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn sfixed32_opt(self) -> ::std::option::Option<super::SFixed32RulesView<'msg>> {
    self.has_sfixed32().then(|| self.sfixed32())
  }
  pub fn sfixed32(self) -> super::SFixed32RulesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SFixed32RulesView::default())
  }

  // sfixed64: optional message validate.SFixed64Rules
  pub fn has_sfixed64(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn sfixed64_opt(self) -> ::std::option::Option<super::SFixed64RulesView<'msg>> {
    self.has_sfixed64().then(|| self.sfixed64())
  }
  pub fn sfixed64(self) -> super::SFixed64RulesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SFixed64RulesView::default())
  }

  // bool: optional message validate.BoolRules
  pub fn has_bool(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn bool_opt(self) -> ::std::option::Option<super::BoolRulesView<'msg>> {
    self.has_bool().then(|| self.bool())
  }
  pub fn bool(self) -> super::BoolRulesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BoolRulesView::default())
  }

  // string: optional message validate.StringRules
  pub fn has_string(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn string_opt(self) -> ::std::option::Option<super::StringRulesView<'msg>> {
    self.has_string().then(|| self.string())
  }
  pub fn string(self) -> super::StringRulesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::StringRulesView::default())
  }

  // bytes: optional message validate.BytesRules
  pub fn has_bytes(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(14)
    }
  }
  pub fn bytes_opt(self) -> ::std::option::Option<super::BytesRulesView<'msg>> {
    self.has_bytes().then(|| self.bytes())
  }
  pub fn bytes(self) -> super::BytesRulesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(14)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BytesRulesView::default())
  }

  // enum: optional message validate.EnumRules
  pub fn has_enum(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(15)
    }
  }
  pub fn enum_opt(self) -> ::std::option::Option<super::EnumRulesView<'msg>> {
    self.has_enum().then(|| self.r#enum())
  }
  pub fn r#enum(self) -> super::EnumRulesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(15)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::EnumRulesView::default())
  }

  // repeated: optional message validate.RepeatedRules
  pub fn has_repeated(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(17)
    }
  }
  pub fn repeated_opt(self) -> ::std::option::Option<super::RepeatedRulesView<'msg>> {
    self.has_repeated().then(|| self.repeated())
  }
  pub fn repeated(self) -> super::RepeatedRulesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(17)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RepeatedRulesView::default())
  }

  // map: optional message validate.MapRules
  pub fn has_map(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(18)
    }
  }
  pub fn map_opt(self) -> ::std::option::Option<super::MapRulesView<'msg>> {
    self.has_map().then(|| self.map())
  }
  pub fn map(self) -> super::MapRulesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(18)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MapRulesView::default())
  }

  // any: optional message validate.AnyRules
  pub fn has_any(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(19)
    }
  }
  pub fn any_opt(self) -> ::std::option::Option<super::AnyRulesView<'msg>> {
    self.has_any().then(|| self.any())
  }
  pub fn any(self) -> super::AnyRulesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(19)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AnyRulesView::default())
  }

  // duration: optional message validate.DurationRules
  pub fn has_duration(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(20)
    }
  }
  pub fn duration_opt(self) -> ::std::option::Option<super::DurationRulesView<'msg>> {
    self.has_duration().then(|| self.duration())
  }
  pub fn duration(self) -> super::DurationRulesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(20)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DurationRulesView::default())
  }

  // timestamp: optional message validate.TimestampRules
  pub fn has_timestamp(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(21)
    }
  }
  pub fn timestamp_opt(self) -> ::std::option::Option<super::TimestampRulesView<'msg>> {
    self.has_timestamp().then(|| self.timestamp())
  }
  pub fn timestamp(self) -> super::TimestampRulesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(21)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TimestampRulesView::default())
  }

  pub fn r#type(self) -> super::field_rules::TypeOneof<'msg> {
    match self.r#type_case() {
      super::field_rules::TypeCase::Float =>
          super::field_rules::TypeOneof::Float(self.float()),
      super::field_rules::TypeCase::Double =>
          super::field_rules::TypeOneof::Double(self.double()),
      super::field_rules::TypeCase::Int32 =>
          super::field_rules::TypeOneof::Int32(self.int32()),
      super::field_rules::TypeCase::Int64 =>
          super::field_rules::TypeOneof::Int64(self.int64()),
      super::field_rules::TypeCase::Uint32 =>
          super::field_rules::TypeOneof::Uint32(self.uint32()),
      super::field_rules::TypeCase::Uint64 =>
          super::field_rules::TypeOneof::Uint64(self.uint64()),
      super::field_rules::TypeCase::Sint32 =>
          super::field_rules::TypeOneof::Sint32(self.sint32()),
      super::field_rules::TypeCase::Sint64 =>
          super::field_rules::TypeOneof::Sint64(self.sint64()),
      super::field_rules::TypeCase::Fixed32 =>
          super::field_rules::TypeOneof::Fixed32(self.fixed32()),
      super::field_rules::TypeCase::Fixed64 =>
          super::field_rules::TypeOneof::Fixed64(self.fixed64()),
      super::field_rules::TypeCase::Sfixed32 =>
          super::field_rules::TypeOneof::Sfixed32(self.sfixed32()),
      super::field_rules::TypeCase::Sfixed64 =>
          super::field_rules::TypeOneof::Sfixed64(self.sfixed64()),
      super::field_rules::TypeCase::Bool =>
          super::field_rules::TypeOneof::Bool(self.bool()),
      super::field_rules::TypeCase::String =>
          super::field_rules::TypeOneof::String(self.string()),
      super::field_rules::TypeCase::Bytes =>
          super::field_rules::TypeOneof::Bytes(self.bytes()),
      super::field_rules::TypeCase::Enum =>
          super::field_rules::TypeOneof::Enum(self.r#enum()),
      super::field_rules::TypeCase::Repeated =>
          super::field_rules::TypeOneof::Repeated(self.repeated()),
      super::field_rules::TypeCase::Map =>
          super::field_rules::TypeOneof::Map(self.map()),
      super::field_rules::TypeCase::Any =>
          super::field_rules::TypeOneof::Any(self.any()),
      super::field_rules::TypeCase::Duration =>
          super::field_rules::TypeOneof::Duration(self.duration()),
      super::field_rules::TypeCase::Timestamp =>
          super::field_rules::TypeOneof::Timestamp(self.timestamp()),
      _ => super::field_rules::TypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn r#type_case(self) -> super::field_rules::TypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::field_rules::TypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `FieldRulesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for FieldRulesView<'_> {}

// SAFETY:
// - `FieldRulesView` is `Send` because while its alive a `FieldRulesMut` cannot.
// - `FieldRulesView` does not use thread-local data.
unsafe impl ::std::marker::Send for FieldRulesView<'_> {}

impl<'msg> ::protobuf::AsView for FieldRulesView<'msg> {
  type Proxied = FieldRules;
  fn as_view(&self) -> ::protobuf::View<'msg, FieldRules> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FieldRulesView<'msg> {
  fn into_view<'shorter>(self) -> FieldRulesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<FieldRules> for FieldRulesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FieldRules {
    let mut dst = FieldRules::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<FieldRules> for FieldRulesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FieldRules {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for FieldRules {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FieldRulesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FieldRulesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FieldRulesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FieldRules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FieldRulesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FieldRulesMut<'msg> {
  type Message = FieldRules;
}

impl ::std::fmt::Debug for FieldRulesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, FieldRules>> for FieldRulesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FieldRules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FieldRulesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, FieldRules> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> FieldRules {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // message: optional message validate.MessageRules
  pub fn has_message(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(16)
    }
  }
  pub fn clear_message(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        16
      );
    }
  }
  pub fn message_opt(&self) -> ::std::option::Option<super::MessageRulesView<'_>> {
    self.has_message().then(|| self.message())
  }
  pub fn message(&self) -> super::MessageRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(16)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MessageRulesView::default())
  }
  pub fn message_mut(&mut self) -> super::MessageRulesMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         16, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_message(&mut self,
    val: impl ::protobuf::IntoProxied<super::MessageRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        16,
        val
      );
    }
  }

  // float: optional message validate.FloatRules
  pub fn has_float(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_float(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn float_opt(&self) -> ::std::option::Option<super::FloatRulesView<'_>> {
    self.has_float().then(|| self.float())
  }
  pub fn float(&self) -> super::FloatRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FloatRulesView::default())
  }
  pub fn float_mut(&mut self) -> super::FloatRulesMut<'_> {
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
  pub fn set_float(&mut self,
    val: impl ::protobuf::IntoProxied<super::FloatRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // double: optional message validate.DoubleRules
  pub fn has_double(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_double(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn double_opt(&self) -> ::std::option::Option<super::DoubleRulesView<'_>> {
    self.has_double().then(|| self.double())
  }
  pub fn double(&self) -> super::DoubleRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DoubleRulesView::default())
  }
  pub fn double_mut(&mut self) -> super::DoubleRulesMut<'_> {
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
  pub fn set_double(&mut self,
    val: impl ::protobuf::IntoProxied<super::DoubleRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // int32: optional message validate.Int32Rules
  pub fn has_int32(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_int32(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn int32_opt(&self) -> ::std::option::Option<super::Int32RulesView<'_>> {
    self.has_int32().then(|| self.int32())
  }
  pub fn int32(&self) -> super::Int32RulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Int32RulesView::default())
  }
  pub fn int32_mut(&mut self) -> super::Int32RulesMut<'_> {
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
  pub fn set_int32(&mut self,
    val: impl ::protobuf::IntoProxied<super::Int32Rules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // int64: optional message validate.Int64Rules
  pub fn has_int64(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_int64(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn int64_opt(&self) -> ::std::option::Option<super::Int64RulesView<'_>> {
    self.has_int64().then(|| self.int64())
  }
  pub fn int64(&self) -> super::Int64RulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Int64RulesView::default())
  }
  pub fn int64_mut(&mut self) -> super::Int64RulesMut<'_> {
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
  pub fn set_int64(&mut self,
    val: impl ::protobuf::IntoProxied<super::Int64Rules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // uint32: optional message validate.UInt32Rules
  pub fn has_uint32(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_uint32(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn uint32_opt(&self) -> ::std::option::Option<super::UInt32RulesView<'_>> {
    self.has_uint32().then(|| self.uint32())
  }
  pub fn uint32(&self) -> super::UInt32RulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::UInt32RulesView::default())
  }
  pub fn uint32_mut(&mut self) -> super::UInt32RulesMut<'_> {
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
  pub fn set_uint32(&mut self,
    val: impl ::protobuf::IntoProxied<super::UInt32Rules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // uint64: optional message validate.UInt64Rules
  pub fn has_uint64(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_uint64(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn uint64_opt(&self) -> ::std::option::Option<super::UInt64RulesView<'_>> {
    self.has_uint64().then(|| self.uint64())
  }
  pub fn uint64(&self) -> super::UInt64RulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::UInt64RulesView::default())
  }
  pub fn uint64_mut(&mut self) -> super::UInt64RulesMut<'_> {
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
  pub fn set_uint64(&mut self,
    val: impl ::protobuf::IntoProxied<super::UInt64Rules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // sint32: optional message validate.SInt32Rules
  pub fn has_sint32(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_sint32(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn sint32_opt(&self) -> ::std::option::Option<super::SInt32RulesView<'_>> {
    self.has_sint32().then(|| self.sint32())
  }
  pub fn sint32(&self) -> super::SInt32RulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SInt32RulesView::default())
  }
  pub fn sint32_mut(&mut self) -> super::SInt32RulesMut<'_> {
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
  pub fn set_sint32(&mut self,
    val: impl ::protobuf::IntoProxied<super::SInt32Rules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // sint64: optional message validate.SInt64Rules
  pub fn has_sint64(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_sint64(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn sint64_opt(&self) -> ::std::option::Option<super::SInt64RulesView<'_>> {
    self.has_sint64().then(|| self.sint64())
  }
  pub fn sint64(&self) -> super::SInt64RulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SInt64RulesView::default())
  }
  pub fn sint64_mut(&mut self) -> super::SInt64RulesMut<'_> {
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
  pub fn set_sint64(&mut self,
    val: impl ::protobuf::IntoProxied<super::SInt64Rules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // fixed32: optional message validate.Fixed32Rules
  pub fn has_fixed32(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_fixed32(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn fixed32_opt(&self) -> ::std::option::Option<super::Fixed32RulesView<'_>> {
    self.has_fixed32().then(|| self.fixed32())
  }
  pub fn fixed32(&self) -> super::Fixed32RulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Fixed32RulesView::default())
  }
  pub fn fixed32_mut(&mut self) -> super::Fixed32RulesMut<'_> {
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
  pub fn set_fixed32(&mut self,
    val: impl ::protobuf::IntoProxied<super::Fixed32Rules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // fixed64: optional message validate.Fixed64Rules
  pub fn has_fixed64(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_fixed64(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn fixed64_opt(&self) -> ::std::option::Option<super::Fixed64RulesView<'_>> {
    self.has_fixed64().then(|| self.fixed64())
  }
  pub fn fixed64(&self) -> super::Fixed64RulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Fixed64RulesView::default())
  }
  pub fn fixed64_mut(&mut self) -> super::Fixed64RulesMut<'_> {
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
  pub fn set_fixed64(&mut self,
    val: impl ::protobuf::IntoProxied<super::Fixed64Rules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  // sfixed32: optional message validate.SFixed32Rules
  pub fn has_sfixed32(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_sfixed32(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn sfixed32_opt(&self) -> ::std::option::Option<super::SFixed32RulesView<'_>> {
    self.has_sfixed32().then(|| self.sfixed32())
  }
  pub fn sfixed32(&self) -> super::SFixed32RulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SFixed32RulesView::default())
  }
  pub fn sfixed32_mut(&mut self) -> super::SFixed32RulesMut<'_> {
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
  pub fn set_sfixed32(&mut self,
    val: impl ::protobuf::IntoProxied<super::SFixed32Rules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // sfixed64: optional message validate.SFixed64Rules
  pub fn has_sfixed64(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_sfixed64(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn sfixed64_opt(&self) -> ::std::option::Option<super::SFixed64RulesView<'_>> {
    self.has_sfixed64().then(|| self.sfixed64())
  }
  pub fn sfixed64(&self) -> super::SFixed64RulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SFixed64RulesView::default())
  }
  pub fn sfixed64_mut(&mut self) -> super::SFixed64RulesMut<'_> {
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
  pub fn set_sfixed64(&mut self,
    val: impl ::protobuf::IntoProxied<super::SFixed64Rules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // bool: optional message validate.BoolRules
  pub fn has_bool(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_bool(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn bool_opt(&self) -> ::std::option::Option<super::BoolRulesView<'_>> {
    self.has_bool().then(|| self.bool())
  }
  pub fn bool(&self) -> super::BoolRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BoolRulesView::default())
  }
  pub fn bool_mut(&mut self) -> super::BoolRulesMut<'_> {
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
  pub fn set_bool(&mut self,
    val: impl ::protobuf::IntoProxied<super::BoolRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val
      );
    }
  }

  // string: optional message validate.StringRules
  pub fn has_string(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_string(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn string_opt(&self) -> ::std::option::Option<super::StringRulesView<'_>> {
    self.has_string().then(|| self.string())
  }
  pub fn string(&self) -> super::StringRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::StringRulesView::default())
  }
  pub fn string_mut(&mut self) -> super::StringRulesMut<'_> {
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
  pub fn set_string(&mut self,
    val: impl ::protobuf::IntoProxied<super::StringRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val
      );
    }
  }

  // bytes: optional message validate.BytesRules
  pub fn has_bytes(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(14)
    }
  }
  pub fn clear_bytes(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        14
      );
    }
  }
  pub fn bytes_opt(&self) -> ::std::option::Option<super::BytesRulesView<'_>> {
    self.has_bytes().then(|| self.bytes())
  }
  pub fn bytes(&self) -> super::BytesRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(14)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BytesRulesView::default())
  }
  pub fn bytes_mut(&mut self) -> super::BytesRulesMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         14, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_bytes(&mut self,
    val: impl ::protobuf::IntoProxied<super::BytesRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        14,
        val
      );
    }
  }

  // enum: optional message validate.EnumRules
  pub fn has_enum(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(15)
    }
  }
  pub fn clear_enum(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        15
      );
    }
  }
  pub fn enum_opt(&self) -> ::std::option::Option<super::EnumRulesView<'_>> {
    self.has_enum().then(|| self.r#enum())
  }
  pub fn r#enum(&self) -> super::EnumRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(15)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::EnumRulesView::default())
  }
  pub fn enum_mut(&mut self) -> super::EnumRulesMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         15, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_enum(&mut self,
    val: impl ::protobuf::IntoProxied<super::EnumRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        15,
        val
      );
    }
  }

  // repeated: optional message validate.RepeatedRules
  pub fn has_repeated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(17)
    }
  }
  pub fn clear_repeated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        17
      );
    }
  }
  pub fn repeated_opt(&self) -> ::std::option::Option<super::RepeatedRulesView<'_>> {
    self.has_repeated().then(|| self.repeated())
  }
  pub fn repeated(&self) -> super::RepeatedRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(17)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RepeatedRulesView::default())
  }
  pub fn repeated_mut(&mut self) -> super::RepeatedRulesMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         17, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_repeated(&mut self,
    val: impl ::protobuf::IntoProxied<super::RepeatedRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        17,
        val
      );
    }
  }

  // map: optional message validate.MapRules
  pub fn has_map(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(18)
    }
  }
  pub fn clear_map(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        18
      );
    }
  }
  pub fn map_opt(&self) -> ::std::option::Option<super::MapRulesView<'_>> {
    self.has_map().then(|| self.map())
  }
  pub fn map(&self) -> super::MapRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(18)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MapRulesView::default())
  }
  pub fn map_mut(&mut self) -> super::MapRulesMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         18, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_map(&mut self,
    val: impl ::protobuf::IntoProxied<super::MapRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        18,
        val
      );
    }
  }

  // any: optional message validate.AnyRules
  pub fn has_any(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(19)
    }
  }
  pub fn clear_any(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        19
      );
    }
  }
  pub fn any_opt(&self) -> ::std::option::Option<super::AnyRulesView<'_>> {
    self.has_any().then(|| self.any())
  }
  pub fn any(&self) -> super::AnyRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(19)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AnyRulesView::default())
  }
  pub fn any_mut(&mut self) -> super::AnyRulesMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         19, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_any(&mut self,
    val: impl ::protobuf::IntoProxied<super::AnyRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        19,
        val
      );
    }
  }

  // duration: optional message validate.DurationRules
  pub fn has_duration(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(20)
    }
  }
  pub fn clear_duration(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        20
      );
    }
  }
  pub fn duration_opt(&self) -> ::std::option::Option<super::DurationRulesView<'_>> {
    self.has_duration().then(|| self.duration())
  }
  pub fn duration(&self) -> super::DurationRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(20)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DurationRulesView::default())
  }
  pub fn duration_mut(&mut self) -> super::DurationRulesMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         20, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_duration(&mut self,
    val: impl ::protobuf::IntoProxied<super::DurationRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        20,
        val
      );
    }
  }

  // timestamp: optional message validate.TimestampRules
  pub fn has_timestamp(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(21)
    }
  }
  pub fn clear_timestamp(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        21
      );
    }
  }
  pub fn timestamp_opt(&self) -> ::std::option::Option<super::TimestampRulesView<'_>> {
    self.has_timestamp().then(|| self.timestamp())
  }
  pub fn timestamp(&self) -> super::TimestampRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(21)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TimestampRulesView::default())
  }
  pub fn timestamp_mut(&mut self) -> super::TimestampRulesMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         21, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_timestamp(&mut self,
    val: impl ::protobuf::IntoProxied<super::TimestampRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        21,
        val
      );
    }
  }

  pub fn r#type(&self) -> super::field_rules::TypeOneof<'_> {
    match &self.r#type_case() {
      super::field_rules::TypeCase::Float =>
          super::field_rules::TypeOneof::Float(self.float()),
      super::field_rules::TypeCase::Double =>
          super::field_rules::TypeOneof::Double(self.double()),
      super::field_rules::TypeCase::Int32 =>
          super::field_rules::TypeOneof::Int32(self.int32()),
      super::field_rules::TypeCase::Int64 =>
          super::field_rules::TypeOneof::Int64(self.int64()),
      super::field_rules::TypeCase::Uint32 =>
          super::field_rules::TypeOneof::Uint32(self.uint32()),
      super::field_rules::TypeCase::Uint64 =>
          super::field_rules::TypeOneof::Uint64(self.uint64()),
      super::field_rules::TypeCase::Sint32 =>
          super::field_rules::TypeOneof::Sint32(self.sint32()),
      super::field_rules::TypeCase::Sint64 =>
          super::field_rules::TypeOneof::Sint64(self.sint64()),
      super::field_rules::TypeCase::Fixed32 =>
          super::field_rules::TypeOneof::Fixed32(self.fixed32()),
      super::field_rules::TypeCase::Fixed64 =>
          super::field_rules::TypeOneof::Fixed64(self.fixed64()),
      super::field_rules::TypeCase::Sfixed32 =>
          super::field_rules::TypeOneof::Sfixed32(self.sfixed32()),
      super::field_rules::TypeCase::Sfixed64 =>
          super::field_rules::TypeOneof::Sfixed64(self.sfixed64()),
      super::field_rules::TypeCase::Bool =>
          super::field_rules::TypeOneof::Bool(self.bool()),
      super::field_rules::TypeCase::String =>
          super::field_rules::TypeOneof::String(self.string()),
      super::field_rules::TypeCase::Bytes =>
          super::field_rules::TypeOneof::Bytes(self.bytes()),
      super::field_rules::TypeCase::Enum =>
          super::field_rules::TypeOneof::Enum(self.r#enum()),
      super::field_rules::TypeCase::Repeated =>
          super::field_rules::TypeOneof::Repeated(self.repeated()),
      super::field_rules::TypeCase::Map =>
          super::field_rules::TypeOneof::Map(self.map()),
      super::field_rules::TypeCase::Any =>
          super::field_rules::TypeOneof::Any(self.any()),
      super::field_rules::TypeCase::Duration =>
          super::field_rules::TypeOneof::Duration(self.duration()),
      super::field_rules::TypeCase::Timestamp =>
          super::field_rules::TypeOneof::Timestamp(self.timestamp()),
      _ => super::field_rules::TypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn r#type_case(&self) -> super::field_rules::TypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::field_rules::TypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `FieldRulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for FieldRulesMut<'_> {}

// SAFETY:
// - `FieldRulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for FieldRulesMut<'_> {}

impl<'msg> ::protobuf::AsView for FieldRulesMut<'msg> {
  type Proxied = FieldRules;
  fn as_view(&self) -> ::protobuf::View<'_, FieldRules> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FieldRulesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, FieldRules>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for FieldRulesMut<'msg> {
  type MutProxied = FieldRules;
  fn as_mut(&mut self) -> FieldRulesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FieldRulesMut<'msg> {
  fn into_mut<'shorter>(self) -> FieldRulesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl FieldRules {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, FieldRules> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FieldRulesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FieldRulesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // message: optional message validate.MessageRules
  pub fn has_message(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(16)
    }
  }
  pub fn clear_message(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        16
      );
    }
  }
  pub fn message_opt(&self) -> ::std::option::Option<super::MessageRulesView<'_>> {
    self.has_message().then(|| self.message())
  }
  pub fn message(&self) -> super::MessageRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(16)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MessageRulesView::default())
  }
  pub fn message_mut(&mut self) -> super::MessageRulesMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         16, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_message(&mut self,
    val: impl ::protobuf::IntoProxied<super::MessageRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        16,
        val
      );
    }
  }

  // float: optional message validate.FloatRules
  pub fn has_float(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_float(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn float_opt(&self) -> ::std::option::Option<super::FloatRulesView<'_>> {
    self.has_float().then(|| self.float())
  }
  pub fn float(&self) -> super::FloatRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FloatRulesView::default())
  }
  pub fn float_mut(&mut self) -> super::FloatRulesMut<'_> {
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
  pub fn set_float(&mut self,
    val: impl ::protobuf::IntoProxied<super::FloatRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // double: optional message validate.DoubleRules
  pub fn has_double(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_double(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn double_opt(&self) -> ::std::option::Option<super::DoubleRulesView<'_>> {
    self.has_double().then(|| self.double())
  }
  pub fn double(&self) -> super::DoubleRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DoubleRulesView::default())
  }
  pub fn double_mut(&mut self) -> super::DoubleRulesMut<'_> {
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
  pub fn set_double(&mut self,
    val: impl ::protobuf::IntoProxied<super::DoubleRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // int32: optional message validate.Int32Rules
  pub fn has_int32(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_int32(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn int32_opt(&self) -> ::std::option::Option<super::Int32RulesView<'_>> {
    self.has_int32().then(|| self.int32())
  }
  pub fn int32(&self) -> super::Int32RulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Int32RulesView::default())
  }
  pub fn int32_mut(&mut self) -> super::Int32RulesMut<'_> {
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
  pub fn set_int32(&mut self,
    val: impl ::protobuf::IntoProxied<super::Int32Rules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // int64: optional message validate.Int64Rules
  pub fn has_int64(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_int64(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn int64_opt(&self) -> ::std::option::Option<super::Int64RulesView<'_>> {
    self.has_int64().then(|| self.int64())
  }
  pub fn int64(&self) -> super::Int64RulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Int64RulesView::default())
  }
  pub fn int64_mut(&mut self) -> super::Int64RulesMut<'_> {
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
  pub fn set_int64(&mut self,
    val: impl ::protobuf::IntoProxied<super::Int64Rules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // uint32: optional message validate.UInt32Rules
  pub fn has_uint32(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_uint32(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn uint32_opt(&self) -> ::std::option::Option<super::UInt32RulesView<'_>> {
    self.has_uint32().then(|| self.uint32())
  }
  pub fn uint32(&self) -> super::UInt32RulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::UInt32RulesView::default())
  }
  pub fn uint32_mut(&mut self) -> super::UInt32RulesMut<'_> {
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
  pub fn set_uint32(&mut self,
    val: impl ::protobuf::IntoProxied<super::UInt32Rules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // uint64: optional message validate.UInt64Rules
  pub fn has_uint64(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_uint64(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn uint64_opt(&self) -> ::std::option::Option<super::UInt64RulesView<'_>> {
    self.has_uint64().then(|| self.uint64())
  }
  pub fn uint64(&self) -> super::UInt64RulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::UInt64RulesView::default())
  }
  pub fn uint64_mut(&mut self) -> super::UInt64RulesMut<'_> {
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
  pub fn set_uint64(&mut self,
    val: impl ::protobuf::IntoProxied<super::UInt64Rules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // sint32: optional message validate.SInt32Rules
  pub fn has_sint32(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_sint32(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn sint32_opt(&self) -> ::std::option::Option<super::SInt32RulesView<'_>> {
    self.has_sint32().then(|| self.sint32())
  }
  pub fn sint32(&self) -> super::SInt32RulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SInt32RulesView::default())
  }
  pub fn sint32_mut(&mut self) -> super::SInt32RulesMut<'_> {
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
  pub fn set_sint32(&mut self,
    val: impl ::protobuf::IntoProxied<super::SInt32Rules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // sint64: optional message validate.SInt64Rules
  pub fn has_sint64(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_sint64(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn sint64_opt(&self) -> ::std::option::Option<super::SInt64RulesView<'_>> {
    self.has_sint64().then(|| self.sint64())
  }
  pub fn sint64(&self) -> super::SInt64RulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SInt64RulesView::default())
  }
  pub fn sint64_mut(&mut self) -> super::SInt64RulesMut<'_> {
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
  pub fn set_sint64(&mut self,
    val: impl ::protobuf::IntoProxied<super::SInt64Rules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // fixed32: optional message validate.Fixed32Rules
  pub fn has_fixed32(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_fixed32(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn fixed32_opt(&self) -> ::std::option::Option<super::Fixed32RulesView<'_>> {
    self.has_fixed32().then(|| self.fixed32())
  }
  pub fn fixed32(&self) -> super::Fixed32RulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Fixed32RulesView::default())
  }
  pub fn fixed32_mut(&mut self) -> super::Fixed32RulesMut<'_> {
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
  pub fn set_fixed32(&mut self,
    val: impl ::protobuf::IntoProxied<super::Fixed32Rules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // fixed64: optional message validate.Fixed64Rules
  pub fn has_fixed64(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_fixed64(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn fixed64_opt(&self) -> ::std::option::Option<super::Fixed64RulesView<'_>> {
    self.has_fixed64().then(|| self.fixed64())
  }
  pub fn fixed64(&self) -> super::Fixed64RulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Fixed64RulesView::default())
  }
  pub fn fixed64_mut(&mut self) -> super::Fixed64RulesMut<'_> {
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
  pub fn set_fixed64(&mut self,
    val: impl ::protobuf::IntoProxied<super::Fixed64Rules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  // sfixed32: optional message validate.SFixed32Rules
  pub fn has_sfixed32(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_sfixed32(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn sfixed32_opt(&self) -> ::std::option::Option<super::SFixed32RulesView<'_>> {
    self.has_sfixed32().then(|| self.sfixed32())
  }
  pub fn sfixed32(&self) -> super::SFixed32RulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SFixed32RulesView::default())
  }
  pub fn sfixed32_mut(&mut self) -> super::SFixed32RulesMut<'_> {
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
  pub fn set_sfixed32(&mut self,
    val: impl ::protobuf::IntoProxied<super::SFixed32Rules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // sfixed64: optional message validate.SFixed64Rules
  pub fn has_sfixed64(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_sfixed64(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn sfixed64_opt(&self) -> ::std::option::Option<super::SFixed64RulesView<'_>> {
    self.has_sfixed64().then(|| self.sfixed64())
  }
  pub fn sfixed64(&self) -> super::SFixed64RulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SFixed64RulesView::default())
  }
  pub fn sfixed64_mut(&mut self) -> super::SFixed64RulesMut<'_> {
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
  pub fn set_sfixed64(&mut self,
    val: impl ::protobuf::IntoProxied<super::SFixed64Rules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // bool: optional message validate.BoolRules
  pub fn has_bool(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_bool(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn bool_opt(&self) -> ::std::option::Option<super::BoolRulesView<'_>> {
    self.has_bool().then(|| self.bool())
  }
  pub fn bool(&self) -> super::BoolRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BoolRulesView::default())
  }
  pub fn bool_mut(&mut self) -> super::BoolRulesMut<'_> {
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
  pub fn set_bool(&mut self,
    val: impl ::protobuf::IntoProxied<super::BoolRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val
      );
    }
  }

  // string: optional message validate.StringRules
  pub fn has_string(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_string(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn string_opt(&self) -> ::std::option::Option<super::StringRulesView<'_>> {
    self.has_string().then(|| self.string())
  }
  pub fn string(&self) -> super::StringRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::StringRulesView::default())
  }
  pub fn string_mut(&mut self) -> super::StringRulesMut<'_> {
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
  pub fn set_string(&mut self,
    val: impl ::protobuf::IntoProxied<super::StringRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val
      );
    }
  }

  // bytes: optional message validate.BytesRules
  pub fn has_bytes(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(14)
    }
  }
  pub fn clear_bytes(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        14
      );
    }
  }
  pub fn bytes_opt(&self) -> ::std::option::Option<super::BytesRulesView<'_>> {
    self.has_bytes().then(|| self.bytes())
  }
  pub fn bytes(&self) -> super::BytesRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(14)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BytesRulesView::default())
  }
  pub fn bytes_mut(&mut self) -> super::BytesRulesMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         14, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_bytes(&mut self,
    val: impl ::protobuf::IntoProxied<super::BytesRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        14,
        val
      );
    }
  }

  // enum: optional message validate.EnumRules
  pub fn has_enum(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(15)
    }
  }
  pub fn clear_enum(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        15
      );
    }
  }
  pub fn enum_opt(&self) -> ::std::option::Option<super::EnumRulesView<'_>> {
    self.has_enum().then(|| self.r#enum())
  }
  pub fn r#enum(&self) -> super::EnumRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(15)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::EnumRulesView::default())
  }
  pub fn enum_mut(&mut self) -> super::EnumRulesMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         15, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_enum(&mut self,
    val: impl ::protobuf::IntoProxied<super::EnumRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        15,
        val
      );
    }
  }

  // repeated: optional message validate.RepeatedRules
  pub fn has_repeated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(17)
    }
  }
  pub fn clear_repeated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        17
      );
    }
  }
  pub fn repeated_opt(&self) -> ::std::option::Option<super::RepeatedRulesView<'_>> {
    self.has_repeated().then(|| self.repeated())
  }
  pub fn repeated(&self) -> super::RepeatedRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(17)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RepeatedRulesView::default())
  }
  pub fn repeated_mut(&mut self) -> super::RepeatedRulesMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         17, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_repeated(&mut self,
    val: impl ::protobuf::IntoProxied<super::RepeatedRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        17,
        val
      );
    }
  }

  // map: optional message validate.MapRules
  pub fn has_map(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(18)
    }
  }
  pub fn clear_map(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        18
      );
    }
  }
  pub fn map_opt(&self) -> ::std::option::Option<super::MapRulesView<'_>> {
    self.has_map().then(|| self.map())
  }
  pub fn map(&self) -> super::MapRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(18)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MapRulesView::default())
  }
  pub fn map_mut(&mut self) -> super::MapRulesMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         18, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_map(&mut self,
    val: impl ::protobuf::IntoProxied<super::MapRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        18,
        val
      );
    }
  }

  // any: optional message validate.AnyRules
  pub fn has_any(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(19)
    }
  }
  pub fn clear_any(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        19
      );
    }
  }
  pub fn any_opt(&self) -> ::std::option::Option<super::AnyRulesView<'_>> {
    self.has_any().then(|| self.any())
  }
  pub fn any(&self) -> super::AnyRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(19)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AnyRulesView::default())
  }
  pub fn any_mut(&mut self) -> super::AnyRulesMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         19, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_any(&mut self,
    val: impl ::protobuf::IntoProxied<super::AnyRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        19,
        val
      );
    }
  }

  // duration: optional message validate.DurationRules
  pub fn has_duration(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(20)
    }
  }
  pub fn clear_duration(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        20
      );
    }
  }
  pub fn duration_opt(&self) -> ::std::option::Option<super::DurationRulesView<'_>> {
    self.has_duration().then(|| self.duration())
  }
  pub fn duration(&self) -> super::DurationRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(20)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DurationRulesView::default())
  }
  pub fn duration_mut(&mut self) -> super::DurationRulesMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         20, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_duration(&mut self,
    val: impl ::protobuf::IntoProxied<super::DurationRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        20,
        val
      );
    }
  }

  // timestamp: optional message validate.TimestampRules
  pub fn has_timestamp(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(21)
    }
  }
  pub fn clear_timestamp(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        21
      );
    }
  }
  pub fn timestamp_opt(&self) -> ::std::option::Option<super::TimestampRulesView<'_>> {
    self.has_timestamp().then(|| self.timestamp())
  }
  pub fn timestamp(&self) -> super::TimestampRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(21)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TimestampRulesView::default())
  }
  pub fn timestamp_mut(&mut self) -> super::TimestampRulesMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         21, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_timestamp(&mut self,
    val: impl ::protobuf::IntoProxied<super::TimestampRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        21,
        val
      );
    }
  }

  pub fn r#type(&self) -> super::field_rules::TypeOneof<'_> {
    match &self.r#type_case() {
      super::field_rules::TypeCase::Float =>
          super::field_rules::TypeOneof::Float(self.float()),
      super::field_rules::TypeCase::Double =>
          super::field_rules::TypeOneof::Double(self.double()),
      super::field_rules::TypeCase::Int32 =>
          super::field_rules::TypeOneof::Int32(self.int32()),
      super::field_rules::TypeCase::Int64 =>
          super::field_rules::TypeOneof::Int64(self.int64()),
      super::field_rules::TypeCase::Uint32 =>
          super::field_rules::TypeOneof::Uint32(self.uint32()),
      super::field_rules::TypeCase::Uint64 =>
          super::field_rules::TypeOneof::Uint64(self.uint64()),
      super::field_rules::TypeCase::Sint32 =>
          super::field_rules::TypeOneof::Sint32(self.sint32()),
      super::field_rules::TypeCase::Sint64 =>
          super::field_rules::TypeOneof::Sint64(self.sint64()),
      super::field_rules::TypeCase::Fixed32 =>
          super::field_rules::TypeOneof::Fixed32(self.fixed32()),
      super::field_rules::TypeCase::Fixed64 =>
          super::field_rules::TypeOneof::Fixed64(self.fixed64()),
      super::field_rules::TypeCase::Sfixed32 =>
          super::field_rules::TypeOneof::Sfixed32(self.sfixed32()),
      super::field_rules::TypeCase::Sfixed64 =>
          super::field_rules::TypeOneof::Sfixed64(self.sfixed64()),
      super::field_rules::TypeCase::Bool =>
          super::field_rules::TypeOneof::Bool(self.bool()),
      super::field_rules::TypeCase::String =>
          super::field_rules::TypeOneof::String(self.string()),
      super::field_rules::TypeCase::Bytes =>
          super::field_rules::TypeOneof::Bytes(self.bytes()),
      super::field_rules::TypeCase::Enum =>
          super::field_rules::TypeOneof::Enum(self.r#enum()),
      super::field_rules::TypeCase::Repeated =>
          super::field_rules::TypeOneof::Repeated(self.repeated()),
      super::field_rules::TypeCase::Map =>
          super::field_rules::TypeOneof::Map(self.map()),
      super::field_rules::TypeCase::Any =>
          super::field_rules::TypeOneof::Any(self.any()),
      super::field_rules::TypeCase::Duration =>
          super::field_rules::TypeOneof::Duration(self.duration()),
      super::field_rules::TypeCase::Timestamp =>
          super::field_rules::TypeOneof::Timestamp(self.timestamp()),
      _ => super::field_rules::TypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn r#type_case(&self) -> super::field_rules::TypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::field_rules::TypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl FieldRules

impl ::std::ops::Drop for FieldRules {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for FieldRules {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for FieldRules {
  type Proxied = Self;
  fn as_view(&self) -> FieldRulesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for FieldRules {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FieldRulesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FieldRules {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::validate__FieldRules_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3333333333333333333333^!|#|$|%|&|(|)|*|+|,|-|.|/|0|1|2|4|5|6|7|8");
        super::validate__MapRules_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$,,/33/");
        super::validate__RepeatedRules_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$,,/3/");
        ::protobuf::__internal::runtime::link_mini_table(
            super::validate__FieldRules_msg_init.0, &[<super::FloatRules as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::DoubleRules as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Int32Rules as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Int64Rules as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::UInt32Rules as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::UInt64Rules as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::SInt32Rules as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::SInt64Rules as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Fixed32Rules as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Fixed64Rules as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::SFixed32Rules as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::SFixed64Rules as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::BoolRules as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::StringRules as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::BytesRules as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::EnumRules as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::MessageRules as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            super::validate__RepeatedRules_msg_init.0,
            super::validate__MapRules_msg_init.0,
            <super::AnyRules as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::DurationRules as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::TimestampRules as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::validate__MapRules_msg_init.0, &[super::validate__FieldRules_msg_init.0,
            super::validate__FieldRules_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::validate__RepeatedRules_msg_init.0, &[super::validate__FieldRules_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::validate__FieldRules_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FieldRules {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FieldRules {
  type Msg = FieldRules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FieldRules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FieldRules {
  type Msg = FieldRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FieldRules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FieldRulesMut<'_> {
  type Msg = FieldRules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FieldRules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FieldRulesMut<'_> {
  type Msg = FieldRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FieldRules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FieldRulesView<'_> {
  type Msg = FieldRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FieldRules> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FieldRulesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod field_rules {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum TypeOneof<'msg> {
  Float(::protobuf::View<'msg, super::super::FloatRules>) = 1,
  Double(::protobuf::View<'msg, super::super::DoubleRules>) = 2,
  Int32(::protobuf::View<'msg, super::super::Int32Rules>) = 3,
  Int64(::protobuf::View<'msg, super::super::Int64Rules>) = 4,
  Uint32(::protobuf::View<'msg, super::super::UInt32Rules>) = 5,
  Uint64(::protobuf::View<'msg, super::super::UInt64Rules>) = 6,
  Sint32(::protobuf::View<'msg, super::super::SInt32Rules>) = 7,
  Sint64(::protobuf::View<'msg, super::super::SInt64Rules>) = 8,
  Fixed32(::protobuf::View<'msg, super::super::Fixed32Rules>) = 9,
  Fixed64(::protobuf::View<'msg, super::super::Fixed64Rules>) = 10,
  Sfixed32(::protobuf::View<'msg, super::super::SFixed32Rules>) = 11,
  Sfixed64(::protobuf::View<'msg, super::super::SFixed64Rules>) = 12,
  Bool(::protobuf::View<'msg, super::super::BoolRules>) = 13,
  String(::protobuf::View<'msg, super::super::StringRules>) = 14,
  Bytes(::protobuf::View<'msg, super::super::BytesRules>) = 15,
  Enum(::protobuf::View<'msg, super::super::EnumRules>) = 16,
  Repeated(::protobuf::View<'msg, super::super::RepeatedRules>) = 18,
  Map(::protobuf::View<'msg, super::super::MapRules>) = 19,
  Any(::protobuf::View<'msg, super::super::AnyRules>) = 20,
  Duration(::protobuf::View<'msg, super::super::DurationRules>) = 21,
  Timestamp(::protobuf::View<'msg, super::super::TimestampRules>) = 22,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum TypeCase {
  Float = 1,
  Double = 2,
  Int32 = 3,
  Int64 = 4,
  Uint32 = 5,
  Uint64 = 6,
  Sint32 = 7,
  Sint64 = 8,
  Fixed32 = 9,
  Fixed64 = 10,
  Sfixed32 = 11,
  Sfixed64 = 12,
  Bool = 13,
  String = 14,
  Bytes = 15,
  Enum = 16,
  Repeated = 18,
  Map = 19,
  Any = 20,
  Duration = 21,
  Timestamp = 22,

  not_set = 0
}

impl TypeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<TypeCase> {
    match v {
      0 => Some(TypeCase::not_set),
      1 => Some(TypeCase::Float),
      2 => Some(TypeCase::Double),
      3 => Some(TypeCase::Int32),
      4 => Some(TypeCase::Int64),
      5 => Some(TypeCase::Uint32),
      6 => Some(TypeCase::Uint64),
      7 => Some(TypeCase::Sint32),
      8 => Some(TypeCase::Sint64),
      9 => Some(TypeCase::Fixed32),
      10 => Some(TypeCase::Fixed64),
      11 => Some(TypeCase::Sfixed32),
      12 => Some(TypeCase::Sfixed64),
      13 => Some(TypeCase::Bool),
      14 => Some(TypeCase::String),
      15 => Some(TypeCase::Bytes),
      16 => Some(TypeCase::Enum),
      18 => Some(TypeCase::Repeated),
      19 => Some(TypeCase::Map),
      20 => Some(TypeCase::Any),
      21 => Some(TypeCase::Duration),
      22 => Some(TypeCase::Timestamp),
      _ => None
    }
  }
}
}  // pub mod field_rules


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut validate__FloatRules_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct FloatRules {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<FloatRules>
}

impl ::protobuf::Message for FloatRules {
  type MessageView<'msg> = FloatRulesView<'msg>;
  type MessageMut<'msg> = FloatRulesMut<'msg>;
}

impl ::std::default::Default for FloatRules {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for FloatRules {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `FloatRules` is `Sync` because it does not implement interior mutability.
//    Neither does `FloatRulesMut`.
unsafe impl ::std::marker::Sync for FloatRules {}

// SAFETY:
// - `FloatRules` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for FloatRules {}

impl ::protobuf::Proxied for FloatRules {
  type View<'msg> = FloatRulesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for FloatRules {}

impl ::protobuf::MutProxied for FloatRules {
  type Mut<'msg> = FloatRulesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FloatRulesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FloatRules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FloatRulesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FloatRulesView<'msg> {
  type Message = FloatRules;
}

impl ::std::fmt::Debug for FloatRulesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FloatRulesView<'_> {
  fn default() -> FloatRulesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, FloatRules>> for FloatRulesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FloatRules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FloatRulesView<'msg> {

  pub fn to_owned(&self) -> FloatRules {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // const: optional float
  pub fn has_const(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn const_opt(self) -> ::std::option::Option<f32> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        0, (0f32).into()
      ).try_into().unwrap()
    }
  }

  // lt: optional float
  pub fn has_lt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn lt_opt(self) -> ::std::option::Option<f32> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        1, (0f32).into()
      ).try_into().unwrap()
    }
  }

  // lte: optional float
  pub fn has_lte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn lte_opt(self) -> ::std::option::Option<f32> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        2, (0f32).into()
      ).try_into().unwrap()
    }
  }

  // gt: optional float
  pub fn has_gt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn gt_opt(self) -> ::std::option::Option<f32> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        3, (0f32).into()
      ).try_into().unwrap()
    }
  }

  // gte: optional float
  pub fn has_gte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn gte_opt(self) -> ::std::option::Option<f32> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        4, (0f32).into()
      ).try_into().unwrap()
    }
  }

  // in: repeated float
  pub fn r#in(self) -> ::protobuf::RepeatedView<'msg, f32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<f32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // not_in: repeated float
  pub fn not_in(self) -> ::protobuf::RepeatedView<'msg, f32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<f32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn ignore_empty_opt(self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `FloatRulesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for FloatRulesView<'_> {}

// SAFETY:
// - `FloatRulesView` is `Send` because while its alive a `FloatRulesMut` cannot.
// - `FloatRulesView` does not use thread-local data.
unsafe impl ::std::marker::Send for FloatRulesView<'_> {}

impl<'msg> ::protobuf::AsView for FloatRulesView<'msg> {
  type Proxied = FloatRules;
  fn as_view(&self) -> ::protobuf::View<'msg, FloatRules> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FloatRulesView<'msg> {
  fn into_view<'shorter>(self) -> FloatRulesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<FloatRules> for FloatRulesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FloatRules {
    let mut dst = FloatRules::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<FloatRules> for FloatRulesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FloatRules {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for FloatRules {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FloatRulesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FloatRulesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FloatRulesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FloatRules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FloatRulesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FloatRulesMut<'msg> {
  type Message = FloatRules;
}

impl ::std::fmt::Debug for FloatRulesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, FloatRules>> for FloatRulesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FloatRules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FloatRulesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, FloatRules> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> FloatRules {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // const: optional float
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<f32> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        0, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_const(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        0, val.into()
      )
    }
  }

  // lt: optional float
  pub fn has_lt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_lt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn lt_opt(&self) -> ::std::option::Option<f32> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        1, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lt(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        1, val.into()
      )
    }
  }

  // lte: optional float
  pub fn has_lte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_lte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn lte_opt(&self) -> ::std::option::Option<f32> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        2, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lte(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        2, val.into()
      )
    }
  }

  // gt: optional float
  pub fn has_gt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_gt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn gt_opt(&self) -> ::std::option::Option<f32> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        3, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gt(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        3, val.into()
      )
    }
  }

  // gte: optional float
  pub fn has_gte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_gte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn gte_opt(&self) -> ::std::option::Option<f32> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        4, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gte(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        4, val.into()
      )
    }
  }

  // in: repeated float
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, f32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<f32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, f32> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<f32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // not_in: repeated float
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, f32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<f32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, f32> {
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<f32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

}

// SAFETY:
// - `FloatRulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for FloatRulesMut<'_> {}

// SAFETY:
// - `FloatRulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for FloatRulesMut<'_> {}

impl<'msg> ::protobuf::AsView for FloatRulesMut<'msg> {
  type Proxied = FloatRules;
  fn as_view(&self) -> ::protobuf::View<'_, FloatRules> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FloatRulesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, FloatRules>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for FloatRulesMut<'msg> {
  type MutProxied = FloatRules;
  fn as_mut(&mut self) -> FloatRulesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FloatRulesMut<'msg> {
  fn into_mut<'shorter>(self) -> FloatRulesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl FloatRules {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, FloatRules> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FloatRulesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FloatRulesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // const: optional float
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<f32> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        0, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_const(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        0, val.into()
      )
    }
  }

  // lt: optional float
  pub fn has_lt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_lt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn lt_opt(&self) -> ::std::option::Option<f32> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        1, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lt(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        1, val.into()
      )
    }
  }

  // lte: optional float
  pub fn has_lte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_lte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn lte_opt(&self) -> ::std::option::Option<f32> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        2, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lte(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        2, val.into()
      )
    }
  }

  // gt: optional float
  pub fn has_gt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_gt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn gt_opt(&self) -> ::std::option::Option<f32> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        3, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gt(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        3, val.into()
      )
    }
  }

  // gte: optional float
  pub fn has_gte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_gte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn gte_opt(&self) -> ::std::option::Option<f32> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        4, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gte(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        4, val.into()
      )
    }
  }

  // in: repeated float
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, f32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<f32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, f32> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<f32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // not_in: repeated float
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, f32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<f32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, f32> {
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<f32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

}  // impl FloatRules

impl ::std::ops::Drop for FloatRules {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for FloatRules {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for FloatRules {
  type Proxied = Self;
  fn as_view(&self) -> FloatRulesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for FloatRules {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FloatRulesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FloatRules {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::validate__FloatRules_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$!!!!!77/");
        ::protobuf::__internal::runtime::link_mini_table(
            super::validate__FloatRules_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::validate__FloatRules_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FloatRules {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FloatRules {
  type Msg = FloatRules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FloatRules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FloatRules {
  type Msg = FloatRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FloatRules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FloatRulesMut<'_> {
  type Msg = FloatRules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FloatRules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FloatRulesMut<'_> {
  type Msg = FloatRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FloatRules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FloatRulesView<'_> {
  type Msg = FloatRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FloatRules> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FloatRulesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut validate__DoubleRules_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DoubleRules {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DoubleRules>
}

impl ::protobuf::Message for DoubleRules {
  type MessageView<'msg> = DoubleRulesView<'msg>;
  type MessageMut<'msg> = DoubleRulesMut<'msg>;
}

impl ::std::default::Default for DoubleRules {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DoubleRules {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DoubleRules` is `Sync` because it does not implement interior mutability.
//    Neither does `DoubleRulesMut`.
unsafe impl ::std::marker::Sync for DoubleRules {}

// SAFETY:
// - `DoubleRules` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DoubleRules {}

impl ::protobuf::Proxied for DoubleRules {
  type View<'msg> = DoubleRulesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DoubleRules {}

impl ::protobuf::MutProxied for DoubleRules {
  type Mut<'msg> = DoubleRulesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DoubleRulesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DoubleRules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DoubleRulesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DoubleRulesView<'msg> {
  type Message = DoubleRules;
}

impl ::std::fmt::Debug for DoubleRulesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DoubleRulesView<'_> {
  fn default() -> DoubleRulesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DoubleRules>> for DoubleRulesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DoubleRules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DoubleRulesView<'msg> {

  pub fn to_owned(&self) -> DoubleRules {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // const: optional double
  pub fn has_const(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn const_opt(self) -> ::std::option::Option<f64> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        0, (0f64).into()
      ).try_into().unwrap()
    }
  }

  // lt: optional double
  pub fn has_lt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn lt_opt(self) -> ::std::option::Option<f64> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        1, (0f64).into()
      ).try_into().unwrap()
    }
  }

  // lte: optional double
  pub fn has_lte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn lte_opt(self) -> ::std::option::Option<f64> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        2, (0f64).into()
      ).try_into().unwrap()
    }
  }

  // gt: optional double
  pub fn has_gt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn gt_opt(self) -> ::std::option::Option<f64> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        3, (0f64).into()
      ).try_into().unwrap()
    }
  }

  // gte: optional double
  pub fn has_gte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn gte_opt(self) -> ::std::option::Option<f64> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        4, (0f64).into()
      ).try_into().unwrap()
    }
  }

  // in: repeated double
  pub fn r#in(self) -> ::protobuf::RepeatedView<'msg, f64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<f64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // not_in: repeated double
  pub fn not_in(self) -> ::protobuf::RepeatedView<'msg, f64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<f64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn ignore_empty_opt(self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `DoubleRulesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DoubleRulesView<'_> {}

// SAFETY:
// - `DoubleRulesView` is `Send` because while its alive a `DoubleRulesMut` cannot.
// - `DoubleRulesView` does not use thread-local data.
unsafe impl ::std::marker::Send for DoubleRulesView<'_> {}

impl<'msg> ::protobuf::AsView for DoubleRulesView<'msg> {
  type Proxied = DoubleRules;
  fn as_view(&self) -> ::protobuf::View<'msg, DoubleRules> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DoubleRulesView<'msg> {
  fn into_view<'shorter>(self) -> DoubleRulesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DoubleRules> for DoubleRulesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DoubleRules {
    let mut dst = DoubleRules::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DoubleRules> for DoubleRulesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DoubleRules {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DoubleRules {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DoubleRulesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DoubleRulesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DoubleRulesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DoubleRules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DoubleRulesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DoubleRulesMut<'msg> {
  type Message = DoubleRules;
}

impl ::std::fmt::Debug for DoubleRulesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DoubleRules>> for DoubleRulesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DoubleRules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DoubleRulesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DoubleRules> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DoubleRules {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // const: optional double
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<f64> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        0, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_const(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        0, val.into()
      )
    }
  }

  // lt: optional double
  pub fn has_lt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_lt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn lt_opt(&self) -> ::std::option::Option<f64> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        1, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lt(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        1, val.into()
      )
    }
  }

  // lte: optional double
  pub fn has_lte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_lte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn lte_opt(&self) -> ::std::option::Option<f64> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        2, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lte(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        2, val.into()
      )
    }
  }

  // gt: optional double
  pub fn has_gt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_gt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn gt_opt(&self) -> ::std::option::Option<f64> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        3, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gt(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        3, val.into()
      )
    }
  }

  // gte: optional double
  pub fn has_gte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_gte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn gte_opt(&self) -> ::std::option::Option<f64> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        4, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gte(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        4, val.into()
      )
    }
  }

  // in: repeated double
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, f64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<f64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, f64> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<f64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // not_in: repeated double
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, f64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<f64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, f64> {
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<f64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

}

// SAFETY:
// - `DoubleRulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DoubleRulesMut<'_> {}

// SAFETY:
// - `DoubleRulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DoubleRulesMut<'_> {}

impl<'msg> ::protobuf::AsView for DoubleRulesMut<'msg> {
  type Proxied = DoubleRules;
  fn as_view(&self) -> ::protobuf::View<'_, DoubleRules> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DoubleRulesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DoubleRules>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DoubleRulesMut<'msg> {
  type MutProxied = DoubleRules;
  fn as_mut(&mut self) -> DoubleRulesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DoubleRulesMut<'msg> {
  fn into_mut<'shorter>(self) -> DoubleRulesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DoubleRules {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DoubleRules> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DoubleRulesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DoubleRulesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // const: optional double
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<f64> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        0, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_const(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        0, val.into()
      )
    }
  }

  // lt: optional double
  pub fn has_lt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_lt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn lt_opt(&self) -> ::std::option::Option<f64> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        1, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lt(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        1, val.into()
      )
    }
  }

  // lte: optional double
  pub fn has_lte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_lte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn lte_opt(&self) -> ::std::option::Option<f64> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        2, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lte(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        2, val.into()
      )
    }
  }

  // gt: optional double
  pub fn has_gt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_gt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn gt_opt(&self) -> ::std::option::Option<f64> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        3, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gt(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        3, val.into()
      )
    }
  }

  // gte: optional double
  pub fn has_gte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_gte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn gte_opt(&self) -> ::std::option::Option<f64> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        4, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gte(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        4, val.into()
      )
    }
  }

  // in: repeated double
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, f64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<f64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, f64> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<f64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // not_in: repeated double
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, f64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<f64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, f64> {
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<f64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

}  // impl DoubleRules

impl ::std::ops::Drop for DoubleRules {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DoubleRules {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DoubleRules {
  type Proxied = Self;
  fn as_view(&self) -> DoubleRulesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DoubleRules {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DoubleRulesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DoubleRules {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::validate__DoubleRules_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$     66/");
        ::protobuf::__internal::runtime::link_mini_table(
            super::validate__DoubleRules_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::validate__DoubleRules_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DoubleRules {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DoubleRules {
  type Msg = DoubleRules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DoubleRules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DoubleRules {
  type Msg = DoubleRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DoubleRules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DoubleRulesMut<'_> {
  type Msg = DoubleRules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DoubleRules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DoubleRulesMut<'_> {
  type Msg = DoubleRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DoubleRules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DoubleRulesView<'_> {
  type Msg = DoubleRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DoubleRules> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DoubleRulesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut validate__Int32Rules_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Int32Rules {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Int32Rules>
}

impl ::protobuf::Message for Int32Rules {
  type MessageView<'msg> = Int32RulesView<'msg>;
  type MessageMut<'msg> = Int32RulesMut<'msg>;
}

impl ::std::default::Default for Int32Rules {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Int32Rules {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Int32Rules` is `Sync` because it does not implement interior mutability.
//    Neither does `Int32RulesMut`.
unsafe impl ::std::marker::Sync for Int32Rules {}

// SAFETY:
// - `Int32Rules` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Int32Rules {}

impl ::protobuf::Proxied for Int32Rules {
  type View<'msg> = Int32RulesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Int32Rules {}

impl ::protobuf::MutProxied for Int32Rules {
  type Mut<'msg> = Int32RulesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Int32RulesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Int32Rules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Int32RulesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for Int32RulesView<'msg> {
  type Message = Int32Rules;
}

impl ::std::fmt::Debug for Int32RulesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for Int32RulesView<'_> {
  fn default() -> Int32RulesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Int32Rules>> for Int32RulesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Int32Rules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Int32RulesView<'msg> {

  pub fn to_owned(&self) -> Int32Rules {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // const: optional int32
  pub fn has_const(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn const_opt(self) -> ::std::option::Option<i32> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // lt: optional int32
  pub fn has_lt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn lt_opt(self) -> ::std::option::Option<i32> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // lte: optional int32
  pub fn has_lte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn lte_opt(self) -> ::std::option::Option<i32> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // gt: optional int32
  pub fn has_gt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn gt_opt(self) -> ::std::option::Option<i32> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // gte: optional int32
  pub fn has_gte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn gte_opt(self) -> ::std::option::Option<i32> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // in: repeated int32
  pub fn r#in(self) -> ::protobuf::RepeatedView<'msg, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // not_in: repeated int32
  pub fn not_in(self) -> ::protobuf::RepeatedView<'msg, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn ignore_empty_opt(self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `Int32RulesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for Int32RulesView<'_> {}

// SAFETY:
// - `Int32RulesView` is `Send` because while its alive a `Int32RulesMut` cannot.
// - `Int32RulesView` does not use thread-local data.
unsafe impl ::std::marker::Send for Int32RulesView<'_> {}

impl<'msg> ::protobuf::AsView for Int32RulesView<'msg> {
  type Proxied = Int32Rules;
  fn as_view(&self) -> ::protobuf::View<'msg, Int32Rules> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Int32RulesView<'msg> {
  fn into_view<'shorter>(self) -> Int32RulesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Int32Rules> for Int32RulesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Int32Rules {
    let mut dst = Int32Rules::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Int32Rules> for Int32RulesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Int32Rules {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Int32Rules {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for Int32RulesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for Int32RulesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct Int32RulesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Int32Rules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Int32RulesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for Int32RulesMut<'msg> {
  type Message = Int32Rules;
}

impl ::std::fmt::Debug for Int32RulesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Int32Rules>> for Int32RulesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Int32Rules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Int32RulesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Int32Rules> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Int32Rules {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // const: optional int32
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<i32> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_const(&mut self, val: i32) {
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

  // lt: optional int32
  pub fn has_lt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_lt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn lt_opt(&self) -> ::std::option::Option<i32> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lt(&mut self, val: i32) {
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

  // lte: optional int32
  pub fn has_lte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_lte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn lte_opt(&self) -> ::std::option::Option<i32> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lte(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

  // gt: optional int32
  pub fn has_gt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_gt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn gt_opt(&self) -> ::std::option::Option<i32> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gt(&mut self, val: i32) {
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

  // gte: optional int32
  pub fn has_gte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_gte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn gte_opt(&self) -> ::std::option::Option<i32> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gte(&mut self, val: i32) {
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

  // in: repeated int32
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i32> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // not_in: repeated int32
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i32> {
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

}

// SAFETY:
// - `Int32RulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for Int32RulesMut<'_> {}

// SAFETY:
// - `Int32RulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for Int32RulesMut<'_> {}

impl<'msg> ::protobuf::AsView for Int32RulesMut<'msg> {
  type Proxied = Int32Rules;
  fn as_view(&self) -> ::protobuf::View<'_, Int32Rules> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Int32RulesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Int32Rules>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for Int32RulesMut<'msg> {
  type MutProxied = Int32Rules;
  fn as_mut(&mut self) -> Int32RulesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for Int32RulesMut<'msg> {
  fn into_mut<'shorter>(self) -> Int32RulesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Int32Rules {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Int32Rules> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> Int32RulesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> Int32RulesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // const: optional int32
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<i32> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_const(&mut self, val: i32) {
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

  // lt: optional int32
  pub fn has_lt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_lt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn lt_opt(&self) -> ::std::option::Option<i32> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lt(&mut self, val: i32) {
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

  // lte: optional int32
  pub fn has_lte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_lte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn lte_opt(&self) -> ::std::option::Option<i32> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lte(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

  // gt: optional int32
  pub fn has_gt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_gt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn gt_opt(&self) -> ::std::option::Option<i32> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gt(&mut self, val: i32) {
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

  // gte: optional int32
  pub fn has_gte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_gte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn gte_opt(&self) -> ::std::option::Option<i32> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gte(&mut self, val: i32) {
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

  // in: repeated int32
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i32> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // not_in: repeated int32
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i32> {
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

}  // impl Int32Rules

impl ::std::ops::Drop for Int32Rules {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Int32Rules {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Int32Rules {
  type Proxied = Self;
  fn as_view(&self) -> Int32RulesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Int32Rules {
  type MutProxied = Self;
  fn as_mut(&mut self) -> Int32RulesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Int32Rules {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::validate__Int32Rules_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$(((((<</");
        ::protobuf::__internal::runtime::link_mini_table(
            super::validate__Int32Rules_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::validate__Int32Rules_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Int32Rules {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Int32Rules {
  type Msg = Int32Rules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Int32Rules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Int32Rules {
  type Msg = Int32Rules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Int32Rules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Int32RulesMut<'_> {
  type Msg = Int32Rules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Int32Rules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Int32RulesMut<'_> {
  type Msg = Int32Rules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Int32Rules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Int32RulesView<'_> {
  type Msg = Int32Rules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Int32Rules> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Int32RulesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut validate__Int64Rules_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Int64Rules {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Int64Rules>
}

impl ::protobuf::Message for Int64Rules {
  type MessageView<'msg> = Int64RulesView<'msg>;
  type MessageMut<'msg> = Int64RulesMut<'msg>;
}

impl ::std::default::Default for Int64Rules {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Int64Rules {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Int64Rules` is `Sync` because it does not implement interior mutability.
//    Neither does `Int64RulesMut`.
unsafe impl ::std::marker::Sync for Int64Rules {}

// SAFETY:
// - `Int64Rules` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Int64Rules {}

impl ::protobuf::Proxied for Int64Rules {
  type View<'msg> = Int64RulesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Int64Rules {}

impl ::protobuf::MutProxied for Int64Rules {
  type Mut<'msg> = Int64RulesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Int64RulesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Int64Rules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Int64RulesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for Int64RulesView<'msg> {
  type Message = Int64Rules;
}

impl ::std::fmt::Debug for Int64RulesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for Int64RulesView<'_> {
  fn default() -> Int64RulesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Int64Rules>> for Int64RulesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Int64Rules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Int64RulesView<'msg> {

  pub fn to_owned(&self) -> Int64Rules {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // const: optional int64
  pub fn has_const(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn const_opt(self) -> ::std::option::Option<i64> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // lt: optional int64
  pub fn has_lt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn lt_opt(self) -> ::std::option::Option<i64> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        1, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // lte: optional int64
  pub fn has_lte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn lte_opt(self) -> ::std::option::Option<i64> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        2, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // gt: optional int64
  pub fn has_gt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn gt_opt(self) -> ::std::option::Option<i64> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        3, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // gte: optional int64
  pub fn has_gte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn gte_opt(self) -> ::std::option::Option<i64> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        4, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // in: repeated int64
  pub fn r#in(self) -> ::protobuf::RepeatedView<'msg, i64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // not_in: repeated int64
  pub fn not_in(self) -> ::protobuf::RepeatedView<'msg, i64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn ignore_empty_opt(self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `Int64RulesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for Int64RulesView<'_> {}

// SAFETY:
// - `Int64RulesView` is `Send` because while its alive a `Int64RulesMut` cannot.
// - `Int64RulesView` does not use thread-local data.
unsafe impl ::std::marker::Send for Int64RulesView<'_> {}

impl<'msg> ::protobuf::AsView for Int64RulesView<'msg> {
  type Proxied = Int64Rules;
  fn as_view(&self) -> ::protobuf::View<'msg, Int64Rules> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Int64RulesView<'msg> {
  fn into_view<'shorter>(self) -> Int64RulesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Int64Rules> for Int64RulesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Int64Rules {
    let mut dst = Int64Rules::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Int64Rules> for Int64RulesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Int64Rules {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Int64Rules {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for Int64RulesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for Int64RulesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct Int64RulesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Int64Rules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Int64RulesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for Int64RulesMut<'msg> {
  type Message = Int64Rules;
}

impl ::std::fmt::Debug for Int64RulesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Int64Rules>> for Int64RulesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Int64Rules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Int64RulesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Int64Rules> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Int64Rules {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // const: optional int64
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<i64> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_const(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        0, val.into()
      )
    }
  }

  // lt: optional int64
  pub fn has_lt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_lt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn lt_opt(&self) -> ::std::option::Option<i64> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        1, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lt(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        1, val.into()
      )
    }
  }

  // lte: optional int64
  pub fn has_lte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_lte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn lte_opt(&self) -> ::std::option::Option<i64> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        2, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lte(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        2, val.into()
      )
    }
  }

  // gt: optional int64
  pub fn has_gt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_gt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn gt_opt(&self) -> ::std::option::Option<i64> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        3, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gt(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        3, val.into()
      )
    }
  }

  // gte: optional int64
  pub fn has_gte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_gte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn gte_opt(&self) -> ::std::option::Option<i64> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        4, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gte(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        4, val.into()
      )
    }
  }

  // in: repeated int64
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, i64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i64> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // not_in: repeated int64
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, i64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i64> {
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

}

// SAFETY:
// - `Int64RulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for Int64RulesMut<'_> {}

// SAFETY:
// - `Int64RulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for Int64RulesMut<'_> {}

impl<'msg> ::protobuf::AsView for Int64RulesMut<'msg> {
  type Proxied = Int64Rules;
  fn as_view(&self) -> ::protobuf::View<'_, Int64Rules> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Int64RulesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Int64Rules>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for Int64RulesMut<'msg> {
  type MutProxied = Int64Rules;
  fn as_mut(&mut self) -> Int64RulesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for Int64RulesMut<'msg> {
  fn into_mut<'shorter>(self) -> Int64RulesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Int64Rules {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Int64Rules> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> Int64RulesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> Int64RulesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // const: optional int64
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<i64> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_const(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        0, val.into()
      )
    }
  }

  // lt: optional int64
  pub fn has_lt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_lt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn lt_opt(&self) -> ::std::option::Option<i64> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        1, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lt(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        1, val.into()
      )
    }
  }

  // lte: optional int64
  pub fn has_lte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_lte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn lte_opt(&self) -> ::std::option::Option<i64> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        2, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lte(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        2, val.into()
      )
    }
  }

  // gt: optional int64
  pub fn has_gt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_gt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn gt_opt(&self) -> ::std::option::Option<i64> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        3, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gt(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        3, val.into()
      )
    }
  }

  // gte: optional int64
  pub fn has_gte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_gte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn gte_opt(&self) -> ::std::option::Option<i64> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        4, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gte(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        4, val.into()
      )
    }
  }

  // in: repeated int64
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, i64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i64> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // not_in: repeated int64
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, i64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i64> {
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

}  // impl Int64Rules

impl ::std::ops::Drop for Int64Rules {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Int64Rules {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Int64Rules {
  type Proxied = Self;
  fn as_view(&self) -> Int64RulesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Int64Rules {
  type MutProxied = Self;
  fn as_mut(&mut self) -> Int64RulesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Int64Rules {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::validate__Int64Rules_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$+++++??/");
        ::protobuf::__internal::runtime::link_mini_table(
            super::validate__Int64Rules_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::validate__Int64Rules_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Int64Rules {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Int64Rules {
  type Msg = Int64Rules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Int64Rules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Int64Rules {
  type Msg = Int64Rules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Int64Rules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Int64RulesMut<'_> {
  type Msg = Int64Rules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Int64Rules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Int64RulesMut<'_> {
  type Msg = Int64Rules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Int64Rules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Int64RulesView<'_> {
  type Msg = Int64Rules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Int64Rules> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Int64RulesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut validate__UInt32Rules_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct UInt32Rules {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<UInt32Rules>
}

impl ::protobuf::Message for UInt32Rules {
  type MessageView<'msg> = UInt32RulesView<'msg>;
  type MessageMut<'msg> = UInt32RulesMut<'msg>;
}

impl ::std::default::Default for UInt32Rules {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for UInt32Rules {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `UInt32Rules` is `Sync` because it does not implement interior mutability.
//    Neither does `UInt32RulesMut`.
unsafe impl ::std::marker::Sync for UInt32Rules {}

// SAFETY:
// - `UInt32Rules` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for UInt32Rules {}

impl ::protobuf::Proxied for UInt32Rules {
  type View<'msg> = UInt32RulesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for UInt32Rules {}

impl ::protobuf::MutProxied for UInt32Rules {
  type Mut<'msg> = UInt32RulesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UInt32RulesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UInt32Rules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UInt32RulesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for UInt32RulesView<'msg> {
  type Message = UInt32Rules;
}

impl ::std::fmt::Debug for UInt32RulesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for UInt32RulesView<'_> {
  fn default() -> UInt32RulesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, UInt32Rules>> for UInt32RulesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UInt32Rules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UInt32RulesView<'msg> {

  pub fn to_owned(&self) -> UInt32Rules {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // const: optional uint32
  pub fn has_const(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn const_opt(self) -> ::std::option::Option<u32> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(self) -> u32 {
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

  // lt: optional uint32
  pub fn has_lt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn lt_opt(self) -> ::std::option::Option<u32> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        1, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // lte: optional uint32
  pub fn has_lte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn lte_opt(self) -> ::std::option::Option<u32> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // gt: optional uint32
  pub fn has_gt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn gt_opt(self) -> ::std::option::Option<u32> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        3, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // gte: optional uint32
  pub fn has_gte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn gte_opt(self) -> ::std::option::Option<u32> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        4, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // in: repeated uint32
  pub fn r#in(self) -> ::protobuf::RepeatedView<'msg, u32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // not_in: repeated uint32
  pub fn not_in(self) -> ::protobuf::RepeatedView<'msg, u32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn ignore_empty_opt(self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `UInt32RulesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for UInt32RulesView<'_> {}

// SAFETY:
// - `UInt32RulesView` is `Send` because while its alive a `UInt32RulesMut` cannot.
// - `UInt32RulesView` does not use thread-local data.
unsafe impl ::std::marker::Send for UInt32RulesView<'_> {}

impl<'msg> ::protobuf::AsView for UInt32RulesView<'msg> {
  type Proxied = UInt32Rules;
  fn as_view(&self) -> ::protobuf::View<'msg, UInt32Rules> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UInt32RulesView<'msg> {
  fn into_view<'shorter>(self) -> UInt32RulesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<UInt32Rules> for UInt32RulesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UInt32Rules {
    let mut dst = UInt32Rules::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<UInt32Rules> for UInt32RulesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UInt32Rules {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for UInt32Rules {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UInt32RulesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UInt32RulesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UInt32RulesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UInt32Rules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UInt32RulesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for UInt32RulesMut<'msg> {
  type Message = UInt32Rules;
}

impl ::std::fmt::Debug for UInt32RulesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, UInt32Rules>> for UInt32RulesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UInt32Rules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UInt32RulesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, UInt32Rules> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> UInt32Rules {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // const: optional uint32
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<u32> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> u32 {
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
  pub fn set_const(&mut self, val: u32) {
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

  // lt: optional uint32
  pub fn has_lt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_lt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn lt_opt(&self) -> ::std::option::Option<u32> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        1, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lt(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        1, val.into()
      )
    }
  }

  // lte: optional uint32
  pub fn has_lte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_lte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn lte_opt(&self) -> ::std::option::Option<u32> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lte(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        2, val.into()
      )
    }
  }

  // gt: optional uint32
  pub fn has_gt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_gt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn gt_opt(&self) -> ::std::option::Option<u32> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        3, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gt(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        3, val.into()
      )
    }
  }

  // gte: optional uint32
  pub fn has_gte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_gte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn gte_opt(&self) -> ::std::option::Option<u32> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        4, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gte(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        4, val.into()
      )
    }
  }

  // in: repeated uint32
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, u32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, u32> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<u32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // not_in: repeated uint32
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, u32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, u32> {
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<u32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

}

// SAFETY:
// - `UInt32RulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for UInt32RulesMut<'_> {}

// SAFETY:
// - `UInt32RulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for UInt32RulesMut<'_> {}

impl<'msg> ::protobuf::AsView for UInt32RulesMut<'msg> {
  type Proxied = UInt32Rules;
  fn as_view(&self) -> ::protobuf::View<'_, UInt32Rules> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UInt32RulesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, UInt32Rules>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for UInt32RulesMut<'msg> {
  type MutProxied = UInt32Rules;
  fn as_mut(&mut self) -> UInt32RulesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for UInt32RulesMut<'msg> {
  fn into_mut<'shorter>(self) -> UInt32RulesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl UInt32Rules {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, UInt32Rules> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> UInt32RulesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> UInt32RulesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // const: optional uint32
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<u32> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> u32 {
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
  pub fn set_const(&mut self, val: u32) {
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

  // lt: optional uint32
  pub fn has_lt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_lt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn lt_opt(&self) -> ::std::option::Option<u32> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        1, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lt(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        1, val.into()
      )
    }
  }

  // lte: optional uint32
  pub fn has_lte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_lte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn lte_opt(&self) -> ::std::option::Option<u32> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lte(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        2, val.into()
      )
    }
  }

  // gt: optional uint32
  pub fn has_gt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_gt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn gt_opt(&self) -> ::std::option::Option<u32> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        3, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gt(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        3, val.into()
      )
    }
  }

  // gte: optional uint32
  pub fn has_gte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_gte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn gte_opt(&self) -> ::std::option::Option<u32> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        4, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gte(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        4, val.into()
      )
    }
  }

  // in: repeated uint32
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, u32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, u32> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<u32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // not_in: repeated uint32
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, u32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, u32> {
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<u32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

}  // impl UInt32Rules

impl ::std::ops::Drop for UInt32Rules {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for UInt32Rules {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for UInt32Rules {
  type Proxied = Self;
  fn as_view(&self) -> UInt32RulesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for UInt32Rules {
  type MutProxied = Self;
  fn as_mut(&mut self) -> UInt32RulesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for UInt32Rules {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::validate__UInt32Rules_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$)))))==/");
        ::protobuf::__internal::runtime::link_mini_table(
            super::validate__UInt32Rules_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::validate__UInt32Rules_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UInt32Rules {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UInt32Rules {
  type Msg = UInt32Rules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UInt32Rules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UInt32Rules {
  type Msg = UInt32Rules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UInt32Rules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UInt32RulesMut<'_> {
  type Msg = UInt32Rules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UInt32Rules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UInt32RulesMut<'_> {
  type Msg = UInt32Rules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UInt32Rules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UInt32RulesView<'_> {
  type Msg = UInt32Rules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UInt32Rules> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UInt32RulesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut validate__UInt64Rules_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct UInt64Rules {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<UInt64Rules>
}

impl ::protobuf::Message for UInt64Rules {
  type MessageView<'msg> = UInt64RulesView<'msg>;
  type MessageMut<'msg> = UInt64RulesMut<'msg>;
}

impl ::std::default::Default for UInt64Rules {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for UInt64Rules {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `UInt64Rules` is `Sync` because it does not implement interior mutability.
//    Neither does `UInt64RulesMut`.
unsafe impl ::std::marker::Sync for UInt64Rules {}

// SAFETY:
// - `UInt64Rules` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for UInt64Rules {}

impl ::protobuf::Proxied for UInt64Rules {
  type View<'msg> = UInt64RulesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for UInt64Rules {}

impl ::protobuf::MutProxied for UInt64Rules {
  type Mut<'msg> = UInt64RulesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UInt64RulesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UInt64Rules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UInt64RulesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for UInt64RulesView<'msg> {
  type Message = UInt64Rules;
}

impl ::std::fmt::Debug for UInt64RulesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for UInt64RulesView<'_> {
  fn default() -> UInt64RulesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, UInt64Rules>> for UInt64RulesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UInt64Rules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UInt64RulesView<'msg> {

  pub fn to_owned(&self) -> UInt64Rules {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // const: optional uint64
  pub fn has_const(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn const_opt(self) -> ::std::option::Option<u64> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(self) -> u64 {
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

  // lt: optional uint64
  pub fn has_lt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn lt_opt(self) -> ::std::option::Option<u64> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // lte: optional uint64
  pub fn has_lte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn lte_opt(self) -> ::std::option::Option<u64> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        2, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // gt: optional uint64
  pub fn has_gt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn gt_opt(self) -> ::std::option::Option<u64> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        3, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // gte: optional uint64
  pub fn has_gte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn gte_opt(self) -> ::std::option::Option<u64> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        4, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // in: repeated uint64
  pub fn r#in(self) -> ::protobuf::RepeatedView<'msg, u64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // not_in: repeated uint64
  pub fn not_in(self) -> ::protobuf::RepeatedView<'msg, u64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn ignore_empty_opt(self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `UInt64RulesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for UInt64RulesView<'_> {}

// SAFETY:
// - `UInt64RulesView` is `Send` because while its alive a `UInt64RulesMut` cannot.
// - `UInt64RulesView` does not use thread-local data.
unsafe impl ::std::marker::Send for UInt64RulesView<'_> {}

impl<'msg> ::protobuf::AsView for UInt64RulesView<'msg> {
  type Proxied = UInt64Rules;
  fn as_view(&self) -> ::protobuf::View<'msg, UInt64Rules> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UInt64RulesView<'msg> {
  fn into_view<'shorter>(self) -> UInt64RulesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<UInt64Rules> for UInt64RulesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UInt64Rules {
    let mut dst = UInt64Rules::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<UInt64Rules> for UInt64RulesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UInt64Rules {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for UInt64Rules {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UInt64RulesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UInt64RulesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UInt64RulesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UInt64Rules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UInt64RulesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for UInt64RulesMut<'msg> {
  type Message = UInt64Rules;
}

impl ::std::fmt::Debug for UInt64RulesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, UInt64Rules>> for UInt64RulesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UInt64Rules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UInt64RulesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, UInt64Rules> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> UInt64Rules {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // const: optional uint64
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<u64> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> u64 {
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
  pub fn set_const(&mut self, val: u64) {
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

  // lt: optional uint64
  pub fn has_lt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_lt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn lt_opt(&self) -> ::std::option::Option<u64> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lt(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        1, val.into()
      )
    }
  }

  // lte: optional uint64
  pub fn has_lte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_lte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn lte_opt(&self) -> ::std::option::Option<u64> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        2, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lte(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        2, val.into()
      )
    }
  }

  // gt: optional uint64
  pub fn has_gt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_gt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn gt_opt(&self) -> ::std::option::Option<u64> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        3, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gt(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        3, val.into()
      )
    }
  }

  // gte: optional uint64
  pub fn has_gte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_gte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn gte_opt(&self) -> ::std::option::Option<u64> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        4, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gte(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        4, val.into()
      )
    }
  }

  // in: repeated uint64
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, u64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, u64> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<u64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // not_in: repeated uint64
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, u64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, u64> {
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<u64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

}

// SAFETY:
// - `UInt64RulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for UInt64RulesMut<'_> {}

// SAFETY:
// - `UInt64RulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for UInt64RulesMut<'_> {}

impl<'msg> ::protobuf::AsView for UInt64RulesMut<'msg> {
  type Proxied = UInt64Rules;
  fn as_view(&self) -> ::protobuf::View<'_, UInt64Rules> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UInt64RulesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, UInt64Rules>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for UInt64RulesMut<'msg> {
  type MutProxied = UInt64Rules;
  fn as_mut(&mut self) -> UInt64RulesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for UInt64RulesMut<'msg> {
  fn into_mut<'shorter>(self) -> UInt64RulesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl UInt64Rules {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, UInt64Rules> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> UInt64RulesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> UInt64RulesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // const: optional uint64
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<u64> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> u64 {
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
  pub fn set_const(&mut self, val: u64) {
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

  // lt: optional uint64
  pub fn has_lt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_lt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn lt_opt(&self) -> ::std::option::Option<u64> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lt(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        1, val.into()
      )
    }
  }

  // lte: optional uint64
  pub fn has_lte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_lte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn lte_opt(&self) -> ::std::option::Option<u64> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        2, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lte(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        2, val.into()
      )
    }
  }

  // gt: optional uint64
  pub fn has_gt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_gt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn gt_opt(&self) -> ::std::option::Option<u64> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        3, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gt(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        3, val.into()
      )
    }
  }

  // gte: optional uint64
  pub fn has_gte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_gte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn gte_opt(&self) -> ::std::option::Option<u64> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        4, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gte(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        4, val.into()
      )
    }
  }

  // in: repeated uint64
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, u64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, u64> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<u64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // not_in: repeated uint64
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, u64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, u64> {
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<u64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

}  // impl UInt64Rules

impl ::std::ops::Drop for UInt64Rules {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for UInt64Rules {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for UInt64Rules {
  type Proxied = Self;
  fn as_view(&self) -> UInt64RulesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for UInt64Rules {
  type MutProxied = Self;
  fn as_mut(&mut self) -> UInt64RulesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for UInt64Rules {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::validate__UInt64Rules_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$,,,,,@@/");
        ::protobuf::__internal::runtime::link_mini_table(
            super::validate__UInt64Rules_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::validate__UInt64Rules_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UInt64Rules {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UInt64Rules {
  type Msg = UInt64Rules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UInt64Rules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UInt64Rules {
  type Msg = UInt64Rules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UInt64Rules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UInt64RulesMut<'_> {
  type Msg = UInt64Rules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UInt64Rules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UInt64RulesMut<'_> {
  type Msg = UInt64Rules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UInt64Rules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UInt64RulesView<'_> {
  type Msg = UInt64Rules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UInt64Rules> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UInt64RulesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut validate__SInt32Rules_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SInt32Rules {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SInt32Rules>
}

impl ::protobuf::Message for SInt32Rules {
  type MessageView<'msg> = SInt32RulesView<'msg>;
  type MessageMut<'msg> = SInt32RulesMut<'msg>;
}

impl ::std::default::Default for SInt32Rules {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SInt32Rules {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SInt32Rules` is `Sync` because it does not implement interior mutability.
//    Neither does `SInt32RulesMut`.
unsafe impl ::std::marker::Sync for SInt32Rules {}

// SAFETY:
// - `SInt32Rules` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SInt32Rules {}

impl ::protobuf::Proxied for SInt32Rules {
  type View<'msg> = SInt32RulesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SInt32Rules {}

impl ::protobuf::MutProxied for SInt32Rules {
  type Mut<'msg> = SInt32RulesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SInt32RulesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SInt32Rules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SInt32RulesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SInt32RulesView<'msg> {
  type Message = SInt32Rules;
}

impl ::std::fmt::Debug for SInt32RulesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SInt32RulesView<'_> {
  fn default() -> SInt32RulesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SInt32Rules>> for SInt32RulesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SInt32Rules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SInt32RulesView<'msg> {

  pub fn to_owned(&self) -> SInt32Rules {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // const: optional sint32
  pub fn has_const(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn const_opt(self) -> ::std::option::Option<i32> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // lt: optional sint32
  pub fn has_lt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn lt_opt(self) -> ::std::option::Option<i32> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // lte: optional sint32
  pub fn has_lte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn lte_opt(self) -> ::std::option::Option<i32> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // gt: optional sint32
  pub fn has_gt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn gt_opt(self) -> ::std::option::Option<i32> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // gte: optional sint32
  pub fn has_gte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn gte_opt(self) -> ::std::option::Option<i32> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // in: repeated sint32
  pub fn r#in(self) -> ::protobuf::RepeatedView<'msg, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // not_in: repeated sint32
  pub fn not_in(self) -> ::protobuf::RepeatedView<'msg, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn ignore_empty_opt(self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `SInt32RulesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SInt32RulesView<'_> {}

// SAFETY:
// - `SInt32RulesView` is `Send` because while its alive a `SInt32RulesMut` cannot.
// - `SInt32RulesView` does not use thread-local data.
unsafe impl ::std::marker::Send for SInt32RulesView<'_> {}

impl<'msg> ::protobuf::AsView for SInt32RulesView<'msg> {
  type Proxied = SInt32Rules;
  fn as_view(&self) -> ::protobuf::View<'msg, SInt32Rules> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SInt32RulesView<'msg> {
  fn into_view<'shorter>(self) -> SInt32RulesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SInt32Rules> for SInt32RulesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SInt32Rules {
    let mut dst = SInt32Rules::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SInt32Rules> for SInt32RulesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SInt32Rules {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SInt32Rules {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SInt32RulesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SInt32RulesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SInt32RulesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SInt32Rules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SInt32RulesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SInt32RulesMut<'msg> {
  type Message = SInt32Rules;
}

impl ::std::fmt::Debug for SInt32RulesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SInt32Rules>> for SInt32RulesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SInt32Rules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SInt32RulesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SInt32Rules> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SInt32Rules {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // const: optional sint32
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<i32> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_const(&mut self, val: i32) {
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

  // lt: optional sint32
  pub fn has_lt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_lt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn lt_opt(&self) -> ::std::option::Option<i32> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lt(&mut self, val: i32) {
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

  // lte: optional sint32
  pub fn has_lte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_lte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn lte_opt(&self) -> ::std::option::Option<i32> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lte(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

  // gt: optional sint32
  pub fn has_gt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_gt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn gt_opt(&self) -> ::std::option::Option<i32> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gt(&mut self, val: i32) {
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

  // gte: optional sint32
  pub fn has_gte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_gte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn gte_opt(&self) -> ::std::option::Option<i32> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gte(&mut self, val: i32) {
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

  // in: repeated sint32
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i32> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // not_in: repeated sint32
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i32> {
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

}

// SAFETY:
// - `SInt32RulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SInt32RulesMut<'_> {}

// SAFETY:
// - `SInt32RulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SInt32RulesMut<'_> {}

impl<'msg> ::protobuf::AsView for SInt32RulesMut<'msg> {
  type Proxied = SInt32Rules;
  fn as_view(&self) -> ::protobuf::View<'_, SInt32Rules> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SInt32RulesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SInt32Rules>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SInt32RulesMut<'msg> {
  type MutProxied = SInt32Rules;
  fn as_mut(&mut self) -> SInt32RulesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SInt32RulesMut<'msg> {
  fn into_mut<'shorter>(self) -> SInt32RulesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SInt32Rules {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SInt32Rules> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SInt32RulesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SInt32RulesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // const: optional sint32
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<i32> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_const(&mut self, val: i32) {
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

  // lt: optional sint32
  pub fn has_lt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_lt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn lt_opt(&self) -> ::std::option::Option<i32> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lt(&mut self, val: i32) {
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

  // lte: optional sint32
  pub fn has_lte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_lte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn lte_opt(&self) -> ::std::option::Option<i32> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lte(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

  // gt: optional sint32
  pub fn has_gt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_gt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn gt_opt(&self) -> ::std::option::Option<i32> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gt(&mut self, val: i32) {
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

  // gte: optional sint32
  pub fn has_gte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_gte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn gte_opt(&self) -> ::std::option::Option<i32> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gte(&mut self, val: i32) {
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

  // in: repeated sint32
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i32> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // not_in: repeated sint32
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i32> {
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

}  // impl SInt32Rules

impl ::std::ops::Drop for SInt32Rules {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SInt32Rules {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SInt32Rules {
  type Proxied = Self;
  fn as_view(&self) -> SInt32RulesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SInt32Rules {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SInt32RulesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SInt32Rules {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::validate__SInt32Rules_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$*****>>/");
        ::protobuf::__internal::runtime::link_mini_table(
            super::validate__SInt32Rules_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::validate__SInt32Rules_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SInt32Rules {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SInt32Rules {
  type Msg = SInt32Rules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SInt32Rules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SInt32Rules {
  type Msg = SInt32Rules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SInt32Rules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SInt32RulesMut<'_> {
  type Msg = SInt32Rules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SInt32Rules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SInt32RulesMut<'_> {
  type Msg = SInt32Rules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SInt32Rules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SInt32RulesView<'_> {
  type Msg = SInt32Rules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SInt32Rules> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SInt32RulesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut validate__SInt64Rules_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SInt64Rules {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SInt64Rules>
}

impl ::protobuf::Message for SInt64Rules {
  type MessageView<'msg> = SInt64RulesView<'msg>;
  type MessageMut<'msg> = SInt64RulesMut<'msg>;
}

impl ::std::default::Default for SInt64Rules {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SInt64Rules {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SInt64Rules` is `Sync` because it does not implement interior mutability.
//    Neither does `SInt64RulesMut`.
unsafe impl ::std::marker::Sync for SInt64Rules {}

// SAFETY:
// - `SInt64Rules` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SInt64Rules {}

impl ::protobuf::Proxied for SInt64Rules {
  type View<'msg> = SInt64RulesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SInt64Rules {}

impl ::protobuf::MutProxied for SInt64Rules {
  type Mut<'msg> = SInt64RulesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SInt64RulesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SInt64Rules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SInt64RulesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SInt64RulesView<'msg> {
  type Message = SInt64Rules;
}

impl ::std::fmt::Debug for SInt64RulesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SInt64RulesView<'_> {
  fn default() -> SInt64RulesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SInt64Rules>> for SInt64RulesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SInt64Rules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SInt64RulesView<'msg> {

  pub fn to_owned(&self) -> SInt64Rules {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // const: optional sint64
  pub fn has_const(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn const_opt(self) -> ::std::option::Option<i64> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // lt: optional sint64
  pub fn has_lt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn lt_opt(self) -> ::std::option::Option<i64> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        1, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // lte: optional sint64
  pub fn has_lte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn lte_opt(self) -> ::std::option::Option<i64> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        2, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // gt: optional sint64
  pub fn has_gt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn gt_opt(self) -> ::std::option::Option<i64> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        3, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // gte: optional sint64
  pub fn has_gte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn gte_opt(self) -> ::std::option::Option<i64> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        4, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // in: repeated sint64
  pub fn r#in(self) -> ::protobuf::RepeatedView<'msg, i64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // not_in: repeated sint64
  pub fn not_in(self) -> ::protobuf::RepeatedView<'msg, i64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn ignore_empty_opt(self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `SInt64RulesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SInt64RulesView<'_> {}

// SAFETY:
// - `SInt64RulesView` is `Send` because while its alive a `SInt64RulesMut` cannot.
// - `SInt64RulesView` does not use thread-local data.
unsafe impl ::std::marker::Send for SInt64RulesView<'_> {}

impl<'msg> ::protobuf::AsView for SInt64RulesView<'msg> {
  type Proxied = SInt64Rules;
  fn as_view(&self) -> ::protobuf::View<'msg, SInt64Rules> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SInt64RulesView<'msg> {
  fn into_view<'shorter>(self) -> SInt64RulesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SInt64Rules> for SInt64RulesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SInt64Rules {
    let mut dst = SInt64Rules::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SInt64Rules> for SInt64RulesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SInt64Rules {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SInt64Rules {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SInt64RulesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SInt64RulesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SInt64RulesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SInt64Rules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SInt64RulesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SInt64RulesMut<'msg> {
  type Message = SInt64Rules;
}

impl ::std::fmt::Debug for SInt64RulesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SInt64Rules>> for SInt64RulesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SInt64Rules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SInt64RulesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SInt64Rules> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SInt64Rules {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // const: optional sint64
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<i64> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_const(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        0, val.into()
      )
    }
  }

  // lt: optional sint64
  pub fn has_lt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_lt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn lt_opt(&self) -> ::std::option::Option<i64> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        1, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lt(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        1, val.into()
      )
    }
  }

  // lte: optional sint64
  pub fn has_lte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_lte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn lte_opt(&self) -> ::std::option::Option<i64> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        2, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lte(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        2, val.into()
      )
    }
  }

  // gt: optional sint64
  pub fn has_gt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_gt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn gt_opt(&self) -> ::std::option::Option<i64> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        3, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gt(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        3, val.into()
      )
    }
  }

  // gte: optional sint64
  pub fn has_gte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_gte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn gte_opt(&self) -> ::std::option::Option<i64> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        4, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gte(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        4, val.into()
      )
    }
  }

  // in: repeated sint64
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, i64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i64> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // not_in: repeated sint64
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, i64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i64> {
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

}

// SAFETY:
// - `SInt64RulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SInt64RulesMut<'_> {}

// SAFETY:
// - `SInt64RulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SInt64RulesMut<'_> {}

impl<'msg> ::protobuf::AsView for SInt64RulesMut<'msg> {
  type Proxied = SInt64Rules;
  fn as_view(&self) -> ::protobuf::View<'_, SInt64Rules> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SInt64RulesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SInt64Rules>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SInt64RulesMut<'msg> {
  type MutProxied = SInt64Rules;
  fn as_mut(&mut self) -> SInt64RulesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SInt64RulesMut<'msg> {
  fn into_mut<'shorter>(self) -> SInt64RulesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SInt64Rules {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SInt64Rules> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SInt64RulesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SInt64RulesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // const: optional sint64
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<i64> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_const(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        0, val.into()
      )
    }
  }

  // lt: optional sint64
  pub fn has_lt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_lt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn lt_opt(&self) -> ::std::option::Option<i64> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        1, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lt(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        1, val.into()
      )
    }
  }

  // lte: optional sint64
  pub fn has_lte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_lte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn lte_opt(&self) -> ::std::option::Option<i64> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        2, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lte(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        2, val.into()
      )
    }
  }

  // gt: optional sint64
  pub fn has_gt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_gt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn gt_opt(&self) -> ::std::option::Option<i64> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        3, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gt(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        3, val.into()
      )
    }
  }

  // gte: optional sint64
  pub fn has_gte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_gte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn gte_opt(&self) -> ::std::option::Option<i64> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        4, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gte(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        4, val.into()
      )
    }
  }

  // in: repeated sint64
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, i64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i64> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // not_in: repeated sint64
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, i64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i64> {
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

}  // impl SInt64Rules

impl ::std::ops::Drop for SInt64Rules {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SInt64Rules {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SInt64Rules {
  type Proxied = Self;
  fn as_view(&self) -> SInt64RulesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SInt64Rules {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SInt64RulesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SInt64Rules {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::validate__SInt64Rules_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$-----AA/");
        ::protobuf::__internal::runtime::link_mini_table(
            super::validate__SInt64Rules_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::validate__SInt64Rules_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SInt64Rules {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SInt64Rules {
  type Msg = SInt64Rules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SInt64Rules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SInt64Rules {
  type Msg = SInt64Rules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SInt64Rules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SInt64RulesMut<'_> {
  type Msg = SInt64Rules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SInt64Rules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SInt64RulesMut<'_> {
  type Msg = SInt64Rules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SInt64Rules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SInt64RulesView<'_> {
  type Msg = SInt64Rules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SInt64Rules> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SInt64RulesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut validate__Fixed32Rules_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Fixed32Rules {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Fixed32Rules>
}

impl ::protobuf::Message for Fixed32Rules {
  type MessageView<'msg> = Fixed32RulesView<'msg>;
  type MessageMut<'msg> = Fixed32RulesMut<'msg>;
}

impl ::std::default::Default for Fixed32Rules {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Fixed32Rules {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Fixed32Rules` is `Sync` because it does not implement interior mutability.
//    Neither does `Fixed32RulesMut`.
unsafe impl ::std::marker::Sync for Fixed32Rules {}

// SAFETY:
// - `Fixed32Rules` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Fixed32Rules {}

impl ::protobuf::Proxied for Fixed32Rules {
  type View<'msg> = Fixed32RulesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Fixed32Rules {}

impl ::protobuf::MutProxied for Fixed32Rules {
  type Mut<'msg> = Fixed32RulesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Fixed32RulesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Fixed32Rules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Fixed32RulesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for Fixed32RulesView<'msg> {
  type Message = Fixed32Rules;
}

impl ::std::fmt::Debug for Fixed32RulesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for Fixed32RulesView<'_> {
  fn default() -> Fixed32RulesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Fixed32Rules>> for Fixed32RulesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Fixed32Rules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Fixed32RulesView<'msg> {

  pub fn to_owned(&self) -> Fixed32Rules {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // const: optional fixed32
  pub fn has_const(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn const_opt(self) -> ::std::option::Option<u32> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(self) -> u32 {
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

  // lt: optional fixed32
  pub fn has_lt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn lt_opt(self) -> ::std::option::Option<u32> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        1, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // lte: optional fixed32
  pub fn has_lte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn lte_opt(self) -> ::std::option::Option<u32> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // gt: optional fixed32
  pub fn has_gt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn gt_opt(self) -> ::std::option::Option<u32> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        3, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // gte: optional fixed32
  pub fn has_gte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn gte_opt(self) -> ::std::option::Option<u32> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        4, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // in: repeated fixed32
  pub fn r#in(self) -> ::protobuf::RepeatedView<'msg, u32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // not_in: repeated fixed32
  pub fn not_in(self) -> ::protobuf::RepeatedView<'msg, u32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn ignore_empty_opt(self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `Fixed32RulesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for Fixed32RulesView<'_> {}

// SAFETY:
// - `Fixed32RulesView` is `Send` because while its alive a `Fixed32RulesMut` cannot.
// - `Fixed32RulesView` does not use thread-local data.
unsafe impl ::std::marker::Send for Fixed32RulesView<'_> {}

impl<'msg> ::protobuf::AsView for Fixed32RulesView<'msg> {
  type Proxied = Fixed32Rules;
  fn as_view(&self) -> ::protobuf::View<'msg, Fixed32Rules> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Fixed32RulesView<'msg> {
  fn into_view<'shorter>(self) -> Fixed32RulesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Fixed32Rules> for Fixed32RulesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Fixed32Rules {
    let mut dst = Fixed32Rules::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Fixed32Rules> for Fixed32RulesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Fixed32Rules {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Fixed32Rules {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for Fixed32RulesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for Fixed32RulesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct Fixed32RulesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Fixed32Rules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Fixed32RulesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for Fixed32RulesMut<'msg> {
  type Message = Fixed32Rules;
}

impl ::std::fmt::Debug for Fixed32RulesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Fixed32Rules>> for Fixed32RulesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Fixed32Rules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Fixed32RulesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Fixed32Rules> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Fixed32Rules {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // const: optional fixed32
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<u32> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> u32 {
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
  pub fn set_const(&mut self, val: u32) {
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

  // lt: optional fixed32
  pub fn has_lt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_lt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn lt_opt(&self) -> ::std::option::Option<u32> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        1, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lt(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        1, val.into()
      )
    }
  }

  // lte: optional fixed32
  pub fn has_lte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_lte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn lte_opt(&self) -> ::std::option::Option<u32> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lte(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        2, val.into()
      )
    }
  }

  // gt: optional fixed32
  pub fn has_gt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_gt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn gt_opt(&self) -> ::std::option::Option<u32> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        3, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gt(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        3, val.into()
      )
    }
  }

  // gte: optional fixed32
  pub fn has_gte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_gte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn gte_opt(&self) -> ::std::option::Option<u32> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        4, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gte(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        4, val.into()
      )
    }
  }

  // in: repeated fixed32
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, u32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, u32> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<u32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // not_in: repeated fixed32
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, u32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, u32> {
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<u32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

}

// SAFETY:
// - `Fixed32RulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for Fixed32RulesMut<'_> {}

// SAFETY:
// - `Fixed32RulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for Fixed32RulesMut<'_> {}

impl<'msg> ::protobuf::AsView for Fixed32RulesMut<'msg> {
  type Proxied = Fixed32Rules;
  fn as_view(&self) -> ::protobuf::View<'_, Fixed32Rules> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Fixed32RulesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Fixed32Rules>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for Fixed32RulesMut<'msg> {
  type MutProxied = Fixed32Rules;
  fn as_mut(&mut self) -> Fixed32RulesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for Fixed32RulesMut<'msg> {
  fn into_mut<'shorter>(self) -> Fixed32RulesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Fixed32Rules {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Fixed32Rules> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> Fixed32RulesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> Fixed32RulesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // const: optional fixed32
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<u32> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> u32 {
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
  pub fn set_const(&mut self, val: u32) {
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

  // lt: optional fixed32
  pub fn has_lt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_lt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn lt_opt(&self) -> ::std::option::Option<u32> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        1, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lt(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        1, val.into()
      )
    }
  }

  // lte: optional fixed32
  pub fn has_lte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_lte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn lte_opt(&self) -> ::std::option::Option<u32> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lte(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        2, val.into()
      )
    }
  }

  // gt: optional fixed32
  pub fn has_gt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_gt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn gt_opt(&self) -> ::std::option::Option<u32> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        3, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gt(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        3, val.into()
      )
    }
  }

  // gte: optional fixed32
  pub fn has_gte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_gte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn gte_opt(&self) -> ::std::option::Option<u32> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        4, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gte(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        4, val.into()
      )
    }
  }

  // in: repeated fixed32
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, u32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, u32> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<u32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // not_in: repeated fixed32
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, u32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, u32> {
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<u32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

}  // impl Fixed32Rules

impl ::std::ops::Drop for Fixed32Rules {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Fixed32Rules {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Fixed32Rules {
  type Proxied = Self;
  fn as_view(&self) -> Fixed32RulesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Fixed32Rules {
  type MutProxied = Self;
  fn as_mut(&mut self) -> Fixed32RulesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Fixed32Rules {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::validate__Fixed32Rules_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$#####88/");
        ::protobuf::__internal::runtime::link_mini_table(
            super::validate__Fixed32Rules_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::validate__Fixed32Rules_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Fixed32Rules {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Fixed32Rules {
  type Msg = Fixed32Rules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Fixed32Rules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Fixed32Rules {
  type Msg = Fixed32Rules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Fixed32Rules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Fixed32RulesMut<'_> {
  type Msg = Fixed32Rules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Fixed32Rules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Fixed32RulesMut<'_> {
  type Msg = Fixed32Rules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Fixed32Rules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Fixed32RulesView<'_> {
  type Msg = Fixed32Rules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Fixed32Rules> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Fixed32RulesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut validate__Fixed64Rules_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Fixed64Rules {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Fixed64Rules>
}

impl ::protobuf::Message for Fixed64Rules {
  type MessageView<'msg> = Fixed64RulesView<'msg>;
  type MessageMut<'msg> = Fixed64RulesMut<'msg>;
}

impl ::std::default::Default for Fixed64Rules {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Fixed64Rules {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Fixed64Rules` is `Sync` because it does not implement interior mutability.
//    Neither does `Fixed64RulesMut`.
unsafe impl ::std::marker::Sync for Fixed64Rules {}

// SAFETY:
// - `Fixed64Rules` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Fixed64Rules {}

impl ::protobuf::Proxied for Fixed64Rules {
  type View<'msg> = Fixed64RulesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Fixed64Rules {}

impl ::protobuf::MutProxied for Fixed64Rules {
  type Mut<'msg> = Fixed64RulesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Fixed64RulesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Fixed64Rules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Fixed64RulesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for Fixed64RulesView<'msg> {
  type Message = Fixed64Rules;
}

impl ::std::fmt::Debug for Fixed64RulesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for Fixed64RulesView<'_> {
  fn default() -> Fixed64RulesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Fixed64Rules>> for Fixed64RulesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Fixed64Rules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Fixed64RulesView<'msg> {

  pub fn to_owned(&self) -> Fixed64Rules {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // const: optional fixed64
  pub fn has_const(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn const_opt(self) -> ::std::option::Option<u64> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(self) -> u64 {
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

  // lt: optional fixed64
  pub fn has_lt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn lt_opt(self) -> ::std::option::Option<u64> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // lte: optional fixed64
  pub fn has_lte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn lte_opt(self) -> ::std::option::Option<u64> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        2, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // gt: optional fixed64
  pub fn has_gt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn gt_opt(self) -> ::std::option::Option<u64> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        3, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // gte: optional fixed64
  pub fn has_gte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn gte_opt(self) -> ::std::option::Option<u64> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        4, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // in: repeated fixed64
  pub fn r#in(self) -> ::protobuf::RepeatedView<'msg, u64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // not_in: repeated fixed64
  pub fn not_in(self) -> ::protobuf::RepeatedView<'msg, u64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn ignore_empty_opt(self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `Fixed64RulesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for Fixed64RulesView<'_> {}

// SAFETY:
// - `Fixed64RulesView` is `Send` because while its alive a `Fixed64RulesMut` cannot.
// - `Fixed64RulesView` does not use thread-local data.
unsafe impl ::std::marker::Send for Fixed64RulesView<'_> {}

impl<'msg> ::protobuf::AsView for Fixed64RulesView<'msg> {
  type Proxied = Fixed64Rules;
  fn as_view(&self) -> ::protobuf::View<'msg, Fixed64Rules> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Fixed64RulesView<'msg> {
  fn into_view<'shorter>(self) -> Fixed64RulesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Fixed64Rules> for Fixed64RulesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Fixed64Rules {
    let mut dst = Fixed64Rules::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Fixed64Rules> for Fixed64RulesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Fixed64Rules {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Fixed64Rules {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for Fixed64RulesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for Fixed64RulesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct Fixed64RulesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Fixed64Rules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Fixed64RulesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for Fixed64RulesMut<'msg> {
  type Message = Fixed64Rules;
}

impl ::std::fmt::Debug for Fixed64RulesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Fixed64Rules>> for Fixed64RulesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Fixed64Rules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Fixed64RulesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Fixed64Rules> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Fixed64Rules {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // const: optional fixed64
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<u64> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> u64 {
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
  pub fn set_const(&mut self, val: u64) {
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

  // lt: optional fixed64
  pub fn has_lt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_lt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn lt_opt(&self) -> ::std::option::Option<u64> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lt(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        1, val.into()
      )
    }
  }

  // lte: optional fixed64
  pub fn has_lte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_lte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn lte_opt(&self) -> ::std::option::Option<u64> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        2, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lte(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        2, val.into()
      )
    }
  }

  // gt: optional fixed64
  pub fn has_gt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_gt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn gt_opt(&self) -> ::std::option::Option<u64> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        3, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gt(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        3, val.into()
      )
    }
  }

  // gte: optional fixed64
  pub fn has_gte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_gte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn gte_opt(&self) -> ::std::option::Option<u64> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        4, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gte(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        4, val.into()
      )
    }
  }

  // in: repeated fixed64
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, u64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, u64> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<u64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // not_in: repeated fixed64
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, u64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, u64> {
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<u64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

}

// SAFETY:
// - `Fixed64RulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for Fixed64RulesMut<'_> {}

// SAFETY:
// - `Fixed64RulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for Fixed64RulesMut<'_> {}

impl<'msg> ::protobuf::AsView for Fixed64RulesMut<'msg> {
  type Proxied = Fixed64Rules;
  fn as_view(&self) -> ::protobuf::View<'_, Fixed64Rules> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Fixed64RulesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Fixed64Rules>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for Fixed64RulesMut<'msg> {
  type MutProxied = Fixed64Rules;
  fn as_mut(&mut self) -> Fixed64RulesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for Fixed64RulesMut<'msg> {
  fn into_mut<'shorter>(self) -> Fixed64RulesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Fixed64Rules {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Fixed64Rules> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> Fixed64RulesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> Fixed64RulesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // const: optional fixed64
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<u64> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> u64 {
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
  pub fn set_const(&mut self, val: u64) {
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

  // lt: optional fixed64
  pub fn has_lt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_lt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn lt_opt(&self) -> ::std::option::Option<u64> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lt(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        1, val.into()
      )
    }
  }

  // lte: optional fixed64
  pub fn has_lte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_lte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn lte_opt(&self) -> ::std::option::Option<u64> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        2, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lte(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        2, val.into()
      )
    }
  }

  // gt: optional fixed64
  pub fn has_gt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_gt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn gt_opt(&self) -> ::std::option::Option<u64> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        3, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gt(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        3, val.into()
      )
    }
  }

  // gte: optional fixed64
  pub fn has_gte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_gte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn gte_opt(&self) -> ::std::option::Option<u64> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        4, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gte(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        4, val.into()
      )
    }
  }

  // in: repeated fixed64
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, u64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, u64> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<u64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // not_in: repeated fixed64
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, u64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, u64> {
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<u64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

}  // impl Fixed64Rules

impl ::std::ops::Drop for Fixed64Rules {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Fixed64Rules {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Fixed64Rules {
  type Proxied = Self;
  fn as_view(&self) -> Fixed64RulesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Fixed64Rules {
  type MutProxied = Self;
  fn as_mut(&mut self) -> Fixed64RulesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Fixed64Rules {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::validate__Fixed64Rules_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$$$$$$99/");
        ::protobuf::__internal::runtime::link_mini_table(
            super::validate__Fixed64Rules_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::validate__Fixed64Rules_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Fixed64Rules {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Fixed64Rules {
  type Msg = Fixed64Rules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Fixed64Rules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Fixed64Rules {
  type Msg = Fixed64Rules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Fixed64Rules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Fixed64RulesMut<'_> {
  type Msg = Fixed64Rules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Fixed64Rules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Fixed64RulesMut<'_> {
  type Msg = Fixed64Rules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Fixed64Rules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Fixed64RulesView<'_> {
  type Msg = Fixed64Rules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Fixed64Rules> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Fixed64RulesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut validate__SFixed32Rules_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SFixed32Rules {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SFixed32Rules>
}

impl ::protobuf::Message for SFixed32Rules {
  type MessageView<'msg> = SFixed32RulesView<'msg>;
  type MessageMut<'msg> = SFixed32RulesMut<'msg>;
}

impl ::std::default::Default for SFixed32Rules {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SFixed32Rules {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SFixed32Rules` is `Sync` because it does not implement interior mutability.
//    Neither does `SFixed32RulesMut`.
unsafe impl ::std::marker::Sync for SFixed32Rules {}

// SAFETY:
// - `SFixed32Rules` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SFixed32Rules {}

impl ::protobuf::Proxied for SFixed32Rules {
  type View<'msg> = SFixed32RulesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SFixed32Rules {}

impl ::protobuf::MutProxied for SFixed32Rules {
  type Mut<'msg> = SFixed32RulesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SFixed32RulesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SFixed32Rules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SFixed32RulesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SFixed32RulesView<'msg> {
  type Message = SFixed32Rules;
}

impl ::std::fmt::Debug for SFixed32RulesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SFixed32RulesView<'_> {
  fn default() -> SFixed32RulesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SFixed32Rules>> for SFixed32RulesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SFixed32Rules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SFixed32RulesView<'msg> {

  pub fn to_owned(&self) -> SFixed32Rules {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // const: optional sfixed32
  pub fn has_const(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn const_opt(self) -> ::std::option::Option<i32> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // lt: optional sfixed32
  pub fn has_lt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn lt_opt(self) -> ::std::option::Option<i32> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // lte: optional sfixed32
  pub fn has_lte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn lte_opt(self) -> ::std::option::Option<i32> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // gt: optional sfixed32
  pub fn has_gt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn gt_opt(self) -> ::std::option::Option<i32> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // gte: optional sfixed32
  pub fn has_gte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn gte_opt(self) -> ::std::option::Option<i32> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // in: repeated sfixed32
  pub fn r#in(self) -> ::protobuf::RepeatedView<'msg, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // not_in: repeated sfixed32
  pub fn not_in(self) -> ::protobuf::RepeatedView<'msg, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn ignore_empty_opt(self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `SFixed32RulesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SFixed32RulesView<'_> {}

// SAFETY:
// - `SFixed32RulesView` is `Send` because while its alive a `SFixed32RulesMut` cannot.
// - `SFixed32RulesView` does not use thread-local data.
unsafe impl ::std::marker::Send for SFixed32RulesView<'_> {}

impl<'msg> ::protobuf::AsView for SFixed32RulesView<'msg> {
  type Proxied = SFixed32Rules;
  fn as_view(&self) -> ::protobuf::View<'msg, SFixed32Rules> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SFixed32RulesView<'msg> {
  fn into_view<'shorter>(self) -> SFixed32RulesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SFixed32Rules> for SFixed32RulesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SFixed32Rules {
    let mut dst = SFixed32Rules::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SFixed32Rules> for SFixed32RulesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SFixed32Rules {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SFixed32Rules {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SFixed32RulesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SFixed32RulesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SFixed32RulesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SFixed32Rules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SFixed32RulesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SFixed32RulesMut<'msg> {
  type Message = SFixed32Rules;
}

impl ::std::fmt::Debug for SFixed32RulesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SFixed32Rules>> for SFixed32RulesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SFixed32Rules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SFixed32RulesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SFixed32Rules> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SFixed32Rules {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // const: optional sfixed32
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<i32> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_const(&mut self, val: i32) {
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

  // lt: optional sfixed32
  pub fn has_lt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_lt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn lt_opt(&self) -> ::std::option::Option<i32> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lt(&mut self, val: i32) {
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

  // lte: optional sfixed32
  pub fn has_lte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_lte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn lte_opt(&self) -> ::std::option::Option<i32> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lte(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

  // gt: optional sfixed32
  pub fn has_gt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_gt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn gt_opt(&self) -> ::std::option::Option<i32> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gt(&mut self, val: i32) {
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

  // gte: optional sfixed32
  pub fn has_gte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_gte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn gte_opt(&self) -> ::std::option::Option<i32> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gte(&mut self, val: i32) {
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

  // in: repeated sfixed32
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i32> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // not_in: repeated sfixed32
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i32> {
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

}

// SAFETY:
// - `SFixed32RulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SFixed32RulesMut<'_> {}

// SAFETY:
// - `SFixed32RulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SFixed32RulesMut<'_> {}

impl<'msg> ::protobuf::AsView for SFixed32RulesMut<'msg> {
  type Proxied = SFixed32Rules;
  fn as_view(&self) -> ::protobuf::View<'_, SFixed32Rules> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SFixed32RulesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SFixed32Rules>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SFixed32RulesMut<'msg> {
  type MutProxied = SFixed32Rules;
  fn as_mut(&mut self) -> SFixed32RulesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SFixed32RulesMut<'msg> {
  fn into_mut<'shorter>(self) -> SFixed32RulesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SFixed32Rules {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SFixed32Rules> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SFixed32RulesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SFixed32RulesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // const: optional sfixed32
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<i32> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_const(&mut self, val: i32) {
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

  // lt: optional sfixed32
  pub fn has_lt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_lt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn lt_opt(&self) -> ::std::option::Option<i32> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lt(&mut self, val: i32) {
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

  // lte: optional sfixed32
  pub fn has_lte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_lte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn lte_opt(&self) -> ::std::option::Option<i32> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lte(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

  // gt: optional sfixed32
  pub fn has_gt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_gt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn gt_opt(&self) -> ::std::option::Option<i32> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gt(&mut self, val: i32) {
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

  // gte: optional sfixed32
  pub fn has_gte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_gte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn gte_opt(&self) -> ::std::option::Option<i32> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gte(&mut self, val: i32) {
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

  // in: repeated sfixed32
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i32> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // not_in: repeated sfixed32
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i32> {
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

}  // impl SFixed32Rules

impl ::std::ops::Drop for SFixed32Rules {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SFixed32Rules {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SFixed32Rules {
  type Proxied = Self;
  fn as_view(&self) -> SFixed32RulesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SFixed32Rules {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SFixed32RulesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SFixed32Rules {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::validate__SFixed32Rules_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$%%%%%::/");
        ::protobuf::__internal::runtime::link_mini_table(
            super::validate__SFixed32Rules_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::validate__SFixed32Rules_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SFixed32Rules {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SFixed32Rules {
  type Msg = SFixed32Rules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SFixed32Rules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SFixed32Rules {
  type Msg = SFixed32Rules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SFixed32Rules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SFixed32RulesMut<'_> {
  type Msg = SFixed32Rules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SFixed32Rules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SFixed32RulesMut<'_> {
  type Msg = SFixed32Rules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SFixed32Rules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SFixed32RulesView<'_> {
  type Msg = SFixed32Rules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SFixed32Rules> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SFixed32RulesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut validate__SFixed64Rules_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SFixed64Rules {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SFixed64Rules>
}

impl ::protobuf::Message for SFixed64Rules {
  type MessageView<'msg> = SFixed64RulesView<'msg>;
  type MessageMut<'msg> = SFixed64RulesMut<'msg>;
}

impl ::std::default::Default for SFixed64Rules {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SFixed64Rules {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SFixed64Rules` is `Sync` because it does not implement interior mutability.
//    Neither does `SFixed64RulesMut`.
unsafe impl ::std::marker::Sync for SFixed64Rules {}

// SAFETY:
// - `SFixed64Rules` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SFixed64Rules {}

impl ::protobuf::Proxied for SFixed64Rules {
  type View<'msg> = SFixed64RulesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SFixed64Rules {}

impl ::protobuf::MutProxied for SFixed64Rules {
  type Mut<'msg> = SFixed64RulesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SFixed64RulesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SFixed64Rules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SFixed64RulesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SFixed64RulesView<'msg> {
  type Message = SFixed64Rules;
}

impl ::std::fmt::Debug for SFixed64RulesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SFixed64RulesView<'_> {
  fn default() -> SFixed64RulesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SFixed64Rules>> for SFixed64RulesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SFixed64Rules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SFixed64RulesView<'msg> {

  pub fn to_owned(&self) -> SFixed64Rules {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // const: optional sfixed64
  pub fn has_const(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn const_opt(self) -> ::std::option::Option<i64> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // lt: optional sfixed64
  pub fn has_lt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn lt_opt(self) -> ::std::option::Option<i64> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        1, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // lte: optional sfixed64
  pub fn has_lte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn lte_opt(self) -> ::std::option::Option<i64> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        2, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // gt: optional sfixed64
  pub fn has_gt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn gt_opt(self) -> ::std::option::Option<i64> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        3, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // gte: optional sfixed64
  pub fn has_gte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn gte_opt(self) -> ::std::option::Option<i64> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        4, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // in: repeated sfixed64
  pub fn r#in(self) -> ::protobuf::RepeatedView<'msg, i64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // not_in: repeated sfixed64
  pub fn not_in(self) -> ::protobuf::RepeatedView<'msg, i64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn ignore_empty_opt(self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `SFixed64RulesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SFixed64RulesView<'_> {}

// SAFETY:
// - `SFixed64RulesView` is `Send` because while its alive a `SFixed64RulesMut` cannot.
// - `SFixed64RulesView` does not use thread-local data.
unsafe impl ::std::marker::Send for SFixed64RulesView<'_> {}

impl<'msg> ::protobuf::AsView for SFixed64RulesView<'msg> {
  type Proxied = SFixed64Rules;
  fn as_view(&self) -> ::protobuf::View<'msg, SFixed64Rules> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SFixed64RulesView<'msg> {
  fn into_view<'shorter>(self) -> SFixed64RulesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SFixed64Rules> for SFixed64RulesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SFixed64Rules {
    let mut dst = SFixed64Rules::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SFixed64Rules> for SFixed64RulesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SFixed64Rules {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SFixed64Rules {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SFixed64RulesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SFixed64RulesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SFixed64RulesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SFixed64Rules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SFixed64RulesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SFixed64RulesMut<'msg> {
  type Message = SFixed64Rules;
}

impl ::std::fmt::Debug for SFixed64RulesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SFixed64Rules>> for SFixed64RulesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SFixed64Rules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SFixed64RulesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SFixed64Rules> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SFixed64Rules {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // const: optional sfixed64
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<i64> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_const(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        0, val.into()
      )
    }
  }

  // lt: optional sfixed64
  pub fn has_lt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_lt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn lt_opt(&self) -> ::std::option::Option<i64> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        1, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lt(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        1, val.into()
      )
    }
  }

  // lte: optional sfixed64
  pub fn has_lte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_lte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn lte_opt(&self) -> ::std::option::Option<i64> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        2, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lte(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        2, val.into()
      )
    }
  }

  // gt: optional sfixed64
  pub fn has_gt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_gt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn gt_opt(&self) -> ::std::option::Option<i64> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        3, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gt(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        3, val.into()
      )
    }
  }

  // gte: optional sfixed64
  pub fn has_gte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_gte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn gte_opt(&self) -> ::std::option::Option<i64> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        4, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gte(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        4, val.into()
      )
    }
  }

  // in: repeated sfixed64
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, i64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i64> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // not_in: repeated sfixed64
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, i64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i64> {
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

}

// SAFETY:
// - `SFixed64RulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SFixed64RulesMut<'_> {}

// SAFETY:
// - `SFixed64RulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SFixed64RulesMut<'_> {}

impl<'msg> ::protobuf::AsView for SFixed64RulesMut<'msg> {
  type Proxied = SFixed64Rules;
  fn as_view(&self) -> ::protobuf::View<'_, SFixed64Rules> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SFixed64RulesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SFixed64Rules>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SFixed64RulesMut<'msg> {
  type MutProxied = SFixed64Rules;
  fn as_mut(&mut self) -> SFixed64RulesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SFixed64RulesMut<'msg> {
  fn into_mut<'shorter>(self) -> SFixed64RulesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SFixed64Rules {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SFixed64Rules> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SFixed64RulesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SFixed64RulesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // const: optional sfixed64
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<i64> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_const(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        0, val.into()
      )
    }
  }

  // lt: optional sfixed64
  pub fn has_lt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_lt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn lt_opt(&self) -> ::std::option::Option<i64> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        1, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lt(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        1, val.into()
      )
    }
  }

  // lte: optional sfixed64
  pub fn has_lte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_lte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn lte_opt(&self) -> ::std::option::Option<i64> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        2, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lte(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        2, val.into()
      )
    }
  }

  // gt: optional sfixed64
  pub fn has_gt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_gt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn gt_opt(&self) -> ::std::option::Option<i64> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        3, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gt(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        3, val.into()
      )
    }
  }

  // gte: optional sfixed64
  pub fn has_gte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_gte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn gte_opt(&self) -> ::std::option::Option<i64> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        4, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gte(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        4, val.into()
      )
    }
  }

  // in: repeated sfixed64
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, i64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i64> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // not_in: repeated sfixed64
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, i64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i64> {
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

}  // impl SFixed64Rules

impl ::std::ops::Drop for SFixed64Rules {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SFixed64Rules {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SFixed64Rules {
  type Proxied = Self;
  fn as_view(&self) -> SFixed64RulesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SFixed64Rules {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SFixed64RulesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SFixed64Rules {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::validate__SFixed64Rules_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$&&&&&;;/");
        ::protobuf::__internal::runtime::link_mini_table(
            super::validate__SFixed64Rules_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::validate__SFixed64Rules_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SFixed64Rules {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SFixed64Rules {
  type Msg = SFixed64Rules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SFixed64Rules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SFixed64Rules {
  type Msg = SFixed64Rules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SFixed64Rules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SFixed64RulesMut<'_> {
  type Msg = SFixed64Rules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SFixed64Rules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SFixed64RulesMut<'_> {
  type Msg = SFixed64Rules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SFixed64Rules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SFixed64RulesView<'_> {
  type Msg = SFixed64Rules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SFixed64Rules> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SFixed64RulesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut validate__BoolRules_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct BoolRules {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<BoolRules>
}

impl ::protobuf::Message for BoolRules {
  type MessageView<'msg> = BoolRulesView<'msg>;
  type MessageMut<'msg> = BoolRulesMut<'msg>;
}

impl ::std::default::Default for BoolRules {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for BoolRules {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `BoolRules` is `Sync` because it does not implement interior mutability.
//    Neither does `BoolRulesMut`.
unsafe impl ::std::marker::Sync for BoolRules {}

// SAFETY:
// - `BoolRules` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for BoolRules {}

impl ::protobuf::Proxied for BoolRules {
  type View<'msg> = BoolRulesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for BoolRules {}

impl ::protobuf::MutProxied for BoolRules {
  type Mut<'msg> = BoolRulesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct BoolRulesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BoolRules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BoolRulesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for BoolRulesView<'msg> {
  type Message = BoolRules;
}

impl ::std::fmt::Debug for BoolRulesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for BoolRulesView<'_> {
  fn default() -> BoolRulesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, BoolRules>> for BoolRulesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BoolRules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BoolRulesView<'msg> {

  pub fn to_owned(&self) -> BoolRules {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // const: optional bool
  pub fn has_const(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn const_opt(self) -> ::std::option::Option<bool> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `BoolRulesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for BoolRulesView<'_> {}

// SAFETY:
// - `BoolRulesView` is `Send` because while its alive a `BoolRulesMut` cannot.
// - `BoolRulesView` does not use thread-local data.
unsafe impl ::std::marker::Send for BoolRulesView<'_> {}

impl<'msg> ::protobuf::AsView for BoolRulesView<'msg> {
  type Proxied = BoolRules;
  fn as_view(&self) -> ::protobuf::View<'msg, BoolRules> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BoolRulesView<'msg> {
  fn into_view<'shorter>(self) -> BoolRulesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<BoolRules> for BoolRulesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BoolRules {
    let mut dst = BoolRules::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<BoolRules> for BoolRulesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BoolRules {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for BoolRules {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BoolRulesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BoolRulesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct BoolRulesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BoolRules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BoolRulesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for BoolRulesMut<'msg> {
  type Message = BoolRules;
}

impl ::std::fmt::Debug for BoolRulesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, BoolRules>> for BoolRulesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BoolRules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BoolRulesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, BoolRules> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> BoolRules {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // const: optional bool
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<bool> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_const(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        0, val.into()
      )
    }
  }

}

// SAFETY:
// - `BoolRulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for BoolRulesMut<'_> {}

// SAFETY:
// - `BoolRulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for BoolRulesMut<'_> {}

impl<'msg> ::protobuf::AsView for BoolRulesMut<'msg> {
  type Proxied = BoolRules;
  fn as_view(&self) -> ::protobuf::View<'_, BoolRules> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BoolRulesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, BoolRules>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for BoolRulesMut<'msg> {
  type MutProxied = BoolRules;
  fn as_mut(&mut self) -> BoolRulesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for BoolRulesMut<'msg> {
  fn into_mut<'shorter>(self) -> BoolRulesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl BoolRules {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, BoolRules> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> BoolRulesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> BoolRulesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // const: optional bool
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<bool> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_const(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        0, val.into()
      )
    }
  }

}  // impl BoolRules

impl ::std::ops::Drop for BoolRules {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for BoolRules {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for BoolRules {
  type Proxied = Self;
  fn as_view(&self) -> BoolRulesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for BoolRules {
  type MutProxied = Self;
  fn as_mut(&mut self) -> BoolRulesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for BoolRules {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::validate__BoolRules_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$/");
        ::protobuf::__internal::runtime::link_mini_table(
            super::validate__BoolRules_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::validate__BoolRules_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BoolRules {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BoolRules {
  type Msg = BoolRules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BoolRules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BoolRules {
  type Msg = BoolRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BoolRules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BoolRulesMut<'_> {
  type Msg = BoolRules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BoolRules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BoolRulesMut<'_> {
  type Msg = BoolRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BoolRules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BoolRulesView<'_> {
  type Msg = BoolRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BoolRules> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BoolRulesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut validate__StringRules_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct StringRules {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<StringRules>
}

impl ::protobuf::Message for StringRules {
  type MessageView<'msg> = StringRulesView<'msg>;
  type MessageMut<'msg> = StringRulesMut<'msg>;
}

impl ::std::default::Default for StringRules {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for StringRules {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `StringRules` is `Sync` because it does not implement interior mutability.
//    Neither does `StringRulesMut`.
unsafe impl ::std::marker::Sync for StringRules {}

// SAFETY:
// - `StringRules` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for StringRules {}

impl ::protobuf::Proxied for StringRules {
  type View<'msg> = StringRulesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for StringRules {}

impl ::protobuf::MutProxied for StringRules {
  type Mut<'msg> = StringRulesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct StringRulesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StringRules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StringRulesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for StringRulesView<'msg> {
  type Message = StringRules;
}

impl ::std::fmt::Debug for StringRulesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for StringRulesView<'_> {
  fn default() -> StringRulesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, StringRules>> for StringRulesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StringRules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StringRulesView<'msg> {

  pub fn to_owned(&self) -> StringRules {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // const: optional string
  pub fn has_const(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn const_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // len: optional uint64
  pub fn has_len(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(18)
    }
  }
  pub fn len_opt(self) -> ::std::option::Option<u64> {
    self.has_len().then(|| self.len())
  }
  pub fn len(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        18, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // min_len: optional uint64
  pub fn has_min_len(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn min_len_opt(self) -> ::std::option::Option<u64> {
    self.has_min_len().then(|| self.min_len())
  }
  pub fn min_len(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // max_len: optional uint64
  pub fn has_max_len(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn max_len_opt(self) -> ::std::option::Option<u64> {
    self.has_max_len().then(|| self.max_len())
  }
  pub fn max_len(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        2, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // len_bytes: optional uint64
  pub fn has_len_bytes(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(19)
    }
  }
  pub fn len_bytes_opt(self) -> ::std::option::Option<u64> {
    self.has_len_bytes().then(|| self.len_bytes())
  }
  pub fn len_bytes(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        19, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // min_bytes: optional uint64
  pub fn has_min_bytes(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn min_bytes_opt(self) -> ::std::option::Option<u64> {
    self.has_min_bytes().then(|| self.min_bytes())
  }
  pub fn min_bytes(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        3, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // max_bytes: optional uint64
  pub fn has_max_bytes(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn max_bytes_opt(self) -> ::std::option::Option<u64> {
    self.has_max_bytes().then(|| self.max_bytes())
  }
  pub fn max_bytes(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        4, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // pattern: optional string
  pub fn has_pattern(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn pattern_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_pattern().then(|| self.pattern())
  }
  pub fn pattern(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // prefix: optional string
  pub fn has_prefix(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn prefix_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_prefix().then(|| self.prefix())
  }
  pub fn prefix(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // suffix: optional string
  pub fn has_suffix(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn suffix_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_suffix().then(|| self.suffix())
  }
  pub fn suffix(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        7, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // contains: optional string
  pub fn has_contains(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn contains_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_contains().then(|| self.contains())
  }
  pub fn contains(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // not_contains: optional string
  pub fn has_not_contains(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(22)
    }
  }
  pub fn not_contains_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_not_contains().then(|| self.not_contains())
  }
  pub fn not_contains(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        22, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // in: repeated string
  pub fn r#in(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        9
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // not_in: repeated string
  pub fn not_in(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        10
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // email: optional bool
  pub fn has_email(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn email_opt(self) -> ::std::option::Option<bool> {
    self.has_email().then(|| self.email())
  }
  pub fn email(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        11, (false).into()
      ).try_into().unwrap()
    }
  }

  // hostname: optional bool
  pub fn has_hostname(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn hostname_opt(self) -> ::std::option::Option<bool> {
    self.has_hostname().then(|| self.hostname())
  }
  pub fn hostname(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        12, (false).into()
      ).try_into().unwrap()
    }
  }

  // ip: optional bool
  pub fn has_ip(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn ip_opt(self) -> ::std::option::Option<bool> {
    self.has_ip().then(|| self.ip())
  }
  pub fn ip(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        13, (false).into()
      ).try_into().unwrap()
    }
  }

  // ipv4: optional bool
  pub fn has_ipv4(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(14)
    }
  }
  pub fn ipv4_opt(self) -> ::std::option::Option<bool> {
    self.has_ipv4().then(|| self.ipv4())
  }
  pub fn ipv4(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        14, (false).into()
      ).try_into().unwrap()
    }
  }

  // ipv6: optional bool
  pub fn has_ipv6(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(15)
    }
  }
  pub fn ipv6_opt(self) -> ::std::option::Option<bool> {
    self.has_ipv6().then(|| self.ipv6())
  }
  pub fn ipv6(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        15, (false).into()
      ).try_into().unwrap()
    }
  }

  // uri: optional bool
  pub fn has_uri(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(16)
    }
  }
  pub fn uri_opt(self) -> ::std::option::Option<bool> {
    self.has_uri().then(|| self.uri())
  }
  pub fn uri(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        16, (false).into()
      ).try_into().unwrap()
    }
  }

  // uri_ref: optional bool
  pub fn has_uri_ref(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(17)
    }
  }
  pub fn uri_ref_opt(self) -> ::std::option::Option<bool> {
    self.has_uri_ref().then(|| self.uri_ref())
  }
  pub fn uri_ref(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        17, (false).into()
      ).try_into().unwrap()
    }
  }

  // address: optional bool
  pub fn has_address(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(20)
    }
  }
  pub fn address_opt(self) -> ::std::option::Option<bool> {
    self.has_address().then(|| self.address())
  }
  pub fn address(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        20, (false).into()
      ).try_into().unwrap()
    }
  }

  // uuid: optional bool
  pub fn has_uuid(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(21)
    }
  }
  pub fn uuid_opt(self) -> ::std::option::Option<bool> {
    self.has_uuid().then(|| self.uuid())
  }
  pub fn uuid(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        21, (false).into()
      ).try_into().unwrap()
    }
  }

  // well_known_regex: optional enum validate.KnownRegex
  pub fn has_well_known_regex(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(23)
    }
  }
  pub fn well_known_regex_opt(self) -> ::std::option::Option<super::KnownRegex> {
    self.has_well_known_regex().then(|| self.well_known_regex())
  }
  pub fn well_known_regex(self) -> super::KnownRegex {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        23, (super::KnownRegex::Unknown).into()
      ).try_into().unwrap()
    }
  }

  // strict: optional bool
  pub fn has_strict(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(24)
    }
  }
  pub fn strict_opt(self) -> ::std::option::Option<bool> {
    self.has_strict().then(|| self.strict())
  }
  pub fn strict(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        24, (true).into()
      ).try_into().unwrap()
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(25)
    }
  }
  pub fn ignore_empty_opt(self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        25, (false).into()
      ).try_into().unwrap()
    }
  }

  pub fn well_known(self) -> super::string_rules::WellKnownOneof<'msg> {
    match self.well_known_case() {
      super::string_rules::WellKnownCase::Email =>
          super::string_rules::WellKnownOneof::Email(self.email()),
      super::string_rules::WellKnownCase::Hostname =>
          super::string_rules::WellKnownOneof::Hostname(self.hostname()),
      super::string_rules::WellKnownCase::Ip =>
          super::string_rules::WellKnownOneof::Ip(self.ip()),
      super::string_rules::WellKnownCase::Ipv4 =>
          super::string_rules::WellKnownOneof::Ipv4(self.ipv4()),
      super::string_rules::WellKnownCase::Ipv6 =>
          super::string_rules::WellKnownOneof::Ipv6(self.ipv6()),
      super::string_rules::WellKnownCase::Uri =>
          super::string_rules::WellKnownOneof::Uri(self.uri()),
      super::string_rules::WellKnownCase::UriRef =>
          super::string_rules::WellKnownOneof::UriRef(self.uri_ref()),
      super::string_rules::WellKnownCase::Address =>
          super::string_rules::WellKnownOneof::Address(self.address()),
      super::string_rules::WellKnownCase::Uuid =>
          super::string_rules::WellKnownOneof::Uuid(self.uuid()),
      super::string_rules::WellKnownCase::WellKnownRegex =>
          super::string_rules::WellKnownOneof::WellKnownRegex(self.well_known_regex()),
      _ => super::string_rules::WellKnownOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn well_known_case(self) -> super::string_rules::WellKnownCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(11);
      super::string_rules::WellKnownCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `StringRulesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for StringRulesView<'_> {}

// SAFETY:
// - `StringRulesView` is `Send` because while its alive a `StringRulesMut` cannot.
// - `StringRulesView` does not use thread-local data.
unsafe impl ::std::marker::Send for StringRulesView<'_> {}

impl<'msg> ::protobuf::AsView for StringRulesView<'msg> {
  type Proxied = StringRules;
  fn as_view(&self) -> ::protobuf::View<'msg, StringRules> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StringRulesView<'msg> {
  fn into_view<'shorter>(self) -> StringRulesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<StringRules> for StringRulesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StringRules {
    let mut dst = StringRules::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<StringRules> for StringRulesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StringRules {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for StringRules {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StringRulesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StringRulesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct StringRulesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StringRules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StringRulesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for StringRulesMut<'msg> {
  type Message = StringRules;
}

impl ::std::fmt::Debug for StringRulesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, StringRules>> for StringRulesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StringRules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StringRulesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, StringRules> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> StringRules {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // const: optional string
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_const(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // len: optional uint64
  pub fn has_len(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(18)
    }
  }
  pub fn clear_len(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        18
      );
    }
  }
  pub fn len_opt(&self) -> ::std::option::Option<u64> {
    self.has_len().then(|| self.len())
  }
  pub fn len(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        18, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_len(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        18, val.into()
      )
    }
  }

  // min_len: optional uint64
  pub fn has_min_len(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_min_len(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn min_len_opt(&self) -> ::std::option::Option<u64> {
    self.has_min_len().then(|| self.min_len())
  }
  pub fn min_len(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_min_len(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        1, val.into()
      )
    }
  }

  // max_len: optional uint64
  pub fn has_max_len(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_max_len(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn max_len_opt(&self) -> ::std::option::Option<u64> {
    self.has_max_len().then(|| self.max_len())
  }
  pub fn max_len(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        2, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_max_len(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        2, val.into()
      )
    }
  }

  // len_bytes: optional uint64
  pub fn has_len_bytes(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(19)
    }
  }
  pub fn clear_len_bytes(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        19
      );
    }
  }
  pub fn len_bytes_opt(&self) -> ::std::option::Option<u64> {
    self.has_len_bytes().then(|| self.len_bytes())
  }
  pub fn len_bytes(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        19, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_len_bytes(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        19, val.into()
      )
    }
  }

  // min_bytes: optional uint64
  pub fn has_min_bytes(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_min_bytes(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn min_bytes_opt(&self) -> ::std::option::Option<u64> {
    self.has_min_bytes().then(|| self.min_bytes())
  }
  pub fn min_bytes(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        3, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_min_bytes(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        3, val.into()
      )
    }
  }

  // max_bytes: optional uint64
  pub fn has_max_bytes(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_max_bytes(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn max_bytes_opt(&self) -> ::std::option::Option<u64> {
    self.has_max_bytes().then(|| self.max_bytes())
  }
  pub fn max_bytes(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        4, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_max_bytes(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        4, val.into()
      )
    }
  }

  // pattern: optional string
  pub fn has_pattern(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_pattern(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn pattern_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_pattern().then(|| self.pattern())
  }
  pub fn pattern(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_pattern(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // prefix: optional string
  pub fn has_prefix(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_prefix(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn prefix_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_prefix().then(|| self.prefix())
  }
  pub fn prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // suffix: optional string
  pub fn has_suffix(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_suffix(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn suffix_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_suffix().then(|| self.suffix())
  }
  pub fn suffix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        7, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_suffix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val);
    }
  }

  // contains: optional string
  pub fn has_contains(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_contains(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn contains_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_contains().then(|| self.contains())
  }
  pub fn contains(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_contains(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val);
    }
  }

  // not_contains: optional string
  pub fn has_not_contains(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(22)
    }
  }
  pub fn clear_not_contains(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        22
      );
    }
  }
  pub fn not_contains_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_not_contains().then(|| self.not_contains())
  }
  pub fn not_contains(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        22, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_not_contains(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        22,
        val);
    }
  }

  // in: repeated string
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        9
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        9,
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        src);
    }
  }

  // not_in: repeated string
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        10
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        10,
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        src);
    }
  }

  // email: optional bool
  pub fn has_email(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_email(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn email_opt(&self) -> ::std::option::Option<bool> {
    self.has_email().then(|| self.email())
  }
  pub fn email(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        11, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_email(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        11, val.into()
      )
    }
  }

  // hostname: optional bool
  pub fn has_hostname(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_hostname(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn hostname_opt(&self) -> ::std::option::Option<bool> {
    self.has_hostname().then(|| self.hostname())
  }
  pub fn hostname(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        12, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_hostname(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        12, val.into()
      )
    }
  }

  // ip: optional bool
  pub fn has_ip(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_ip(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn ip_opt(&self) -> ::std::option::Option<bool> {
    self.has_ip().then(|| self.ip())
  }
  pub fn ip(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        13, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ip(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        13, val.into()
      )
    }
  }

  // ipv4: optional bool
  pub fn has_ipv4(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(14)
    }
  }
  pub fn clear_ipv4(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        14
      );
    }
  }
  pub fn ipv4_opt(&self) -> ::std::option::Option<bool> {
    self.has_ipv4().then(|| self.ipv4())
  }
  pub fn ipv4(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        14, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ipv4(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        14, val.into()
      )
    }
  }

  // ipv6: optional bool
  pub fn has_ipv6(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(15)
    }
  }
  pub fn clear_ipv6(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        15
      );
    }
  }
  pub fn ipv6_opt(&self) -> ::std::option::Option<bool> {
    self.has_ipv6().then(|| self.ipv6())
  }
  pub fn ipv6(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        15, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ipv6(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        15, val.into()
      )
    }
  }

  // uri: optional bool
  pub fn has_uri(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(16)
    }
  }
  pub fn clear_uri(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        16
      );
    }
  }
  pub fn uri_opt(&self) -> ::std::option::Option<bool> {
    self.has_uri().then(|| self.uri())
  }
  pub fn uri(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        16, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_uri(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        16, val.into()
      )
    }
  }

  // uri_ref: optional bool
  pub fn has_uri_ref(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(17)
    }
  }
  pub fn clear_uri_ref(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        17
      );
    }
  }
  pub fn uri_ref_opt(&self) -> ::std::option::Option<bool> {
    self.has_uri_ref().then(|| self.uri_ref())
  }
  pub fn uri_ref(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        17, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_uri_ref(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        17, val.into()
      )
    }
  }

  // address: optional bool
  pub fn has_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(20)
    }
  }
  pub fn clear_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        20
      );
    }
  }
  pub fn address_opt(&self) -> ::std::option::Option<bool> {
    self.has_address().then(|| self.address())
  }
  pub fn address(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        20, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_address(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        20, val.into()
      )
    }
  }

  // uuid: optional bool
  pub fn has_uuid(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(21)
    }
  }
  pub fn clear_uuid(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        21
      );
    }
  }
  pub fn uuid_opt(&self) -> ::std::option::Option<bool> {
    self.has_uuid().then(|| self.uuid())
  }
  pub fn uuid(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        21, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_uuid(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        21, val.into()
      )
    }
  }

  // well_known_regex: optional enum validate.KnownRegex
  pub fn has_well_known_regex(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(23)
    }
  }
  pub fn clear_well_known_regex(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        23
      );
    }
  }
  pub fn well_known_regex_opt(&self) -> ::std::option::Option<super::KnownRegex> {
    self.has_well_known_regex().then(|| self.well_known_regex())
  }
  pub fn well_known_regex(&self) -> super::KnownRegex {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        23, (super::KnownRegex::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_well_known_regex(&mut self, val: super::KnownRegex) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        23, val.into()
      )
    }
  }

  // strict: optional bool
  pub fn has_strict(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(24)
    }
  }
  pub fn clear_strict(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        24
      );
    }
  }
  pub fn strict_opt(&self) -> ::std::option::Option<bool> {
    self.has_strict().then(|| self.strict())
  }
  pub fn strict(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        24, (true).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_strict(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        24, val.into()
      )
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(25)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        25
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        25, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        25, val.into()
      )
    }
  }

  pub fn well_known(&self) -> super::string_rules::WellKnownOneof<'_> {
    match &self.well_known_case() {
      super::string_rules::WellKnownCase::Email =>
          super::string_rules::WellKnownOneof::Email(self.email()),
      super::string_rules::WellKnownCase::Hostname =>
          super::string_rules::WellKnownOneof::Hostname(self.hostname()),
      super::string_rules::WellKnownCase::Ip =>
          super::string_rules::WellKnownOneof::Ip(self.ip()),
      super::string_rules::WellKnownCase::Ipv4 =>
          super::string_rules::WellKnownOneof::Ipv4(self.ipv4()),
      super::string_rules::WellKnownCase::Ipv6 =>
          super::string_rules::WellKnownOneof::Ipv6(self.ipv6()),
      super::string_rules::WellKnownCase::Uri =>
          super::string_rules::WellKnownOneof::Uri(self.uri()),
      super::string_rules::WellKnownCase::UriRef =>
          super::string_rules::WellKnownOneof::UriRef(self.uri_ref()),
      super::string_rules::WellKnownCase::Address =>
          super::string_rules::WellKnownOneof::Address(self.address()),
      super::string_rules::WellKnownCase::Uuid =>
          super::string_rules::WellKnownOneof::Uuid(self.uuid()),
      super::string_rules::WellKnownCase::WellKnownRegex =>
          super::string_rules::WellKnownOneof::WellKnownRegex(self.well_known_regex()),
      _ => super::string_rules::WellKnownOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn well_known_case(&self) -> super::string_rules::WellKnownCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(11);
      super::string_rules::WellKnownCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `StringRulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for StringRulesMut<'_> {}

// SAFETY:
// - `StringRulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for StringRulesMut<'_> {}

impl<'msg> ::protobuf::AsView for StringRulesMut<'msg> {
  type Proxied = StringRules;
  fn as_view(&self) -> ::protobuf::View<'_, StringRules> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StringRulesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, StringRules>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for StringRulesMut<'msg> {
  type MutProxied = StringRules;
  fn as_mut(&mut self) -> StringRulesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for StringRulesMut<'msg> {
  fn into_mut<'shorter>(self) -> StringRulesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl StringRules {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, StringRules> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> StringRulesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> StringRulesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // const: optional string
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_const(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // len: optional uint64
  pub fn has_len(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(18)
    }
  }
  pub fn clear_len(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        18
      );
    }
  }
  pub fn len_opt(&self) -> ::std::option::Option<u64> {
    self.has_len().then(|| self.len())
  }
  pub fn len(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        18, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_len(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        18, val.into()
      )
    }
  }

  // min_len: optional uint64
  pub fn has_min_len(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_min_len(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn min_len_opt(&self) -> ::std::option::Option<u64> {
    self.has_min_len().then(|| self.min_len())
  }
  pub fn min_len(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_min_len(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        1, val.into()
      )
    }
  }

  // max_len: optional uint64
  pub fn has_max_len(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_max_len(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn max_len_opt(&self) -> ::std::option::Option<u64> {
    self.has_max_len().then(|| self.max_len())
  }
  pub fn max_len(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        2, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_max_len(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        2, val.into()
      )
    }
  }

  // len_bytes: optional uint64
  pub fn has_len_bytes(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(19)
    }
  }
  pub fn clear_len_bytes(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        19
      );
    }
  }
  pub fn len_bytes_opt(&self) -> ::std::option::Option<u64> {
    self.has_len_bytes().then(|| self.len_bytes())
  }
  pub fn len_bytes(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        19, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_len_bytes(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        19, val.into()
      )
    }
  }

  // min_bytes: optional uint64
  pub fn has_min_bytes(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_min_bytes(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn min_bytes_opt(&self) -> ::std::option::Option<u64> {
    self.has_min_bytes().then(|| self.min_bytes())
  }
  pub fn min_bytes(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        3, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_min_bytes(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        3, val.into()
      )
    }
  }

  // max_bytes: optional uint64
  pub fn has_max_bytes(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_max_bytes(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn max_bytes_opt(&self) -> ::std::option::Option<u64> {
    self.has_max_bytes().then(|| self.max_bytes())
  }
  pub fn max_bytes(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        4, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_max_bytes(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        4, val.into()
      )
    }
  }

  // pattern: optional string
  pub fn has_pattern(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_pattern(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn pattern_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_pattern().then(|| self.pattern())
  }
  pub fn pattern(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_pattern(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // prefix: optional string
  pub fn has_prefix(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_prefix(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn prefix_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_prefix().then(|| self.prefix())
  }
  pub fn prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // suffix: optional string
  pub fn has_suffix(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_suffix(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn suffix_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_suffix().then(|| self.suffix())
  }
  pub fn suffix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        7, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_suffix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val);
    }
  }

  // contains: optional string
  pub fn has_contains(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_contains(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn contains_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_contains().then(|| self.contains())
  }
  pub fn contains(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_contains(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val);
    }
  }

  // not_contains: optional string
  pub fn has_not_contains(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(22)
    }
  }
  pub fn clear_not_contains(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        22
      );
    }
  }
  pub fn not_contains_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_not_contains().then(|| self.not_contains())
  }
  pub fn not_contains(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        22, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_not_contains(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        22,
        val);
    }
  }

  // in: repeated string
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        9
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        9,
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        src);
    }
  }

  // not_in: repeated string
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        10
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        10,
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        src);
    }
  }

  // email: optional bool
  pub fn has_email(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_email(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn email_opt(&self) -> ::std::option::Option<bool> {
    self.has_email().then(|| self.email())
  }
  pub fn email(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        11, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_email(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        11, val.into()
      )
    }
  }

  // hostname: optional bool
  pub fn has_hostname(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_hostname(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn hostname_opt(&self) -> ::std::option::Option<bool> {
    self.has_hostname().then(|| self.hostname())
  }
  pub fn hostname(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        12, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_hostname(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        12, val.into()
      )
    }
  }

  // ip: optional bool
  pub fn has_ip(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_ip(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn ip_opt(&self) -> ::std::option::Option<bool> {
    self.has_ip().then(|| self.ip())
  }
  pub fn ip(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        13, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ip(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        13, val.into()
      )
    }
  }

  // ipv4: optional bool
  pub fn has_ipv4(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(14)
    }
  }
  pub fn clear_ipv4(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        14
      );
    }
  }
  pub fn ipv4_opt(&self) -> ::std::option::Option<bool> {
    self.has_ipv4().then(|| self.ipv4())
  }
  pub fn ipv4(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        14, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ipv4(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        14, val.into()
      )
    }
  }

  // ipv6: optional bool
  pub fn has_ipv6(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(15)
    }
  }
  pub fn clear_ipv6(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        15
      );
    }
  }
  pub fn ipv6_opt(&self) -> ::std::option::Option<bool> {
    self.has_ipv6().then(|| self.ipv6())
  }
  pub fn ipv6(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        15, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ipv6(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        15, val.into()
      )
    }
  }

  // uri: optional bool
  pub fn has_uri(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(16)
    }
  }
  pub fn clear_uri(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        16
      );
    }
  }
  pub fn uri_opt(&self) -> ::std::option::Option<bool> {
    self.has_uri().then(|| self.uri())
  }
  pub fn uri(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        16, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_uri(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        16, val.into()
      )
    }
  }

  // uri_ref: optional bool
  pub fn has_uri_ref(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(17)
    }
  }
  pub fn clear_uri_ref(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        17
      );
    }
  }
  pub fn uri_ref_opt(&self) -> ::std::option::Option<bool> {
    self.has_uri_ref().then(|| self.uri_ref())
  }
  pub fn uri_ref(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        17, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_uri_ref(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        17, val.into()
      )
    }
  }

  // address: optional bool
  pub fn has_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(20)
    }
  }
  pub fn clear_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        20
      );
    }
  }
  pub fn address_opt(&self) -> ::std::option::Option<bool> {
    self.has_address().then(|| self.address())
  }
  pub fn address(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        20, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_address(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        20, val.into()
      )
    }
  }

  // uuid: optional bool
  pub fn has_uuid(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(21)
    }
  }
  pub fn clear_uuid(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        21
      );
    }
  }
  pub fn uuid_opt(&self) -> ::std::option::Option<bool> {
    self.has_uuid().then(|| self.uuid())
  }
  pub fn uuid(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        21, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_uuid(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        21, val.into()
      )
    }
  }

  // well_known_regex: optional enum validate.KnownRegex
  pub fn has_well_known_regex(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(23)
    }
  }
  pub fn clear_well_known_regex(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        23
      );
    }
  }
  pub fn well_known_regex_opt(&self) -> ::std::option::Option<super::KnownRegex> {
    self.has_well_known_regex().then(|| self.well_known_regex())
  }
  pub fn well_known_regex(&self) -> super::KnownRegex {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        23, (super::KnownRegex::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_well_known_regex(&mut self, val: super::KnownRegex) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        23, val.into()
      )
    }
  }

  // strict: optional bool
  pub fn has_strict(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(24)
    }
  }
  pub fn clear_strict(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        24
      );
    }
  }
  pub fn strict_opt(&self) -> ::std::option::Option<bool> {
    self.has_strict().then(|| self.strict())
  }
  pub fn strict(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        24, (true).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_strict(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        24, val.into()
      )
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(25)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        25
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        25, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        25, val.into()
      )
    }
  }

  pub fn well_known(&self) -> super::string_rules::WellKnownOneof<'_> {
    match &self.well_known_case() {
      super::string_rules::WellKnownCase::Email =>
          super::string_rules::WellKnownOneof::Email(self.email()),
      super::string_rules::WellKnownCase::Hostname =>
          super::string_rules::WellKnownOneof::Hostname(self.hostname()),
      super::string_rules::WellKnownCase::Ip =>
          super::string_rules::WellKnownOneof::Ip(self.ip()),
      super::string_rules::WellKnownCase::Ipv4 =>
          super::string_rules::WellKnownOneof::Ipv4(self.ipv4()),
      super::string_rules::WellKnownCase::Ipv6 =>
          super::string_rules::WellKnownOneof::Ipv6(self.ipv6()),
      super::string_rules::WellKnownCase::Uri =>
          super::string_rules::WellKnownOneof::Uri(self.uri()),
      super::string_rules::WellKnownCase::UriRef =>
          super::string_rules::WellKnownOneof::UriRef(self.uri_ref()),
      super::string_rules::WellKnownCase::Address =>
          super::string_rules::WellKnownOneof::Address(self.address()),
      super::string_rules::WellKnownCase::Uuid =>
          super::string_rules::WellKnownOneof::Uuid(self.uuid()),
      super::string_rules::WellKnownCase::WellKnownRegex =>
          super::string_rules::WellKnownOneof::WellKnownRegex(self.well_known_regex()),
      _ => super::string_rules::WellKnownOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn well_known_case(&self) -> super::string_rules::WellKnownCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(11);
      super::string_rules::WellKnownCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl StringRules

impl ::std::ops::Drop for StringRules {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for StringRules {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for StringRules {
  type Proxied = Self;
  fn as_view(&self) -> StringRulesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for StringRules {
  type MutProxied = Self;
  fn as_mut(&mut self) -> StringRulesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for StringRules {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::validate__StringRules_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1,,,,1111EE///////,,//14//^.|/|0|1|2|3|4|7|8|:");
        ::protobuf::__internal::runtime::link_mini_table(
            super::validate__StringRules_msg_init.0, &[], &[<super::KnownRegex as ::protobuf::__internal::runtime::AssociatedMiniTableEnum>::mini_table(),
            ]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::validate__StringRules_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StringRules {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StringRules {
  type Msg = StringRules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StringRules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StringRules {
  type Msg = StringRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StringRules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StringRulesMut<'_> {
  type Msg = StringRules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StringRules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StringRulesMut<'_> {
  type Msg = StringRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StringRules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StringRulesView<'_> {
  type Msg = StringRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StringRules> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StringRulesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod string_rules {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum WellKnownOneof<'msg> {
  Email(bool) = 12,
  Hostname(bool) = 13,
  Ip(bool) = 14,
  Ipv4(bool) = 15,
  Ipv6(bool) = 16,
  Uri(bool) = 17,
  UriRef(bool) = 18,
  Address(bool) = 21,
  Uuid(bool) = 22,
  WellKnownRegex(::protobuf::View<'msg, super::super::KnownRegex>) = 24,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum WellKnownCase {
  Email = 12,
  Hostname = 13,
  Ip = 14,
  Ipv4 = 15,
  Ipv6 = 16,
  Uri = 17,
  UriRef = 18,
  Address = 21,
  Uuid = 22,
  WellKnownRegex = 24,

  not_set = 0
}

impl WellKnownCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<WellKnownCase> {
    match v {
      0 => Some(WellKnownCase::not_set),
      12 => Some(WellKnownCase::Email),
      13 => Some(WellKnownCase::Hostname),
      14 => Some(WellKnownCase::Ip),
      15 => Some(WellKnownCase::Ipv4),
      16 => Some(WellKnownCase::Ipv6),
      17 => Some(WellKnownCase::Uri),
      18 => Some(WellKnownCase::UriRef),
      21 => Some(WellKnownCase::Address),
      22 => Some(WellKnownCase::Uuid),
      24 => Some(WellKnownCase::WellKnownRegex),
      _ => None
    }
  }
}
}  // pub mod string_rules


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut validate__BytesRules_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct BytesRules {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<BytesRules>
}

impl ::protobuf::Message for BytesRules {
  type MessageView<'msg> = BytesRulesView<'msg>;
  type MessageMut<'msg> = BytesRulesMut<'msg>;
}

impl ::std::default::Default for BytesRules {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for BytesRules {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `BytesRules` is `Sync` because it does not implement interior mutability.
//    Neither does `BytesRulesMut`.
unsafe impl ::std::marker::Sync for BytesRules {}

// SAFETY:
// - `BytesRules` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for BytesRules {}

impl ::protobuf::Proxied for BytesRules {
  type View<'msg> = BytesRulesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for BytesRules {}

impl ::protobuf::MutProxied for BytesRules {
  type Mut<'msg> = BytesRulesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct BytesRulesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BytesRules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BytesRulesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for BytesRulesView<'msg> {
  type Message = BytesRules;
}

impl ::std::fmt::Debug for BytesRulesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for BytesRulesView<'_> {
  fn default() -> BytesRulesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, BytesRules>> for BytesRulesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BytesRules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BytesRulesView<'msg> {

  pub fn to_owned(&self) -> BytesRules {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // const: optional bytes
  pub fn has_const(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn const_opt(self) -> ::std::option::Option<&'msg [u8]> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // len: optional uint64
  pub fn has_len(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn len_opt(self) -> ::std::option::Option<u64> {
    self.has_len().then(|| self.len())
  }
  pub fn len(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        12, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // min_len: optional uint64
  pub fn has_min_len(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn min_len_opt(self) -> ::std::option::Option<u64> {
    self.has_min_len().then(|| self.min_len())
  }
  pub fn min_len(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // max_len: optional uint64
  pub fn has_max_len(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn max_len_opt(self) -> ::std::option::Option<u64> {
    self.has_max_len().then(|| self.max_len())
  }
  pub fn max_len(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        2, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // pattern: optional string
  pub fn has_pattern(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn pattern_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_pattern().then(|| self.pattern())
  }
  pub fn pattern(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // prefix: optional bytes
  pub fn has_prefix(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn prefix_opt(self) -> ::std::option::Option<&'msg [u8]> {
    self.has_prefix().then(|| self.prefix())
  }
  pub fn prefix(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // suffix: optional bytes
  pub fn has_suffix(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn suffix_opt(self) -> ::std::option::Option<&'msg [u8]> {
    self.has_suffix().then(|| self.suffix())
  }
  pub fn suffix(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // contains: optional bytes
  pub fn has_contains(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn contains_opt(self) -> ::std::option::Option<&'msg [u8]> {
    self.has_contains().then(|| self.contains())
  }
  pub fn contains(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // in: repeated bytes
  pub fn r#in(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoBytes> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoBytes>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // not_in: repeated bytes
  pub fn not_in(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoBytes> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        8
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoBytes>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // ip: optional bool
  pub fn has_ip(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn ip_opt(self) -> ::std::option::Option<bool> {
    self.has_ip().then(|| self.ip())
  }
  pub fn ip(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        9, (false).into()
      ).try_into().unwrap()
    }
  }

  // ipv4: optional bool
  pub fn has_ipv4(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn ipv4_opt(self) -> ::std::option::Option<bool> {
    self.has_ipv4().then(|| self.ipv4())
  }
  pub fn ipv4(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        10, (false).into()
      ).try_into().unwrap()
    }
  }

  // ipv6: optional bool
  pub fn has_ipv6(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn ipv6_opt(self) -> ::std::option::Option<bool> {
    self.has_ipv6().then(|| self.ipv6())
  }
  pub fn ipv6(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        11, (false).into()
      ).try_into().unwrap()
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn ignore_empty_opt(self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        13, (false).into()
      ).try_into().unwrap()
    }
  }

  pub fn well_known(self) -> super::bytes_rules::WellKnownOneof<'msg> {
    match self.well_known_case() {
      super::bytes_rules::WellKnownCase::Ip =>
          super::bytes_rules::WellKnownOneof::Ip(self.ip()),
      super::bytes_rules::WellKnownCase::Ipv4 =>
          super::bytes_rules::WellKnownOneof::Ipv4(self.ipv4()),
      super::bytes_rules::WellKnownCase::Ipv6 =>
          super::bytes_rules::WellKnownOneof::Ipv6(self.ipv6()),
      _ => super::bytes_rules::WellKnownOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn well_known_case(self) -> super::bytes_rules::WellKnownCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(9);
      super::bytes_rules::WellKnownCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `BytesRulesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for BytesRulesView<'_> {}

// SAFETY:
// - `BytesRulesView` is `Send` because while its alive a `BytesRulesMut` cannot.
// - `BytesRulesView` does not use thread-local data.
unsafe impl ::std::marker::Send for BytesRulesView<'_> {}

impl<'msg> ::protobuf::AsView for BytesRulesView<'msg> {
  type Proxied = BytesRules;
  fn as_view(&self) -> ::protobuf::View<'msg, BytesRules> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BytesRulesView<'msg> {
  fn into_view<'shorter>(self) -> BytesRulesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<BytesRules> for BytesRulesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BytesRules {
    let mut dst = BytesRules::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<BytesRules> for BytesRulesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BytesRules {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for BytesRules {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BytesRulesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BytesRulesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct BytesRulesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BytesRules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BytesRulesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for BytesRulesMut<'msg> {
  type Message = BytesRules;
}

impl ::std::fmt::Debug for BytesRulesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, BytesRules>> for BytesRulesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BytesRules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BytesRulesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, BytesRules> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> BytesRules {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // const: optional bytes
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<&'_ [u8]> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_const(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // len: optional uint64
  pub fn has_len(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_len(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn len_opt(&self) -> ::std::option::Option<u64> {
    self.has_len().then(|| self.len())
  }
  pub fn len(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        12, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_len(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        12, val.into()
      )
    }
  }

  // min_len: optional uint64
  pub fn has_min_len(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_min_len(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn min_len_opt(&self) -> ::std::option::Option<u64> {
    self.has_min_len().then(|| self.min_len())
  }
  pub fn min_len(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_min_len(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        1, val.into()
      )
    }
  }

  // max_len: optional uint64
  pub fn has_max_len(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_max_len(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn max_len_opt(&self) -> ::std::option::Option<u64> {
    self.has_max_len().then(|| self.max_len())
  }
  pub fn max_len(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        2, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_max_len(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        2, val.into()
      )
    }
  }

  // pattern: optional string
  pub fn has_pattern(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_pattern(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn pattern_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_pattern().then(|| self.pattern())
  }
  pub fn pattern(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_pattern(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // prefix: optional bytes
  pub fn has_prefix(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_prefix(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn prefix_opt(&self) -> ::std::option::Option<&'_ [u8]> {
    self.has_prefix().then(|| self.prefix())
  }
  pub fn prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // suffix: optional bytes
  pub fn has_suffix(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_suffix(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn suffix_opt(&self) -> ::std::option::Option<&'_ [u8]> {
    self.has_suffix().then(|| self.suffix())
  }
  pub fn suffix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_suffix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // contains: optional bytes
  pub fn has_contains(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_contains(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn contains_opt(&self) -> ::std::option::Option<&'_ [u8]> {
    self.has_contains().then(|| self.contains())
  }
  pub fn contains(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_contains(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // in: repeated bytes
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoBytes> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoBytes>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoBytes> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        7,
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoBytes>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        src);
    }
  }

  // not_in: repeated bytes
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoBytes> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        8
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoBytes>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoBytes> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        8,
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoBytes>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        src);
    }
  }

  // ip: optional bool
  pub fn has_ip(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_ip(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn ip_opt(&self) -> ::std::option::Option<bool> {
    self.has_ip().then(|| self.ip())
  }
  pub fn ip(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        9, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ip(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        9, val.into()
      )
    }
  }

  // ipv4: optional bool
  pub fn has_ipv4(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_ipv4(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn ipv4_opt(&self) -> ::std::option::Option<bool> {
    self.has_ipv4().then(|| self.ipv4())
  }
  pub fn ipv4(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        10, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ipv4(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        10, val.into()
      )
    }
  }

  // ipv6: optional bool
  pub fn has_ipv6(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_ipv6(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn ipv6_opt(&self) -> ::std::option::Option<bool> {
    self.has_ipv6().then(|| self.ipv6())
  }
  pub fn ipv6(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        11, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ipv6(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        11, val.into()
      )
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        13, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        13, val.into()
      )
    }
  }

  pub fn well_known(&self) -> super::bytes_rules::WellKnownOneof<'_> {
    match &self.well_known_case() {
      super::bytes_rules::WellKnownCase::Ip =>
          super::bytes_rules::WellKnownOneof::Ip(self.ip()),
      super::bytes_rules::WellKnownCase::Ipv4 =>
          super::bytes_rules::WellKnownOneof::Ipv4(self.ipv4()),
      super::bytes_rules::WellKnownCase::Ipv6 =>
          super::bytes_rules::WellKnownOneof::Ipv6(self.ipv6()),
      _ => super::bytes_rules::WellKnownOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn well_known_case(&self) -> super::bytes_rules::WellKnownCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(9);
      super::bytes_rules::WellKnownCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `BytesRulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for BytesRulesMut<'_> {}

// SAFETY:
// - `BytesRulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for BytesRulesMut<'_> {}

impl<'msg> ::protobuf::AsView for BytesRulesMut<'msg> {
  type Proxied = BytesRules;
  fn as_view(&self) -> ::protobuf::View<'_, BytesRules> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BytesRulesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, BytesRules>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for BytesRulesMut<'msg> {
  type MutProxied = BytesRules;
  fn as_mut(&mut self) -> BytesRulesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for BytesRulesMut<'msg> {
  fn into_mut<'shorter>(self) -> BytesRulesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl BytesRules {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, BytesRules> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> BytesRulesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> BytesRulesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // const: optional bytes
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<&'_ [u8]> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_const(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // len: optional uint64
  pub fn has_len(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_len(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn len_opt(&self) -> ::std::option::Option<u64> {
    self.has_len().then(|| self.len())
  }
  pub fn len(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        12, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_len(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        12, val.into()
      )
    }
  }

  // min_len: optional uint64
  pub fn has_min_len(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_min_len(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn min_len_opt(&self) -> ::std::option::Option<u64> {
    self.has_min_len().then(|| self.min_len())
  }
  pub fn min_len(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_min_len(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        1, val.into()
      )
    }
  }

  // max_len: optional uint64
  pub fn has_max_len(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_max_len(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn max_len_opt(&self) -> ::std::option::Option<u64> {
    self.has_max_len().then(|| self.max_len())
  }
  pub fn max_len(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        2, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_max_len(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        2, val.into()
      )
    }
  }

  // pattern: optional string
  pub fn has_pattern(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_pattern(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn pattern_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_pattern().then(|| self.pattern())
  }
  pub fn pattern(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_pattern(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // prefix: optional bytes
  pub fn has_prefix(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_prefix(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn prefix_opt(&self) -> ::std::option::Option<&'_ [u8]> {
    self.has_prefix().then(|| self.prefix())
  }
  pub fn prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // suffix: optional bytes
  pub fn has_suffix(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_suffix(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn suffix_opt(&self) -> ::std::option::Option<&'_ [u8]> {
    self.has_suffix().then(|| self.suffix())
  }
  pub fn suffix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_suffix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // contains: optional bytes
  pub fn has_contains(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_contains(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn contains_opt(&self) -> ::std::option::Option<&'_ [u8]> {
    self.has_contains().then(|| self.contains())
  }
  pub fn contains(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_contains(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // in: repeated bytes
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoBytes> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoBytes>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoBytes> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        7,
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoBytes>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        src);
    }
  }

  // not_in: repeated bytes
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoBytes> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        8
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoBytes>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoBytes> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        8,
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoBytes>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        src);
    }
  }

  // ip: optional bool
  pub fn has_ip(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_ip(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn ip_opt(&self) -> ::std::option::Option<bool> {
    self.has_ip().then(|| self.ip())
  }
  pub fn ip(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        9, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ip(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        9, val.into()
      )
    }
  }

  // ipv4: optional bool
  pub fn has_ipv4(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_ipv4(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn ipv4_opt(&self) -> ::std::option::Option<bool> {
    self.has_ipv4().then(|| self.ipv4())
  }
  pub fn ipv4(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        10, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ipv4(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        10, val.into()
      )
    }
  }

  // ipv6: optional bool
  pub fn has_ipv6(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_ipv6(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn ipv6_opt(&self) -> ::std::option::Option<bool> {
    self.has_ipv6().then(|| self.ipv6())
  }
  pub fn ipv6(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        11, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ipv6(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        11, val.into()
      )
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        13, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        13, val.into()
      )
    }
  }

  pub fn well_known(&self) -> super::bytes_rules::WellKnownOneof<'_> {
    match &self.well_known_case() {
      super::bytes_rules::WellKnownCase::Ip =>
          super::bytes_rules::WellKnownOneof::Ip(self.ip()),
      super::bytes_rules::WellKnownCase::Ipv4 =>
          super::bytes_rules::WellKnownOneof::Ipv4(self.ipv4()),
      super::bytes_rules::WellKnownCase::Ipv6 =>
          super::bytes_rules::WellKnownOneof::Ipv6(self.ipv6()),
      _ => super::bytes_rules::WellKnownOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn well_known_case(&self) -> super::bytes_rules::WellKnownCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(9);
      super::bytes_rules::WellKnownCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl BytesRules

impl ::std::ops::Drop for BytesRules {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for BytesRules {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for BytesRules {
  type Proxied = Self;
  fn as_view(&self) -> BytesRulesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for BytesRules {
  type MutProxied = Self;
  fn as_mut(&mut self) -> BytesRulesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for BytesRules {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::validate__BytesRules_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$0,,1000DD///,/^,|-|.");
        ::protobuf::__internal::runtime::link_mini_table(
            super::validate__BytesRules_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::validate__BytesRules_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BytesRules {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BytesRules {
  type Msg = BytesRules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BytesRules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BytesRules {
  type Msg = BytesRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BytesRules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BytesRulesMut<'_> {
  type Msg = BytesRules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BytesRules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BytesRulesMut<'_> {
  type Msg = BytesRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BytesRules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BytesRulesView<'_> {
  type Msg = BytesRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BytesRules> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BytesRulesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod bytes_rules {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum WellKnownOneof<'msg> {
  Ip(bool) = 10,
  Ipv4(bool) = 11,
  Ipv6(bool) = 12,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum WellKnownCase {
  Ip = 10,
  Ipv4 = 11,
  Ipv6 = 12,

  not_set = 0
}

impl WellKnownCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<WellKnownCase> {
    match v {
      0 => Some(WellKnownCase::not_set),
      10 => Some(WellKnownCase::Ip),
      11 => Some(WellKnownCase::Ipv4),
      12 => Some(WellKnownCase::Ipv6),
      _ => None
    }
  }
}
}  // pub mod bytes_rules


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut validate__EnumRules_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct EnumRules {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<EnumRules>
}

impl ::protobuf::Message for EnumRules {
  type MessageView<'msg> = EnumRulesView<'msg>;
  type MessageMut<'msg> = EnumRulesMut<'msg>;
}

impl ::std::default::Default for EnumRules {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for EnumRules {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `EnumRules` is `Sync` because it does not implement interior mutability.
//    Neither does `EnumRulesMut`.
unsafe impl ::std::marker::Sync for EnumRules {}

// SAFETY:
// - `EnumRules` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for EnumRules {}

impl ::protobuf::Proxied for EnumRules {
  type View<'msg> = EnumRulesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for EnumRules {}

impl ::protobuf::MutProxied for EnumRules {
  type Mut<'msg> = EnumRulesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct EnumRulesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EnumRules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EnumRulesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for EnumRulesView<'msg> {
  type Message = EnumRules;
}

impl ::std::fmt::Debug for EnumRulesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for EnumRulesView<'_> {
  fn default() -> EnumRulesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, EnumRules>> for EnumRulesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EnumRules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EnumRulesView<'msg> {

  pub fn to_owned(&self) -> EnumRules {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // const: optional int32
  pub fn has_const(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn const_opt(self) -> ::std::option::Option<i32> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // defined_only: optional bool
  pub fn has_defined_only(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn defined_only_opt(self) -> ::std::option::Option<bool> {
    self.has_defined_only().then(|| self.defined_only())
  }
  pub fn defined_only(self) -> bool {
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

  // in: repeated int32
  pub fn r#in(self) -> ::protobuf::RepeatedView<'msg, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // not_in: repeated int32
  pub fn not_in(self) -> ::protobuf::RepeatedView<'msg, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `EnumRulesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for EnumRulesView<'_> {}

// SAFETY:
// - `EnumRulesView` is `Send` because while its alive a `EnumRulesMut` cannot.
// - `EnumRulesView` does not use thread-local data.
unsafe impl ::std::marker::Send for EnumRulesView<'_> {}

impl<'msg> ::protobuf::AsView for EnumRulesView<'msg> {
  type Proxied = EnumRules;
  fn as_view(&self) -> ::protobuf::View<'msg, EnumRules> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EnumRulesView<'msg> {
  fn into_view<'shorter>(self) -> EnumRulesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<EnumRules> for EnumRulesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EnumRules {
    let mut dst = EnumRules::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<EnumRules> for EnumRulesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EnumRules {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for EnumRules {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for EnumRulesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for EnumRulesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct EnumRulesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EnumRules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EnumRulesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for EnumRulesMut<'msg> {
  type Message = EnumRules;
}

impl ::std::fmt::Debug for EnumRulesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, EnumRules>> for EnumRulesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EnumRules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EnumRulesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, EnumRules> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> EnumRules {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // const: optional int32
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<i32> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_const(&mut self, val: i32) {
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

  // defined_only: optional bool
  pub fn has_defined_only(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_defined_only(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn defined_only_opt(&self) -> ::std::option::Option<bool> {
    self.has_defined_only().then(|| self.defined_only())
  }
  pub fn defined_only(&self) -> bool {
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
  pub fn set_defined_only(&mut self, val: bool) {
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

  // in: repeated int32
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i32> {
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // not_in: repeated int32
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i32> {
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

}

// SAFETY:
// - `EnumRulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for EnumRulesMut<'_> {}

// SAFETY:
// - `EnumRulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for EnumRulesMut<'_> {}

impl<'msg> ::protobuf::AsView for EnumRulesMut<'msg> {
  type Proxied = EnumRules;
  fn as_view(&self) -> ::protobuf::View<'_, EnumRules> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EnumRulesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, EnumRules>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for EnumRulesMut<'msg> {
  type MutProxied = EnumRules;
  fn as_mut(&mut self) -> EnumRulesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for EnumRulesMut<'msg> {
  fn into_mut<'shorter>(self) -> EnumRulesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl EnumRules {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, EnumRules> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> EnumRulesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> EnumRulesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // const: optional int32
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<i32> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_const(&mut self, val: i32) {
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

  // defined_only: optional bool
  pub fn has_defined_only(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_defined_only(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn defined_only_opt(&self) -> ::std::option::Option<bool> {
    self.has_defined_only().then(|| self.defined_only())
  }
  pub fn defined_only(&self) -> bool {
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
  pub fn set_defined_only(&mut self, val: bool) {
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

  // in: repeated int32
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i32> {
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // not_in: repeated int32
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i32> {
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

}  // impl EnumRules

impl ::std::ops::Drop for EnumRules {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for EnumRules {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for EnumRules {
  type Proxied = Self;
  fn as_view(&self) -> EnumRulesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for EnumRules {
  type MutProxied = Self;
  fn as_mut(&mut self) -> EnumRulesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for EnumRules {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::validate__EnumRules_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$(/<<");
        ::protobuf::__internal::runtime::link_mini_table(
            super::validate__EnumRules_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::validate__EnumRules_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EnumRules {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EnumRules {
  type Msg = EnumRules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EnumRules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EnumRules {
  type Msg = EnumRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EnumRules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EnumRulesMut<'_> {
  type Msg = EnumRules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EnumRules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EnumRulesMut<'_> {
  type Msg = EnumRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EnumRules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EnumRulesView<'_> {
  type Msg = EnumRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EnumRules> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EnumRulesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut validate__MessageRules_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MessageRules {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MessageRules>
}

impl ::protobuf::Message for MessageRules {
  type MessageView<'msg> = MessageRulesView<'msg>;
  type MessageMut<'msg> = MessageRulesMut<'msg>;
}

impl ::std::default::Default for MessageRules {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MessageRules {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MessageRules` is `Sync` because it does not implement interior mutability.
//    Neither does `MessageRulesMut`.
unsafe impl ::std::marker::Sync for MessageRules {}

// SAFETY:
// - `MessageRules` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for MessageRules {}

impl ::protobuf::Proxied for MessageRules {
  type View<'msg> = MessageRulesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MessageRules {}

impl ::protobuf::MutProxied for MessageRules {
  type Mut<'msg> = MessageRulesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MessageRulesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MessageRules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MessageRulesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MessageRulesView<'msg> {
  type Message = MessageRules;
}

impl ::std::fmt::Debug for MessageRulesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MessageRulesView<'_> {
  fn default() -> MessageRulesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MessageRules>> for MessageRulesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MessageRules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MessageRulesView<'msg> {

  pub fn to_owned(&self) -> MessageRules {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // skip: optional bool
  pub fn has_skip(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn skip_opt(self) -> ::std::option::Option<bool> {
    self.has_skip().then(|| self.skip())
  }
  pub fn skip(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }

  // required: optional bool
  pub fn has_required(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn required_opt(self) -> ::std::option::Option<bool> {
    self.has_required().then(|| self.required())
  }
  pub fn required(self) -> bool {
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
// - `MessageRulesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for MessageRulesView<'_> {}

// SAFETY:
// - `MessageRulesView` is `Send` because while its alive a `MessageRulesMut` cannot.
// - `MessageRulesView` does not use thread-local data.
unsafe impl ::std::marker::Send for MessageRulesView<'_> {}

impl<'msg> ::protobuf::AsView for MessageRulesView<'msg> {
  type Proxied = MessageRules;
  fn as_view(&self) -> ::protobuf::View<'msg, MessageRules> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MessageRulesView<'msg> {
  fn into_view<'shorter>(self) -> MessageRulesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MessageRules> for MessageRulesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MessageRules {
    let mut dst = MessageRules::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MessageRules> for MessageRulesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MessageRules {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for MessageRules {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MessageRulesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MessageRulesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MessageRulesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MessageRules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MessageRulesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MessageRulesMut<'msg> {
  type Message = MessageRules;
}

impl ::std::fmt::Debug for MessageRulesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MessageRules>> for MessageRulesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MessageRules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MessageRulesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MessageRules> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> MessageRules {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // skip: optional bool
  pub fn has_skip(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_skip(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn skip_opt(&self) -> ::std::option::Option<bool> {
    self.has_skip().then(|| self.skip())
  }
  pub fn skip(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_skip(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        0, val.into()
      )
    }
  }

  // required: optional bool
  pub fn has_required(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_required(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn required_opt(&self) -> ::std::option::Option<bool> {
    self.has_required().then(|| self.required())
  }
  pub fn required(&self) -> bool {
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
  pub fn set_required(&mut self, val: bool) {
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
// - `MessageRulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for MessageRulesMut<'_> {}

// SAFETY:
// - `MessageRulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for MessageRulesMut<'_> {}

impl<'msg> ::protobuf::AsView for MessageRulesMut<'msg> {
  type Proxied = MessageRules;
  fn as_view(&self) -> ::protobuf::View<'_, MessageRules> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MessageRulesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MessageRules>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for MessageRulesMut<'msg> {
  type MutProxied = MessageRules;
  fn as_mut(&mut self) -> MessageRulesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MessageRulesMut<'msg> {
  fn into_mut<'shorter>(self) -> MessageRulesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MessageRules {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MessageRules> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MessageRulesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MessageRulesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // skip: optional bool
  pub fn has_skip(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_skip(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn skip_opt(&self) -> ::std::option::Option<bool> {
    self.has_skip().then(|| self.skip())
  }
  pub fn skip(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_skip(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        0, val.into()
      )
    }
  }

  // required: optional bool
  pub fn has_required(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_required(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn required_opt(&self) -> ::std::option::Option<bool> {
    self.has_required().then(|| self.required())
  }
  pub fn required(&self) -> bool {
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
  pub fn set_required(&mut self, val: bool) {
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

}  // impl MessageRules

impl ::std::ops::Drop for MessageRules {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MessageRules {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MessageRules {
  type Proxied = Self;
  fn as_view(&self) -> MessageRulesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MessageRules {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MessageRulesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MessageRules {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::validate__MessageRules_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$//");
        ::protobuf::__internal::runtime::link_mini_table(
            super::validate__MessageRules_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::validate__MessageRules_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MessageRules {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MessageRules {
  type Msg = MessageRules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MessageRules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MessageRules {
  type Msg = MessageRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MessageRules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MessageRulesMut<'_> {
  type Msg = MessageRules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MessageRules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MessageRulesMut<'_> {
  type Msg = MessageRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MessageRules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MessageRulesView<'_> {
  type Msg = MessageRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MessageRules> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MessageRulesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut validate__RepeatedRules_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RepeatedRules {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RepeatedRules>
}

impl ::protobuf::Message for RepeatedRules {
  type MessageView<'msg> = RepeatedRulesView<'msg>;
  type MessageMut<'msg> = RepeatedRulesMut<'msg>;
}

impl ::std::default::Default for RepeatedRules {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RepeatedRules {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RepeatedRules` is `Sync` because it does not implement interior mutability.
//    Neither does `RepeatedRulesMut`.
unsafe impl ::std::marker::Sync for RepeatedRules {}

// SAFETY:
// - `RepeatedRules` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RepeatedRules {}

impl ::protobuf::Proxied for RepeatedRules {
  type View<'msg> = RepeatedRulesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RepeatedRules {}

impl ::protobuf::MutProxied for RepeatedRules {
  type Mut<'msg> = RepeatedRulesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RepeatedRulesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RepeatedRules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RepeatedRulesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RepeatedRulesView<'msg> {
  type Message = RepeatedRules;
}

impl ::std::fmt::Debug for RepeatedRulesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RepeatedRulesView<'_> {
  fn default() -> RepeatedRulesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RepeatedRules>> for RepeatedRulesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RepeatedRules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RepeatedRulesView<'msg> {

  pub fn to_owned(&self) -> RepeatedRules {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // min_items: optional uint64
  pub fn has_min_items(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn min_items_opt(self) -> ::std::option::Option<u64> {
    self.has_min_items().then(|| self.min_items())
  }
  pub fn min_items(self) -> u64 {
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

  // max_items: optional uint64
  pub fn has_max_items(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn max_items_opt(self) -> ::std::option::Option<u64> {
    self.has_max_items().then(|| self.max_items())
  }
  pub fn max_items(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // unique: optional bool
  pub fn has_unique(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn unique_opt(self) -> ::std::option::Option<bool> {
    self.has_unique().then(|| self.unique())
  }
  pub fn unique(self) -> bool {
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

  // items: optional message validate.FieldRules
  pub fn has_items(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn items_opt(self) -> ::std::option::Option<super::FieldRulesView<'msg>> {
    self.has_items().then(|| self.items())
  }
  pub fn items(self) -> super::FieldRulesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldRulesView::default())
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn ignore_empty_opt(self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        4, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `RepeatedRulesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RepeatedRulesView<'_> {}

// SAFETY:
// - `RepeatedRulesView` is `Send` because while its alive a `RepeatedRulesMut` cannot.
// - `RepeatedRulesView` does not use thread-local data.
unsafe impl ::std::marker::Send for RepeatedRulesView<'_> {}

impl<'msg> ::protobuf::AsView for RepeatedRulesView<'msg> {
  type Proxied = RepeatedRules;
  fn as_view(&self) -> ::protobuf::View<'msg, RepeatedRules> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RepeatedRulesView<'msg> {
  fn into_view<'shorter>(self) -> RepeatedRulesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RepeatedRules> for RepeatedRulesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RepeatedRules {
    let mut dst = RepeatedRules::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RepeatedRules> for RepeatedRulesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RepeatedRules {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RepeatedRules {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RepeatedRulesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RepeatedRulesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RepeatedRulesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RepeatedRules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RepeatedRulesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RepeatedRulesMut<'msg> {
  type Message = RepeatedRules;
}

impl ::std::fmt::Debug for RepeatedRulesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RepeatedRules>> for RepeatedRulesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RepeatedRules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RepeatedRulesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RepeatedRules> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RepeatedRules {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // min_items: optional uint64
  pub fn has_min_items(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_min_items(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn min_items_opt(&self) -> ::std::option::Option<u64> {
    self.has_min_items().then(|| self.min_items())
  }
  pub fn min_items(&self) -> u64 {
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
  pub fn set_min_items(&mut self, val: u64) {
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

  // max_items: optional uint64
  pub fn has_max_items(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_max_items(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn max_items_opt(&self) -> ::std::option::Option<u64> {
    self.has_max_items().then(|| self.max_items())
  }
  pub fn max_items(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_max_items(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        1, val.into()
      )
    }
  }

  // unique: optional bool
  pub fn has_unique(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_unique(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn unique_opt(&self) -> ::std::option::Option<bool> {
    self.has_unique().then(|| self.unique())
  }
  pub fn unique(&self) -> bool {
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
  pub fn set_unique(&mut self, val: bool) {
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

  // items: optional message validate.FieldRules
  pub fn has_items(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_items(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn items_opt(&self) -> ::std::option::Option<super::FieldRulesView<'_>> {
    self.has_items().then(|| self.items())
  }
  pub fn items(&self) -> super::FieldRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldRulesView::default())
  }
  pub fn items_mut(&mut self) -> super::FieldRulesMut<'_> {
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
  pub fn set_items(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        4, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        4, val.into()
      )
    }
  }

}

// SAFETY:
// - `RepeatedRulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RepeatedRulesMut<'_> {}

// SAFETY:
// - `RepeatedRulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RepeatedRulesMut<'_> {}

impl<'msg> ::protobuf::AsView for RepeatedRulesMut<'msg> {
  type Proxied = RepeatedRules;
  fn as_view(&self) -> ::protobuf::View<'_, RepeatedRules> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RepeatedRulesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RepeatedRules>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RepeatedRulesMut<'msg> {
  type MutProxied = RepeatedRules;
  fn as_mut(&mut self) -> RepeatedRulesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RepeatedRulesMut<'msg> {
  fn into_mut<'shorter>(self) -> RepeatedRulesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RepeatedRules {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RepeatedRules> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RepeatedRulesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RepeatedRulesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // min_items: optional uint64
  pub fn has_min_items(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_min_items(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn min_items_opt(&self) -> ::std::option::Option<u64> {
    self.has_min_items().then(|| self.min_items())
  }
  pub fn min_items(&self) -> u64 {
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
  pub fn set_min_items(&mut self, val: u64) {
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

  // max_items: optional uint64
  pub fn has_max_items(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_max_items(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn max_items_opt(&self) -> ::std::option::Option<u64> {
    self.has_max_items().then(|| self.max_items())
  }
  pub fn max_items(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_max_items(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        1, val.into()
      )
    }
  }

  // unique: optional bool
  pub fn has_unique(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_unique(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn unique_opt(&self) -> ::std::option::Option<bool> {
    self.has_unique().then(|| self.unique())
  }
  pub fn unique(&self) -> bool {
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
  pub fn set_unique(&mut self, val: bool) {
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

  // items: optional message validate.FieldRules
  pub fn has_items(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_items(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn items_opt(&self) -> ::std::option::Option<super::FieldRulesView<'_>> {
    self.has_items().then(|| self.items())
  }
  pub fn items(&self) -> super::FieldRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldRulesView::default())
  }
  pub fn items_mut(&mut self) -> super::FieldRulesMut<'_> {
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
  pub fn set_items(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        4, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        4, val.into()
      )
    }
  }

}  // impl RepeatedRules

impl ::std::ops::Drop for RepeatedRules {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RepeatedRules {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RepeatedRules {
  type Proxied = Self;
  fn as_view(&self) -> RepeatedRulesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RepeatedRules {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RepeatedRulesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RepeatedRules {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::FieldRules as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::validate__RepeatedRules_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RepeatedRules {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RepeatedRules {
  type Msg = RepeatedRules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RepeatedRules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RepeatedRules {
  type Msg = RepeatedRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RepeatedRules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RepeatedRulesMut<'_> {
  type Msg = RepeatedRules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RepeatedRules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RepeatedRulesMut<'_> {
  type Msg = RepeatedRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RepeatedRules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RepeatedRulesView<'_> {
  type Msg = RepeatedRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RepeatedRules> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RepeatedRulesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut validate__MapRules_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MapRules {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MapRules>
}

impl ::protobuf::Message for MapRules {
  type MessageView<'msg> = MapRulesView<'msg>;
  type MessageMut<'msg> = MapRulesMut<'msg>;
}

impl ::std::default::Default for MapRules {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MapRules {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MapRules` is `Sync` because it does not implement interior mutability.
//    Neither does `MapRulesMut`.
unsafe impl ::std::marker::Sync for MapRules {}

// SAFETY:
// - `MapRules` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for MapRules {}

impl ::protobuf::Proxied for MapRules {
  type View<'msg> = MapRulesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MapRules {}

impl ::protobuf::MutProxied for MapRules {
  type Mut<'msg> = MapRulesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MapRulesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MapRules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MapRulesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MapRulesView<'msg> {
  type Message = MapRules;
}

impl ::std::fmt::Debug for MapRulesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MapRulesView<'_> {
  fn default() -> MapRulesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MapRules>> for MapRulesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MapRules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MapRulesView<'msg> {

  pub fn to_owned(&self) -> MapRules {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // min_pairs: optional uint64
  pub fn has_min_pairs(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn min_pairs_opt(self) -> ::std::option::Option<u64> {
    self.has_min_pairs().then(|| self.min_pairs())
  }
  pub fn min_pairs(self) -> u64 {
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

  // max_pairs: optional uint64
  pub fn has_max_pairs(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn max_pairs_opt(self) -> ::std::option::Option<u64> {
    self.has_max_pairs().then(|| self.max_pairs())
  }
  pub fn max_pairs(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // no_sparse: optional bool
  pub fn has_no_sparse(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn no_sparse_opt(self) -> ::std::option::Option<bool> {
    self.has_no_sparse().then(|| self.no_sparse())
  }
  pub fn no_sparse(self) -> bool {
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

  // keys: optional message validate.FieldRules
  pub fn has_keys(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn keys_opt(self) -> ::std::option::Option<super::FieldRulesView<'msg>> {
    self.has_keys().then(|| self.keys())
  }
  pub fn keys(self) -> super::FieldRulesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldRulesView::default())
  }

  // values: optional message validate.FieldRules
  pub fn has_values(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn values_opt(self) -> ::std::option::Option<super::FieldRulesView<'msg>> {
    self.has_values().then(|| self.values())
  }
  pub fn values(self) -> super::FieldRulesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldRulesView::default())
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn ignore_empty_opt(self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        5, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `MapRulesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for MapRulesView<'_> {}

// SAFETY:
// - `MapRulesView` is `Send` because while its alive a `MapRulesMut` cannot.
// - `MapRulesView` does not use thread-local data.
unsafe impl ::std::marker::Send for MapRulesView<'_> {}

impl<'msg> ::protobuf::AsView for MapRulesView<'msg> {
  type Proxied = MapRules;
  fn as_view(&self) -> ::protobuf::View<'msg, MapRules> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MapRulesView<'msg> {
  fn into_view<'shorter>(self) -> MapRulesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MapRules> for MapRulesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MapRules {
    let mut dst = MapRules::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MapRules> for MapRulesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MapRules {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for MapRules {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MapRulesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MapRulesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MapRulesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MapRules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MapRulesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MapRulesMut<'msg> {
  type Message = MapRules;
}

impl ::std::fmt::Debug for MapRulesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MapRules>> for MapRulesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MapRules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MapRulesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MapRules> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> MapRules {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // min_pairs: optional uint64
  pub fn has_min_pairs(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_min_pairs(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn min_pairs_opt(&self) -> ::std::option::Option<u64> {
    self.has_min_pairs().then(|| self.min_pairs())
  }
  pub fn min_pairs(&self) -> u64 {
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
  pub fn set_min_pairs(&mut self, val: u64) {
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

  // max_pairs: optional uint64
  pub fn has_max_pairs(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_max_pairs(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn max_pairs_opt(&self) -> ::std::option::Option<u64> {
    self.has_max_pairs().then(|| self.max_pairs())
  }
  pub fn max_pairs(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_max_pairs(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        1, val.into()
      )
    }
  }

  // no_sparse: optional bool
  pub fn has_no_sparse(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_no_sparse(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn no_sparse_opt(&self) -> ::std::option::Option<bool> {
    self.has_no_sparse().then(|| self.no_sparse())
  }
  pub fn no_sparse(&self) -> bool {
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
  pub fn set_no_sparse(&mut self, val: bool) {
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

  // keys: optional message validate.FieldRules
  pub fn has_keys(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_keys(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn keys_opt(&self) -> ::std::option::Option<super::FieldRulesView<'_>> {
    self.has_keys().then(|| self.keys())
  }
  pub fn keys(&self) -> super::FieldRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldRulesView::default())
  }
  pub fn keys_mut(&mut self) -> super::FieldRulesMut<'_> {
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
  pub fn set_keys(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // values: optional message validate.FieldRules
  pub fn has_values(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_values(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn values_opt(&self) -> ::std::option::Option<super::FieldRulesView<'_>> {
    self.has_values().then(|| self.values())
  }
  pub fn values(&self) -> super::FieldRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldRulesView::default())
  }
  pub fn values_mut(&mut self) -> super::FieldRulesMut<'_> {
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
  pub fn set_values(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        5, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        5, val.into()
      )
    }
  }

}

// SAFETY:
// - `MapRulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for MapRulesMut<'_> {}

// SAFETY:
// - `MapRulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for MapRulesMut<'_> {}

impl<'msg> ::protobuf::AsView for MapRulesMut<'msg> {
  type Proxied = MapRules;
  fn as_view(&self) -> ::protobuf::View<'_, MapRules> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MapRulesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MapRules>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for MapRulesMut<'msg> {
  type MutProxied = MapRules;
  fn as_mut(&mut self) -> MapRulesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MapRulesMut<'msg> {
  fn into_mut<'shorter>(self) -> MapRulesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MapRules {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MapRules> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MapRulesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MapRulesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // min_pairs: optional uint64
  pub fn has_min_pairs(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_min_pairs(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn min_pairs_opt(&self) -> ::std::option::Option<u64> {
    self.has_min_pairs().then(|| self.min_pairs())
  }
  pub fn min_pairs(&self) -> u64 {
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
  pub fn set_min_pairs(&mut self, val: u64) {
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

  // max_pairs: optional uint64
  pub fn has_max_pairs(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_max_pairs(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn max_pairs_opt(&self) -> ::std::option::Option<u64> {
    self.has_max_pairs().then(|| self.max_pairs())
  }
  pub fn max_pairs(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_max_pairs(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        1, val.into()
      )
    }
  }

  // no_sparse: optional bool
  pub fn has_no_sparse(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_no_sparse(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn no_sparse_opt(&self) -> ::std::option::Option<bool> {
    self.has_no_sparse().then(|| self.no_sparse())
  }
  pub fn no_sparse(&self) -> bool {
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
  pub fn set_no_sparse(&mut self, val: bool) {
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

  // keys: optional message validate.FieldRules
  pub fn has_keys(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_keys(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn keys_opt(&self) -> ::std::option::Option<super::FieldRulesView<'_>> {
    self.has_keys().then(|| self.keys())
  }
  pub fn keys(&self) -> super::FieldRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldRulesView::default())
  }
  pub fn keys_mut(&mut self) -> super::FieldRulesMut<'_> {
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
  pub fn set_keys(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // values: optional message validate.FieldRules
  pub fn has_values(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_values(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn values_opt(&self) -> ::std::option::Option<super::FieldRulesView<'_>> {
    self.has_values().then(|| self.values())
  }
  pub fn values(&self) -> super::FieldRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldRulesView::default())
  }
  pub fn values_mut(&mut self) -> super::FieldRulesMut<'_> {
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
  pub fn set_values(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // ignore_empty: optional bool
  pub fn has_ignore_empty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_ignore_empty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn ignore_empty_opt(&self) -> ::std::option::Option<bool> {
    self.has_ignore_empty().then(|| self.ignore_empty())
  }
  pub fn ignore_empty(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        5, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_empty(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        5, val.into()
      )
    }
  }

}  // impl MapRules

impl ::std::ops::Drop for MapRules {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MapRules {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MapRules {
  type Proxied = Self;
  fn as_view(&self) -> MapRulesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MapRules {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MapRulesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MapRules {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::FieldRules as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::validate__MapRules_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MapRules {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MapRules {
  type Msg = MapRules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MapRules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MapRules {
  type Msg = MapRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MapRules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MapRulesMut<'_> {
  type Msg = MapRules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MapRules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MapRulesMut<'_> {
  type Msg = MapRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MapRules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MapRulesView<'_> {
  type Msg = MapRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MapRules> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MapRulesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut validate__AnyRules_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AnyRules {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AnyRules>
}

impl ::protobuf::Message for AnyRules {
  type MessageView<'msg> = AnyRulesView<'msg>;
  type MessageMut<'msg> = AnyRulesMut<'msg>;
}

impl ::std::default::Default for AnyRules {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AnyRules {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AnyRules` is `Sync` because it does not implement interior mutability.
//    Neither does `AnyRulesMut`.
unsafe impl ::std::marker::Sync for AnyRules {}

// SAFETY:
// - `AnyRules` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for AnyRules {}

impl ::protobuf::Proxied for AnyRules {
  type View<'msg> = AnyRulesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AnyRules {}

impl ::protobuf::MutProxied for AnyRules {
  type Mut<'msg> = AnyRulesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AnyRulesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AnyRules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AnyRulesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AnyRulesView<'msg> {
  type Message = AnyRules;
}

impl ::std::fmt::Debug for AnyRulesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AnyRulesView<'_> {
  fn default() -> AnyRulesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AnyRules>> for AnyRulesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AnyRules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AnyRulesView<'msg> {

  pub fn to_owned(&self) -> AnyRules {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // required: optional bool
  pub fn has_required(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn required_opt(self) -> ::std::option::Option<bool> {
    self.has_required().then(|| self.required())
  }
  pub fn required(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }

  // in: repeated string
  pub fn r#in(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
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

  // not_in: repeated string
  pub fn not_in(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `AnyRulesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AnyRulesView<'_> {}

// SAFETY:
// - `AnyRulesView` is `Send` because while its alive a `AnyRulesMut` cannot.
// - `AnyRulesView` does not use thread-local data.
unsafe impl ::std::marker::Send for AnyRulesView<'_> {}

impl<'msg> ::protobuf::AsView for AnyRulesView<'msg> {
  type Proxied = AnyRules;
  fn as_view(&self) -> ::protobuf::View<'msg, AnyRules> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AnyRulesView<'msg> {
  fn into_view<'shorter>(self) -> AnyRulesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AnyRules> for AnyRulesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AnyRules {
    let mut dst = AnyRules::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AnyRules> for AnyRulesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AnyRules {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for AnyRules {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AnyRulesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AnyRulesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AnyRulesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AnyRules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AnyRulesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AnyRulesMut<'msg> {
  type Message = AnyRules;
}

impl ::std::fmt::Debug for AnyRulesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AnyRules>> for AnyRulesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AnyRules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AnyRulesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AnyRules> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> AnyRules {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // required: optional bool
  pub fn has_required(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_required(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn required_opt(&self) -> ::std::option::Option<bool> {
    self.has_required().then(|| self.required())
  }
  pub fn required(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_required(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        0, val.into()
      )
    }
  }

  // in: repeated string
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // not_in: repeated string
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}

// SAFETY:
// - `AnyRulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AnyRulesMut<'_> {}

// SAFETY:
// - `AnyRulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AnyRulesMut<'_> {}

impl<'msg> ::protobuf::AsView for AnyRulesMut<'msg> {
  type Proxied = AnyRules;
  fn as_view(&self) -> ::protobuf::View<'_, AnyRules> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AnyRulesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AnyRules>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AnyRulesMut<'msg> {
  type MutProxied = AnyRules;
  fn as_mut(&mut self) -> AnyRulesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AnyRulesMut<'msg> {
  fn into_mut<'shorter>(self) -> AnyRulesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AnyRules {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AnyRules> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AnyRulesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AnyRulesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // required: optional bool
  pub fn has_required(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_required(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn required_opt(&self) -> ::std::option::Option<bool> {
    self.has_required().then(|| self.required())
  }
  pub fn required(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_required(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        0, val.into()
      )
    }
  }

  // in: repeated string
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // not_in: repeated string
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}  // impl AnyRules

impl ::std::ops::Drop for AnyRules {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AnyRules {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AnyRules {
  type Proxied = Self;
  fn as_view(&self) -> AnyRulesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AnyRules {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AnyRulesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AnyRules {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::validate__AnyRules_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$/EE");
        ::protobuf::__internal::runtime::link_mini_table(
            super::validate__AnyRules_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::validate__AnyRules_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AnyRules {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AnyRules {
  type Msg = AnyRules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AnyRules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AnyRules {
  type Msg = AnyRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AnyRules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AnyRulesMut<'_> {
  type Msg = AnyRules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AnyRules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AnyRulesMut<'_> {
  type Msg = AnyRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AnyRules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AnyRulesView<'_> {
  type Msg = AnyRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AnyRules> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AnyRulesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut validate__DurationRules_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DurationRules {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DurationRules>
}

impl ::protobuf::Message for DurationRules {
  type MessageView<'msg> = DurationRulesView<'msg>;
  type MessageMut<'msg> = DurationRulesMut<'msg>;
}

impl ::std::default::Default for DurationRules {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DurationRules {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DurationRules` is `Sync` because it does not implement interior mutability.
//    Neither does `DurationRulesMut`.
unsafe impl ::std::marker::Sync for DurationRules {}

// SAFETY:
// - `DurationRules` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DurationRules {}

impl ::protobuf::Proxied for DurationRules {
  type View<'msg> = DurationRulesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DurationRules {}

impl ::protobuf::MutProxied for DurationRules {
  type Mut<'msg> = DurationRulesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DurationRulesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DurationRules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DurationRulesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DurationRulesView<'msg> {
  type Message = DurationRules;
}

impl ::std::fmt::Debug for DurationRulesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DurationRulesView<'_> {
  fn default() -> DurationRulesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DurationRules>> for DurationRulesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DurationRules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DurationRulesView<'msg> {

  pub fn to_owned(&self) -> DurationRules {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // required: optional bool
  pub fn has_required(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn required_opt(self) -> ::std::option::Option<bool> {
    self.has_required().then(|| self.required())
  }
  pub fn required(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }

  // const: optional message google.protobuf.Duration
  pub fn has_const(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn const_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // lt: optional message google.protobuf.Duration
  pub fn has_lt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn lt_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // lte: optional message google.protobuf.Duration
  pub fn has_lte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn lte_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // gt: optional message google.protobuf.Duration
  pub fn has_gt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn gt_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // gte: optional message google.protobuf.Duration
  pub fn has_gte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn gte_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // in: repeated message google.protobuf.Duration
  pub fn r#in(self) -> ::protobuf::RepeatedView<'msg, ::protobuf_well_known_types::Duration> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf_well_known_types::Duration>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // not_in: repeated message google.protobuf.Duration
  pub fn not_in(self) -> ::protobuf::RepeatedView<'msg, ::protobuf_well_known_types::Duration> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf_well_known_types::Duration>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `DurationRulesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DurationRulesView<'_> {}

// SAFETY:
// - `DurationRulesView` is `Send` because while its alive a `DurationRulesMut` cannot.
// - `DurationRulesView` does not use thread-local data.
unsafe impl ::std::marker::Send for DurationRulesView<'_> {}

impl<'msg> ::protobuf::AsView for DurationRulesView<'msg> {
  type Proxied = DurationRules;
  fn as_view(&self) -> ::protobuf::View<'msg, DurationRules> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DurationRulesView<'msg> {
  fn into_view<'shorter>(self) -> DurationRulesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DurationRules> for DurationRulesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DurationRules {
    let mut dst = DurationRules::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DurationRules> for DurationRulesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DurationRules {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DurationRules {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DurationRulesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DurationRulesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DurationRulesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DurationRules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DurationRulesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DurationRulesMut<'msg> {
  type Message = DurationRules;
}

impl ::std::fmt::Debug for DurationRulesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DurationRules>> for DurationRulesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DurationRules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DurationRulesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DurationRules> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DurationRules {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // required: optional bool
  pub fn has_required(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_required(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn required_opt(&self) -> ::std::option::Option<bool> {
    self.has_required().then(|| self.required())
  }
  pub fn required(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_required(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        0, val.into()
      )
    }
  }

  // const: optional message google.protobuf.Duration
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn const_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_const(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // lt: optional message google.protobuf.Duration
  pub fn has_lt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_lt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn lt_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn lt_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_lt(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // lte: optional message google.protobuf.Duration
  pub fn has_lte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_lte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn lte_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn lte_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_lte(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // gt: optional message google.protobuf.Duration
  pub fn has_gt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_gt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn gt_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn gt_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_gt(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // gte: optional message google.protobuf.Duration
  pub fn has_gte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_gte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn gte_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn gte_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_gte(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // in: repeated message google.protobuf.Duration
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, ::protobuf_well_known_types::Duration> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf_well_known_types::Duration>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf_well_known_types::Duration> {
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf_well_known_types::Duration>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // not_in: repeated message google.protobuf.Duration
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, ::protobuf_well_known_types::Duration> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf_well_known_types::Duration>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf_well_known_types::Duration> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        7,
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf_well_known_types::Duration>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        src);
    }
  }

}

// SAFETY:
// - `DurationRulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DurationRulesMut<'_> {}

// SAFETY:
// - `DurationRulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DurationRulesMut<'_> {}

impl<'msg> ::protobuf::AsView for DurationRulesMut<'msg> {
  type Proxied = DurationRules;
  fn as_view(&self) -> ::protobuf::View<'_, DurationRules> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DurationRulesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DurationRules>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DurationRulesMut<'msg> {
  type MutProxied = DurationRules;
  fn as_mut(&mut self) -> DurationRulesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DurationRulesMut<'msg> {
  fn into_mut<'shorter>(self) -> DurationRulesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DurationRules {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DurationRules> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DurationRulesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DurationRulesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // required: optional bool
  pub fn has_required(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_required(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn required_opt(&self) -> ::std::option::Option<bool> {
    self.has_required().then(|| self.required())
  }
  pub fn required(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_required(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        0, val.into()
      )
    }
  }

  // const: optional message google.protobuf.Duration
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn const_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_const(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // lt: optional message google.protobuf.Duration
  pub fn has_lt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_lt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn lt_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn lt_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_lt(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // lte: optional message google.protobuf.Duration
  pub fn has_lte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_lte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn lte_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn lte_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_lte(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // gt: optional message google.protobuf.Duration
  pub fn has_gt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_gt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn gt_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn gt_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_gt(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // gte: optional message google.protobuf.Duration
  pub fn has_gte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_gte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn gte_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn gte_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_gte(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // in: repeated message google.protobuf.Duration
  pub fn r#in(&self) -> ::protobuf::RepeatedView<'_, ::protobuf_well_known_types::Duration> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf_well_known_types::Duration>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn r#in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf_well_known_types::Duration> {
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
  pub fn set_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf_well_known_types::Duration>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // not_in: repeated message google.protobuf.Duration
  pub fn not_in(&self) -> ::protobuf::RepeatedView<'_, ::protobuf_well_known_types::Duration> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf_well_known_types::Duration>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn not_in_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf_well_known_types::Duration> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        7,
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
  pub fn set_not_in(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf_well_known_types::Duration>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        src);
    }
  }

}  // impl DurationRules

impl ::std::ops::Drop for DurationRules {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DurationRules {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DurationRules {
  type Proxied = Self;
  fn as_view(&self) -> DurationRulesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DurationRules {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DurationRulesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DurationRules {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::validate__DurationRules_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$/33333GG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::validate__DurationRules_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::validate__DurationRules_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DurationRules {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DurationRules {
  type Msg = DurationRules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DurationRules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DurationRules {
  type Msg = DurationRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DurationRules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DurationRulesMut<'_> {
  type Msg = DurationRules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DurationRules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DurationRulesMut<'_> {
  type Msg = DurationRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DurationRules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DurationRulesView<'_> {
  type Msg = DurationRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DurationRules> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DurationRulesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut validate__TimestampRules_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TimestampRules {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TimestampRules>
}

impl ::protobuf::Message for TimestampRules {
  type MessageView<'msg> = TimestampRulesView<'msg>;
  type MessageMut<'msg> = TimestampRulesMut<'msg>;
}

impl ::std::default::Default for TimestampRules {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TimestampRules {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TimestampRules` is `Sync` because it does not implement interior mutability.
//    Neither does `TimestampRulesMut`.
unsafe impl ::std::marker::Sync for TimestampRules {}

// SAFETY:
// - `TimestampRules` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TimestampRules {}

impl ::protobuf::Proxied for TimestampRules {
  type View<'msg> = TimestampRulesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TimestampRules {}

impl ::protobuf::MutProxied for TimestampRules {
  type Mut<'msg> = TimestampRulesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TimestampRulesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TimestampRules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TimestampRulesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TimestampRulesView<'msg> {
  type Message = TimestampRules;
}

impl ::std::fmt::Debug for TimestampRulesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TimestampRulesView<'_> {
  fn default() -> TimestampRulesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TimestampRules>> for TimestampRulesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TimestampRules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TimestampRulesView<'msg> {

  pub fn to_owned(&self) -> TimestampRules {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // required: optional bool
  pub fn has_required(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn required_opt(self) -> ::std::option::Option<bool> {
    self.has_required().then(|| self.required())
  }
  pub fn required(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }

  // const: optional message google.protobuf.Timestamp
  pub fn has_const(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn const_opt(self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'msg>> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

  // lt: optional message google.protobuf.Timestamp
  pub fn has_lt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn lt_opt(self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'msg>> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

  // lte: optional message google.protobuf.Timestamp
  pub fn has_lte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn lte_opt(self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'msg>> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

  // gt: optional message google.protobuf.Timestamp
  pub fn has_gt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn gt_opt(self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'msg>> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

  // gte: optional message google.protobuf.Timestamp
  pub fn has_gte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn gte_opt(self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'msg>> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

  // lt_now: optional bool
  pub fn has_lt_now(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn lt_now_opt(self) -> ::std::option::Option<bool> {
    self.has_lt_now().then(|| self.lt_now())
  }
  pub fn lt_now(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        6, (false).into()
      ).try_into().unwrap()
    }
  }

  // gt_now: optional bool
  pub fn has_gt_now(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn gt_now_opt(self) -> ::std::option::Option<bool> {
    self.has_gt_now().then(|| self.gt_now())
  }
  pub fn gt_now(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }

  // within: optional message google.protobuf.Duration
  pub fn has_within(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn within_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_within().then(|| self.within())
  }
  pub fn within(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

}

// SAFETY:
// - `TimestampRulesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TimestampRulesView<'_> {}

// SAFETY:
// - `TimestampRulesView` is `Send` because while its alive a `TimestampRulesMut` cannot.
// - `TimestampRulesView` does not use thread-local data.
unsafe impl ::std::marker::Send for TimestampRulesView<'_> {}

impl<'msg> ::protobuf::AsView for TimestampRulesView<'msg> {
  type Proxied = TimestampRules;
  fn as_view(&self) -> ::protobuf::View<'msg, TimestampRules> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TimestampRulesView<'msg> {
  fn into_view<'shorter>(self) -> TimestampRulesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TimestampRules> for TimestampRulesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TimestampRules {
    let mut dst = TimestampRules::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TimestampRules> for TimestampRulesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TimestampRules {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TimestampRules {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TimestampRulesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TimestampRulesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TimestampRulesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TimestampRules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TimestampRulesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TimestampRulesMut<'msg> {
  type Message = TimestampRules;
}

impl ::std::fmt::Debug for TimestampRulesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TimestampRules>> for TimestampRulesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TimestampRules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TimestampRulesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TimestampRules> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TimestampRules {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // required: optional bool
  pub fn has_required(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_required(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn required_opt(&self) -> ::std::option::Option<bool> {
    self.has_required().then(|| self.required())
  }
  pub fn required(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_required(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        0, val.into()
      )
    }
  }

  // const: optional message google.protobuf.Timestamp
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn const_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_const(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // lt: optional message google.protobuf.Timestamp
  pub fn has_lt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_lt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn lt_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn lt_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_lt(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // lte: optional message google.protobuf.Timestamp
  pub fn has_lte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_lte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn lte_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn lte_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_lte(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // gt: optional message google.protobuf.Timestamp
  pub fn has_gt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_gt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn gt_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn gt_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_gt(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // gte: optional message google.protobuf.Timestamp
  pub fn has_gte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_gte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn gte_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn gte_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_gte(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // lt_now: optional bool
  pub fn has_lt_now(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_lt_now(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn lt_now_opt(&self) -> ::std::option::Option<bool> {
    self.has_lt_now().then(|| self.lt_now())
  }
  pub fn lt_now(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        6, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lt_now(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        6, val.into()
      )
    }
  }

  // gt_now: optional bool
  pub fn has_gt_now(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_gt_now(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn gt_now_opt(&self) -> ::std::option::Option<bool> {
    self.has_gt_now().then(|| self.gt_now())
  }
  pub fn gt_now(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gt_now(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

  // within: optional message google.protobuf.Duration
  pub fn has_within(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_within(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn within_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_within().then(|| self.within())
  }
  pub fn within(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn within_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_within(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

}

// SAFETY:
// - `TimestampRulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TimestampRulesMut<'_> {}

// SAFETY:
// - `TimestampRulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TimestampRulesMut<'_> {}

impl<'msg> ::protobuf::AsView for TimestampRulesMut<'msg> {
  type Proxied = TimestampRules;
  fn as_view(&self) -> ::protobuf::View<'_, TimestampRules> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TimestampRulesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TimestampRules>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TimestampRulesMut<'msg> {
  type MutProxied = TimestampRules;
  fn as_mut(&mut self) -> TimestampRulesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TimestampRulesMut<'msg> {
  fn into_mut<'shorter>(self) -> TimestampRulesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TimestampRules {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TimestampRules> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TimestampRulesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TimestampRulesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // required: optional bool
  pub fn has_required(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_required(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn required_opt(&self) -> ::std::option::Option<bool> {
    self.has_required().then(|| self.required())
  }
  pub fn required(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_required(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        0, val.into()
      )
    }
  }

  // const: optional message google.protobuf.Timestamp
  pub fn has_const(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_const(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn const_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_const().then(|| self.r#const())
  }
  pub fn r#const(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn const_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_const(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // lt: optional message google.protobuf.Timestamp
  pub fn has_lt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_lt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn lt_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_lt().then(|| self.lt())
  }
  pub fn lt(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn lt_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_lt(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // lte: optional message google.protobuf.Timestamp
  pub fn has_lte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_lte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn lte_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_lte().then(|| self.lte())
  }
  pub fn lte(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn lte_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_lte(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // gt: optional message google.protobuf.Timestamp
  pub fn has_gt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_gt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn gt_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_gt().then(|| self.gt())
  }
  pub fn gt(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn gt_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_gt(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // gte: optional message google.protobuf.Timestamp
  pub fn has_gte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_gte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn gte_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_gte().then(|| self.gte())
  }
  pub fn gte(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn gte_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_gte(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // lt_now: optional bool
  pub fn has_lt_now(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_lt_now(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn lt_now_opt(&self) -> ::std::option::Option<bool> {
    self.has_lt_now().then(|| self.lt_now())
  }
  pub fn lt_now(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        6, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lt_now(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        6, val.into()
      )
    }
  }

  // gt_now: optional bool
  pub fn has_gt_now(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_gt_now(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn gt_now_opt(&self) -> ::std::option::Option<bool> {
    self.has_gt_now().then(|| self.gt_now())
  }
  pub fn gt_now(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gt_now(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

  // within: optional message google.protobuf.Duration
  pub fn has_within(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_within(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn within_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_within().then(|| self.within())
  }
  pub fn within(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn within_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_within(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

}  // impl TimestampRules

impl ::std::ops::Drop for TimestampRules {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TimestampRules {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TimestampRules {
  type Proxied = Self;
  fn as_view(&self) -> TimestampRulesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TimestampRules {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TimestampRulesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TimestampRules {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::validate__TimestampRules_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$/33333//3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::validate__TimestampRules_msg_init.0, &[<::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::validate__TimestampRules_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TimestampRules {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TimestampRules {
  type Msg = TimestampRules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TimestampRules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TimestampRules {
  type Msg = TimestampRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TimestampRules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TimestampRulesMut<'_> {
  type Msg = TimestampRules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TimestampRules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TimestampRulesMut<'_> {
  type Msg = TimestampRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TimestampRules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TimestampRulesView<'_> {
  type Msg = TimestampRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TimestampRules> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TimestampRulesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct KnownRegex(i32);

#[allow(non_upper_case_globals)]
impl KnownRegex {
  pub const Unknown: KnownRegex = KnownRegex(0);
  pub const HttpHeaderName: KnownRegex = KnownRegex(1);
  pub const HttpHeaderValue: KnownRegex = KnownRegex(2);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unknown",
      1 => "HttpHeaderName",
      2 => "HttpHeaderValue",
      _ => return None
    })
  }
}

impl ::std::convert::From<KnownRegex> for i32 {
  fn from(val: KnownRegex) -> i32 {
    val.0
  }
}

impl ::std::convert::TryFrom<i32> for KnownRegex {
  type Error = ::protobuf::UnknownEnumValue<Self>;

  fn try_from(val: i32) -> ::std::result::Result<KnownRegex, Self::Error> {
    if <Self as ::protobuf::__internal::Enum>::is_known(val) {
      Ok(Self(val))
    } else {
      Err(::protobuf::UnknownEnumValue::new(::protobuf::__internal::Private, val))
    }
  }
}

impl ::std::default::Default for KnownRegex {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for KnownRegex {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "KnownRegex::{}", constant_name)
    } else {
      write!(f, "KnownRegex::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for KnownRegex {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for KnownRegex {}

impl ::protobuf::Proxied for KnownRegex {
  type View<'a> = KnownRegex;
}

impl ::protobuf::AsView for KnownRegex {
  type Proxied = KnownRegex;

  fn as_view(&self) -> KnownRegex {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for KnownRegex {
  fn into_view<'shorter>(self) -> KnownRegex where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for KnownRegex {
  const NAME: &'static str = "KnownRegex";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2)
  }
}

impl ::protobuf::__internal::EntityType for KnownRegex {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTableEnum for KnownRegex {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTableEnumPtr {
    static MINI_TABLE: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableEnumInitPtr> =
        ::std::sync::OnceLock::new();
    MINI_TABLE.get_or_init(|| unsafe {
      ::protobuf::__internal::runtime::MiniTableEnumInitPtr(
          ::protobuf::__internal::runtime::build_enum_mini_table("!)"))
    }).0
  }
}





