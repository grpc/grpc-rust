const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.34.0-release");
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FieldBehavior(i32);

#[allow(non_upper_case_globals)]
impl FieldBehavior {
  pub const Unspecified: FieldBehavior = FieldBehavior(0);
  pub const Optional: FieldBehavior = FieldBehavior(1);
  pub const Required: FieldBehavior = FieldBehavior(2);
  pub const OutputOnly: FieldBehavior = FieldBehavior(3);
  pub const InputOnly: FieldBehavior = FieldBehavior(4);
  pub const Immutable: FieldBehavior = FieldBehavior(5);
  pub const UnorderedList: FieldBehavior = FieldBehavior(6);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unspecified",
      1 => "Optional",
      2 => "Required",
      3 => "OutputOnly",
      4 => "InputOnly",
      5 => "Immutable",
      6 => "UnorderedList",
      _ => return None
    })
  }
}

impl ::std::convert::From<FieldBehavior> for i32 {
  fn from(val: FieldBehavior) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for FieldBehavior {
  fn from(val: i32) -> FieldBehavior {
    Self(val)
  }
}

impl ::std::default::Default for FieldBehavior {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for FieldBehavior {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "FieldBehavior::{}", constant_name)
    } else {
      write!(f, "FieldBehavior::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for FieldBehavior {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for FieldBehavior {}

impl ::protobuf::Proxied for FieldBehavior {
  type View<'a> = FieldBehavior;
}

impl ::protobuf::AsView for FieldBehavior {
  type Proxied = FieldBehavior;

  fn as_view(&self) -> FieldBehavior {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FieldBehavior {
  fn into_view<'shorter>(self) -> FieldBehavior where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for FieldBehavior {
  const NAME: &'static str = "FieldBehavior";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4|5|6)
  }
}

impl ::protobuf::__internal::runtime::EntityType for FieldBehavior {
    type Tag = ::protobuf::__internal::runtime::EnumTag;
}


