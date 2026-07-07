const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__v3__HttpStatus_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HttpStatus {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HttpStatus>
}

impl ::protobuf::Message for HttpStatus {
  type MessageView<'msg> = HttpStatusView<'msg>;
  type MessageMut<'msg> = HttpStatusMut<'msg>;
}

impl ::std::default::Default for HttpStatus {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HttpStatus {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HttpStatus` is `Sync` because it does not implement interior mutability.
//    Neither does `HttpStatusMut`.
unsafe impl ::std::marker::Sync for HttpStatus {}

// SAFETY:
// - `HttpStatus` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HttpStatus {}

impl ::protobuf::Proxied for HttpStatus {
  type View<'msg> = HttpStatusView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HttpStatus {}

impl ::protobuf::MutProxied for HttpStatus {
  type Mut<'msg> = HttpStatusMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HttpStatusView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpStatus>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpStatusView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HttpStatusView<'msg> {
  type Message = HttpStatus;
}

impl ::std::fmt::Debug for HttpStatusView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HttpStatusView<'_> {
  fn default() -> HttpStatusView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HttpStatus>> for HttpStatusView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpStatus>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpStatusView<'msg> {

  pub fn to_owned(&self) -> HttpStatus {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // code: optional enum envoy.type.v3.StatusCode
  pub fn code(self) -> super::StatusCode {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::StatusCode::Empty).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `HttpStatusView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HttpStatusView<'_> {}

// SAFETY:
// - `HttpStatusView` is `Send` because while its alive a `HttpStatusMut` cannot.
// - `HttpStatusView` does not use thread-local data.
unsafe impl ::std::marker::Send for HttpStatusView<'_> {}

impl<'msg> ::protobuf::AsView for HttpStatusView<'msg> {
  type Proxied = HttpStatus;
  fn as_view(&self) -> ::protobuf::View<'msg, HttpStatus> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpStatusView<'msg> {
  fn into_view<'shorter>(self) -> HttpStatusView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpStatus> for HttpStatusView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpStatus {
    let mut dst = HttpStatus::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpStatus> for HttpStatusMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpStatus {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HttpStatus {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpStatusView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpStatusMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HttpStatusMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpStatus>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpStatusMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HttpStatusMut<'msg> {
  type Message = HttpStatus;
}

impl ::std::fmt::Debug for HttpStatusMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HttpStatus>> for HttpStatusMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpStatus>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpStatusMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpStatus> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HttpStatus {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // code: optional enum envoy.type.v3.StatusCode
  pub fn code(&self) -> super::StatusCode {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::StatusCode::Empty).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_code(&mut self, val: super::StatusCode) {
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

}

// SAFETY:
// - `HttpStatusMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HttpStatusMut<'_> {}

// SAFETY:
// - `HttpStatusMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HttpStatusMut<'_> {}

impl<'msg> ::protobuf::AsView for HttpStatusMut<'msg> {
  type Proxied = HttpStatus;
  fn as_view(&self) -> ::protobuf::View<'_, HttpStatus> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpStatusMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HttpStatus>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HttpStatusMut<'msg> {
  type MutProxied = HttpStatus;
  fn as_mut(&mut self) -> HttpStatusMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HttpStatusMut<'msg> {
  fn into_mut<'shorter>(self) -> HttpStatusMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HttpStatus {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HttpStatus> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HttpStatusView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HttpStatusMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // code: optional enum envoy.type.v3.StatusCode
  pub fn code(&self) -> super::StatusCode {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::StatusCode::Empty).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_code(&mut self, val: super::StatusCode) {
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

}  // impl HttpStatus

impl ::std::ops::Drop for HttpStatus {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HttpStatus {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HttpStatus {
  type Proxied = Self;
  fn as_view(&self) -> HttpStatusView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HttpStatus {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HttpStatusMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HttpStatus {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__type__v3__HttpStatus_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__v3__HttpStatus_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__v3__HttpStatus_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpStatus {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpStatus {
  type Msg = HttpStatus;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpStatus> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpStatus {
  type Msg = HttpStatus;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpStatus> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpStatusMut<'_> {
  type Msg = HttpStatus;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpStatus> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpStatusMut<'_> {
  type Msg = HttpStatus;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpStatus> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpStatusView<'_> {
  type Msg = HttpStatus;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpStatus> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpStatusMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StatusCode(i32);

#[allow(non_upper_case_globals)]
impl StatusCode {
  pub const Empty: StatusCode = StatusCode(0);
  pub const Continue: StatusCode = StatusCode(100);
  pub const Ok: StatusCode = StatusCode(200);
  pub const Created: StatusCode = StatusCode(201);
  pub const Accepted: StatusCode = StatusCode(202);
  pub const Nonauthoritativeinformation: StatusCode = StatusCode(203);
  pub const Nocontent: StatusCode = StatusCode(204);
  pub const Resetcontent: StatusCode = StatusCode(205);
  pub const Partialcontent: StatusCode = StatusCode(206);
  pub const Multistatus: StatusCode = StatusCode(207);
  pub const Alreadyreported: StatusCode = StatusCode(208);
  pub const Imused: StatusCode = StatusCode(226);
  pub const Multiplechoices: StatusCode = StatusCode(300);
  pub const Movedpermanently: StatusCode = StatusCode(301);
  pub const Found: StatusCode = StatusCode(302);
  pub const Seeother: StatusCode = StatusCode(303);
  pub const Notmodified: StatusCode = StatusCode(304);
  pub const Useproxy: StatusCode = StatusCode(305);
  pub const Temporaryredirect: StatusCode = StatusCode(307);
  pub const Permanentredirect: StatusCode = StatusCode(308);
  pub const Badrequest: StatusCode = StatusCode(400);
  pub const Unauthorized: StatusCode = StatusCode(401);
  pub const Paymentrequired: StatusCode = StatusCode(402);
  pub const Forbidden: StatusCode = StatusCode(403);
  pub const Notfound: StatusCode = StatusCode(404);
  pub const Methodnotallowed: StatusCode = StatusCode(405);
  pub const Notacceptable: StatusCode = StatusCode(406);
  pub const Proxyauthenticationrequired: StatusCode = StatusCode(407);
  pub const Requesttimeout: StatusCode = StatusCode(408);
  pub const Conflict: StatusCode = StatusCode(409);
  pub const Gone: StatusCode = StatusCode(410);
  pub const Lengthrequired: StatusCode = StatusCode(411);
  pub const Preconditionfailed: StatusCode = StatusCode(412);
  pub const Payloadtoolarge: StatusCode = StatusCode(413);
  pub const Uritoolong: StatusCode = StatusCode(414);
  pub const Unsupportedmediatype: StatusCode = StatusCode(415);
  pub const Rangenotsatisfiable: StatusCode = StatusCode(416);
  pub const Expectationfailed: StatusCode = StatusCode(417);
  pub const Misdirectedrequest: StatusCode = StatusCode(421);
  pub const Unprocessableentity: StatusCode = StatusCode(422);
  pub const Locked: StatusCode = StatusCode(423);
  pub const Faileddependency: StatusCode = StatusCode(424);
  pub const Upgraderequired: StatusCode = StatusCode(426);
  pub const Preconditionrequired: StatusCode = StatusCode(428);
  pub const Toomanyrequests: StatusCode = StatusCode(429);
  pub const Requestheaderfieldstoolarge: StatusCode = StatusCode(431);
  pub const Internalservererror: StatusCode = StatusCode(500);
  pub const Notimplemented: StatusCode = StatusCode(501);
  pub const Badgateway: StatusCode = StatusCode(502);
  pub const Serviceunavailable: StatusCode = StatusCode(503);
  pub const Gatewaytimeout: StatusCode = StatusCode(504);
  pub const Httpversionnotsupported: StatusCode = StatusCode(505);
  pub const Variantalsonegotiates: StatusCode = StatusCode(506);
  pub const Insufficientstorage: StatusCode = StatusCode(507);
  pub const Loopdetected: StatusCode = StatusCode(508);
  pub const Notextended: StatusCode = StatusCode(510);
  pub const Networkauthenticationrequired: StatusCode = StatusCode(511);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Empty",
      100 => "Continue",
      200 => "Ok",
      201 => "Created",
      202 => "Accepted",
      203 => "Nonauthoritativeinformation",
      204 => "Nocontent",
      205 => "Resetcontent",
      206 => "Partialcontent",
      207 => "Multistatus",
      208 => "Alreadyreported",
      226 => "Imused",
      300 => "Multiplechoices",
      301 => "Movedpermanently",
      302 => "Found",
      303 => "Seeother",
      304 => "Notmodified",
      305 => "Useproxy",
      307 => "Temporaryredirect",
      308 => "Permanentredirect",
      400 => "Badrequest",
      401 => "Unauthorized",
      402 => "Paymentrequired",
      403 => "Forbidden",
      404 => "Notfound",
      405 => "Methodnotallowed",
      406 => "Notacceptable",
      407 => "Proxyauthenticationrequired",
      408 => "Requesttimeout",
      409 => "Conflict",
      410 => "Gone",
      411 => "Lengthrequired",
      412 => "Preconditionfailed",
      413 => "Payloadtoolarge",
      414 => "Uritoolong",
      415 => "Unsupportedmediatype",
      416 => "Rangenotsatisfiable",
      417 => "Expectationfailed",
      421 => "Misdirectedrequest",
      422 => "Unprocessableentity",
      423 => "Locked",
      424 => "Faileddependency",
      426 => "Upgraderequired",
      428 => "Preconditionrequired",
      429 => "Toomanyrequests",
      431 => "Requestheaderfieldstoolarge",
      500 => "Internalservererror",
      501 => "Notimplemented",
      502 => "Badgateway",
      503 => "Serviceunavailable",
      504 => "Gatewaytimeout",
      505 => "Httpversionnotsupported",
      506 => "Variantalsonegotiates",
      507 => "Insufficientstorage",
      508 => "Loopdetected",
      510 => "Notextended",
      511 => "Networkauthenticationrequired",
      _ => return None
    })
  }
}

impl ::std::convert::From<StatusCode> for i32 {
  fn from(val: StatusCode) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for StatusCode {
  fn from(val: i32) -> StatusCode {
    Self(val)
  }
}

impl ::std::default::Default for StatusCode {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for StatusCode {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "StatusCode::{}", constant_name)
    } else {
      write!(f, "StatusCode::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for StatusCode {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for StatusCode {}

impl ::protobuf::Proxied for StatusCode {
  type View<'a> = StatusCode;
}

impl ::protobuf::AsView for StatusCode {
  type Proxied = StatusCode;

  fn as_view(&self) -> StatusCode {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StatusCode {
  fn into_view<'shorter>(self) -> StatusCode where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for StatusCode {
  const NAME: &'static str = "StatusCode";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|100|200|201|202|203|204|205|206|207|208|226|300|301|302|303|304|305|307|308|400|401|402|403|404|405|406|407|408|409|410|411|412|413|414|415|416|417|421|422|423|424|426|428|429|431|500|501|502|503|504|505|506|507|508|510|511)
  }
}

impl ::protobuf::__internal::EntityType for StatusCode {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


