const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__JsonFormatOptions_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct JsonFormatOptions {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<JsonFormatOptions>
}

impl ::protobuf::Message for JsonFormatOptions {
  type MessageView<'msg> = JsonFormatOptionsView<'msg>;
  type MessageMut<'msg> = JsonFormatOptionsMut<'msg>;
}

impl ::std::default::Default for JsonFormatOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for JsonFormatOptions {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `JsonFormatOptions` is `Sync` because it does not implement interior mutability.
//    Neither does `JsonFormatOptionsMut`.
unsafe impl ::std::marker::Sync for JsonFormatOptions {}

// SAFETY:
// - `JsonFormatOptions` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for JsonFormatOptions {}

impl ::protobuf::Proxied for JsonFormatOptions {
  type View<'msg> = JsonFormatOptionsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for JsonFormatOptions {}

impl ::protobuf::MutProxied for JsonFormatOptions {
  type Mut<'msg> = JsonFormatOptionsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct JsonFormatOptionsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, JsonFormatOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for JsonFormatOptionsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for JsonFormatOptionsView<'msg> {
  type Message = JsonFormatOptions;
}

impl ::std::fmt::Debug for JsonFormatOptionsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for JsonFormatOptionsView<'_> {
  fn default() -> JsonFormatOptionsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, JsonFormatOptions>> for JsonFormatOptionsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, JsonFormatOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> JsonFormatOptionsView<'msg> {

  pub fn to_owned(&self) -> JsonFormatOptions {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // sort_properties: optional bool
  pub fn sort_properties(self) -> bool {
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
// - `JsonFormatOptionsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for JsonFormatOptionsView<'_> {}

// SAFETY:
// - `JsonFormatOptionsView` is `Send` because while its alive a `JsonFormatOptionsMut` cannot.
// - `JsonFormatOptionsView` does not use thread-local data.
unsafe impl ::std::marker::Send for JsonFormatOptionsView<'_> {}

impl<'msg> ::protobuf::AsView for JsonFormatOptionsView<'msg> {
  type Proxied = JsonFormatOptions;
  fn as_view(&self) -> ::protobuf::View<'msg, JsonFormatOptions> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for JsonFormatOptionsView<'msg> {
  fn into_view<'shorter>(self) -> JsonFormatOptionsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<JsonFormatOptions> for JsonFormatOptionsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> JsonFormatOptions {
    let mut dst = JsonFormatOptions::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<JsonFormatOptions> for JsonFormatOptionsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> JsonFormatOptions {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for JsonFormatOptions {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for JsonFormatOptionsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for JsonFormatOptionsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct JsonFormatOptionsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, JsonFormatOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for JsonFormatOptionsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for JsonFormatOptionsMut<'msg> {
  type Message = JsonFormatOptions;
}

impl ::std::fmt::Debug for JsonFormatOptionsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, JsonFormatOptions>> for JsonFormatOptionsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, JsonFormatOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> JsonFormatOptionsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, JsonFormatOptions> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> JsonFormatOptions {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // sort_properties: optional bool
  pub fn sort_properties(&self) -> bool {
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
  pub fn set_sort_properties(&mut self, val: bool) {
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
// - `JsonFormatOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for JsonFormatOptionsMut<'_> {}

// SAFETY:
// - `JsonFormatOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for JsonFormatOptionsMut<'_> {}

impl<'msg> ::protobuf::AsView for JsonFormatOptionsMut<'msg> {
  type Proxied = JsonFormatOptions;
  fn as_view(&self) -> ::protobuf::View<'_, JsonFormatOptions> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for JsonFormatOptionsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, JsonFormatOptions>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for JsonFormatOptionsMut<'msg> {
  type MutProxied = JsonFormatOptions;
  fn as_mut(&mut self) -> JsonFormatOptionsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for JsonFormatOptionsMut<'msg> {
  fn into_mut<'shorter>(self) -> JsonFormatOptionsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl JsonFormatOptions {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, JsonFormatOptions> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> JsonFormatOptionsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> JsonFormatOptionsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // sort_properties: optional bool
  pub fn sort_properties(&self) -> bool {
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
  pub fn set_sort_properties(&mut self, val: bool) {
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

}  // impl JsonFormatOptions

impl ::std::ops::Drop for JsonFormatOptions {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for JsonFormatOptions {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for JsonFormatOptions {
  type Proxied = Self;
  fn as_view(&self) -> JsonFormatOptionsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for JsonFormatOptions {
  type MutProxied = Self;
  fn as_mut(&mut self) -> JsonFormatOptionsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for JsonFormatOptions {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__JsonFormatOptions_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__JsonFormatOptions_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__JsonFormatOptions_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for JsonFormatOptions {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for JsonFormatOptions {
  type Msg = JsonFormatOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JsonFormatOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JsonFormatOptions {
  type Msg = JsonFormatOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JsonFormatOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for JsonFormatOptionsMut<'_> {
  type Msg = JsonFormatOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JsonFormatOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JsonFormatOptionsMut<'_> {
  type Msg = JsonFormatOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JsonFormatOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JsonFormatOptionsView<'_> {
  type Msg = JsonFormatOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JsonFormatOptions> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for JsonFormatOptionsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__SubstitutionFormatString_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SubstitutionFormatString {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SubstitutionFormatString>
}

impl ::protobuf::Message for SubstitutionFormatString {
  type MessageView<'msg> = SubstitutionFormatStringView<'msg>;
  type MessageMut<'msg> = SubstitutionFormatStringMut<'msg>;
}

impl ::std::default::Default for SubstitutionFormatString {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SubstitutionFormatString {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SubstitutionFormatString` is `Sync` because it does not implement interior mutability.
//    Neither does `SubstitutionFormatStringMut`.
unsafe impl ::std::marker::Sync for SubstitutionFormatString {}

// SAFETY:
// - `SubstitutionFormatString` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SubstitutionFormatString {}

impl ::protobuf::Proxied for SubstitutionFormatString {
  type View<'msg> = SubstitutionFormatStringView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SubstitutionFormatString {}

impl ::protobuf::MutProxied for SubstitutionFormatString {
  type Mut<'msg> = SubstitutionFormatStringMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SubstitutionFormatStringView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SubstitutionFormatString>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SubstitutionFormatStringView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SubstitutionFormatStringView<'msg> {
  type Message = SubstitutionFormatString;
}

impl ::std::fmt::Debug for SubstitutionFormatStringView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SubstitutionFormatStringView<'_> {
  fn default() -> SubstitutionFormatStringView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SubstitutionFormatString>> for SubstitutionFormatStringView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SubstitutionFormatString>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SubstitutionFormatStringView<'msg> {

  pub fn to_owned(&self) -> SubstitutionFormatString {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // text_format: optional string
  pub fn has_text_format(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn text_format_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_text_format().then(|| self.text_format())
  }
  pub fn text_format(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // json_format: optional message google.protobuf.Struct
  pub fn has_json_format(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn json_format_opt(self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'msg>> {
    self.has_json_format().then(|| self.json_format())
  }
  pub fn json_format(self) -> ::protobuf_well_known_types::StructView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }

  // text_format_source: optional message envoy.config.core.v3.DataSource
  pub fn has_text_format_source(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn text_format_source_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'msg>> {
    self.has_text_format_source().then(|| self.text_format_source())
  }
  pub fn text_format_source(self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }

  // omit_empty_values: optional bool
  pub fn omit_empty_values(self) -> bool {
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

  // content_type: optional string
  pub fn content_type(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // formatters: repeated message envoy.config.core.v3.TypedExtensionConfig
  pub fn formatters(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // json_format_options: optional message envoy.config.core.v3.JsonFormatOptions
  pub fn has_json_format_options(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn json_format_options_opt(self) -> ::std::option::Option<super::JsonFormatOptionsView<'msg>> {
    self.has_json_format_options().then(|| self.json_format_options())
  }
  pub fn json_format_options(self) -> super::JsonFormatOptionsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::JsonFormatOptionsView::default())
  }

  pub fn format(self) -> super::substitution_format_string::FormatOneof<'msg> {
    match self.format_case() {
      super::substitution_format_string::FormatCase::TextFormat =>
          super::substitution_format_string::FormatOneof::TextFormat(self.text_format()),
      super::substitution_format_string::FormatCase::JsonFormat =>
          super::substitution_format_string::FormatOneof::JsonFormat(self.json_format()),
      super::substitution_format_string::FormatCase::TextFormatSource =>
          super::substitution_format_string::FormatOneof::TextFormatSource(self.text_format_source()),
      _ => super::substitution_format_string::FormatOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn format_case(self) -> super::substitution_format_string::FormatCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::substitution_format_string::FormatCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `SubstitutionFormatStringView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SubstitutionFormatStringView<'_> {}

// SAFETY:
// - `SubstitutionFormatStringView` is `Send` because while its alive a `SubstitutionFormatStringMut` cannot.
// - `SubstitutionFormatStringView` does not use thread-local data.
unsafe impl ::std::marker::Send for SubstitutionFormatStringView<'_> {}

impl<'msg> ::protobuf::AsView for SubstitutionFormatStringView<'msg> {
  type Proxied = SubstitutionFormatString;
  fn as_view(&self) -> ::protobuf::View<'msg, SubstitutionFormatString> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SubstitutionFormatStringView<'msg> {
  fn into_view<'shorter>(self) -> SubstitutionFormatStringView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SubstitutionFormatString> for SubstitutionFormatStringView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SubstitutionFormatString {
    let mut dst = SubstitutionFormatString::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SubstitutionFormatString> for SubstitutionFormatStringMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SubstitutionFormatString {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SubstitutionFormatString {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SubstitutionFormatStringView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SubstitutionFormatStringMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SubstitutionFormatStringMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SubstitutionFormatString>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SubstitutionFormatStringMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SubstitutionFormatStringMut<'msg> {
  type Message = SubstitutionFormatString;
}

impl ::std::fmt::Debug for SubstitutionFormatStringMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SubstitutionFormatString>> for SubstitutionFormatStringMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SubstitutionFormatString>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SubstitutionFormatStringMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SubstitutionFormatString> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SubstitutionFormatString {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // text_format: optional string
  pub fn has_text_format(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_text_format(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn text_format_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_text_format().then(|| self.text_format())
  }
  pub fn text_format(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_text_format(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // json_format: optional message google.protobuf.Struct
  pub fn has_json_format(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_json_format(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn json_format_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_json_format().then(|| self.json_format())
  }
  pub fn json_format(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn json_format_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
  pub fn set_json_format(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // text_format_source: optional message envoy.config.core.v3.DataSource
  pub fn has_text_format_source(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_text_format_source(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn text_format_source_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_>> {
    self.has_text_format_source().then(|| self.text_format_source())
  }
  pub fn text_format_source(&self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }
  pub fn text_format_source_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceMut<'_> {
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
  pub fn set_text_format_source(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::DataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // omit_empty_values: optional bool
  pub fn omit_empty_values(&self) -> bool {
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
  pub fn set_omit_empty_values(&mut self, val: bool) {
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

  // content_type: optional string
  pub fn content_type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_content_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // formatters: repeated message envoy.config.core.v3.TypedExtensionConfig
  pub fn formatters(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn formatters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
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
  pub fn set_formatters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // json_format_options: optional message envoy.config.core.v3.JsonFormatOptions
  pub fn has_json_format_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_json_format_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn json_format_options_opt(&self) -> ::std::option::Option<super::JsonFormatOptionsView<'_>> {
    self.has_json_format_options().then(|| self.json_format_options())
  }
  pub fn json_format_options(&self) -> super::JsonFormatOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::JsonFormatOptionsView::default())
  }
  pub fn json_format_options_mut(&mut self) -> super::JsonFormatOptionsMut<'_> {
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
  pub fn set_json_format_options(&mut self,
    val: impl ::protobuf::IntoProxied<super::JsonFormatOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  pub fn format(&self) -> super::substitution_format_string::FormatOneof<'_> {
    match &self.format_case() {
      super::substitution_format_string::FormatCase::TextFormat =>
          super::substitution_format_string::FormatOneof::TextFormat(self.text_format()),
      super::substitution_format_string::FormatCase::JsonFormat =>
          super::substitution_format_string::FormatOneof::JsonFormat(self.json_format()),
      super::substitution_format_string::FormatCase::TextFormatSource =>
          super::substitution_format_string::FormatOneof::TextFormatSource(self.text_format_source()),
      _ => super::substitution_format_string::FormatOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn format_case(&self) -> super::substitution_format_string::FormatCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::substitution_format_string::FormatCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `SubstitutionFormatStringMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SubstitutionFormatStringMut<'_> {}

// SAFETY:
// - `SubstitutionFormatStringMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SubstitutionFormatStringMut<'_> {}

impl<'msg> ::protobuf::AsView for SubstitutionFormatStringMut<'msg> {
  type Proxied = SubstitutionFormatString;
  fn as_view(&self) -> ::protobuf::View<'_, SubstitutionFormatString> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SubstitutionFormatStringMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SubstitutionFormatString>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SubstitutionFormatStringMut<'msg> {
  type MutProxied = SubstitutionFormatString;
  fn as_mut(&mut self) -> SubstitutionFormatStringMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SubstitutionFormatStringMut<'msg> {
  fn into_mut<'shorter>(self) -> SubstitutionFormatStringMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SubstitutionFormatString {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SubstitutionFormatString> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SubstitutionFormatStringView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SubstitutionFormatStringMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // text_format: optional string
  pub fn has_text_format(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_text_format(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn text_format_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_text_format().then(|| self.text_format())
  }
  pub fn text_format(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_text_format(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // json_format: optional message google.protobuf.Struct
  pub fn has_json_format(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_json_format(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn json_format_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_json_format().then(|| self.json_format())
  }
  pub fn json_format(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn json_format_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
  pub fn set_json_format(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // text_format_source: optional message envoy.config.core.v3.DataSource
  pub fn has_text_format_source(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_text_format_source(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn text_format_source_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_>> {
    self.has_text_format_source().then(|| self.text_format_source())
  }
  pub fn text_format_source(&self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }
  pub fn text_format_source_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceMut<'_> {
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
  pub fn set_text_format_source(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::DataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // omit_empty_values: optional bool
  pub fn omit_empty_values(&self) -> bool {
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
  pub fn set_omit_empty_values(&mut self, val: bool) {
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

  // content_type: optional string
  pub fn content_type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_content_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // formatters: repeated message envoy.config.core.v3.TypedExtensionConfig
  pub fn formatters(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn formatters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
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
  pub fn set_formatters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // json_format_options: optional message envoy.config.core.v3.JsonFormatOptions
  pub fn has_json_format_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_json_format_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn json_format_options_opt(&self) -> ::std::option::Option<super::JsonFormatOptionsView<'_>> {
    self.has_json_format_options().then(|| self.json_format_options())
  }
  pub fn json_format_options(&self) -> super::JsonFormatOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::JsonFormatOptionsView::default())
  }
  pub fn json_format_options_mut(&mut self) -> super::JsonFormatOptionsMut<'_> {
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
  pub fn set_json_format_options(&mut self,
    val: impl ::protobuf::IntoProxied<super::JsonFormatOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  pub fn format(&self) -> super::substitution_format_string::FormatOneof<'_> {
    match &self.format_case() {
      super::substitution_format_string::FormatCase::TextFormat =>
          super::substitution_format_string::FormatOneof::TextFormat(self.text_format()),
      super::substitution_format_string::FormatCase::JsonFormat =>
          super::substitution_format_string::FormatOneof::JsonFormat(self.json_format()),
      super::substitution_format_string::FormatCase::TextFormatSource =>
          super::substitution_format_string::FormatOneof::TextFormatSource(self.text_format_source()),
      _ => super::substitution_format_string::FormatOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn format_case(&self) -> super::substitution_format_string::FormatCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::substitution_format_string::FormatCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl SubstitutionFormatString

impl ::std::ops::Drop for SubstitutionFormatString {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SubstitutionFormatString {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SubstitutionFormatString {
  type Proxied = Self;
  fn as_view(&self) -> SubstitutionFormatStringView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SubstitutionFormatString {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SubstitutionFormatStringMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SubstitutionFormatString {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__SubstitutionFormatString_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1T3/P1X3G3^!|#|&");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__SubstitutionFormatString_msg_init.0, &[<::protobuf_well_known_types::Struct as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::DataSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::JsonFormatOptions as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__SubstitutionFormatString_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SubstitutionFormatString {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SubstitutionFormatString {
  type Msg = SubstitutionFormatString;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SubstitutionFormatString> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SubstitutionFormatString {
  type Msg = SubstitutionFormatString;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SubstitutionFormatString> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SubstitutionFormatStringMut<'_> {
  type Msg = SubstitutionFormatString;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SubstitutionFormatString> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SubstitutionFormatStringMut<'_> {
  type Msg = SubstitutionFormatString;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SubstitutionFormatString> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SubstitutionFormatStringView<'_> {
  type Msg = SubstitutionFormatString;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SubstitutionFormatString> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SubstitutionFormatStringMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod substitution_format_string {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum FormatOneof<'msg> {
  TextFormat(&'msg ::protobuf::ProtoStr) = 1,
  JsonFormat(::protobuf::View<'msg, ::protobuf_well_known_types::Struct>) = 2,
  TextFormatSource(::protobuf::View<'msg, crate::xds::generated::envoy::config::core::v3::base::DataSource>) = 5,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum FormatCase {
  TextFormat = 1,
  JsonFormat = 2,
  TextFormatSource = 5,

  not_set = 0
}

impl FormatCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<FormatCase> {
    match v {
      0 => Some(FormatCase::not_set),
      1 => Some(FormatCase::TextFormat),
      2 => Some(FormatCase::JsonFormat),
      5 => Some(FormatCase::TextFormatSource),
      _ => None
    }
  }
}
}  // pub mod substitution_format_string


