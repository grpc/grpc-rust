const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RateLimitUnit(i32);

#[allow(non_upper_case_globals)]
impl RateLimitUnit {
  pub const Unknown: RateLimitUnit = RateLimitUnit(0);
  pub const Second: RateLimitUnit = RateLimitUnit(1);
  pub const Minute: RateLimitUnit = RateLimitUnit(2);
  pub const Hour: RateLimitUnit = RateLimitUnit(3);
  pub const Day: RateLimitUnit = RateLimitUnit(4);
  pub const Month: RateLimitUnit = RateLimitUnit(5);
  pub const Year: RateLimitUnit = RateLimitUnit(6);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unknown",
      1 => "Second",
      2 => "Minute",
      3 => "Hour",
      4 => "Day",
      5 => "Month",
      6 => "Year",
      _ => return None
    })
  }
}

impl ::std::convert::From<RateLimitUnit> for i32 {
  fn from(val: RateLimitUnit) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for RateLimitUnit {
  fn from(val: i32) -> RateLimitUnit {
    Self(val)
  }
}

impl ::std::default::Default for RateLimitUnit {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for RateLimitUnit {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "RateLimitUnit::{}", constant_name)
    } else {
      write!(f, "RateLimitUnit::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for RateLimitUnit {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for RateLimitUnit {}

impl ::protobuf::Proxied for RateLimitUnit {
  type View<'a> = RateLimitUnit;
}

impl ::protobuf::AsView for RateLimitUnit {
  type Proxied = RateLimitUnit;

  fn as_view(&self) -> RateLimitUnit {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RateLimitUnit {
  fn into_view<'shorter>(self) -> RateLimitUnit where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for RateLimitUnit {
  const NAME: &'static str = "RateLimitUnit";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4|5|6)
  }
}

impl ::protobuf::__internal::EntityType for RateLimitUnit {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


