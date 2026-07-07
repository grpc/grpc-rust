const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__ProxyProtocolPassThroughTLVs_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ProxyProtocolPassThroughTLVs {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ProxyProtocolPassThroughTLVs>
}

impl ::protobuf::Message for ProxyProtocolPassThroughTLVs {
  type MessageView<'msg> = ProxyProtocolPassThroughTLVsView<'msg>;
  type MessageMut<'msg> = ProxyProtocolPassThroughTLVsMut<'msg>;
}

impl ::std::default::Default for ProxyProtocolPassThroughTLVs {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ProxyProtocolPassThroughTLVs {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ProxyProtocolPassThroughTLVs` is `Sync` because it does not implement interior mutability.
//    Neither does `ProxyProtocolPassThroughTLVsMut`.
unsafe impl ::std::marker::Sync for ProxyProtocolPassThroughTLVs {}

// SAFETY:
// - `ProxyProtocolPassThroughTLVs` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ProxyProtocolPassThroughTLVs {}

impl ::protobuf::Proxied for ProxyProtocolPassThroughTLVs {
  type View<'msg> = ProxyProtocolPassThroughTLVsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ProxyProtocolPassThroughTLVs {}

impl ::protobuf::MutProxied for ProxyProtocolPassThroughTLVs {
  type Mut<'msg> = ProxyProtocolPassThroughTLVsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ProxyProtocolPassThroughTLVsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ProxyProtocolPassThroughTLVs>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ProxyProtocolPassThroughTLVsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ProxyProtocolPassThroughTLVsView<'msg> {
  type Message = ProxyProtocolPassThroughTLVs;
}

impl ::std::fmt::Debug for ProxyProtocolPassThroughTLVsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ProxyProtocolPassThroughTLVsView<'_> {
  fn default() -> ProxyProtocolPassThroughTLVsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ProxyProtocolPassThroughTLVs>> for ProxyProtocolPassThroughTLVsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ProxyProtocolPassThroughTLVs>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ProxyProtocolPassThroughTLVsView<'msg> {

  pub fn to_owned(&self) -> ProxyProtocolPassThroughTLVs {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // match_type: optional enum envoy.config.core.v3.ProxyProtocolPassThroughTLVs.PassTLVsMatchType
  pub fn match_type(self) -> super::proxy_protocol_pass_through_t_l_vs::PassTLVsMatchType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::proxy_protocol_pass_through_t_l_vs::PassTLVsMatchType::IncludeAll).into()
      ).try_into().unwrap()
    }
  }

  // tlv_type: repeated uint32
  pub fn tlv_type(self) -> ::protobuf::RepeatedView<'msg, u32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `ProxyProtocolPassThroughTLVsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ProxyProtocolPassThroughTLVsView<'_> {}

// SAFETY:
// - `ProxyProtocolPassThroughTLVsView` is `Send` because while its alive a `ProxyProtocolPassThroughTLVsMut` cannot.
// - `ProxyProtocolPassThroughTLVsView` does not use thread-local data.
unsafe impl ::std::marker::Send for ProxyProtocolPassThroughTLVsView<'_> {}

impl<'msg> ::protobuf::AsView for ProxyProtocolPassThroughTLVsView<'msg> {
  type Proxied = ProxyProtocolPassThroughTLVs;
  fn as_view(&self) -> ::protobuf::View<'msg, ProxyProtocolPassThroughTLVs> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ProxyProtocolPassThroughTLVsView<'msg> {
  fn into_view<'shorter>(self) -> ProxyProtocolPassThroughTLVsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ProxyProtocolPassThroughTLVs> for ProxyProtocolPassThroughTLVsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ProxyProtocolPassThroughTLVs {
    let mut dst = ProxyProtocolPassThroughTLVs::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ProxyProtocolPassThroughTLVs> for ProxyProtocolPassThroughTLVsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ProxyProtocolPassThroughTLVs {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ProxyProtocolPassThroughTLVs {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ProxyProtocolPassThroughTLVsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ProxyProtocolPassThroughTLVsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ProxyProtocolPassThroughTLVsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ProxyProtocolPassThroughTLVs>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ProxyProtocolPassThroughTLVsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ProxyProtocolPassThroughTLVsMut<'msg> {
  type Message = ProxyProtocolPassThroughTLVs;
}

impl ::std::fmt::Debug for ProxyProtocolPassThroughTLVsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ProxyProtocolPassThroughTLVs>> for ProxyProtocolPassThroughTLVsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ProxyProtocolPassThroughTLVs>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ProxyProtocolPassThroughTLVsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ProxyProtocolPassThroughTLVs> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ProxyProtocolPassThroughTLVs {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // match_type: optional enum envoy.config.core.v3.ProxyProtocolPassThroughTLVs.PassTLVsMatchType
  pub fn match_type(&self) -> super::proxy_protocol_pass_through_t_l_vs::PassTLVsMatchType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::proxy_protocol_pass_through_t_l_vs::PassTLVsMatchType::IncludeAll).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_match_type(&mut self, val: super::proxy_protocol_pass_through_t_l_vs::PassTLVsMatchType) {
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

  // tlv_type: repeated uint32
  pub fn tlv_type(&self) -> ::protobuf::RepeatedView<'_, u32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn tlv_type_mut(&mut self) -> ::protobuf::RepeatedMut<'_, u32> {
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
  pub fn set_tlv_type(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<u32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `ProxyProtocolPassThroughTLVsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ProxyProtocolPassThroughTLVsMut<'_> {}

// SAFETY:
// - `ProxyProtocolPassThroughTLVsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ProxyProtocolPassThroughTLVsMut<'_> {}

impl<'msg> ::protobuf::AsView for ProxyProtocolPassThroughTLVsMut<'msg> {
  type Proxied = ProxyProtocolPassThroughTLVs;
  fn as_view(&self) -> ::protobuf::View<'_, ProxyProtocolPassThroughTLVs> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ProxyProtocolPassThroughTLVsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ProxyProtocolPassThroughTLVs>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ProxyProtocolPassThroughTLVsMut<'msg> {
  type MutProxied = ProxyProtocolPassThroughTLVs;
  fn as_mut(&mut self) -> ProxyProtocolPassThroughTLVsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ProxyProtocolPassThroughTLVsMut<'msg> {
  fn into_mut<'shorter>(self) -> ProxyProtocolPassThroughTLVsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ProxyProtocolPassThroughTLVs {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ProxyProtocolPassThroughTLVs> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ProxyProtocolPassThroughTLVsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ProxyProtocolPassThroughTLVsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // match_type: optional enum envoy.config.core.v3.ProxyProtocolPassThroughTLVs.PassTLVsMatchType
  pub fn match_type(&self) -> super::proxy_protocol_pass_through_t_l_vs::PassTLVsMatchType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::proxy_protocol_pass_through_t_l_vs::PassTLVsMatchType::IncludeAll).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_match_type(&mut self, val: super::proxy_protocol_pass_through_t_l_vs::PassTLVsMatchType) {
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

  // tlv_type: repeated uint32
  pub fn tlv_type(&self) -> ::protobuf::RepeatedView<'_, u32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn tlv_type_mut(&mut self) -> ::protobuf::RepeatedMut<'_, u32> {
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
  pub fn set_tlv_type(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<u32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl ProxyProtocolPassThroughTLVs

impl ::std::ops::Drop for ProxyProtocolPassThroughTLVs {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ProxyProtocolPassThroughTLVs {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ProxyProtocolPassThroughTLVs {
  type Proxied = Self;
  fn as_view(&self) -> ProxyProtocolPassThroughTLVsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ProxyProtocolPassThroughTLVs {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ProxyProtocolPassThroughTLVsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ProxyProtocolPassThroughTLVs {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__ProxyProtocolPassThroughTLVs_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$N.P=");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__ProxyProtocolPassThroughTLVs_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__ProxyProtocolPassThroughTLVs_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ProxyProtocolPassThroughTLVs {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ProxyProtocolPassThroughTLVs {
  type Msg = ProxyProtocolPassThroughTLVs;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProxyProtocolPassThroughTLVs> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ProxyProtocolPassThroughTLVs {
  type Msg = ProxyProtocolPassThroughTLVs;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProxyProtocolPassThroughTLVs> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ProxyProtocolPassThroughTLVsMut<'_> {
  type Msg = ProxyProtocolPassThroughTLVs;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProxyProtocolPassThroughTLVs> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ProxyProtocolPassThroughTLVsMut<'_> {
  type Msg = ProxyProtocolPassThroughTLVs;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProxyProtocolPassThroughTLVs> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ProxyProtocolPassThroughTLVsView<'_> {
  type Msg = ProxyProtocolPassThroughTLVs;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProxyProtocolPassThroughTLVs> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ProxyProtocolPassThroughTLVsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod proxy_protocol_pass_through_t_l_vs {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PassTLVsMatchType(i32);

#[allow(non_upper_case_globals)]
impl PassTLVsMatchType {
  pub const IncludeAll: PassTLVsMatchType = PassTLVsMatchType(0);
  pub const Include: PassTLVsMatchType = PassTLVsMatchType(1);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "IncludeAll",
      1 => "Include",
      _ => return None
    })
  }
}

impl ::std::convert::From<PassTLVsMatchType> for i32 {
  fn from(val: PassTLVsMatchType) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for PassTLVsMatchType {
  fn from(val: i32) -> PassTLVsMatchType {
    Self(val)
  }
}

impl ::std::default::Default for PassTLVsMatchType {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for PassTLVsMatchType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "PassTLVsMatchType::{}", constant_name)
    } else {
      write!(f, "PassTLVsMatchType::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for PassTLVsMatchType {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for PassTLVsMatchType {}

impl ::protobuf::Proxied for PassTLVsMatchType {
  type View<'a> = PassTLVsMatchType;
}

impl ::protobuf::AsView for PassTLVsMatchType {
  type Proxied = PassTLVsMatchType;

  fn as_view(&self) -> PassTLVsMatchType {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PassTLVsMatchType {
  fn into_view<'shorter>(self) -> PassTLVsMatchType where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for PassTLVsMatchType {
  const NAME: &'static str = "PassTLVsMatchType";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1)
  }
}

impl ::protobuf::__internal::EntityType for PassTLVsMatchType {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod proxy_protocol_pass_through_t_l_vs


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__TlvEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TlvEntry {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TlvEntry>
}

impl ::protobuf::Message for TlvEntry {
  type MessageView<'msg> = TlvEntryView<'msg>;
  type MessageMut<'msg> = TlvEntryMut<'msg>;
}

impl ::std::default::Default for TlvEntry {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TlvEntry {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TlvEntry` is `Sync` because it does not implement interior mutability.
//    Neither does `TlvEntryMut`.
unsafe impl ::std::marker::Sync for TlvEntry {}

// SAFETY:
// - `TlvEntry` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TlvEntry {}

impl ::protobuf::Proxied for TlvEntry {
  type View<'msg> = TlvEntryView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TlvEntry {}

impl ::protobuf::MutProxied for TlvEntry {
  type Mut<'msg> = TlvEntryMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TlvEntryView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TlvEntry>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TlvEntryView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TlvEntryView<'msg> {
  type Message = TlvEntry;
}

impl ::std::fmt::Debug for TlvEntryView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TlvEntryView<'_> {
  fn default() -> TlvEntryView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TlvEntry>> for TlvEntryView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TlvEntry>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TlvEntryView<'msg> {

  pub fn to_owned(&self) -> TlvEntry {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // type: optional uint32
  pub fn r#type(self) -> u32 {
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

  // value: optional bytes
  pub fn value(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // format_string: optional message envoy.config.core.v3.SubstitutionFormatString
  pub fn has_format_string(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn format_string_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringView<'msg>> {
    self.has_format_string().then(|| self.format_string())
  }
  pub fn format_string(self) -> crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringView::default())
  }

}

// SAFETY:
// - `TlvEntryView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TlvEntryView<'_> {}

// SAFETY:
// - `TlvEntryView` is `Send` because while its alive a `TlvEntryMut` cannot.
// - `TlvEntryView` does not use thread-local data.
unsafe impl ::std::marker::Send for TlvEntryView<'_> {}

impl<'msg> ::protobuf::AsView for TlvEntryView<'msg> {
  type Proxied = TlvEntry;
  fn as_view(&self) -> ::protobuf::View<'msg, TlvEntry> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TlvEntryView<'msg> {
  fn into_view<'shorter>(self) -> TlvEntryView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TlvEntry> for TlvEntryView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TlvEntry {
    let mut dst = TlvEntry::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TlvEntry> for TlvEntryMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TlvEntry {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TlvEntry {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TlvEntryView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TlvEntryMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TlvEntryMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TlvEntry>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TlvEntryMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TlvEntryMut<'msg> {
  type Message = TlvEntry;
}

impl ::std::fmt::Debug for TlvEntryMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TlvEntry>> for TlvEntryMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TlvEntry>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TlvEntryMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TlvEntry> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TlvEntry {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // type: optional uint32
  pub fn r#type(&self) -> u32 {
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
  pub fn set_type(&mut self, val: u32) {
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

  // value: optional bytes
  pub fn value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // format_string: optional message envoy.config.core.v3.SubstitutionFormatString
  pub fn has_format_string(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_format_string(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn format_string_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringView<'_>> {
    self.has_format_string().then(|| self.format_string())
  }
  pub fn format_string(&self) -> crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringView::default())
  }
  pub fn format_string_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringMut<'_> {
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
  pub fn set_format_string(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatString>) {

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
// - `TlvEntryMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TlvEntryMut<'_> {}

// SAFETY:
// - `TlvEntryMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TlvEntryMut<'_> {}

impl<'msg> ::protobuf::AsView for TlvEntryMut<'msg> {
  type Proxied = TlvEntry;
  fn as_view(&self) -> ::protobuf::View<'_, TlvEntry> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TlvEntryMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TlvEntry>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TlvEntryMut<'msg> {
  type MutProxied = TlvEntry;
  fn as_mut(&mut self) -> TlvEntryMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TlvEntryMut<'msg> {
  fn into_mut<'shorter>(self) -> TlvEntryMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TlvEntry {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TlvEntry> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TlvEntryView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TlvEntryMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // type: optional uint32
  pub fn r#type(&self) -> u32 {
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
  pub fn set_type(&mut self, val: u32) {
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

  // value: optional bytes
  pub fn value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // format_string: optional message envoy.config.core.v3.SubstitutionFormatString
  pub fn has_format_string(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_format_string(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn format_string_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringView<'_>> {
    self.has_format_string().then(|| self.format_string())
  }
  pub fn format_string(&self) -> crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringView::default())
  }
  pub fn format_string_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringMut<'_> {
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
  pub fn set_format_string(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl TlvEntry

impl ::std::ops::Drop for TlvEntry {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TlvEntry {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TlvEntry {
  type Proxied = Self;
  fn as_view(&self) -> TlvEntryView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TlvEntry {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TlvEntryMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TlvEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__TlvEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$)P0P3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__TlvEntry_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatString as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__TlvEntry_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TlvEntry {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TlvEntry {
  type Msg = TlvEntry;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlvEntry> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TlvEntry {
  type Msg = TlvEntry;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlvEntry> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TlvEntryMut<'_> {
  type Msg = TlvEntry;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlvEntry> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TlvEntryMut<'_> {
  type Msg = TlvEntry;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlvEntry> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TlvEntryView<'_> {
  type Msg = TlvEntry;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlvEntry> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TlvEntryMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__ProxyProtocolConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ProxyProtocolConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ProxyProtocolConfig>
}

impl ::protobuf::Message for ProxyProtocolConfig {
  type MessageView<'msg> = ProxyProtocolConfigView<'msg>;
  type MessageMut<'msg> = ProxyProtocolConfigMut<'msg>;
}

impl ::std::default::Default for ProxyProtocolConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ProxyProtocolConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ProxyProtocolConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `ProxyProtocolConfigMut`.
unsafe impl ::std::marker::Sync for ProxyProtocolConfig {}

// SAFETY:
// - `ProxyProtocolConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ProxyProtocolConfig {}

impl ::protobuf::Proxied for ProxyProtocolConfig {
  type View<'msg> = ProxyProtocolConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ProxyProtocolConfig {}

impl ::protobuf::MutProxied for ProxyProtocolConfig {
  type Mut<'msg> = ProxyProtocolConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ProxyProtocolConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ProxyProtocolConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ProxyProtocolConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ProxyProtocolConfigView<'msg> {
  type Message = ProxyProtocolConfig;
}

impl ::std::fmt::Debug for ProxyProtocolConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ProxyProtocolConfigView<'_> {
  fn default() -> ProxyProtocolConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ProxyProtocolConfig>> for ProxyProtocolConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ProxyProtocolConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ProxyProtocolConfigView<'msg> {

  pub fn to_owned(&self) -> ProxyProtocolConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // version: optional enum envoy.config.core.v3.ProxyProtocolConfig.Version
  pub fn version(self) -> super::proxy_protocol_config::Version {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::proxy_protocol_config::Version::V1).into()
      ).try_into().unwrap()
    }
  }

  // pass_through_tlvs: optional message envoy.config.core.v3.ProxyProtocolPassThroughTLVs
  pub fn has_pass_through_tlvs(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn pass_through_tlvs_opt(self) -> ::std::option::Option<super::ProxyProtocolPassThroughTLVsView<'msg>> {
    self.has_pass_through_tlvs().then(|| self.pass_through_tlvs())
  }
  pub fn pass_through_tlvs(self) -> super::ProxyProtocolPassThroughTLVsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ProxyProtocolPassThroughTLVsView::default())
  }

  // added_tlvs: repeated message envoy.config.core.v3.TlvEntry
  pub fn added_tlvs(self) -> ::protobuf::RepeatedView<'msg, super::TlvEntry> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::TlvEntry>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `ProxyProtocolConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ProxyProtocolConfigView<'_> {}

// SAFETY:
// - `ProxyProtocolConfigView` is `Send` because while its alive a `ProxyProtocolConfigMut` cannot.
// - `ProxyProtocolConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for ProxyProtocolConfigView<'_> {}

impl<'msg> ::protobuf::AsView for ProxyProtocolConfigView<'msg> {
  type Proxied = ProxyProtocolConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, ProxyProtocolConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ProxyProtocolConfigView<'msg> {
  fn into_view<'shorter>(self) -> ProxyProtocolConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ProxyProtocolConfig> for ProxyProtocolConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ProxyProtocolConfig {
    let mut dst = ProxyProtocolConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ProxyProtocolConfig> for ProxyProtocolConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ProxyProtocolConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ProxyProtocolConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ProxyProtocolConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ProxyProtocolConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ProxyProtocolConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ProxyProtocolConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ProxyProtocolConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ProxyProtocolConfigMut<'msg> {
  type Message = ProxyProtocolConfig;
}

impl ::std::fmt::Debug for ProxyProtocolConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ProxyProtocolConfig>> for ProxyProtocolConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ProxyProtocolConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ProxyProtocolConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ProxyProtocolConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ProxyProtocolConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // version: optional enum envoy.config.core.v3.ProxyProtocolConfig.Version
  pub fn version(&self) -> super::proxy_protocol_config::Version {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::proxy_protocol_config::Version::V1).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_version(&mut self, val: super::proxy_protocol_config::Version) {
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

  // pass_through_tlvs: optional message envoy.config.core.v3.ProxyProtocolPassThroughTLVs
  pub fn has_pass_through_tlvs(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_pass_through_tlvs(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn pass_through_tlvs_opt(&self) -> ::std::option::Option<super::ProxyProtocolPassThroughTLVsView<'_>> {
    self.has_pass_through_tlvs().then(|| self.pass_through_tlvs())
  }
  pub fn pass_through_tlvs(&self) -> super::ProxyProtocolPassThroughTLVsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ProxyProtocolPassThroughTLVsView::default())
  }
  pub fn pass_through_tlvs_mut(&mut self) -> super::ProxyProtocolPassThroughTLVsMut<'_> {
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
  pub fn set_pass_through_tlvs(&mut self,
    val: impl ::protobuf::IntoProxied<super::ProxyProtocolPassThroughTLVs>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // added_tlvs: repeated message envoy.config.core.v3.TlvEntry
  pub fn added_tlvs(&self) -> ::protobuf::RepeatedView<'_, super::TlvEntry> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::TlvEntry>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn added_tlvs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::TlvEntry> {
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
  pub fn set_added_tlvs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::TlvEntry>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}

// SAFETY:
// - `ProxyProtocolConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ProxyProtocolConfigMut<'_> {}

// SAFETY:
// - `ProxyProtocolConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ProxyProtocolConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for ProxyProtocolConfigMut<'msg> {
  type Proxied = ProxyProtocolConfig;
  fn as_view(&self) -> ::protobuf::View<'_, ProxyProtocolConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ProxyProtocolConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ProxyProtocolConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ProxyProtocolConfigMut<'msg> {
  type MutProxied = ProxyProtocolConfig;
  fn as_mut(&mut self) -> ProxyProtocolConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ProxyProtocolConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> ProxyProtocolConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ProxyProtocolConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ProxyProtocolConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ProxyProtocolConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ProxyProtocolConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // version: optional enum envoy.config.core.v3.ProxyProtocolConfig.Version
  pub fn version(&self) -> super::proxy_protocol_config::Version {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::proxy_protocol_config::Version::V1).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_version(&mut self, val: super::proxy_protocol_config::Version) {
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

  // pass_through_tlvs: optional message envoy.config.core.v3.ProxyProtocolPassThroughTLVs
  pub fn has_pass_through_tlvs(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_pass_through_tlvs(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn pass_through_tlvs_opt(&self) -> ::std::option::Option<super::ProxyProtocolPassThroughTLVsView<'_>> {
    self.has_pass_through_tlvs().then(|| self.pass_through_tlvs())
  }
  pub fn pass_through_tlvs(&self) -> super::ProxyProtocolPassThroughTLVsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ProxyProtocolPassThroughTLVsView::default())
  }
  pub fn pass_through_tlvs_mut(&mut self) -> super::ProxyProtocolPassThroughTLVsMut<'_> {
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
  pub fn set_pass_through_tlvs(&mut self,
    val: impl ::protobuf::IntoProxied<super::ProxyProtocolPassThroughTLVs>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // added_tlvs: repeated message envoy.config.core.v3.TlvEntry
  pub fn added_tlvs(&self) -> ::protobuf::RepeatedView<'_, super::TlvEntry> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::TlvEntry>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn added_tlvs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::TlvEntry> {
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
  pub fn set_added_tlvs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::TlvEntry>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}  // impl ProxyProtocolConfig

impl ::std::ops::Drop for ProxyProtocolConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ProxyProtocolConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ProxyProtocolConfig {
  type Proxied = Self;
  fn as_view(&self) -> ProxyProtocolConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ProxyProtocolConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ProxyProtocolConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ProxyProtocolConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__ProxyProtocolConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.P3G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__ProxyProtocolConfig_msg_init.0, &[<super::ProxyProtocolPassThroughTLVs as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::TlvEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__ProxyProtocolConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ProxyProtocolConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ProxyProtocolConfig {
  type Msg = ProxyProtocolConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProxyProtocolConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ProxyProtocolConfig {
  type Msg = ProxyProtocolConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProxyProtocolConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ProxyProtocolConfigMut<'_> {
  type Msg = ProxyProtocolConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProxyProtocolConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ProxyProtocolConfigMut<'_> {
  type Msg = ProxyProtocolConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProxyProtocolConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ProxyProtocolConfigView<'_> {
  type Msg = ProxyProtocolConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProxyProtocolConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ProxyProtocolConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod proxy_protocol_config {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Version(i32);

#[allow(non_upper_case_globals)]
impl Version {
  pub const V1: Version = Version(0);
  pub const V2: Version = Version(1);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "V1",
      1 => "V2",
      _ => return None
    })
  }
}

impl ::std::convert::From<Version> for i32 {
  fn from(val: Version) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for Version {
  fn from(val: i32) -> Version {
    Self(val)
  }
}

impl ::std::default::Default for Version {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for Version {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "Version::{}", constant_name)
    } else {
      write!(f, "Version::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for Version {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for Version {}

impl ::protobuf::Proxied for Version {
  type View<'a> = Version;
}

impl ::protobuf::AsView for Version {
  type Proxied = Version;

  fn as_view(&self) -> Version {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Version {
  fn into_view<'shorter>(self) -> Version where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for Version {
  const NAME: &'static str = "Version";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1)
  }
}

impl ::protobuf::__internal::EntityType for Version {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod proxy_protocol_config


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__PerHostConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct PerHostConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PerHostConfig>
}

impl ::protobuf::Message for PerHostConfig {
  type MessageView<'msg> = PerHostConfigView<'msg>;
  type MessageMut<'msg> = PerHostConfigMut<'msg>;
}

impl ::std::default::Default for PerHostConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for PerHostConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `PerHostConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `PerHostConfigMut`.
unsafe impl ::std::marker::Sync for PerHostConfig {}

// SAFETY:
// - `PerHostConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for PerHostConfig {}

impl ::protobuf::Proxied for PerHostConfig {
  type View<'msg> = PerHostConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PerHostConfig {}

impl ::protobuf::MutProxied for PerHostConfig {
  type Mut<'msg> = PerHostConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PerHostConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PerHostConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PerHostConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PerHostConfigView<'msg> {
  type Message = PerHostConfig;
}

impl ::std::fmt::Debug for PerHostConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PerHostConfigView<'_> {
  fn default() -> PerHostConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, PerHostConfig>> for PerHostConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PerHostConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PerHostConfigView<'msg> {

  pub fn to_owned(&self) -> PerHostConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // added_tlvs: repeated message envoy.config.core.v3.TlvEntry
  pub fn added_tlvs(self) -> ::protobuf::RepeatedView<'msg, super::TlvEntry> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::TlvEntry>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `PerHostConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for PerHostConfigView<'_> {}

// SAFETY:
// - `PerHostConfigView` is `Send` because while its alive a `PerHostConfigMut` cannot.
// - `PerHostConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for PerHostConfigView<'_> {}

impl<'msg> ::protobuf::AsView for PerHostConfigView<'msg> {
  type Proxied = PerHostConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, PerHostConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PerHostConfigView<'msg> {
  fn into_view<'shorter>(self) -> PerHostConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PerHostConfig> for PerHostConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PerHostConfig {
    let mut dst = PerHostConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PerHostConfig> for PerHostConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PerHostConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for PerHostConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PerHostConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PerHostConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PerHostConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PerHostConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PerHostConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PerHostConfigMut<'msg> {
  type Message = PerHostConfig;
}

impl ::std::fmt::Debug for PerHostConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, PerHostConfig>> for PerHostConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PerHostConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PerHostConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PerHostConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> PerHostConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // added_tlvs: repeated message envoy.config.core.v3.TlvEntry
  pub fn added_tlvs(&self) -> ::protobuf::RepeatedView<'_, super::TlvEntry> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::TlvEntry>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn added_tlvs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::TlvEntry> {
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
  pub fn set_added_tlvs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::TlvEntry>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `PerHostConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for PerHostConfigMut<'_> {}

// SAFETY:
// - `PerHostConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for PerHostConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for PerHostConfigMut<'msg> {
  type Proxied = PerHostConfig;
  fn as_view(&self) -> ::protobuf::View<'_, PerHostConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PerHostConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PerHostConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for PerHostConfigMut<'msg> {
  type MutProxied = PerHostConfig;
  fn as_mut(&mut self) -> PerHostConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PerHostConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> PerHostConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PerHostConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PerHostConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PerHostConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PerHostConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // added_tlvs: repeated message envoy.config.core.v3.TlvEntry
  pub fn added_tlvs(&self) -> ::protobuf::RepeatedView<'_, super::TlvEntry> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::TlvEntry>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn added_tlvs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::TlvEntry> {
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
  pub fn set_added_tlvs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::TlvEntry>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl PerHostConfig

impl ::std::ops::Drop for PerHostConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PerHostConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PerHostConfig {
  type Proxied = Self;
  fn as_view(&self) -> PerHostConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PerHostConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PerHostConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PerHostConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__PerHostConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__PerHostConfig_msg_init.0, &[<super::TlvEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__PerHostConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PerHostConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PerHostConfig {
  type Msg = PerHostConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PerHostConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PerHostConfig {
  type Msg = PerHostConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PerHostConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PerHostConfigMut<'_> {
  type Msg = PerHostConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PerHostConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PerHostConfigMut<'_> {
  type Msg = PerHostConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PerHostConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PerHostConfigView<'_> {
  type Msg = PerHostConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PerHostConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PerHostConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



