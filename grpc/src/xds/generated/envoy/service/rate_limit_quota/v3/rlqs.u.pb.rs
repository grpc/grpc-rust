const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__rate_0limit_0quota__v3__RateLimitQuotaUsageReports_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RateLimitQuotaUsageReports {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RateLimitQuotaUsageReports>
}

impl ::protobuf::Message for RateLimitQuotaUsageReports {
  type MessageView<'msg> = RateLimitQuotaUsageReportsView<'msg>;
  type MessageMut<'msg> = RateLimitQuotaUsageReportsMut<'msg>;
}

impl ::std::default::Default for RateLimitQuotaUsageReports {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RateLimitQuotaUsageReports {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RateLimitQuotaUsageReports` is `Sync` because it does not implement interior mutability.
//    Neither does `RateLimitQuotaUsageReportsMut`.
unsafe impl ::std::marker::Sync for RateLimitQuotaUsageReports {}

// SAFETY:
// - `RateLimitQuotaUsageReports` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RateLimitQuotaUsageReports {}

impl ::protobuf::Proxied for RateLimitQuotaUsageReports {
  type View<'msg> = RateLimitQuotaUsageReportsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RateLimitQuotaUsageReports {}

impl ::protobuf::MutProxied for RateLimitQuotaUsageReports {
  type Mut<'msg> = RateLimitQuotaUsageReportsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RateLimitQuotaUsageReportsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RateLimitQuotaUsageReports>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RateLimitQuotaUsageReportsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RateLimitQuotaUsageReportsView<'msg> {
  type Message = RateLimitQuotaUsageReports;
}

impl ::std::fmt::Debug for RateLimitQuotaUsageReportsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RateLimitQuotaUsageReportsView<'_> {
  fn default() -> RateLimitQuotaUsageReportsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RateLimitQuotaUsageReports>> for RateLimitQuotaUsageReportsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RateLimitQuotaUsageReports>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RateLimitQuotaUsageReportsView<'msg> {

  pub fn to_owned(&self) -> RateLimitQuotaUsageReports {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // domain: optional string
  pub fn domain(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // bucket_quota_usages: repeated message envoy.service.rate_limit_quota.v3.RateLimitQuotaUsageReports.BucketQuotaUsage
  pub fn bucket_quota_usages(self) -> ::protobuf::RepeatedView<'msg, super::rate_limit_quota_usage_reports::BucketQuotaUsage> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::rate_limit_quota_usage_reports::BucketQuotaUsage>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `RateLimitQuotaUsageReportsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RateLimitQuotaUsageReportsView<'_> {}

// SAFETY:
// - `RateLimitQuotaUsageReportsView` is `Send` because while its alive a `RateLimitQuotaUsageReportsMut` cannot.
// - `RateLimitQuotaUsageReportsView` does not use thread-local data.
unsafe impl ::std::marker::Send for RateLimitQuotaUsageReportsView<'_> {}

impl<'msg> ::protobuf::AsView for RateLimitQuotaUsageReportsView<'msg> {
  type Proxied = RateLimitQuotaUsageReports;
  fn as_view(&self) -> ::protobuf::View<'msg, RateLimitQuotaUsageReports> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RateLimitQuotaUsageReportsView<'msg> {
  fn into_view<'shorter>(self) -> RateLimitQuotaUsageReportsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RateLimitQuotaUsageReports> for RateLimitQuotaUsageReportsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RateLimitQuotaUsageReports {
    let mut dst = RateLimitQuotaUsageReports::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RateLimitQuotaUsageReports> for RateLimitQuotaUsageReportsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RateLimitQuotaUsageReports {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RateLimitQuotaUsageReports {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RateLimitQuotaUsageReportsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RateLimitQuotaUsageReportsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RateLimitQuotaUsageReportsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RateLimitQuotaUsageReports>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RateLimitQuotaUsageReportsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RateLimitQuotaUsageReportsMut<'msg> {
  type Message = RateLimitQuotaUsageReports;
}

impl ::std::fmt::Debug for RateLimitQuotaUsageReportsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RateLimitQuotaUsageReports>> for RateLimitQuotaUsageReportsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RateLimitQuotaUsageReports>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RateLimitQuotaUsageReportsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RateLimitQuotaUsageReports> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RateLimitQuotaUsageReports {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // domain: optional string
  pub fn domain(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_domain(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // bucket_quota_usages: repeated message envoy.service.rate_limit_quota.v3.RateLimitQuotaUsageReports.BucketQuotaUsage
  pub fn bucket_quota_usages(&self) -> ::protobuf::RepeatedView<'_, super::rate_limit_quota_usage_reports::BucketQuotaUsage> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::rate_limit_quota_usage_reports::BucketQuotaUsage>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn bucket_quota_usages_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::rate_limit_quota_usage_reports::BucketQuotaUsage> {
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
  pub fn set_bucket_quota_usages(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::rate_limit_quota_usage_reports::BucketQuotaUsage>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `RateLimitQuotaUsageReportsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RateLimitQuotaUsageReportsMut<'_> {}

// SAFETY:
// - `RateLimitQuotaUsageReportsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RateLimitQuotaUsageReportsMut<'_> {}

impl<'msg> ::protobuf::AsView for RateLimitQuotaUsageReportsMut<'msg> {
  type Proxied = RateLimitQuotaUsageReports;
  fn as_view(&self) -> ::protobuf::View<'_, RateLimitQuotaUsageReports> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RateLimitQuotaUsageReportsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RateLimitQuotaUsageReports>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RateLimitQuotaUsageReportsMut<'msg> {
  type MutProxied = RateLimitQuotaUsageReports;
  fn as_mut(&mut self) -> RateLimitQuotaUsageReportsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RateLimitQuotaUsageReportsMut<'msg> {
  fn into_mut<'shorter>(self) -> RateLimitQuotaUsageReportsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RateLimitQuotaUsageReports {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RateLimitQuotaUsageReports> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RateLimitQuotaUsageReportsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RateLimitQuotaUsageReportsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // domain: optional string
  pub fn domain(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_domain(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // bucket_quota_usages: repeated message envoy.service.rate_limit_quota.v3.RateLimitQuotaUsageReports.BucketQuotaUsage
  pub fn bucket_quota_usages(&self) -> ::protobuf::RepeatedView<'_, super::rate_limit_quota_usage_reports::BucketQuotaUsage> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::rate_limit_quota_usage_reports::BucketQuotaUsage>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn bucket_quota_usages_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::rate_limit_quota_usage_reports::BucketQuotaUsage> {
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
  pub fn set_bucket_quota_usages(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::rate_limit_quota_usage_reports::BucketQuotaUsage>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl RateLimitQuotaUsageReports

impl ::std::ops::Drop for RateLimitQuotaUsageReports {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RateLimitQuotaUsageReports {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RateLimitQuotaUsageReports {
  type Proxied = Self;
  fn as_view(&self) -> RateLimitQuotaUsageReportsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RateLimitQuotaUsageReports {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RateLimitQuotaUsageReportsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RateLimitQuotaUsageReports {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__rate_0limit_0quota__v3__RateLimitQuotaUsageReports_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1XG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__rate_0limit_0quota__v3__RateLimitQuotaUsageReports_msg_init.0, &[<super::rate_limit_quota_usage_reports::BucketQuotaUsage as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__rate_0limit_0quota__v3__RateLimitQuotaUsageReports_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RateLimitQuotaUsageReports {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RateLimitQuotaUsageReports {
  type Msg = RateLimitQuotaUsageReports;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitQuotaUsageReports> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RateLimitQuotaUsageReports {
  type Msg = RateLimitQuotaUsageReports;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitQuotaUsageReports> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RateLimitQuotaUsageReportsMut<'_> {
  type Msg = RateLimitQuotaUsageReports;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitQuotaUsageReports> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RateLimitQuotaUsageReportsMut<'_> {
  type Msg = RateLimitQuotaUsageReports;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitQuotaUsageReports> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RateLimitQuotaUsageReportsView<'_> {
  type Msg = RateLimitQuotaUsageReports;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitQuotaUsageReports> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RateLimitQuotaUsageReportsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod rate_limit_quota_usage_reports {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__rate_0limit_0quota__v3__RateLimitQuotaUsageReports__BucketQuotaUsage_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct BucketQuotaUsage {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<BucketQuotaUsage>
}

impl ::protobuf::Message for BucketQuotaUsage {
  type MessageView<'msg> = BucketQuotaUsageView<'msg>;
  type MessageMut<'msg> = BucketQuotaUsageMut<'msg>;
}

impl ::std::default::Default for BucketQuotaUsage {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for BucketQuotaUsage {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `BucketQuotaUsage` is `Sync` because it does not implement interior mutability.
//    Neither does `BucketQuotaUsageMut`.
unsafe impl ::std::marker::Sync for BucketQuotaUsage {}

// SAFETY:
// - `BucketQuotaUsage` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for BucketQuotaUsage {}

impl ::protobuf::Proxied for BucketQuotaUsage {
  type View<'msg> = BucketQuotaUsageView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for BucketQuotaUsage {}

impl ::protobuf::MutProxied for BucketQuotaUsage {
  type Mut<'msg> = BucketQuotaUsageMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct BucketQuotaUsageView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BucketQuotaUsage>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BucketQuotaUsageView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for BucketQuotaUsageView<'msg> {
  type Message = BucketQuotaUsage;
}

impl ::std::fmt::Debug for BucketQuotaUsageView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for BucketQuotaUsageView<'_> {
  fn default() -> BucketQuotaUsageView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, BucketQuotaUsage>> for BucketQuotaUsageView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BucketQuotaUsage>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BucketQuotaUsageView<'msg> {

  pub fn to_owned(&self) -> BucketQuotaUsage {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // bucket_id: optional message envoy.service.rate_limit_quota.v3.BucketId
  pub fn has_bucket_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn bucket_id_opt(self) -> ::std::option::Option<super::super::BucketIdView<'msg>> {
    self.has_bucket_id().then(|| self.bucket_id())
  }
  pub fn bucket_id(self) -> super::super::BucketIdView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::BucketIdView::default())
  }

  // time_elapsed: optional message google.protobuf.Duration
  pub fn has_time_elapsed(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn time_elapsed_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_time_elapsed().then(|| self.time_elapsed())
  }
  pub fn time_elapsed(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // num_requests_allowed: optional uint64
  pub fn num_requests_allowed(self) -> u64 {
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

  // num_requests_denied: optional uint64
  pub fn num_requests_denied(self) -> u64 {
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

}

// SAFETY:
// - `BucketQuotaUsageView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for BucketQuotaUsageView<'_> {}

// SAFETY:
// - `BucketQuotaUsageView` is `Send` because while its alive a `BucketQuotaUsageMut` cannot.
// - `BucketQuotaUsageView` does not use thread-local data.
unsafe impl ::std::marker::Send for BucketQuotaUsageView<'_> {}

impl<'msg> ::protobuf::AsView for BucketQuotaUsageView<'msg> {
  type Proxied = BucketQuotaUsage;
  fn as_view(&self) -> ::protobuf::View<'msg, BucketQuotaUsage> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BucketQuotaUsageView<'msg> {
  fn into_view<'shorter>(self) -> BucketQuotaUsageView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<BucketQuotaUsage> for BucketQuotaUsageView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BucketQuotaUsage {
    let mut dst = BucketQuotaUsage::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<BucketQuotaUsage> for BucketQuotaUsageMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BucketQuotaUsage {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for BucketQuotaUsage {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BucketQuotaUsageView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BucketQuotaUsageMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct BucketQuotaUsageMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BucketQuotaUsage>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BucketQuotaUsageMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for BucketQuotaUsageMut<'msg> {
  type Message = BucketQuotaUsage;
}

impl ::std::fmt::Debug for BucketQuotaUsageMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, BucketQuotaUsage>> for BucketQuotaUsageMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BucketQuotaUsage>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BucketQuotaUsageMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, BucketQuotaUsage> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> BucketQuotaUsage {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // bucket_id: optional message envoy.service.rate_limit_quota.v3.BucketId
  pub fn has_bucket_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_bucket_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn bucket_id_opt(&self) -> ::std::option::Option<super::super::BucketIdView<'_>> {
    self.has_bucket_id().then(|| self.bucket_id())
  }
  pub fn bucket_id(&self) -> super::super::BucketIdView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::BucketIdView::default())
  }
  pub fn bucket_id_mut(&mut self) -> super::super::BucketIdMut<'_> {
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
  pub fn set_bucket_id(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::BucketId>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // time_elapsed: optional message google.protobuf.Duration
  pub fn has_time_elapsed(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_time_elapsed(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn time_elapsed_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_time_elapsed().then(|| self.time_elapsed())
  }
  pub fn time_elapsed(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn time_elapsed_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_time_elapsed(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // num_requests_allowed: optional uint64
  pub fn num_requests_allowed(&self) -> u64 {
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
  pub fn set_num_requests_allowed(&mut self, val: u64) {
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

  // num_requests_denied: optional uint64
  pub fn num_requests_denied(&self) -> u64 {
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
  pub fn set_num_requests_denied(&mut self, val: u64) {
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

}

// SAFETY:
// - `BucketQuotaUsageMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for BucketQuotaUsageMut<'_> {}

// SAFETY:
// - `BucketQuotaUsageMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for BucketQuotaUsageMut<'_> {}

impl<'msg> ::protobuf::AsView for BucketQuotaUsageMut<'msg> {
  type Proxied = BucketQuotaUsage;
  fn as_view(&self) -> ::protobuf::View<'_, BucketQuotaUsage> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BucketQuotaUsageMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, BucketQuotaUsage>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for BucketQuotaUsageMut<'msg> {
  type MutProxied = BucketQuotaUsage;
  fn as_mut(&mut self) -> BucketQuotaUsageMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for BucketQuotaUsageMut<'msg> {
  fn into_mut<'shorter>(self) -> BucketQuotaUsageMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl BucketQuotaUsage {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, BucketQuotaUsage> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> BucketQuotaUsageView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> BucketQuotaUsageMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // bucket_id: optional message envoy.service.rate_limit_quota.v3.BucketId
  pub fn has_bucket_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_bucket_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn bucket_id_opt(&self) -> ::std::option::Option<super::super::BucketIdView<'_>> {
    self.has_bucket_id().then(|| self.bucket_id())
  }
  pub fn bucket_id(&self) -> super::super::BucketIdView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::BucketIdView::default())
  }
  pub fn bucket_id_mut(&mut self) -> super::super::BucketIdMut<'_> {
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
  pub fn set_bucket_id(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::BucketId>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // time_elapsed: optional message google.protobuf.Duration
  pub fn has_time_elapsed(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_time_elapsed(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn time_elapsed_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_time_elapsed().then(|| self.time_elapsed())
  }
  pub fn time_elapsed(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn time_elapsed_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_time_elapsed(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // num_requests_allowed: optional uint64
  pub fn num_requests_allowed(&self) -> u64 {
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
  pub fn set_num_requests_allowed(&mut self, val: u64) {
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

  // num_requests_denied: optional uint64
  pub fn num_requests_denied(&self) -> u64 {
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
  pub fn set_num_requests_denied(&mut self, val: u64) {
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

}  // impl BucketQuotaUsage

impl ::std::ops::Drop for BucketQuotaUsage {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for BucketQuotaUsage {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for BucketQuotaUsage {
  type Proxied = Self;
  fn as_view(&self) -> BucketQuotaUsageView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for BucketQuotaUsage {
  type MutProxied = Self;
  fn as_mut(&mut self) -> BucketQuotaUsageMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for BucketQuotaUsage {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::rate_limit_quota_usage_reports::envoy__service__rate_0limit_0quota__v3__RateLimitQuotaUsageReports__BucketQuotaUsage_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33,P,P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::rate_limit_quota_usage_reports::envoy__service__rate_0limit_0quota__v3__RateLimitQuotaUsageReports__BucketQuotaUsage_msg_init.0, &[<super::super::BucketId as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::rate_limit_quota_usage_reports::envoy__service__rate_0limit_0quota__v3__RateLimitQuotaUsageReports__BucketQuotaUsage_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BucketQuotaUsage {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BucketQuotaUsage {
  type Msg = BucketQuotaUsage;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BucketQuotaUsage> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BucketQuotaUsage {
  type Msg = BucketQuotaUsage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BucketQuotaUsage> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BucketQuotaUsageMut<'_> {
  type Msg = BucketQuotaUsage;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BucketQuotaUsage> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BucketQuotaUsageMut<'_> {
  type Msg = BucketQuotaUsage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BucketQuotaUsage> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BucketQuotaUsageView<'_> {
  type Msg = BucketQuotaUsage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BucketQuotaUsage> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BucketQuotaUsageMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod rate_limit_quota_usage_reports


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__rate_0limit_0quota__v3__RateLimitQuotaResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RateLimitQuotaResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RateLimitQuotaResponse>
}

impl ::protobuf::Message for RateLimitQuotaResponse {
  type MessageView<'msg> = RateLimitQuotaResponseView<'msg>;
  type MessageMut<'msg> = RateLimitQuotaResponseMut<'msg>;
}

impl ::std::default::Default for RateLimitQuotaResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RateLimitQuotaResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RateLimitQuotaResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `RateLimitQuotaResponseMut`.
unsafe impl ::std::marker::Sync for RateLimitQuotaResponse {}

// SAFETY:
// - `RateLimitQuotaResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RateLimitQuotaResponse {}

impl ::protobuf::Proxied for RateLimitQuotaResponse {
  type View<'msg> = RateLimitQuotaResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RateLimitQuotaResponse {}

impl ::protobuf::MutProxied for RateLimitQuotaResponse {
  type Mut<'msg> = RateLimitQuotaResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RateLimitQuotaResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RateLimitQuotaResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RateLimitQuotaResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RateLimitQuotaResponseView<'msg> {
  type Message = RateLimitQuotaResponse;
}

impl ::std::fmt::Debug for RateLimitQuotaResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RateLimitQuotaResponseView<'_> {
  fn default() -> RateLimitQuotaResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RateLimitQuotaResponse>> for RateLimitQuotaResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RateLimitQuotaResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RateLimitQuotaResponseView<'msg> {

  pub fn to_owned(&self) -> RateLimitQuotaResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // bucket_action: repeated message envoy.service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction
  pub fn bucket_action(self) -> ::protobuf::RepeatedView<'msg, super::rate_limit_quota_response::BucketAction> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::rate_limit_quota_response::BucketAction>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `RateLimitQuotaResponseView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RateLimitQuotaResponseView<'_> {}

// SAFETY:
// - `RateLimitQuotaResponseView` is `Send` because while its alive a `RateLimitQuotaResponseMut` cannot.
// - `RateLimitQuotaResponseView` does not use thread-local data.
unsafe impl ::std::marker::Send for RateLimitQuotaResponseView<'_> {}

impl<'msg> ::protobuf::AsView for RateLimitQuotaResponseView<'msg> {
  type Proxied = RateLimitQuotaResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, RateLimitQuotaResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RateLimitQuotaResponseView<'msg> {
  fn into_view<'shorter>(self) -> RateLimitQuotaResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RateLimitQuotaResponse> for RateLimitQuotaResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RateLimitQuotaResponse {
    let mut dst = RateLimitQuotaResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RateLimitQuotaResponse> for RateLimitQuotaResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RateLimitQuotaResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RateLimitQuotaResponse {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RateLimitQuotaResponseView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RateLimitQuotaResponseMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RateLimitQuotaResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RateLimitQuotaResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RateLimitQuotaResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RateLimitQuotaResponseMut<'msg> {
  type Message = RateLimitQuotaResponse;
}

impl ::std::fmt::Debug for RateLimitQuotaResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RateLimitQuotaResponse>> for RateLimitQuotaResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RateLimitQuotaResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RateLimitQuotaResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RateLimitQuotaResponse> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RateLimitQuotaResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // bucket_action: repeated message envoy.service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction
  pub fn bucket_action(&self) -> ::protobuf::RepeatedView<'_, super::rate_limit_quota_response::BucketAction> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::rate_limit_quota_response::BucketAction>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn bucket_action_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::rate_limit_quota_response::BucketAction> {
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
  pub fn set_bucket_action(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::rate_limit_quota_response::BucketAction>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `RateLimitQuotaResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RateLimitQuotaResponseMut<'_> {}

// SAFETY:
// - `RateLimitQuotaResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RateLimitQuotaResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for RateLimitQuotaResponseMut<'msg> {
  type Proxied = RateLimitQuotaResponse;
  fn as_view(&self) -> ::protobuf::View<'_, RateLimitQuotaResponse> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RateLimitQuotaResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RateLimitQuotaResponse>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RateLimitQuotaResponseMut<'msg> {
  type MutProxied = RateLimitQuotaResponse;
  fn as_mut(&mut self) -> RateLimitQuotaResponseMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RateLimitQuotaResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> RateLimitQuotaResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RateLimitQuotaResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RateLimitQuotaResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RateLimitQuotaResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RateLimitQuotaResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // bucket_action: repeated message envoy.service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction
  pub fn bucket_action(&self) -> ::protobuf::RepeatedView<'_, super::rate_limit_quota_response::BucketAction> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::rate_limit_quota_response::BucketAction>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn bucket_action_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::rate_limit_quota_response::BucketAction> {
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
  pub fn set_bucket_action(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::rate_limit_quota_response::BucketAction>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl RateLimitQuotaResponse

impl ::std::ops::Drop for RateLimitQuotaResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RateLimitQuotaResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RateLimitQuotaResponse {
  type Proxied = Self;
  fn as_view(&self) -> RateLimitQuotaResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RateLimitQuotaResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RateLimitQuotaResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RateLimitQuotaResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__rate_0limit_0quota__v3__RateLimitQuotaResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__rate_0limit_0quota__v3__RateLimitQuotaResponse_msg_init.0, &[<super::rate_limit_quota_response::BucketAction as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__rate_0limit_0quota__v3__RateLimitQuotaResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RateLimitQuotaResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RateLimitQuotaResponse {
  type Msg = RateLimitQuotaResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitQuotaResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RateLimitQuotaResponse {
  type Msg = RateLimitQuotaResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitQuotaResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RateLimitQuotaResponseMut<'_> {
  type Msg = RateLimitQuotaResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitQuotaResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RateLimitQuotaResponseMut<'_> {
  type Msg = RateLimitQuotaResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitQuotaResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RateLimitQuotaResponseView<'_> {
  type Msg = RateLimitQuotaResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitQuotaResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RateLimitQuotaResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod rate_limit_quota_response {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__rate_0limit_0quota__v3__RateLimitQuotaResponse__BucketAction_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct BucketAction {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<BucketAction>
}

impl ::protobuf::Message for BucketAction {
  type MessageView<'msg> = BucketActionView<'msg>;
  type MessageMut<'msg> = BucketActionMut<'msg>;
}

impl ::std::default::Default for BucketAction {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for BucketAction {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `BucketAction` is `Sync` because it does not implement interior mutability.
//    Neither does `BucketActionMut`.
unsafe impl ::std::marker::Sync for BucketAction {}

// SAFETY:
// - `BucketAction` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for BucketAction {}

impl ::protobuf::Proxied for BucketAction {
  type View<'msg> = BucketActionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for BucketAction {}

impl ::protobuf::MutProxied for BucketAction {
  type Mut<'msg> = BucketActionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct BucketActionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BucketAction>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BucketActionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for BucketActionView<'msg> {
  type Message = BucketAction;
}

impl ::std::fmt::Debug for BucketActionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for BucketActionView<'_> {
  fn default() -> BucketActionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, BucketAction>> for BucketActionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BucketAction>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BucketActionView<'msg> {

  pub fn to_owned(&self) -> BucketAction {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // bucket_id: optional message envoy.service.rate_limit_quota.v3.BucketId
  pub fn has_bucket_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn bucket_id_opt(self) -> ::std::option::Option<super::super::BucketIdView<'msg>> {
    self.has_bucket_id().then(|| self.bucket_id())
  }
  pub fn bucket_id(self) -> super::super::BucketIdView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::BucketIdView::default())
  }

  // quota_assignment_action: optional message envoy.service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction.QuotaAssignmentAction
  pub fn has_quota_assignment_action(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn quota_assignment_action_opt(self) -> ::std::option::Option<super::super::rate_limit_quota_response::bucket_action::QuotaAssignmentActionView<'msg>> {
    self.has_quota_assignment_action().then(|| self.quota_assignment_action())
  }
  pub fn quota_assignment_action(self) -> super::super::rate_limit_quota_response::bucket_action::QuotaAssignmentActionView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::rate_limit_quota_response::bucket_action::QuotaAssignmentActionView::default())
  }

  // abandon_action: optional message envoy.service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction.AbandonAction
  pub fn has_abandon_action(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn abandon_action_opt(self) -> ::std::option::Option<super::super::rate_limit_quota_response::bucket_action::AbandonActionView<'msg>> {
    self.has_abandon_action().then(|| self.abandon_action())
  }
  pub fn abandon_action(self) -> super::super::rate_limit_quota_response::bucket_action::AbandonActionView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::rate_limit_quota_response::bucket_action::AbandonActionView::default())
  }

  pub fn bucket_action(self) -> super::super::rate_limit_quota_response::bucket_action::BucketActionOneof<'msg> {
    match self.bucket_action_case() {
      super::super::rate_limit_quota_response::bucket_action::BucketActionCase::QuotaAssignmentAction =>
          super::super::rate_limit_quota_response::bucket_action::BucketActionOneof::QuotaAssignmentAction(self.quota_assignment_action()),
      super::super::rate_limit_quota_response::bucket_action::BucketActionCase::AbandonAction =>
          super::super::rate_limit_quota_response::bucket_action::BucketActionOneof::AbandonAction(self.abandon_action()),
      _ => super::super::rate_limit_quota_response::bucket_action::BucketActionOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn bucket_action_case(self) -> super::super::rate_limit_quota_response::bucket_action::BucketActionCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::rate_limit_quota_response::bucket_action::BucketActionCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `BucketActionView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for BucketActionView<'_> {}

// SAFETY:
// - `BucketActionView` is `Send` because while its alive a `BucketActionMut` cannot.
// - `BucketActionView` does not use thread-local data.
unsafe impl ::std::marker::Send for BucketActionView<'_> {}

impl<'msg> ::protobuf::AsView for BucketActionView<'msg> {
  type Proxied = BucketAction;
  fn as_view(&self) -> ::protobuf::View<'msg, BucketAction> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BucketActionView<'msg> {
  fn into_view<'shorter>(self) -> BucketActionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<BucketAction> for BucketActionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BucketAction {
    let mut dst = BucketAction::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<BucketAction> for BucketActionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BucketAction {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for BucketAction {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BucketActionView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BucketActionMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct BucketActionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BucketAction>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BucketActionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for BucketActionMut<'msg> {
  type Message = BucketAction;
}

impl ::std::fmt::Debug for BucketActionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, BucketAction>> for BucketActionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BucketAction>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BucketActionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, BucketAction> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> BucketAction {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // bucket_id: optional message envoy.service.rate_limit_quota.v3.BucketId
  pub fn has_bucket_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_bucket_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn bucket_id_opt(&self) -> ::std::option::Option<super::super::BucketIdView<'_>> {
    self.has_bucket_id().then(|| self.bucket_id())
  }
  pub fn bucket_id(&self) -> super::super::BucketIdView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::BucketIdView::default())
  }
  pub fn bucket_id_mut(&mut self) -> super::super::BucketIdMut<'_> {
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
  pub fn set_bucket_id(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::BucketId>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // quota_assignment_action: optional message envoy.service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction.QuotaAssignmentAction
  pub fn has_quota_assignment_action(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_quota_assignment_action(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn quota_assignment_action_opt(&self) -> ::std::option::Option<super::super::rate_limit_quota_response::bucket_action::QuotaAssignmentActionView<'_>> {
    self.has_quota_assignment_action().then(|| self.quota_assignment_action())
  }
  pub fn quota_assignment_action(&self) -> super::super::rate_limit_quota_response::bucket_action::QuotaAssignmentActionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::rate_limit_quota_response::bucket_action::QuotaAssignmentActionView::default())
  }
  pub fn quota_assignment_action_mut(&mut self) -> super::super::rate_limit_quota_response::bucket_action::QuotaAssignmentActionMut<'_> {
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
  pub fn set_quota_assignment_action(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::rate_limit_quota_response::bucket_action::QuotaAssignmentAction>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // abandon_action: optional message envoy.service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction.AbandonAction
  pub fn has_abandon_action(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_abandon_action(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn abandon_action_opt(&self) -> ::std::option::Option<super::super::rate_limit_quota_response::bucket_action::AbandonActionView<'_>> {
    self.has_abandon_action().then(|| self.abandon_action())
  }
  pub fn abandon_action(&self) -> super::super::rate_limit_quota_response::bucket_action::AbandonActionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::rate_limit_quota_response::bucket_action::AbandonActionView::default())
  }
  pub fn abandon_action_mut(&mut self) -> super::super::rate_limit_quota_response::bucket_action::AbandonActionMut<'_> {
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
  pub fn set_abandon_action(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::rate_limit_quota_response::bucket_action::AbandonAction>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn bucket_action(&self) -> super::super::rate_limit_quota_response::bucket_action::BucketActionOneof<'_> {
    match &self.bucket_action_case() {
      super::super::rate_limit_quota_response::bucket_action::BucketActionCase::QuotaAssignmentAction =>
          super::super::rate_limit_quota_response::bucket_action::BucketActionOneof::QuotaAssignmentAction(self.quota_assignment_action()),
      super::super::rate_limit_quota_response::bucket_action::BucketActionCase::AbandonAction =>
          super::super::rate_limit_quota_response::bucket_action::BucketActionOneof::AbandonAction(self.abandon_action()),
      _ => super::super::rate_limit_quota_response::bucket_action::BucketActionOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn bucket_action_case(&self) -> super::super::rate_limit_quota_response::bucket_action::BucketActionCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::rate_limit_quota_response::bucket_action::BucketActionCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `BucketActionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for BucketActionMut<'_> {}

// SAFETY:
// - `BucketActionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for BucketActionMut<'_> {}

impl<'msg> ::protobuf::AsView for BucketActionMut<'msg> {
  type Proxied = BucketAction;
  fn as_view(&self) -> ::protobuf::View<'_, BucketAction> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BucketActionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, BucketAction>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for BucketActionMut<'msg> {
  type MutProxied = BucketAction;
  fn as_mut(&mut self) -> BucketActionMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for BucketActionMut<'msg> {
  fn into_mut<'shorter>(self) -> BucketActionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl BucketAction {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, BucketAction> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> BucketActionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> BucketActionMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // bucket_id: optional message envoy.service.rate_limit_quota.v3.BucketId
  pub fn has_bucket_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_bucket_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn bucket_id_opt(&self) -> ::std::option::Option<super::super::BucketIdView<'_>> {
    self.has_bucket_id().then(|| self.bucket_id())
  }
  pub fn bucket_id(&self) -> super::super::BucketIdView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::BucketIdView::default())
  }
  pub fn bucket_id_mut(&mut self) -> super::super::BucketIdMut<'_> {
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
  pub fn set_bucket_id(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::BucketId>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // quota_assignment_action: optional message envoy.service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction.QuotaAssignmentAction
  pub fn has_quota_assignment_action(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_quota_assignment_action(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn quota_assignment_action_opt(&self) -> ::std::option::Option<super::super::rate_limit_quota_response::bucket_action::QuotaAssignmentActionView<'_>> {
    self.has_quota_assignment_action().then(|| self.quota_assignment_action())
  }
  pub fn quota_assignment_action(&self) -> super::super::rate_limit_quota_response::bucket_action::QuotaAssignmentActionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::rate_limit_quota_response::bucket_action::QuotaAssignmentActionView::default())
  }
  pub fn quota_assignment_action_mut(&mut self) -> super::super::rate_limit_quota_response::bucket_action::QuotaAssignmentActionMut<'_> {
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
  pub fn set_quota_assignment_action(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::rate_limit_quota_response::bucket_action::QuotaAssignmentAction>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // abandon_action: optional message envoy.service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction.AbandonAction
  pub fn has_abandon_action(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_abandon_action(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn abandon_action_opt(&self) -> ::std::option::Option<super::super::rate_limit_quota_response::bucket_action::AbandonActionView<'_>> {
    self.has_abandon_action().then(|| self.abandon_action())
  }
  pub fn abandon_action(&self) -> super::super::rate_limit_quota_response::bucket_action::AbandonActionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::rate_limit_quota_response::bucket_action::AbandonActionView::default())
  }
  pub fn abandon_action_mut(&mut self) -> super::super::rate_limit_quota_response::bucket_action::AbandonActionMut<'_> {
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
  pub fn set_abandon_action(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::rate_limit_quota_response::bucket_action::AbandonAction>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn bucket_action(&self) -> super::super::rate_limit_quota_response::bucket_action::BucketActionOneof<'_> {
    match &self.bucket_action_case() {
      super::super::rate_limit_quota_response::bucket_action::BucketActionCase::QuotaAssignmentAction =>
          super::super::rate_limit_quota_response::bucket_action::BucketActionOneof::QuotaAssignmentAction(self.quota_assignment_action()),
      super::super::rate_limit_quota_response::bucket_action::BucketActionCase::AbandonAction =>
          super::super::rate_limit_quota_response::bucket_action::BucketActionOneof::AbandonAction(self.abandon_action()),
      _ => super::super::rate_limit_quota_response::bucket_action::BucketActionOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn bucket_action_case(&self) -> super::super::rate_limit_quota_response::bucket_action::BucketActionCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::rate_limit_quota_response::bucket_action::BucketActionCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl BucketAction

impl ::std::ops::Drop for BucketAction {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for BucketAction {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for BucketAction {
  type Proxied = Self;
  fn as_view(&self) -> BucketActionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for BucketAction {
  type MutProxied = Self;
  fn as_mut(&mut self) -> BucketActionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for BucketAction {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::rate_limit_quota_response::envoy__service__rate_0limit_0quota__v3__RateLimitQuotaResponse__BucketAction_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333^#|$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::rate_limit_quota_response::envoy__service__rate_0limit_0quota__v3__RateLimitQuotaResponse__BucketAction_msg_init.0, &[<super::super::BucketId as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::rate_limit_quota_response::bucket_action::QuotaAssignmentAction as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::rate_limit_quota_response::bucket_action::AbandonAction as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::rate_limit_quota_response::envoy__service__rate_0limit_0quota__v3__RateLimitQuotaResponse__BucketAction_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BucketAction {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BucketAction {
  type Msg = BucketAction;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BucketAction> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BucketAction {
  type Msg = BucketAction;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BucketAction> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BucketActionMut<'_> {
  type Msg = BucketAction;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BucketAction> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BucketActionMut<'_> {
  type Msg = BucketAction;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BucketAction> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BucketActionView<'_> {
  type Msg = BucketAction;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BucketAction> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BucketActionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod bucket_action {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__rate_0limit_0quota__v3__RateLimitQuotaResponse__BucketAction__QuotaAssignmentAction_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct QuotaAssignmentAction {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<QuotaAssignmentAction>
}

impl ::protobuf::Message for QuotaAssignmentAction {
  type MessageView<'msg> = QuotaAssignmentActionView<'msg>;
  type MessageMut<'msg> = QuotaAssignmentActionMut<'msg>;
}

impl ::std::default::Default for QuotaAssignmentAction {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for QuotaAssignmentAction {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `QuotaAssignmentAction` is `Sync` because it does not implement interior mutability.
//    Neither does `QuotaAssignmentActionMut`.
unsafe impl ::std::marker::Sync for QuotaAssignmentAction {}

// SAFETY:
// - `QuotaAssignmentAction` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for QuotaAssignmentAction {}

impl ::protobuf::Proxied for QuotaAssignmentAction {
  type View<'msg> = QuotaAssignmentActionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for QuotaAssignmentAction {}

impl ::protobuf::MutProxied for QuotaAssignmentAction {
  type Mut<'msg> = QuotaAssignmentActionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct QuotaAssignmentActionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, QuotaAssignmentAction>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for QuotaAssignmentActionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for QuotaAssignmentActionView<'msg> {
  type Message = QuotaAssignmentAction;
}

impl ::std::fmt::Debug for QuotaAssignmentActionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for QuotaAssignmentActionView<'_> {
  fn default() -> QuotaAssignmentActionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, QuotaAssignmentAction>> for QuotaAssignmentActionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, QuotaAssignmentAction>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> QuotaAssignmentActionView<'msg> {

  pub fn to_owned(&self) -> QuotaAssignmentAction {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // assignment_time_to_live: optional message google.protobuf.Duration
  pub fn has_assignment_time_to_live(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn assignment_time_to_live_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_assignment_time_to_live().then(|| self.assignment_time_to_live())
  }
  pub fn assignment_time_to_live(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // rate_limit_strategy: optional message envoy.type.v3.RateLimitStrategy
  pub fn has_rate_limit_strategy(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn rate_limit_strategy_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyView<'msg>> {
    self.has_rate_limit_strategy().then(|| self.rate_limit_strategy())
  }
  pub fn rate_limit_strategy(self) -> crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyView::default())
  }

}

// SAFETY:
// - `QuotaAssignmentActionView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for QuotaAssignmentActionView<'_> {}

// SAFETY:
// - `QuotaAssignmentActionView` is `Send` because while its alive a `QuotaAssignmentActionMut` cannot.
// - `QuotaAssignmentActionView` does not use thread-local data.
unsafe impl ::std::marker::Send for QuotaAssignmentActionView<'_> {}

impl<'msg> ::protobuf::AsView for QuotaAssignmentActionView<'msg> {
  type Proxied = QuotaAssignmentAction;
  fn as_view(&self) -> ::protobuf::View<'msg, QuotaAssignmentAction> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for QuotaAssignmentActionView<'msg> {
  fn into_view<'shorter>(self) -> QuotaAssignmentActionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<QuotaAssignmentAction> for QuotaAssignmentActionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> QuotaAssignmentAction {
    let mut dst = QuotaAssignmentAction::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<QuotaAssignmentAction> for QuotaAssignmentActionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> QuotaAssignmentAction {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for QuotaAssignmentAction {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for QuotaAssignmentActionView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for QuotaAssignmentActionMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct QuotaAssignmentActionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, QuotaAssignmentAction>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for QuotaAssignmentActionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for QuotaAssignmentActionMut<'msg> {
  type Message = QuotaAssignmentAction;
}

impl ::std::fmt::Debug for QuotaAssignmentActionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, QuotaAssignmentAction>> for QuotaAssignmentActionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, QuotaAssignmentAction>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> QuotaAssignmentActionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, QuotaAssignmentAction> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> QuotaAssignmentAction {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // assignment_time_to_live: optional message google.protobuf.Duration
  pub fn has_assignment_time_to_live(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_assignment_time_to_live(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn assignment_time_to_live_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_assignment_time_to_live().then(|| self.assignment_time_to_live())
  }
  pub fn assignment_time_to_live(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn assignment_time_to_live_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_assignment_time_to_live(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // rate_limit_strategy: optional message envoy.type.v3.RateLimitStrategy
  pub fn has_rate_limit_strategy(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_rate_limit_strategy(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn rate_limit_strategy_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyView<'_>> {
    self.has_rate_limit_strategy().then(|| self.rate_limit_strategy())
  }
  pub fn rate_limit_strategy(&self) -> crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyView::default())
  }
  pub fn rate_limit_strategy_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyMut<'_> {
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
  pub fn set_rate_limit_strategy(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategy>) {

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
// - `QuotaAssignmentActionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for QuotaAssignmentActionMut<'_> {}

// SAFETY:
// - `QuotaAssignmentActionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for QuotaAssignmentActionMut<'_> {}

impl<'msg> ::protobuf::AsView for QuotaAssignmentActionMut<'msg> {
  type Proxied = QuotaAssignmentAction;
  fn as_view(&self) -> ::protobuf::View<'_, QuotaAssignmentAction> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for QuotaAssignmentActionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, QuotaAssignmentAction>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for QuotaAssignmentActionMut<'msg> {
  type MutProxied = QuotaAssignmentAction;
  fn as_mut(&mut self) -> QuotaAssignmentActionMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for QuotaAssignmentActionMut<'msg> {
  fn into_mut<'shorter>(self) -> QuotaAssignmentActionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl QuotaAssignmentAction {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, QuotaAssignmentAction> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> QuotaAssignmentActionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> QuotaAssignmentActionMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // assignment_time_to_live: optional message google.protobuf.Duration
  pub fn has_assignment_time_to_live(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_assignment_time_to_live(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn assignment_time_to_live_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_assignment_time_to_live().then(|| self.assignment_time_to_live())
  }
  pub fn assignment_time_to_live(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn assignment_time_to_live_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_assignment_time_to_live(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // rate_limit_strategy: optional message envoy.type.v3.RateLimitStrategy
  pub fn has_rate_limit_strategy(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_rate_limit_strategy(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn rate_limit_strategy_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyView<'_>> {
    self.has_rate_limit_strategy().then(|| self.rate_limit_strategy())
  }
  pub fn rate_limit_strategy(&self) -> crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyView::default())
  }
  pub fn rate_limit_strategy_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyMut<'_> {
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
  pub fn set_rate_limit_strategy(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategy>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl QuotaAssignmentAction

impl ::std::ops::Drop for QuotaAssignmentAction {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for QuotaAssignmentAction {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for QuotaAssignmentAction {
  type Proxied = Self;
  fn as_view(&self) -> QuotaAssignmentActionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for QuotaAssignmentAction {
  type MutProxied = Self;
  fn as_mut(&mut self) -> QuotaAssignmentActionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for QuotaAssignmentAction {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::rate_limit_quota_response::bucket_action::envoy__service__rate_0limit_0quota__v3__RateLimitQuotaResponse__BucketAction__QuotaAssignmentAction_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$a33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::rate_limit_quota_response::bucket_action::envoy__service__rate_0limit_0quota__v3__RateLimitQuotaResponse__BucketAction__QuotaAssignmentAction_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategy as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::rate_limit_quota_response::bucket_action::envoy__service__rate_0limit_0quota__v3__RateLimitQuotaResponse__BucketAction__QuotaAssignmentAction_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for QuotaAssignmentAction {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for QuotaAssignmentAction {
  type Msg = QuotaAssignmentAction;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<QuotaAssignmentAction> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for QuotaAssignmentAction {
  type Msg = QuotaAssignmentAction;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<QuotaAssignmentAction> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for QuotaAssignmentActionMut<'_> {
  type Msg = QuotaAssignmentAction;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<QuotaAssignmentAction> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for QuotaAssignmentActionMut<'_> {
  type Msg = QuotaAssignmentAction;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<QuotaAssignmentAction> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for QuotaAssignmentActionView<'_> {
  type Msg = QuotaAssignmentAction;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<QuotaAssignmentAction> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for QuotaAssignmentActionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__rate_0limit_0quota__v3__RateLimitQuotaResponse__BucketAction__AbandonAction_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AbandonAction {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AbandonAction>
}

impl ::protobuf::Message for AbandonAction {
  type MessageView<'msg> = AbandonActionView<'msg>;
  type MessageMut<'msg> = AbandonActionMut<'msg>;
}

impl ::std::default::Default for AbandonAction {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AbandonAction {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AbandonAction` is `Sync` because it does not implement interior mutability.
//    Neither does `AbandonActionMut`.
unsafe impl ::std::marker::Sync for AbandonAction {}

// SAFETY:
// - `AbandonAction` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for AbandonAction {}

impl ::protobuf::Proxied for AbandonAction {
  type View<'msg> = AbandonActionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AbandonAction {}

impl ::protobuf::MutProxied for AbandonAction {
  type Mut<'msg> = AbandonActionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AbandonActionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AbandonAction>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AbandonActionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AbandonActionView<'msg> {
  type Message = AbandonAction;
}

impl ::std::fmt::Debug for AbandonActionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AbandonActionView<'_> {
  fn default() -> AbandonActionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AbandonAction>> for AbandonActionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AbandonAction>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AbandonActionView<'msg> {

  pub fn to_owned(&self) -> AbandonAction {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `AbandonActionView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AbandonActionView<'_> {}

// SAFETY:
// - `AbandonActionView` is `Send` because while its alive a `AbandonActionMut` cannot.
// - `AbandonActionView` does not use thread-local data.
unsafe impl ::std::marker::Send for AbandonActionView<'_> {}

impl<'msg> ::protobuf::AsView for AbandonActionView<'msg> {
  type Proxied = AbandonAction;
  fn as_view(&self) -> ::protobuf::View<'msg, AbandonAction> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AbandonActionView<'msg> {
  fn into_view<'shorter>(self) -> AbandonActionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AbandonAction> for AbandonActionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AbandonAction {
    let mut dst = AbandonAction::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AbandonAction> for AbandonActionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AbandonAction {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for AbandonAction {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AbandonActionView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AbandonActionMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AbandonActionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AbandonAction>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AbandonActionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AbandonActionMut<'msg> {
  type Message = AbandonAction;
}

impl ::std::fmt::Debug for AbandonActionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AbandonAction>> for AbandonActionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AbandonAction>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AbandonActionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AbandonAction> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> AbandonAction {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `AbandonActionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AbandonActionMut<'_> {}

// SAFETY:
// - `AbandonActionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AbandonActionMut<'_> {}

impl<'msg> ::protobuf::AsView for AbandonActionMut<'msg> {
  type Proxied = AbandonAction;
  fn as_view(&self) -> ::protobuf::View<'_, AbandonAction> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AbandonActionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AbandonAction>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AbandonActionMut<'msg> {
  type MutProxied = AbandonAction;
  fn as_mut(&mut self) -> AbandonActionMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AbandonActionMut<'msg> {
  fn into_mut<'shorter>(self) -> AbandonActionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AbandonAction {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AbandonAction> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AbandonActionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AbandonActionMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl AbandonAction

impl ::std::ops::Drop for AbandonAction {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AbandonAction {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AbandonAction {
  type Proxied = Self;
  fn as_view(&self) -> AbandonActionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AbandonAction {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AbandonActionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AbandonAction {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::rate_limit_quota_response::bucket_action::envoy__service__rate_0limit_0quota__v3__RateLimitQuotaResponse__BucketAction__AbandonAction_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::rate_limit_quota_response::bucket_action::envoy__service__rate_0limit_0quota__v3__RateLimitQuotaResponse__BucketAction__AbandonAction_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::rate_limit_quota_response::bucket_action::envoy__service__rate_0limit_0quota__v3__RateLimitQuotaResponse__BucketAction__AbandonAction_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AbandonAction {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AbandonAction {
  type Msg = AbandonAction;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AbandonAction> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AbandonAction {
  type Msg = AbandonAction;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AbandonAction> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AbandonActionMut<'_> {
  type Msg = AbandonAction;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AbandonAction> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AbandonActionMut<'_> {
  type Msg = AbandonAction;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AbandonAction> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AbandonActionView<'_> {
  type Msg = AbandonAction;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AbandonAction> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AbandonActionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum BucketActionOneof<'msg> {
  QuotaAssignmentAction(::protobuf::View<'msg, super::super::super::rate_limit_quota_response::bucket_action::QuotaAssignmentAction>) = 2,
  AbandonAction(::protobuf::View<'msg, super::super::super::rate_limit_quota_response::bucket_action::AbandonAction>) = 3,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum BucketActionCase {
  QuotaAssignmentAction = 2,
  AbandonAction = 3,

  not_set = 0
}

impl BucketActionCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<BucketActionCase> {
    match v {
      0 => Some(BucketActionCase::not_set),
      2 => Some(BucketActionCase::QuotaAssignmentAction),
      3 => Some(BucketActionCase::AbandonAction),
      _ => None
    }
  }
}
}  // pub mod bucket_action


}  // pub mod rate_limit_quota_response


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__rate_0limit_0quota__v3__BucketId_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct BucketId {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<BucketId>
}

impl ::protobuf::Message for BucketId {
  type MessageView<'msg> = BucketIdView<'msg>;
  type MessageMut<'msg> = BucketIdMut<'msg>;
}

impl ::std::default::Default for BucketId {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for BucketId {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `BucketId` is `Sync` because it does not implement interior mutability.
//    Neither does `BucketIdMut`.
unsafe impl ::std::marker::Sync for BucketId {}

// SAFETY:
// - `BucketId` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for BucketId {}

impl ::protobuf::Proxied for BucketId {
  type View<'msg> = BucketIdView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for BucketId {}

impl ::protobuf::MutProxied for BucketId {
  type Mut<'msg> = BucketIdMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct BucketIdView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BucketId>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BucketIdView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for BucketIdView<'msg> {
  type Message = BucketId;
}

impl ::std::fmt::Debug for BucketIdView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for BucketIdView<'_> {
  fn default() -> BucketIdView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, BucketId>> for BucketIdView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BucketId>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BucketIdView<'msg> {

  pub fn to_owned(&self) -> BucketId {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // bucket: repeated message envoy.service.rate_limit_quota.v3.BucketId.BucketEntry
  pub fn bucket(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(0)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

}

// SAFETY:
// - `BucketIdView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for BucketIdView<'_> {}

// SAFETY:
// - `BucketIdView` is `Send` because while its alive a `BucketIdMut` cannot.
// - `BucketIdView` does not use thread-local data.
unsafe impl ::std::marker::Send for BucketIdView<'_> {}

impl<'msg> ::protobuf::AsView for BucketIdView<'msg> {
  type Proxied = BucketId;
  fn as_view(&self) -> ::protobuf::View<'msg, BucketId> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BucketIdView<'msg> {
  fn into_view<'shorter>(self) -> BucketIdView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<BucketId> for BucketIdView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BucketId {
    let mut dst = BucketId::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<BucketId> for BucketIdMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BucketId {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for BucketId {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BucketIdView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BucketIdMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct BucketIdMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BucketId>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BucketIdMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for BucketIdMut<'msg> {
  type Message = BucketId;
}

impl ::std::fmt::Debug for BucketIdMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, BucketId>> for BucketIdMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BucketId>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BucketIdMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, BucketId> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> BucketId {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // bucket: repeated message envoy.service.rate_limit_quota.v3.BucketId.BucketEntry
  pub fn bucket(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(0)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn bucket_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          0, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_bucket(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `BucketIdMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for BucketIdMut<'_> {}

// SAFETY:
// - `BucketIdMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for BucketIdMut<'_> {}

impl<'msg> ::protobuf::AsView for BucketIdMut<'msg> {
  type Proxied = BucketId;
  fn as_view(&self) -> ::protobuf::View<'_, BucketId> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BucketIdMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, BucketId>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for BucketIdMut<'msg> {
  type MutProxied = BucketId;
  fn as_mut(&mut self) -> BucketIdMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for BucketIdMut<'msg> {
  fn into_mut<'shorter>(self) -> BucketIdMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl BucketId {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, BucketId> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> BucketIdView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> BucketIdMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // bucket: repeated message envoy.service.rate_limit_quota.v3.BucketId.BucketEntry
  pub fn bucket(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(0)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn bucket_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          0, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_bucket(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl BucketId

impl ::std::ops::Drop for BucketId {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for BucketId {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for BucketId {
  type Proxied = Self;
  fn as_view(&self) -> BucketIdView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for BucketId {
  type MutProxied = Self;
  fn as_mut(&mut self) -> BucketIdMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for BucketId {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__rate_0limit_0quota__v3__BucketId_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__rate_0limit_0quota__v3__BucketId_msg_init.0, &[<super::bucket_id::BucketEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__rate_0limit_0quota__v3__BucketId_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BucketId {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BucketId {
  type Msg = BucketId;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BucketId> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BucketId {
  type Msg = BucketId;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BucketId> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BucketIdMut<'_> {
  type Msg = BucketId;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BucketId> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BucketIdMut<'_> {
  type Msg = BucketId;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BucketId> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BucketIdView<'_> {
  type Msg = BucketId;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BucketId> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BucketIdMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod bucket_id {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__rate_0limit_0quota__v3__BucketId__BucketEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct BucketEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for BucketEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::bucket_id::envoy__service__rate_0limit_0quota__v3__BucketId__BucketEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::bucket_id::envoy__service__rate_0limit_0quota__v3__BucketId__BucketEntry_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::bucket_id::envoy__service__rate_0limit_0quota__v3__BucketId__BucketEntry_msg_init.0)
      }).0
    }
  }
}

}  // pub mod bucket_id


