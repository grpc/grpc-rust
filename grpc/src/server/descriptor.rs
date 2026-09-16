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

/// Pure metadata about a single gRPC method.
///
/// This is a data class — it carries no handler logic. It describes what a
/// method looks like (its path) without specifying how it's implemented.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct MethodDescriptor {
    /// Full method path, e.g., `"/helloworld.Greeter/SayHello"`.
    full_path: String,
}

impl MethodDescriptor {
    /// Creates a descriptor for the given method path.
    pub fn new(full_path: impl Into<String>) -> Self {
        Self {
            full_path: full_path.into(),
        }
    }

    /// Returns the full method path, e.g., `"/helloworld.Greeter/SayHello"`.
    pub fn full_path(&self) -> &str {
        &self.full_path
    }

    /// Consumes the descriptor, returning its owned full method path.
    pub fn into_full_path(self) -> String {
        self.full_path
    }
}

/// Pure metadata about a gRPC service.
///
/// This is a data class — it carries no handler logic. It describes what a
/// service looks like (its name and the methods it contains) without
/// specifying how they're implemented.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ServiceDescriptor {
    /// Fully qualified service name, e.g., `"helloworld.Greeter"`.
    name: String,
    /// Descriptors for all methods in this service.
    methods: Vec<MethodDescriptor>,
}

impl ServiceDescriptor {
    /// Creates a descriptor for the given service name and methods.
    pub fn new(name: impl Into<String>, methods: Vec<MethodDescriptor>) -> Self {
        Self {
            name: name.into(),
            methods,
        }
    }

    /// Returns the fully qualified service name, e.g., `"helloworld.Greeter"`.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the descriptors for all methods in this service.
    pub fn methods(&self) -> &[MethodDescriptor] {
        &self.methods
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_descriptor_exposes_path() {
        let desc = MethodDescriptor::new("/pkg.Svc/Method");
        assert_eq!(desc.full_path(), "/pkg.Svc/Method");
    }

    #[test]
    fn method_descriptor_new_accepts_string() {
        let desc = MethodDescriptor::new("/pkg.Svc/Method".to_string());
        assert_eq!(desc.full_path(), "/pkg.Svc/Method");
    }

    #[test]
    fn method_descriptor_into_full_path() {
        let desc = MethodDescriptor::new("/pkg.Svc/Method");
        assert_eq!(desc.into_full_path(), "/pkg.Svc/Method");
    }

    #[test]
    fn method_descriptor_clone() {
        let desc = MethodDescriptor::new("/pkg.Svc/Method");
        let cloned = desc.clone();
        assert_eq!(cloned.full_path(), desc.full_path());
    }

    #[test]
    fn service_descriptor_exposes_name_and_methods() {
        let desc = ServiceDescriptor::new(
            "pkg.Svc",
            vec![
                MethodDescriptor::new("/pkg.Svc/M1"),
                MethodDescriptor::new("/pkg.Svc/M2"),
            ],
        );
        assert_eq!(desc.name(), "pkg.Svc");
        assert_eq!(desc.methods().len(), 2);
        assert_eq!(desc.methods()[0].full_path(), "/pkg.Svc/M1");
    }

    #[test]
    fn service_descriptor_empty_methods() {
        let desc = ServiceDescriptor::new("pkg.Empty", vec![]);
        assert_eq!(desc.methods().len(), 0);
    }

    #[test]
    fn service_descriptor_clone() {
        let desc = ServiceDescriptor::new("pkg.Svc", vec![MethodDescriptor::new("/pkg.Svc/M1")]);
        let cloned = desc.clone();
        assert_eq!(cloned.name(), desc.name());
        assert_eq!(cloned.methods().len(), desc.methods().len());
    }
}
