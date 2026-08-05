/*
 *
 * Copyright 2026 gRPC authors.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to
 * deal in the Software without restriction, including without limitation the
 * rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
 * sell copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 *
 */

//! Core gRPC types common to clients and servers.
//!
//! This module provides the fundamental types used in gRPC communication, such
//! as message traits.
//!
//! Most applications should not need to use these types directly, as they are
//! typically used by generated code.  However, they may be necessary when
//! implementing custom interceptors or advanced features.
//!
//! # Key Concepts
//!
//! - **[`SendMessage`] / [`RecvMessage`]:** Traits for encoding and decoding
//!   messages.

use std::any::TypeId;

use bytes::Buf;

/// Represents a message sent by either a client or a server.
#[allow(unused)]
pub trait SendMessage: Send + Sync {
    /// Encodes the message (`self`) as binary data.
    fn encode(&self) -> Result<Box<dyn Buf + Send + Sync>, String>;

    #[doc(hidden)]
    unsafe fn _ptr_for(&self, id: TypeId) -> Option<*const ()> {
        None
    }
}

/// Represents a message received by either a client or a server.
#[allow(unused)]
pub trait RecvMessage: Send + Sync {
    /// Encodes `data` into `self`.
    fn decode(&mut self, data: &mut dyn Buf) -> Result<(), String>;

    #[doc(hidden)]
    unsafe fn _ptr_for(&mut self, id: TypeId) -> Option<*mut ()> {
        None
    }
}

/// Describes what underlying message is inside a [`SendMessage`] or
/// [`RecvMessage`] so that it can be downcast, e.g. by interceptors.
///
/// Allows for safe downcasting to views containing a lifetime.
pub trait MessageType {
    /// The message view's type, which may have a lifetime.
    type Target<'a>;
}

fn msg_type_id<T: MessageType>() -> TypeId
where
    T::Target<'static>: 'static,
{
    TypeId::of::<T::Target<'static>>()
}

impl dyn SendMessage + '_ {
    /// Downcasts the SendMessage to T::Target if the SendMessage contains a T.
    pub fn downcast_ref<T: MessageType>(&self) -> Option<&T::Target<'_>>
    where
        T::Target<'static>: 'static,
    {
        unsafe {
            if let Some(ptr) = self._ptr_for(msg_type_id::<T>()) {
                Some(&*(ptr as *mut T::Target<'_>))
            } else {
                None
            }
        }
    }
}

#[allow(unused)]
impl dyn RecvMessage + '_ {
    /// Downcasts the RecvMessage to T::Target if the RecvMessage contains a T.
    pub fn downcast_mut<T: MessageType>(&mut self) -> Option<&mut T::Target<'_>>
    where
        T::Target<'static>: 'static,
    {
        unsafe {
            if let Some(ptr) = self._ptr_for(msg_type_id::<T>()) {
                Some(&mut *(ptr as *mut T::Target<'_>))
            } else {
                None
            }
        }
    }
}
