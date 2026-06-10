const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.34.0-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__Schema_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Schema {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Schema>
}

impl ::protobuf::Message for Schema {}

impl ::std::default::Default for Schema {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Schema {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Schema` is `Sync` because it does not implement interior mutability.
//    Neither does `SchemaMut`.
unsafe impl Sync for Schema {}

// SAFETY:
// - `Schema` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Schema {}

impl ::protobuf::Proxied for Schema {
  type View<'msg> = SchemaView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Schema {}

impl ::protobuf::MutProxied for Schema {
  type Mut<'msg> = SchemaMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SchemaView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Schema>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SchemaView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SchemaView<'msg> {
  type Message = Schema;
}

impl ::std::fmt::Debug for SchemaView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SchemaView<'_> {
  fn default() -> SchemaView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Schema>> for SchemaView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Schema>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SchemaView<'msg> {

  pub fn to_owned(&self) -> Schema {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // name: optional string
  pub fn name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // type: optional enum google.pubsub.v1.Schema.Type
  pub fn r#type(self) -> super::schema::Type {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::schema::Type::Unspecified).into()
      ).try_into().unwrap()
    }
  }

  // definition: optional string
  pub fn definition(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `SchemaView` is `Sync` because it does not support mutation.
unsafe impl Sync for SchemaView<'_> {}

// SAFETY:
// - `SchemaView` is `Send` because while its alive a `SchemaMut` cannot.
// - `SchemaView` does not use thread-local data.
unsafe impl Send for SchemaView<'_> {}

impl<'msg> ::protobuf::AsView for SchemaView<'msg> {
  type Proxied = Schema;
  fn as_view(&self) -> ::protobuf::View<'msg, Schema> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SchemaView<'msg> {
  fn into_view<'shorter>(self) -> SchemaView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Schema> for SchemaView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Schema {
    let mut dst = Schema::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Schema> for SchemaMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Schema {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Schema {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SchemaView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SchemaMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SchemaMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Schema>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SchemaMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SchemaMut<'msg> {
  type Message = Schema;
}

impl ::std::fmt::Debug for SchemaMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Schema>> for SchemaMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Schema>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SchemaMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Schema> {
    self.inner
  }

  pub fn to_owned(&self) -> Schema {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // type: optional enum google.pubsub.v1.Schema.Type
  pub fn r#type(&self) -> super::schema::Type {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::schema::Type::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_type(&mut self, val: super::schema::Type) {
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

  // definition: optional string
  pub fn definition(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_definition(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}

// SAFETY:
// - `SchemaMut` does not perform any shared mutation.
unsafe impl Send for SchemaMut<'_> {}

// SAFETY:
// - `SchemaMut` does not perform any shared mutation.
unsafe impl Sync for SchemaMut<'_> {}

impl<'msg> ::protobuf::AsView for SchemaMut<'msg> {
  type Proxied = Schema;
  fn as_view(&self) -> ::protobuf::View<'_, Schema> {
    SchemaView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SchemaMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Schema>
  where
      'msg: 'shorter {
    SchemaView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for SchemaMut<'msg> {
  type MutProxied = Schema;
  fn as_mut(&mut self) -> SchemaMut<'msg> {
    SchemaMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SchemaMut<'msg> {
  fn into_mut<'shorter>(self) -> SchemaMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Schema {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Schema> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SchemaView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SchemaMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // type: optional enum google.pubsub.v1.Schema.Type
  pub fn r#type(&self) -> super::schema::Type {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::schema::Type::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_type(&mut self, val: super::schema::Type) {
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

  // definition: optional string
  pub fn definition(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_definition(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}  // impl Schema

impl ::std::ops::Drop for Schema {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Schema {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Schema {
  type Proxied = Self;
  fn as_view(&self) -> SchemaView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Schema {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SchemaMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Schema {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__Schema_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X.P1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__Schema_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__Schema_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Schema {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Schema {
  type Msg = Schema;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Schema> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Schema {
  type Msg = Schema;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Schema> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SchemaMut<'_> {
  type Msg = Schema;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Schema> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SchemaMut<'_> {
  type Msg = Schema;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Schema> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SchemaView<'_> {
  type Msg = Schema;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Schema> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SchemaMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod schema {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Type(i32);

#[allow(non_upper_case_globals)]
impl Type {
  pub const Unspecified: Type = Type(0);
  pub const ProtocolBuffer: Type = Type(1);
  pub const Avro: Type = Type(2);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unspecified",
      1 => "ProtocolBuffer",
      2 => "Avro",
      _ => return None
    })
  }
}

impl ::std::convert::From<Type> for i32 {
  fn from(val: Type) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for Type {
  fn from(val: i32) -> Type {
    Self(val)
  }
}

impl ::std::default::Default for Type {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for Type {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "Type::{}", constant_name)
    } else {
      write!(f, "Type::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for Type {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for Type {}

impl ::protobuf::Proxied for Type {
  type View<'a> = Type;
}

impl ::protobuf::AsView for Type {
  type Proxied = Type;

  fn as_view(&self) -> Type {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Type {
  fn into_view<'shorter>(self) -> Type where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for Type {
  const NAME: &'static str = "Type";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Type {
    type Tag = ::protobuf::__internal::runtime::EnumTag;
}


}  // pub mod schema


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__CreateSchemaRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CreateSchemaRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CreateSchemaRequest>
}

impl ::protobuf::Message for CreateSchemaRequest {}

impl ::std::default::Default for CreateSchemaRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CreateSchemaRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CreateSchemaRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `CreateSchemaRequestMut`.
unsafe impl Sync for CreateSchemaRequest {}

// SAFETY:
// - `CreateSchemaRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for CreateSchemaRequest {}

impl ::protobuf::Proxied for CreateSchemaRequest {
  type View<'msg> = CreateSchemaRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CreateSchemaRequest {}

impl ::protobuf::MutProxied for CreateSchemaRequest {
  type Mut<'msg> = CreateSchemaRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CreateSchemaRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CreateSchemaRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CreateSchemaRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CreateSchemaRequestView<'msg> {
  type Message = CreateSchemaRequest;
}

impl ::std::fmt::Debug for CreateSchemaRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CreateSchemaRequestView<'_> {
  fn default() -> CreateSchemaRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CreateSchemaRequest>> for CreateSchemaRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CreateSchemaRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CreateSchemaRequestView<'msg> {

  pub fn to_owned(&self) -> CreateSchemaRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // parent: optional string
  pub fn parent(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // schema: optional message google.pubsub.v1.Schema
  pub fn has_schema(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn schema_opt(self) -> ::protobuf::Optional<super::SchemaView<'msg>> {
        ::protobuf::Optional::new(self.schema(), self.has_schema())
  }
  pub fn schema(self) -> super::SchemaView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SchemaView::default())
  }

  // schema_id: optional string
  pub fn schema_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `CreateSchemaRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for CreateSchemaRequestView<'_> {}

// SAFETY:
// - `CreateSchemaRequestView` is `Send` because while its alive a `CreateSchemaRequestMut` cannot.
// - `CreateSchemaRequestView` does not use thread-local data.
unsafe impl Send for CreateSchemaRequestView<'_> {}

impl<'msg> ::protobuf::AsView for CreateSchemaRequestView<'msg> {
  type Proxied = CreateSchemaRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, CreateSchemaRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CreateSchemaRequestView<'msg> {
  fn into_view<'shorter>(self) -> CreateSchemaRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CreateSchemaRequest> for CreateSchemaRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CreateSchemaRequest {
    let mut dst = CreateSchemaRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CreateSchemaRequest> for CreateSchemaRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CreateSchemaRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for CreateSchemaRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for CreateSchemaRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for CreateSchemaRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CreateSchemaRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CreateSchemaRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CreateSchemaRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CreateSchemaRequestMut<'msg> {
  type Message = CreateSchemaRequest;
}

impl ::std::fmt::Debug for CreateSchemaRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CreateSchemaRequest>> for CreateSchemaRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CreateSchemaRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CreateSchemaRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CreateSchemaRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> CreateSchemaRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // parent: optional string
  pub fn parent(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_parent(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // schema: optional message google.pubsub.v1.Schema
  pub fn has_schema(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_schema(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn schema_opt(&self) -> ::protobuf::Optional<super::SchemaView<'_>> {
        ::protobuf::Optional::new(self.schema(), self.has_schema())
  }
  pub fn schema(&self) -> super::SchemaView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SchemaView::default())
  }
  pub fn schema_mut(&mut self) -> super::SchemaMut<'_> {
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
  pub fn set_schema(&mut self,
    val: impl ::protobuf::IntoProxied<super::Schema>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // schema_id: optional string
  pub fn schema_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_schema_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}

// SAFETY:
// - `CreateSchemaRequestMut` does not perform any shared mutation.
unsafe impl Send for CreateSchemaRequestMut<'_> {}

// SAFETY:
// - `CreateSchemaRequestMut` does not perform any shared mutation.
unsafe impl Sync for CreateSchemaRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for CreateSchemaRequestMut<'msg> {
  type Proxied = CreateSchemaRequest;
  fn as_view(&self) -> ::protobuf::View<'_, CreateSchemaRequest> {
    CreateSchemaRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CreateSchemaRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CreateSchemaRequest>
  where
      'msg: 'shorter {
    CreateSchemaRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for CreateSchemaRequestMut<'msg> {
  type MutProxied = CreateSchemaRequest;
  fn as_mut(&mut self) -> CreateSchemaRequestMut<'msg> {
    CreateSchemaRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CreateSchemaRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> CreateSchemaRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CreateSchemaRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CreateSchemaRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CreateSchemaRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CreateSchemaRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // parent: optional string
  pub fn parent(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_parent(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // schema: optional message google.pubsub.v1.Schema
  pub fn has_schema(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_schema(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn schema_opt(&self) -> ::protobuf::Optional<super::SchemaView<'_>> {
        ::protobuf::Optional::new(self.schema(), self.has_schema())
  }
  pub fn schema(&self) -> super::SchemaView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SchemaView::default())
  }
  pub fn schema_mut(&mut self) -> super::SchemaMut<'_> {
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
  pub fn set_schema(&mut self,
    val: impl ::protobuf::IntoProxied<super::Schema>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // schema_id: optional string
  pub fn schema_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_schema_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}  // impl CreateSchemaRequest

impl ::std::ops::Drop for CreateSchemaRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CreateSchemaRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CreateSchemaRequest {
  type Proxied = Self;
  fn as_view(&self) -> CreateSchemaRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CreateSchemaRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CreateSchemaRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CreateSchemaRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__CreateSchemaRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X31X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__CreateSchemaRequest_msg_init.0, &[<super::Schema as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__CreateSchemaRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CreateSchemaRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CreateSchemaRequest {
  type Msg = CreateSchemaRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CreateSchemaRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CreateSchemaRequest {
  type Msg = CreateSchemaRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CreateSchemaRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CreateSchemaRequestMut<'_> {
  type Msg = CreateSchemaRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CreateSchemaRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CreateSchemaRequestMut<'_> {
  type Msg = CreateSchemaRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CreateSchemaRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CreateSchemaRequestView<'_> {
  type Msg = CreateSchemaRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CreateSchemaRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CreateSchemaRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__GetSchemaRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GetSchemaRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GetSchemaRequest>
}

impl ::protobuf::Message for GetSchemaRequest {}

impl ::std::default::Default for GetSchemaRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GetSchemaRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GetSchemaRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `GetSchemaRequestMut`.
unsafe impl Sync for GetSchemaRequest {}

// SAFETY:
// - `GetSchemaRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetSchemaRequest {}

impl ::protobuf::Proxied for GetSchemaRequest {
  type View<'msg> = GetSchemaRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetSchemaRequest {}

impl ::protobuf::MutProxied for GetSchemaRequest {
  type Mut<'msg> = GetSchemaRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetSchemaRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetSchemaRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetSchemaRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetSchemaRequestView<'msg> {
  type Message = GetSchemaRequest;
}

impl ::std::fmt::Debug for GetSchemaRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GetSchemaRequestView<'_> {
  fn default() -> GetSchemaRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GetSchemaRequest>> for GetSchemaRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetSchemaRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetSchemaRequestView<'msg> {

  pub fn to_owned(&self) -> GetSchemaRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // name: optional string
  pub fn name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // view: optional enum google.pubsub.v1.SchemaView
  pub fn view(self) -> super::SchemaView_ {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::SchemaView_::Unspecified).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `GetSchemaRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetSchemaRequestView<'_> {}

// SAFETY:
// - `GetSchemaRequestView` is `Send` because while its alive a `GetSchemaRequestMut` cannot.
// - `GetSchemaRequestView` does not use thread-local data.
unsafe impl Send for GetSchemaRequestView<'_> {}

impl<'msg> ::protobuf::AsView for GetSchemaRequestView<'msg> {
  type Proxied = GetSchemaRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, GetSchemaRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetSchemaRequestView<'msg> {
  fn into_view<'shorter>(self) -> GetSchemaRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetSchemaRequest> for GetSchemaRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetSchemaRequest {
    let mut dst = GetSchemaRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetSchemaRequest> for GetSchemaRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetSchemaRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for GetSchemaRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetSchemaRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetSchemaRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetSchemaRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetSchemaRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetSchemaRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetSchemaRequestMut<'msg> {
  type Message = GetSchemaRequest;
}

impl ::std::fmt::Debug for GetSchemaRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GetSchemaRequest>> for GetSchemaRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetSchemaRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetSchemaRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GetSchemaRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> GetSchemaRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // view: optional enum google.pubsub.v1.SchemaView
  pub fn view(&self) -> super::SchemaView_ {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::SchemaView_::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_view(&mut self, val: super::SchemaView_) {
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
// - `GetSchemaRequestMut` does not perform any shared mutation.
unsafe impl Send for GetSchemaRequestMut<'_> {}

// SAFETY:
// - `GetSchemaRequestMut` does not perform any shared mutation.
unsafe impl Sync for GetSchemaRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for GetSchemaRequestMut<'msg> {
  type Proxied = GetSchemaRequest;
  fn as_view(&self) -> ::protobuf::View<'_, GetSchemaRequest> {
    GetSchemaRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetSchemaRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetSchemaRequest>
  where
      'msg: 'shorter {
    GetSchemaRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for GetSchemaRequestMut<'msg> {
  type MutProxied = GetSchemaRequest;
  fn as_mut(&mut self) -> GetSchemaRequestMut<'msg> {
    GetSchemaRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetSchemaRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> GetSchemaRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetSchemaRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GetSchemaRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GetSchemaRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GetSchemaRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // view: optional enum google.pubsub.v1.SchemaView
  pub fn view(&self) -> super::SchemaView_ {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::SchemaView_::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_view(&mut self, val: super::SchemaView_) {
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

}  // impl GetSchemaRequest

impl ::std::ops::Drop for GetSchemaRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GetSchemaRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetSchemaRequest {
  type Proxied = Self;
  fn as_view(&self) -> GetSchemaRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetSchemaRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetSchemaRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GetSchemaRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__GetSchemaRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X.P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__GetSchemaRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__GetSchemaRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetSchemaRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetSchemaRequest {
  type Msg = GetSchemaRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetSchemaRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetSchemaRequest {
  type Msg = GetSchemaRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetSchemaRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetSchemaRequestMut<'_> {
  type Msg = GetSchemaRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetSchemaRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetSchemaRequestMut<'_> {
  type Msg = GetSchemaRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetSchemaRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetSchemaRequestView<'_> {
  type Msg = GetSchemaRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetSchemaRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetSchemaRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__ListSchemasRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ListSchemasRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ListSchemasRequest>
}

impl ::protobuf::Message for ListSchemasRequest {}

impl ::std::default::Default for ListSchemasRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ListSchemasRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ListSchemasRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `ListSchemasRequestMut`.
unsafe impl Sync for ListSchemasRequest {}

// SAFETY:
// - `ListSchemasRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ListSchemasRequest {}

impl ::protobuf::Proxied for ListSchemasRequest {
  type View<'msg> = ListSchemasRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ListSchemasRequest {}

impl ::protobuf::MutProxied for ListSchemasRequest {
  type Mut<'msg> = ListSchemasRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ListSchemasRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListSchemasRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListSchemasRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ListSchemasRequestView<'msg> {
  type Message = ListSchemasRequest;
}

impl ::std::fmt::Debug for ListSchemasRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ListSchemasRequestView<'_> {
  fn default() -> ListSchemasRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ListSchemasRequest>> for ListSchemasRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListSchemasRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListSchemasRequestView<'msg> {

  pub fn to_owned(&self) -> ListSchemasRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // parent: optional string
  pub fn parent(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // view: optional enum google.pubsub.v1.SchemaView
  pub fn view(self) -> super::SchemaView_ {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::SchemaView_::Unspecified).into()
      ).try_into().unwrap()
    }
  }

  // page_size: optional int32
  pub fn page_size(self) -> i32 {
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

  // page_token: optional string
  pub fn page_token(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `ListSchemasRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for ListSchemasRequestView<'_> {}

// SAFETY:
// - `ListSchemasRequestView` is `Send` because while its alive a `ListSchemasRequestMut` cannot.
// - `ListSchemasRequestView` does not use thread-local data.
unsafe impl Send for ListSchemasRequestView<'_> {}

impl<'msg> ::protobuf::AsView for ListSchemasRequestView<'msg> {
  type Proxied = ListSchemasRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, ListSchemasRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListSchemasRequestView<'msg> {
  fn into_view<'shorter>(self) -> ListSchemasRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ListSchemasRequest> for ListSchemasRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListSchemasRequest {
    let mut dst = ListSchemasRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ListSchemasRequest> for ListSchemasRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListSchemasRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ListSchemasRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ListSchemasRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ListSchemasRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ListSchemasRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListSchemasRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListSchemasRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ListSchemasRequestMut<'msg> {
  type Message = ListSchemasRequest;
}

impl ::std::fmt::Debug for ListSchemasRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ListSchemasRequest>> for ListSchemasRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListSchemasRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListSchemasRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ListSchemasRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> ListSchemasRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // parent: optional string
  pub fn parent(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_parent(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // view: optional enum google.pubsub.v1.SchemaView
  pub fn view(&self) -> super::SchemaView_ {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::SchemaView_::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_view(&mut self, val: super::SchemaView_) {
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

  // page_size: optional int32
  pub fn page_size(&self) -> i32 {
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
  pub fn set_page_size(&mut self, val: i32) {
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

  // page_token: optional string
  pub fn page_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_page_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

}

// SAFETY:
// - `ListSchemasRequestMut` does not perform any shared mutation.
unsafe impl Send for ListSchemasRequestMut<'_> {}

// SAFETY:
// - `ListSchemasRequestMut` does not perform any shared mutation.
unsafe impl Sync for ListSchemasRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for ListSchemasRequestMut<'msg> {
  type Proxied = ListSchemasRequest;
  fn as_view(&self) -> ::protobuf::View<'_, ListSchemasRequest> {
    ListSchemasRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListSchemasRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ListSchemasRequest>
  where
      'msg: 'shorter {
    ListSchemasRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ListSchemasRequestMut<'msg> {
  type MutProxied = ListSchemasRequest;
  fn as_mut(&mut self) -> ListSchemasRequestMut<'msg> {
    ListSchemasRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ListSchemasRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> ListSchemasRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ListSchemasRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ListSchemasRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ListSchemasRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ListSchemasRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // parent: optional string
  pub fn parent(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_parent(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // view: optional enum google.pubsub.v1.SchemaView
  pub fn view(&self) -> super::SchemaView_ {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::SchemaView_::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_view(&mut self, val: super::SchemaView_) {
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

  // page_size: optional int32
  pub fn page_size(&self) -> i32 {
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
  pub fn set_page_size(&mut self, val: i32) {
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

  // page_token: optional string
  pub fn page_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_page_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

}  // impl ListSchemasRequest

impl ::std::ops::Drop for ListSchemasRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ListSchemasRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ListSchemasRequest {
  type Proxied = Self;
  fn as_view(&self) -> ListSchemasRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ListSchemasRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ListSchemasRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ListSchemasRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__ListSchemasRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X.P(P1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__ListSchemasRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__ListSchemasRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListSchemasRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListSchemasRequest {
  type Msg = ListSchemasRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListSchemasRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListSchemasRequest {
  type Msg = ListSchemasRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListSchemasRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListSchemasRequestMut<'_> {
  type Msg = ListSchemasRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListSchemasRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListSchemasRequestMut<'_> {
  type Msg = ListSchemasRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListSchemasRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListSchemasRequestView<'_> {
  type Msg = ListSchemasRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListSchemasRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListSchemasRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__ListSchemasResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ListSchemasResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ListSchemasResponse>
}

impl ::protobuf::Message for ListSchemasResponse {}

impl ::std::default::Default for ListSchemasResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ListSchemasResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ListSchemasResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `ListSchemasResponseMut`.
unsafe impl Sync for ListSchemasResponse {}

// SAFETY:
// - `ListSchemasResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ListSchemasResponse {}

impl ::protobuf::Proxied for ListSchemasResponse {
  type View<'msg> = ListSchemasResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ListSchemasResponse {}

impl ::protobuf::MutProxied for ListSchemasResponse {
  type Mut<'msg> = ListSchemasResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ListSchemasResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListSchemasResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListSchemasResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ListSchemasResponseView<'msg> {
  type Message = ListSchemasResponse;
}

impl ::std::fmt::Debug for ListSchemasResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ListSchemasResponseView<'_> {
  fn default() -> ListSchemasResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ListSchemasResponse>> for ListSchemasResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListSchemasResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListSchemasResponseView<'msg> {

  pub fn to_owned(&self) -> ListSchemasResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // schemas: repeated message google.pubsub.v1.Schema
  pub fn schemas(self) -> ::protobuf::RepeatedView<'msg, super::Schema> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Schema>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // next_page_token: optional string
  pub fn next_page_token(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `ListSchemasResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for ListSchemasResponseView<'_> {}

// SAFETY:
// - `ListSchemasResponseView` is `Send` because while its alive a `ListSchemasResponseMut` cannot.
// - `ListSchemasResponseView` does not use thread-local data.
unsafe impl Send for ListSchemasResponseView<'_> {}

impl<'msg> ::protobuf::AsView for ListSchemasResponseView<'msg> {
  type Proxied = ListSchemasResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, ListSchemasResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListSchemasResponseView<'msg> {
  fn into_view<'shorter>(self) -> ListSchemasResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ListSchemasResponse> for ListSchemasResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListSchemasResponse {
    let mut dst = ListSchemasResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ListSchemasResponse> for ListSchemasResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListSchemasResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ListSchemasResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ListSchemasResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ListSchemasResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ListSchemasResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListSchemasResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListSchemasResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ListSchemasResponseMut<'msg> {
  type Message = ListSchemasResponse;
}

impl ::std::fmt::Debug for ListSchemasResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ListSchemasResponse>> for ListSchemasResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListSchemasResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListSchemasResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ListSchemasResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> ListSchemasResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // schemas: repeated message google.pubsub.v1.Schema
  pub fn schemas(&self) -> ::protobuf::RepeatedView<'_, super::Schema> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Schema>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn schemas_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Schema> {
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
  pub fn set_schemas(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Schema>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // next_page_token: optional string
  pub fn next_page_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_next_page_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `ListSchemasResponseMut` does not perform any shared mutation.
unsafe impl Send for ListSchemasResponseMut<'_> {}

// SAFETY:
// - `ListSchemasResponseMut` does not perform any shared mutation.
unsafe impl Sync for ListSchemasResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for ListSchemasResponseMut<'msg> {
  type Proxied = ListSchemasResponse;
  fn as_view(&self) -> ::protobuf::View<'_, ListSchemasResponse> {
    ListSchemasResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListSchemasResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ListSchemasResponse>
  where
      'msg: 'shorter {
    ListSchemasResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ListSchemasResponseMut<'msg> {
  type MutProxied = ListSchemasResponse;
  fn as_mut(&mut self) -> ListSchemasResponseMut<'msg> {
    ListSchemasResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ListSchemasResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> ListSchemasResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ListSchemasResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ListSchemasResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ListSchemasResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ListSchemasResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // schemas: repeated message google.pubsub.v1.Schema
  pub fn schemas(&self) -> ::protobuf::RepeatedView<'_, super::Schema> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Schema>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn schemas_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Schema> {
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
  pub fn set_schemas(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Schema>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // next_page_token: optional string
  pub fn next_page_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_next_page_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl ListSchemasResponse

impl ::std::ops::Drop for ListSchemasResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ListSchemasResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ListSchemasResponse {
  type Proxied = Self;
  fn as_view(&self) -> ListSchemasResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ListSchemasResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ListSchemasResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ListSchemasResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__ListSchemasResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__ListSchemasResponse_msg_init.0, &[<super::Schema as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__ListSchemasResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListSchemasResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListSchemasResponse {
  type Msg = ListSchemasResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListSchemasResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListSchemasResponse {
  type Msg = ListSchemasResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListSchemasResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListSchemasResponseMut<'_> {
  type Msg = ListSchemasResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListSchemasResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListSchemasResponseMut<'_> {
  type Msg = ListSchemasResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListSchemasResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListSchemasResponseView<'_> {
  type Msg = ListSchemasResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListSchemasResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListSchemasResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__DeleteSchemaRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DeleteSchemaRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DeleteSchemaRequest>
}

impl ::protobuf::Message for DeleteSchemaRequest {}

impl ::std::default::Default for DeleteSchemaRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DeleteSchemaRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DeleteSchemaRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `DeleteSchemaRequestMut`.
unsafe impl Sync for DeleteSchemaRequest {}

// SAFETY:
// - `DeleteSchemaRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for DeleteSchemaRequest {}

impl ::protobuf::Proxied for DeleteSchemaRequest {
  type View<'msg> = DeleteSchemaRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DeleteSchemaRequest {}

impl ::protobuf::MutProxied for DeleteSchemaRequest {
  type Mut<'msg> = DeleteSchemaRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DeleteSchemaRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DeleteSchemaRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DeleteSchemaRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DeleteSchemaRequestView<'msg> {
  type Message = DeleteSchemaRequest;
}

impl ::std::fmt::Debug for DeleteSchemaRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DeleteSchemaRequestView<'_> {
  fn default() -> DeleteSchemaRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DeleteSchemaRequest>> for DeleteSchemaRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DeleteSchemaRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DeleteSchemaRequestView<'msg> {

  pub fn to_owned(&self) -> DeleteSchemaRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // name: optional string
  pub fn name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `DeleteSchemaRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for DeleteSchemaRequestView<'_> {}

// SAFETY:
// - `DeleteSchemaRequestView` is `Send` because while its alive a `DeleteSchemaRequestMut` cannot.
// - `DeleteSchemaRequestView` does not use thread-local data.
unsafe impl Send for DeleteSchemaRequestView<'_> {}

impl<'msg> ::protobuf::AsView for DeleteSchemaRequestView<'msg> {
  type Proxied = DeleteSchemaRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, DeleteSchemaRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DeleteSchemaRequestView<'msg> {
  fn into_view<'shorter>(self) -> DeleteSchemaRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DeleteSchemaRequest> for DeleteSchemaRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DeleteSchemaRequest {
    let mut dst = DeleteSchemaRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DeleteSchemaRequest> for DeleteSchemaRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DeleteSchemaRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for DeleteSchemaRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DeleteSchemaRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DeleteSchemaRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DeleteSchemaRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DeleteSchemaRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DeleteSchemaRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DeleteSchemaRequestMut<'msg> {
  type Message = DeleteSchemaRequest;
}

impl ::std::fmt::Debug for DeleteSchemaRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DeleteSchemaRequest>> for DeleteSchemaRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DeleteSchemaRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DeleteSchemaRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DeleteSchemaRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> DeleteSchemaRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `DeleteSchemaRequestMut` does not perform any shared mutation.
unsafe impl Send for DeleteSchemaRequestMut<'_> {}

// SAFETY:
// - `DeleteSchemaRequestMut` does not perform any shared mutation.
unsafe impl Sync for DeleteSchemaRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for DeleteSchemaRequestMut<'msg> {
  type Proxied = DeleteSchemaRequest;
  fn as_view(&self) -> ::protobuf::View<'_, DeleteSchemaRequest> {
    DeleteSchemaRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DeleteSchemaRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DeleteSchemaRequest>
  where
      'msg: 'shorter {
    DeleteSchemaRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for DeleteSchemaRequestMut<'msg> {
  type MutProxied = DeleteSchemaRequest;
  fn as_mut(&mut self) -> DeleteSchemaRequestMut<'msg> {
    DeleteSchemaRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DeleteSchemaRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> DeleteSchemaRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DeleteSchemaRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DeleteSchemaRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DeleteSchemaRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DeleteSchemaRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl DeleteSchemaRequest

impl ::std::ops::Drop for DeleteSchemaRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DeleteSchemaRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DeleteSchemaRequest {
  type Proxied = Self;
  fn as_view(&self) -> DeleteSchemaRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DeleteSchemaRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DeleteSchemaRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DeleteSchemaRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__DeleteSchemaRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__DeleteSchemaRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__DeleteSchemaRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DeleteSchemaRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DeleteSchemaRequest {
  type Msg = DeleteSchemaRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeleteSchemaRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeleteSchemaRequest {
  type Msg = DeleteSchemaRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeleteSchemaRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DeleteSchemaRequestMut<'_> {
  type Msg = DeleteSchemaRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeleteSchemaRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeleteSchemaRequestMut<'_> {
  type Msg = DeleteSchemaRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeleteSchemaRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeleteSchemaRequestView<'_> {
  type Msg = DeleteSchemaRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeleteSchemaRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DeleteSchemaRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__ValidateSchemaRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ValidateSchemaRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ValidateSchemaRequest>
}

impl ::protobuf::Message for ValidateSchemaRequest {}

impl ::std::default::Default for ValidateSchemaRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ValidateSchemaRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ValidateSchemaRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `ValidateSchemaRequestMut`.
unsafe impl Sync for ValidateSchemaRequest {}

// SAFETY:
// - `ValidateSchemaRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ValidateSchemaRequest {}

impl ::protobuf::Proxied for ValidateSchemaRequest {
  type View<'msg> = ValidateSchemaRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ValidateSchemaRequest {}

impl ::protobuf::MutProxied for ValidateSchemaRequest {
  type Mut<'msg> = ValidateSchemaRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ValidateSchemaRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ValidateSchemaRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ValidateSchemaRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ValidateSchemaRequestView<'msg> {
  type Message = ValidateSchemaRequest;
}

impl ::std::fmt::Debug for ValidateSchemaRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ValidateSchemaRequestView<'_> {
  fn default() -> ValidateSchemaRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ValidateSchemaRequest>> for ValidateSchemaRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ValidateSchemaRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ValidateSchemaRequestView<'msg> {

  pub fn to_owned(&self) -> ValidateSchemaRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // parent: optional string
  pub fn parent(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // schema: optional message google.pubsub.v1.Schema
  pub fn has_schema(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn schema_opt(self) -> ::protobuf::Optional<super::SchemaView<'msg>> {
        ::protobuf::Optional::new(self.schema(), self.has_schema())
  }
  pub fn schema(self) -> super::SchemaView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SchemaView::default())
  }

}

// SAFETY:
// - `ValidateSchemaRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for ValidateSchemaRequestView<'_> {}

// SAFETY:
// - `ValidateSchemaRequestView` is `Send` because while its alive a `ValidateSchemaRequestMut` cannot.
// - `ValidateSchemaRequestView` does not use thread-local data.
unsafe impl Send for ValidateSchemaRequestView<'_> {}

impl<'msg> ::protobuf::AsView for ValidateSchemaRequestView<'msg> {
  type Proxied = ValidateSchemaRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, ValidateSchemaRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ValidateSchemaRequestView<'msg> {
  fn into_view<'shorter>(self) -> ValidateSchemaRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ValidateSchemaRequest> for ValidateSchemaRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ValidateSchemaRequest {
    let mut dst = ValidateSchemaRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ValidateSchemaRequest> for ValidateSchemaRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ValidateSchemaRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ValidateSchemaRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ValidateSchemaRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ValidateSchemaRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ValidateSchemaRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ValidateSchemaRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ValidateSchemaRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ValidateSchemaRequestMut<'msg> {
  type Message = ValidateSchemaRequest;
}

impl ::std::fmt::Debug for ValidateSchemaRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ValidateSchemaRequest>> for ValidateSchemaRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ValidateSchemaRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ValidateSchemaRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ValidateSchemaRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> ValidateSchemaRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // parent: optional string
  pub fn parent(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_parent(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // schema: optional message google.pubsub.v1.Schema
  pub fn has_schema(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_schema(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn schema_opt(&self) -> ::protobuf::Optional<super::SchemaView<'_>> {
        ::protobuf::Optional::new(self.schema(), self.has_schema())
  }
  pub fn schema(&self) -> super::SchemaView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SchemaView::default())
  }
  pub fn schema_mut(&mut self) -> super::SchemaMut<'_> {
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
  pub fn set_schema(&mut self,
    val: impl ::protobuf::IntoProxied<super::Schema>) {

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
// - `ValidateSchemaRequestMut` does not perform any shared mutation.
unsafe impl Send for ValidateSchemaRequestMut<'_> {}

// SAFETY:
// - `ValidateSchemaRequestMut` does not perform any shared mutation.
unsafe impl Sync for ValidateSchemaRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for ValidateSchemaRequestMut<'msg> {
  type Proxied = ValidateSchemaRequest;
  fn as_view(&self) -> ::protobuf::View<'_, ValidateSchemaRequest> {
    ValidateSchemaRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ValidateSchemaRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ValidateSchemaRequest>
  where
      'msg: 'shorter {
    ValidateSchemaRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ValidateSchemaRequestMut<'msg> {
  type MutProxied = ValidateSchemaRequest;
  fn as_mut(&mut self) -> ValidateSchemaRequestMut<'msg> {
    ValidateSchemaRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ValidateSchemaRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> ValidateSchemaRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ValidateSchemaRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ValidateSchemaRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ValidateSchemaRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ValidateSchemaRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // parent: optional string
  pub fn parent(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_parent(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // schema: optional message google.pubsub.v1.Schema
  pub fn has_schema(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_schema(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn schema_opt(&self) -> ::protobuf::Optional<super::SchemaView<'_>> {
        ::protobuf::Optional::new(self.schema(), self.has_schema())
  }
  pub fn schema(&self) -> super::SchemaView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SchemaView::default())
  }
  pub fn schema_mut(&mut self) -> super::SchemaMut<'_> {
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
  pub fn set_schema(&mut self,
    val: impl ::protobuf::IntoProxied<super::Schema>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl ValidateSchemaRequest

impl ::std::ops::Drop for ValidateSchemaRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ValidateSchemaRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ValidateSchemaRequest {
  type Proxied = Self;
  fn as_view(&self) -> ValidateSchemaRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ValidateSchemaRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ValidateSchemaRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ValidateSchemaRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__ValidateSchemaRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__ValidateSchemaRequest_msg_init.0, &[<super::Schema as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__ValidateSchemaRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ValidateSchemaRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ValidateSchemaRequest {
  type Msg = ValidateSchemaRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValidateSchemaRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ValidateSchemaRequest {
  type Msg = ValidateSchemaRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValidateSchemaRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ValidateSchemaRequestMut<'_> {
  type Msg = ValidateSchemaRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValidateSchemaRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ValidateSchemaRequestMut<'_> {
  type Msg = ValidateSchemaRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValidateSchemaRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ValidateSchemaRequestView<'_> {
  type Msg = ValidateSchemaRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValidateSchemaRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ValidateSchemaRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__ValidateSchemaResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ValidateSchemaResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ValidateSchemaResponse>
}

impl ::protobuf::Message for ValidateSchemaResponse {}

impl ::std::default::Default for ValidateSchemaResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ValidateSchemaResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ValidateSchemaResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `ValidateSchemaResponseMut`.
unsafe impl Sync for ValidateSchemaResponse {}

// SAFETY:
// - `ValidateSchemaResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ValidateSchemaResponse {}

impl ::protobuf::Proxied for ValidateSchemaResponse {
  type View<'msg> = ValidateSchemaResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ValidateSchemaResponse {}

impl ::protobuf::MutProxied for ValidateSchemaResponse {
  type Mut<'msg> = ValidateSchemaResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ValidateSchemaResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ValidateSchemaResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ValidateSchemaResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ValidateSchemaResponseView<'msg> {
  type Message = ValidateSchemaResponse;
}

impl ::std::fmt::Debug for ValidateSchemaResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ValidateSchemaResponseView<'_> {
  fn default() -> ValidateSchemaResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ValidateSchemaResponse>> for ValidateSchemaResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ValidateSchemaResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ValidateSchemaResponseView<'msg> {

  pub fn to_owned(&self) -> ValidateSchemaResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `ValidateSchemaResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for ValidateSchemaResponseView<'_> {}

// SAFETY:
// - `ValidateSchemaResponseView` is `Send` because while its alive a `ValidateSchemaResponseMut` cannot.
// - `ValidateSchemaResponseView` does not use thread-local data.
unsafe impl Send for ValidateSchemaResponseView<'_> {}

impl<'msg> ::protobuf::AsView for ValidateSchemaResponseView<'msg> {
  type Proxied = ValidateSchemaResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, ValidateSchemaResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ValidateSchemaResponseView<'msg> {
  fn into_view<'shorter>(self) -> ValidateSchemaResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ValidateSchemaResponse> for ValidateSchemaResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ValidateSchemaResponse {
    let mut dst = ValidateSchemaResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ValidateSchemaResponse> for ValidateSchemaResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ValidateSchemaResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ValidateSchemaResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ValidateSchemaResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ValidateSchemaResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ValidateSchemaResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ValidateSchemaResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ValidateSchemaResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ValidateSchemaResponseMut<'msg> {
  type Message = ValidateSchemaResponse;
}

impl ::std::fmt::Debug for ValidateSchemaResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ValidateSchemaResponse>> for ValidateSchemaResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ValidateSchemaResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ValidateSchemaResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ValidateSchemaResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> ValidateSchemaResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `ValidateSchemaResponseMut` does not perform any shared mutation.
unsafe impl Send for ValidateSchemaResponseMut<'_> {}

// SAFETY:
// - `ValidateSchemaResponseMut` does not perform any shared mutation.
unsafe impl Sync for ValidateSchemaResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for ValidateSchemaResponseMut<'msg> {
  type Proxied = ValidateSchemaResponse;
  fn as_view(&self) -> ::protobuf::View<'_, ValidateSchemaResponse> {
    ValidateSchemaResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ValidateSchemaResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ValidateSchemaResponse>
  where
      'msg: 'shorter {
    ValidateSchemaResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ValidateSchemaResponseMut<'msg> {
  type MutProxied = ValidateSchemaResponse;
  fn as_mut(&mut self) -> ValidateSchemaResponseMut<'msg> {
    ValidateSchemaResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ValidateSchemaResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> ValidateSchemaResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ValidateSchemaResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ValidateSchemaResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ValidateSchemaResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ValidateSchemaResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl ValidateSchemaResponse

impl ::std::ops::Drop for ValidateSchemaResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ValidateSchemaResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ValidateSchemaResponse {
  type Proxied = Self;
  fn as_view(&self) -> ValidateSchemaResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ValidateSchemaResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ValidateSchemaResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ValidateSchemaResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__ValidateSchemaResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__ValidateSchemaResponse_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__ValidateSchemaResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ValidateSchemaResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ValidateSchemaResponse {
  type Msg = ValidateSchemaResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValidateSchemaResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ValidateSchemaResponse {
  type Msg = ValidateSchemaResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValidateSchemaResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ValidateSchemaResponseMut<'_> {
  type Msg = ValidateSchemaResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValidateSchemaResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ValidateSchemaResponseMut<'_> {
  type Msg = ValidateSchemaResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValidateSchemaResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ValidateSchemaResponseView<'_> {
  type Msg = ValidateSchemaResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValidateSchemaResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ValidateSchemaResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__ValidateMessageRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ValidateMessageRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ValidateMessageRequest>
}

impl ::protobuf::Message for ValidateMessageRequest {}

impl ::std::default::Default for ValidateMessageRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ValidateMessageRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ValidateMessageRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `ValidateMessageRequestMut`.
unsafe impl Sync for ValidateMessageRequest {}

// SAFETY:
// - `ValidateMessageRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ValidateMessageRequest {}

impl ::protobuf::Proxied for ValidateMessageRequest {
  type View<'msg> = ValidateMessageRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ValidateMessageRequest {}

impl ::protobuf::MutProxied for ValidateMessageRequest {
  type Mut<'msg> = ValidateMessageRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ValidateMessageRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ValidateMessageRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ValidateMessageRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ValidateMessageRequestView<'msg> {
  type Message = ValidateMessageRequest;
}

impl ::std::fmt::Debug for ValidateMessageRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ValidateMessageRequestView<'_> {
  fn default() -> ValidateMessageRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ValidateMessageRequest>> for ValidateMessageRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ValidateMessageRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ValidateMessageRequestView<'msg> {

  pub fn to_owned(&self) -> ValidateMessageRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // parent: optional string
  pub fn parent(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // name: optional string
  pub fn has_name(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn name_opt(self) -> ::protobuf::Optional<&'msg ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.name(), self.has_name())
  }
  pub fn name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // schema: optional message google.pubsub.v1.Schema
  pub fn has_schema(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn schema_opt(self) -> ::protobuf::Optional<super::SchemaView<'msg>> {
        ::protobuf::Optional::new(self.schema(), self.has_schema())
  }
  pub fn schema(self) -> super::SchemaView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SchemaView::default())
  }

  // message: optional bytes
  pub fn message(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // encoding: optional enum google.pubsub.v1.Encoding
  pub fn encoding(self) -> super::Encoding {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::Encoding::Unspecified).into()
      ).try_into().unwrap()
    }
  }

  pub fn schema_spec(self) -> super::validate_message_request::SchemaSpecOneof<'msg> {
    match self.schema_spec_case() {
      super::validate_message_request::SchemaSpecCase::Name =>
          super::validate_message_request::SchemaSpecOneof::Name(self.name()),
      super::validate_message_request::SchemaSpecCase::Schema =>
          super::validate_message_request::SchemaSpecOneof::Schema(self.schema()),
      _ => super::validate_message_request::SchemaSpecOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn schema_spec_case(self) -> super::validate_message_request::SchemaSpecCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::validate_message_request::SchemaSpecCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ValidateMessageRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for ValidateMessageRequestView<'_> {}

// SAFETY:
// - `ValidateMessageRequestView` is `Send` because while its alive a `ValidateMessageRequestMut` cannot.
// - `ValidateMessageRequestView` does not use thread-local data.
unsafe impl Send for ValidateMessageRequestView<'_> {}

impl<'msg> ::protobuf::AsView for ValidateMessageRequestView<'msg> {
  type Proxied = ValidateMessageRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, ValidateMessageRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ValidateMessageRequestView<'msg> {
  fn into_view<'shorter>(self) -> ValidateMessageRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ValidateMessageRequest> for ValidateMessageRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ValidateMessageRequest {
    let mut dst = ValidateMessageRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ValidateMessageRequest> for ValidateMessageRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ValidateMessageRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ValidateMessageRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ValidateMessageRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ValidateMessageRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ValidateMessageRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ValidateMessageRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ValidateMessageRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ValidateMessageRequestMut<'msg> {
  type Message = ValidateMessageRequest;
}

impl ::std::fmt::Debug for ValidateMessageRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ValidateMessageRequest>> for ValidateMessageRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ValidateMessageRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ValidateMessageRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ValidateMessageRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> ValidateMessageRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // parent: optional string
  pub fn parent(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_parent(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // name: optional string
  pub fn has_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn name_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.name(), self.has_name())
  }
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // schema: optional message google.pubsub.v1.Schema
  pub fn has_schema(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_schema(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn schema_opt(&self) -> ::protobuf::Optional<super::SchemaView<'_>> {
        ::protobuf::Optional::new(self.schema(), self.has_schema())
  }
  pub fn schema(&self) -> super::SchemaView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SchemaView::default())
  }
  pub fn schema_mut(&mut self) -> super::SchemaMut<'_> {
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
  pub fn set_schema(&mut self,
    val: impl ::protobuf::IntoProxied<super::Schema>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // message: optional bytes
  pub fn message(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_message(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // encoding: optional enum google.pubsub.v1.Encoding
  pub fn encoding(&self) -> super::Encoding {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::Encoding::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_encoding(&mut self, val: super::Encoding) {
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

  pub fn schema_spec(&self) -> super::validate_message_request::SchemaSpecOneof<'_> {
    match &self.schema_spec_case() {
      super::validate_message_request::SchemaSpecCase::Name =>
          super::validate_message_request::SchemaSpecOneof::Name(self.name()),
      super::validate_message_request::SchemaSpecCase::Schema =>
          super::validate_message_request::SchemaSpecOneof::Schema(self.schema()),
      _ => super::validate_message_request::SchemaSpecOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn schema_spec_case(&self) -> super::validate_message_request::SchemaSpecCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::validate_message_request::SchemaSpecCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ValidateMessageRequestMut` does not perform any shared mutation.
unsafe impl Send for ValidateMessageRequestMut<'_> {}

// SAFETY:
// - `ValidateMessageRequestMut` does not perform any shared mutation.
unsafe impl Sync for ValidateMessageRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for ValidateMessageRequestMut<'msg> {
  type Proxied = ValidateMessageRequest;
  fn as_view(&self) -> ::protobuf::View<'_, ValidateMessageRequest> {
    ValidateMessageRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ValidateMessageRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ValidateMessageRequest>
  where
      'msg: 'shorter {
    ValidateMessageRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ValidateMessageRequestMut<'msg> {
  type MutProxied = ValidateMessageRequest;
  fn as_mut(&mut self) -> ValidateMessageRequestMut<'msg> {
    ValidateMessageRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ValidateMessageRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> ValidateMessageRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ValidateMessageRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ValidateMessageRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ValidateMessageRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ValidateMessageRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // parent: optional string
  pub fn parent(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_parent(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // name: optional string
  pub fn has_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn name_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.name(), self.has_name())
  }
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // schema: optional message google.pubsub.v1.Schema
  pub fn has_schema(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_schema(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn schema_opt(&self) -> ::protobuf::Optional<super::SchemaView<'_>> {
        ::protobuf::Optional::new(self.schema(), self.has_schema())
  }
  pub fn schema(&self) -> super::SchemaView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SchemaView::default())
  }
  pub fn schema_mut(&mut self) -> super::SchemaMut<'_> {
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
  pub fn set_schema(&mut self,
    val: impl ::protobuf::IntoProxied<super::Schema>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // message: optional bytes
  pub fn message(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_message(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // encoding: optional enum google.pubsub.v1.Encoding
  pub fn encoding(&self) -> super::Encoding {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::Encoding::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_encoding(&mut self, val: super::Encoding) {
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

  pub fn schema_spec(&self) -> super::validate_message_request::SchemaSpecOneof<'_> {
    match &self.schema_spec_case() {
      super::validate_message_request::SchemaSpecCase::Name =>
          super::validate_message_request::SchemaSpecOneof::Name(self.name()),
      super::validate_message_request::SchemaSpecCase::Schema =>
          super::validate_message_request::SchemaSpecOneof::Schema(self.schema()),
      _ => super::validate_message_request::SchemaSpecOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn schema_spec_case(&self) -> super::validate_message_request::SchemaSpecCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::validate_message_request::SchemaSpecCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl ValidateMessageRequest

impl ::std::ops::Drop for ValidateMessageRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ValidateMessageRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ValidateMessageRequest {
  type Proxied = Self;
  fn as_view(&self) -> ValidateMessageRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ValidateMessageRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ValidateMessageRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ValidateMessageRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__ValidateMessageRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1T30P.P^#|$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__ValidateMessageRequest_msg_init.0, &[<super::Schema as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__ValidateMessageRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ValidateMessageRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ValidateMessageRequest {
  type Msg = ValidateMessageRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValidateMessageRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ValidateMessageRequest {
  type Msg = ValidateMessageRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValidateMessageRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ValidateMessageRequestMut<'_> {
  type Msg = ValidateMessageRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValidateMessageRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ValidateMessageRequestMut<'_> {
  type Msg = ValidateMessageRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValidateMessageRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ValidateMessageRequestView<'_> {
  type Msg = ValidateMessageRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValidateMessageRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ValidateMessageRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod validate_message_request {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum SchemaSpecOneof<'msg> {
  Name(&'msg ::protobuf::ProtoStr) = 2,
  Schema(::protobuf::View<'msg, super::super::Schema>) = 3,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum SchemaSpecCase {
  Name = 2,
  Schema = 3,

  not_set = 0
}

impl SchemaSpecCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<SchemaSpecCase> {
    match v {
      0 => Some(SchemaSpecCase::not_set),
      2 => Some(SchemaSpecCase::Name),
      3 => Some(SchemaSpecCase::Schema),
      _ => None
    }
  }
}
}  // pub mod validate_message_request


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__ValidateMessageResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ValidateMessageResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ValidateMessageResponse>
}

impl ::protobuf::Message for ValidateMessageResponse {}

impl ::std::default::Default for ValidateMessageResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ValidateMessageResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ValidateMessageResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `ValidateMessageResponseMut`.
unsafe impl Sync for ValidateMessageResponse {}

// SAFETY:
// - `ValidateMessageResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ValidateMessageResponse {}

impl ::protobuf::Proxied for ValidateMessageResponse {
  type View<'msg> = ValidateMessageResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ValidateMessageResponse {}

impl ::protobuf::MutProxied for ValidateMessageResponse {
  type Mut<'msg> = ValidateMessageResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ValidateMessageResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ValidateMessageResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ValidateMessageResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ValidateMessageResponseView<'msg> {
  type Message = ValidateMessageResponse;
}

impl ::std::fmt::Debug for ValidateMessageResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ValidateMessageResponseView<'_> {
  fn default() -> ValidateMessageResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ValidateMessageResponse>> for ValidateMessageResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ValidateMessageResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ValidateMessageResponseView<'msg> {

  pub fn to_owned(&self) -> ValidateMessageResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `ValidateMessageResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for ValidateMessageResponseView<'_> {}

// SAFETY:
// - `ValidateMessageResponseView` is `Send` because while its alive a `ValidateMessageResponseMut` cannot.
// - `ValidateMessageResponseView` does not use thread-local data.
unsafe impl Send for ValidateMessageResponseView<'_> {}

impl<'msg> ::protobuf::AsView for ValidateMessageResponseView<'msg> {
  type Proxied = ValidateMessageResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, ValidateMessageResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ValidateMessageResponseView<'msg> {
  fn into_view<'shorter>(self) -> ValidateMessageResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ValidateMessageResponse> for ValidateMessageResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ValidateMessageResponse {
    let mut dst = ValidateMessageResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ValidateMessageResponse> for ValidateMessageResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ValidateMessageResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ValidateMessageResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ValidateMessageResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ValidateMessageResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ValidateMessageResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ValidateMessageResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ValidateMessageResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ValidateMessageResponseMut<'msg> {
  type Message = ValidateMessageResponse;
}

impl ::std::fmt::Debug for ValidateMessageResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ValidateMessageResponse>> for ValidateMessageResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ValidateMessageResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ValidateMessageResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ValidateMessageResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> ValidateMessageResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `ValidateMessageResponseMut` does not perform any shared mutation.
unsafe impl Send for ValidateMessageResponseMut<'_> {}

// SAFETY:
// - `ValidateMessageResponseMut` does not perform any shared mutation.
unsafe impl Sync for ValidateMessageResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for ValidateMessageResponseMut<'msg> {
  type Proxied = ValidateMessageResponse;
  fn as_view(&self) -> ::protobuf::View<'_, ValidateMessageResponse> {
    ValidateMessageResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ValidateMessageResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ValidateMessageResponse>
  where
      'msg: 'shorter {
    ValidateMessageResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ValidateMessageResponseMut<'msg> {
  type MutProxied = ValidateMessageResponse;
  fn as_mut(&mut self) -> ValidateMessageResponseMut<'msg> {
    ValidateMessageResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ValidateMessageResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> ValidateMessageResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ValidateMessageResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ValidateMessageResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ValidateMessageResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ValidateMessageResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl ValidateMessageResponse

impl ::std::ops::Drop for ValidateMessageResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ValidateMessageResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ValidateMessageResponse {
  type Proxied = Self;
  fn as_view(&self) -> ValidateMessageResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ValidateMessageResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ValidateMessageResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ValidateMessageResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__ValidateMessageResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__ValidateMessageResponse_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__ValidateMessageResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ValidateMessageResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ValidateMessageResponse {
  type Msg = ValidateMessageResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValidateMessageResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ValidateMessageResponse {
  type Msg = ValidateMessageResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValidateMessageResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ValidateMessageResponseMut<'_> {
  type Msg = ValidateMessageResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValidateMessageResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ValidateMessageResponseMut<'_> {
  type Msg = ValidateMessageResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValidateMessageResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ValidateMessageResponseView<'_> {
  type Msg = ValidateMessageResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValidateMessageResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ValidateMessageResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SchemaView_(i32);

#[allow(non_upper_case_globals)]
impl SchemaView_ {
  pub const Unspecified: SchemaView_ = SchemaView_(0);
  pub const Basic: SchemaView_ = SchemaView_(1);
  pub const Full: SchemaView_ = SchemaView_(2);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unspecified",
      1 => "Basic",
      2 => "Full",
      _ => return None
    })
  }
}

impl ::std::convert::From<SchemaView_> for i32 {
  fn from(val: SchemaView_) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for SchemaView_ {
  fn from(val: i32) -> SchemaView_ {
    Self(val)
  }
}

impl ::std::default::Default for SchemaView_ {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for SchemaView_ {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "SchemaView_::{}", constant_name)
    } else {
      write!(f, "SchemaView_::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for SchemaView_ {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for SchemaView_ {}

impl ::protobuf::Proxied for SchemaView_ {
  type View<'a> = SchemaView_;
}

impl ::protobuf::AsView for SchemaView_ {
  type Proxied = SchemaView_;

  fn as_view(&self) -> SchemaView_ {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SchemaView_ {
  fn into_view<'shorter>(self) -> SchemaView_ where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for SchemaView_ {
  const NAME: &'static str = "SchemaView_";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2)
  }
}

impl ::protobuf::__internal::runtime::EntityType for SchemaView_ {
    type Tag = ::protobuf::__internal::runtime::EnumTag;
}


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Encoding(i32);

#[allow(non_upper_case_globals)]
impl Encoding {
  pub const Unspecified: Encoding = Encoding(0);
  pub const Json: Encoding = Encoding(1);
  pub const Binary: Encoding = Encoding(2);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unspecified",
      1 => "Json",
      2 => "Binary",
      _ => return None
    })
  }
}

impl ::std::convert::From<Encoding> for i32 {
  fn from(val: Encoding) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for Encoding {
  fn from(val: i32) -> Encoding {
    Self(val)
  }
}

impl ::std::default::Default for Encoding {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for Encoding {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "Encoding::{}", constant_name)
    } else {
      write!(f, "Encoding::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for Encoding {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for Encoding {}

impl ::protobuf::Proxied for Encoding {
  type View<'a> = Encoding;
}

impl ::protobuf::AsView for Encoding {
  type Proxied = Encoding;

  fn as_view(&self) -> Encoding {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Encoding {
  fn into_view<'shorter>(self) -> Encoding where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for Encoding {
  const NAME: &'static str = "Encoding";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Encoding {
    type Tag = ::protobuf::__internal::runtime::EnumTag;
}


