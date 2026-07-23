/*
 *
 * Copyright 2025 gRPC authors.
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

use super::super::std_messages::{
    BadRequest, DebugInfo, ErrorInfo, Help, LocalizedMessage, PreconditionFailure, QuotaFailure,
    RequestInfo, ResourceInfo, RetryInfo,
};

/// Wraps the structs corresponding to the standard error messages, allowing
/// the implementation and handling of vectors containing any of them.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum ErrorDetail {
    /// Wraps the [`RetryInfo`] struct.
    RetryInfo(RetryInfo),

    /// Wraps the [`DebugInfo`] struct.
    DebugInfo(DebugInfo),

    /// Wraps the [`QuotaFailure`] struct.
    QuotaFailure(QuotaFailure),

    /// Wraps the [`ErrorInfo`] struct.
    ErrorInfo(ErrorInfo),

    /// Wraps the [`PreconditionFailure`] struct.
    PreconditionFailure(PreconditionFailure),

    /// Wraps the [`BadRequest`] struct.
    BadRequest(BadRequest),

    /// Wraps the [`RequestInfo`] struct.
    RequestInfo(RequestInfo),

    /// Wraps the [`ResourceInfo`] struct.
    ResourceInfo(ResourceInfo),

    /// Wraps the [`Help`] struct.
    Help(Help),

    /// Wraps the [`LocalizedMessage`] struct.
    LocalizedMessage(LocalizedMessage),
}

impl From<RetryInfo> for ErrorDetail {
    fn from(err_detail: RetryInfo) -> Self {
        ErrorDetail::RetryInfo(err_detail)
    }
}

impl From<DebugInfo> for ErrorDetail {
    fn from(err_detail: DebugInfo) -> Self {
        ErrorDetail::DebugInfo(err_detail)
    }
}

impl From<QuotaFailure> for ErrorDetail {
    fn from(err_detail: QuotaFailure) -> Self {
        ErrorDetail::QuotaFailure(err_detail)
    }
}

impl From<ErrorInfo> for ErrorDetail {
    fn from(err_detail: ErrorInfo) -> Self {
        ErrorDetail::ErrorInfo(err_detail)
    }
}

impl From<PreconditionFailure> for ErrorDetail {
    fn from(err_detail: PreconditionFailure) -> Self {
        ErrorDetail::PreconditionFailure(err_detail)
    }
}

impl From<BadRequest> for ErrorDetail {
    fn from(err_detail: BadRequest) -> Self {
        ErrorDetail::BadRequest(err_detail)
    }
}

impl From<RequestInfo> for ErrorDetail {
    fn from(err_detail: RequestInfo) -> Self {
        ErrorDetail::RequestInfo(err_detail)
    }
}

impl From<ResourceInfo> for ErrorDetail {
    fn from(err_detail: ResourceInfo) -> Self {
        ErrorDetail::ResourceInfo(err_detail)
    }
}

impl From<Help> for ErrorDetail {
    fn from(err_detail: Help) -> Self {
        ErrorDetail::Help(err_detail)
    }
}

impl From<LocalizedMessage> for ErrorDetail {
    fn from(err_detail: LocalizedMessage) -> Self {
        ErrorDetail::LocalizedMessage(err_detail)
    }
}
