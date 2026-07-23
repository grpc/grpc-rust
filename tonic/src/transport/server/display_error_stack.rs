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

use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

pub(crate) struct DisplayErrorStack<'a>(pub(crate) &'a (dyn Error + 'static));

impl<'a> Display for DisplayErrorStack<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)?;
        let mut next = self.0.source();
        while let Some(err) = next {
            write!(f, ": {err}")?;
            next = err.source();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::transport::server::display_error_stack::DisplayErrorStack;
    use std::error::Error;
    use std::fmt;
    use std::fmt::{Display, Formatter};
    use std::sync::Arc;

    #[test]
    fn test_display_error_stack() {
        #[derive(Debug)]
        struct TestError(&'static str, Option<Arc<TestError>>);

        impl Display for TestError {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl Error for TestError {
            fn source(&self) -> Option<&(dyn Error + 'static)> {
                self.1.as_ref().map(|e| e as &(dyn Error + 'static))
            }
        }

        let a = Arc::new(TestError("a", None));
        let b = Arc::new(TestError("b", Some(a.clone())));
        let c = Arc::new(TestError("c", Some(b.clone())));

        assert_eq!("a", DisplayErrorStack(&a).to_string());
        assert_eq!("b: a", DisplayErrorStack(&b).to_string());
        assert_eq!("c: b: a", DisplayErrorStack(&c).to_string());
    }
}
