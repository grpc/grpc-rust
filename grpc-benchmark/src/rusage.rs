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

#[derive(Debug)]
pub(crate) struct Rusage {
    user_time_ns: i64,
    system_time_ns: i64,
}

impl Rusage {
    #[cfg(unix)]
    pub(crate) fn now() -> Result<Self, String> {
        use nix::sys::resource::UsageWho;
        use nix::sys::resource::getrusage;
        use nix::sys::time::TimeValLike;

        let usage =
            getrusage(UsageWho::RUSAGE_SELF).map_err(|e| format!("failed to get rusage: {}", e))?;

        Ok(Rusage {
            user_time_ns: usage.user_time().num_nanoseconds(),
            system_time_ns: usage.system_time().num_nanoseconds(),
        })
    }

    #[cfg(not(unix))]
    pub(crate) fn now() -> Result<Rusage, String> {
        Ok(Rusage {
            user_time_ns: 0,
            system_time_ns: 0,
        })
    }

    pub(crate) fn user_time_nanos(&self) -> i64 {
        self.user_time_ns
    }

    pub(crate) fn system_time_nanos(&self) -> i64 {
        self.system_time_ns
    }
}
