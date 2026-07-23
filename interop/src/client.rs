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

use crate::TestAssertion;
use tonic::async_trait;

#[async_trait]
pub trait InteropTest: Send {
    async fn empty_unary(&mut self, assertions: &mut Vec<TestAssertion>);

    async fn large_unary(&mut self, assertions: &mut Vec<TestAssertion>);

    async fn client_streaming(&mut self, assertions: &mut Vec<TestAssertion>);

    async fn server_streaming(&mut self, assertions: &mut Vec<TestAssertion>);

    async fn ping_pong(&mut self, assertions: &mut Vec<TestAssertion>);

    async fn empty_stream(&mut self, assertions: &mut Vec<TestAssertion>);

    async fn status_code_and_message(&mut self, assertions: &mut Vec<TestAssertion>);

    async fn special_status_message(&mut self, assertions: &mut Vec<TestAssertion>);

    async fn unimplemented_method(&mut self, assertions: &mut Vec<TestAssertion>);

    async fn custom_metadata(&mut self, assertions: &mut Vec<TestAssertion>);
}

#[async_trait]
pub trait InteropTestUnimplemented: Send {
    async fn unimplemented_service(&mut self, assertions: &mut Vec<TestAssertion>);
}
