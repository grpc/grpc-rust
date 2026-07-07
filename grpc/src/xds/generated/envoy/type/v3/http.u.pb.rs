const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CodecClientType(i32);

#[allow(non_upper_case_globals)]
impl CodecClientType {
  pub const Http1: CodecClientType = CodecClientType(0);
  pub const Http2: CodecClientType = CodecClientType(1);
  pub const Http3: CodecClientType = CodecClientType(2);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Http1",
      1 => "Http2",
      2 => "Http3",
      _ => return None
    })
  }
}

impl ::std::convert::From<CodecClientType> for i32 {
  fn from(val: CodecClientType) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for CodecClientType {
  fn from(val: i32) -> CodecClientType {
    Self(val)
  }
}

impl ::std::default::Default for CodecClientType {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for CodecClientType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "CodecClientType::{}", constant_name)
    } else {
      write!(f, "CodecClientType::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for CodecClientType {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for CodecClientType {}

impl ::protobuf::Proxied for CodecClientType {
  type View<'a> = CodecClientType;
}

impl ::protobuf::AsView for CodecClientType {
  type Proxied = CodecClientType;

  fn as_view(&self) -> CodecClientType {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CodecClientType {
  fn into_view<'shorter>(self) -> CodecClientType where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for CodecClientType {
  const NAME: &'static str = "CodecClientType";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2)
  }
}

impl ::protobuf::__internal::EntityType for CodecClientType {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


