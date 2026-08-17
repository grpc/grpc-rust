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

use std::collections::HashSet;

use super::{Attributes, Method, Service};
use crate::{
    body_type_token, box_future_token, box_stream_token, format_method_name, format_method_path,
    format_service_name, generate_doc_comment, generate_doc_comments, naive_snake_case,
    server_grpc_token, streaming_type_token, wrapper_token,
};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, Lit, LitStr};

#[allow(clippy::too_many_arguments)]
pub(crate) fn generate_internal<T: Service>(
    service: &T,
    emit_package: bool,
    proto_path: &str,
    compile_well_known_types: bool,
    attributes: &Attributes,
    disable_comments: &HashSet<String>,
    use_arc_self: bool,
    generate_default_stubs: bool,
    local: bool,
) -> TokenStream {
    let methods = generate_methods(
        service,
        emit_package,
        proto_path,
        compile_well_known_types,
        use_arc_self,
        generate_default_stubs,
        local,
    );

    let server_service = quote::format_ident!("{}Server", service.name());
    let server_trait = quote::format_ident!("{}", service.name());
    let server_mod = quote::format_ident!("{}_server", naive_snake_case(service.name()));
    let trait_attributes = attributes.for_trait(service.name());
    let generated_trait = generate_trait(
        service,
        emit_package,
        proto_path,
        compile_well_known_types,
        server_trait.clone(),
        disable_comments,
        use_arc_self,
        generate_default_stubs,
        trait_attributes,
        local,
    );
    let package = if emit_package { service.package() } else { "" };
    // Transport based implementations
    let service_name = format_service_name(service, emit_package);

    let service_doc = if disable_comments.contains(&service_name) {
        TokenStream::new()
    } else {
        generate_doc_comments(service.comment())
    };

    let named = generate_named(&server_service, &service_name);
    let mod_attributes = attributes.for_mod(package);
    let struct_attributes = attributes.for_struct(&service_name);

    let configure_compression_methods = quote! {
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }

        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    };

    let configure_max_message_size_methods = quote! {
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }

        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    };

    let wrapper = wrapper_token(local);
    let from_ident = if local {
        quote::format_ident!("from_rc")
    } else {
        quote::format_ident!("from_arc")
    };
    let body_type = body_type_token(local);
    let box_future = box_future_token(local);
    let body_bounds = if local {
        quote! {
            B: Body + 'static,
            B::Error: Into<StdError> + 'static,
        }
    } else {
        quote! {
            B: Body + std::marker::Send + 'static,
            B::Error: Into<StdError> + std::marker::Send + 'static,
        }
    };

    quote! {
        /// Generated server implementations.
        #(#mod_attributes)*
        pub mod #server_mod {
            #![allow(
                unused_variables,
                dead_code,
                missing_docs,
                clippy::wildcard_imports,
                // will trigger if compression is disabled
                clippy::let_unit_value,
            )]
            use tonic::codegen::*;

            #generated_trait

            #service_doc
            #(#struct_attributes)*
            #[derive(Debug)]
            pub struct #server_service<T> {
                inner: #wrapper<T>,
                accept_compression_encodings: EnabledCompressionEncodings,
                send_compression_encodings: EnabledCompressionEncodings,
                max_decoding_message_size: Option<usize>,
                max_encoding_message_size: Option<usize>,
            }

            impl<T> #server_service<T> {
                pub fn new(inner: T) -> Self {
                    Self::#from_ident(#wrapper::new(inner))
                }

                pub fn #from_ident(inner: #wrapper<T>) -> Self {
                    Self {
                        inner,
                        accept_compression_encodings: Default::default(),
                        send_compression_encodings: Default::default(),
                        max_decoding_message_size: None,
                        max_encoding_message_size: None,
                    }
                }

                pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
                where
                    F: tonic::service::Interceptor,
                {
                    InterceptedService::new(Self::new(inner), interceptor)
                }

                #configure_compression_methods

                #configure_max_message_size_methods
            }

            impl<T, B> tonic::codegen::Service<http::Request<B>> for #server_service<T>
                where
                    T: #server_trait,
                    #body_bounds
            {
                type Response = http::Response<#body_type>;
                type Error = std::convert::Infallible;
                type Future = #box_future<Self::Response, Self::Error>;

                fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<std::result::Result<(), Self::Error>> {
                    Poll::Ready(Ok(()))
                }

                fn call(&mut self, req: http::Request<B>) -> Self::Future {
                    match req.uri().path() {
                        #methods

                        _ => Box::pin(async move {
                            let mut response = http::Response::new(#body_type::default());
                            let headers = response.headers_mut();
                            headers.insert(tonic::Status::GRPC_STATUS, (tonic::Code::Unimplemented as i32).into());
                            headers.insert(http::header::CONTENT_TYPE, tonic::metadata::GRPC_CONTENT_TYPE);
                            Ok(response)
                        }),
                    }
                }
            }

            impl<T> Clone for #server_service<T> {
                fn clone(&self) -> Self {
                    let inner = self.inner.clone();
                    Self {
                        inner,
                        accept_compression_encodings: self.accept_compression_encodings,
                        send_compression_encodings: self.send_compression_encodings,
                        max_decoding_message_size: self.max_decoding_message_size,
                        max_encoding_message_size: self.max_encoding_message_size,
                    }
                }
            }

            #named
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn generate_trait<T: Service>(
    service: &T,
    emit_package: bool,
    proto_path: &str,
    compile_well_known_types: bool,
    server_trait: Ident,
    disable_comments: &HashSet<String>,
    use_arc_self: bool,
    generate_default_stubs: bool,
    trait_attributes: Vec<syn::Attribute>,
    local: bool,
) -> TokenStream {
    let methods = generate_trait_methods(
        service,
        emit_package,
        proto_path,
        compile_well_known_types,
        disable_comments,
        use_arc_self,
        generate_default_stubs,
        local,
    );
    let trait_doc = generate_doc_comment(format!(
        " Generated trait containing gRPC methods that should be implemented for use with {}Server.",
        service.name()
    ));
    let async_trait_attr = if local {
        quote!(#[async_trait(?Send)])
    } else {
        quote!(#[async_trait])
    };
    let supertraits = if local {
        quote!('static)
    } else {
        quote!(std::marker::Send + std::marker::Sync + 'static)
    };

    quote! {
        #trait_doc
        #(#trait_attributes)*
        #async_trait_attr
        pub trait #server_trait : #supertraits {
            #methods
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn generate_trait_methods<T: Service>(
    service: &T,
    emit_package: bool,
    proto_path: &str,
    compile_well_known_types: bool,
    disable_comments: &HashSet<String>,
    use_arc_self: bool,
    generate_default_stubs: bool,
    local: bool,
) -> TokenStream {
    let mut stream = TokenStream::new();
    let streaming = streaming_type_token(local);
    let box_stream = box_stream_token(local);
    let stream_bound = if local {
        quote!('static)
    } else {
        quote!(std::marker::Send + 'static)
    };

    for method in service.methods() {
        let name = quote::format_ident!("{}", method.name());

        let (req_message, res_message) =
            method.request_response_name(proto_path, compile_well_known_types);

        let method_doc =
            if disable_comments.contains(&format_method_name(service, method, emit_package)) {
                TokenStream::new()
            } else {
                generate_doc_comments(method.comment())
            };

        let self_param = if use_arc_self {
            quote!(self: std::sync::Arc<Self>)
        } else {
            quote!(&self)
        };

        let method = match (
            method.client_streaming(),
            method.server_streaming(),
            generate_default_stubs,
        ) {
            (false, false, true) => {
                quote! {
                    #method_doc
                    async fn #name(#self_param, request: tonic::Request<#req_message>)
                        -> std::result::Result<tonic::Response<#res_message>, tonic::Status> {
                        Err(tonic::Status::unimplemented("Not yet implemented"))
                    }
                }
            }
            (false, false, false) => {
                quote! {
                    #method_doc
                    async fn #name(#self_param, request: tonic::Request<#req_message>)
                        -> std::result::Result<tonic::Response<#res_message>, tonic::Status>;
                }
            }
            (true, false, true) => {
                quote! {
                    #method_doc
                    async fn #name(#self_param, request: tonic::Request<#streaming<#req_message>>)
                        -> std::result::Result<tonic::Response<#res_message>, tonic::Status> {
                        Err(tonic::Status::unimplemented("Not yet implemented"))
                    }
                }
            }
            (true, false, false) => {
                quote! {
                    #method_doc
                    async fn #name(#self_param, request: tonic::Request<#streaming<#req_message>>)
                        -> std::result::Result<tonic::Response<#res_message>, tonic::Status>;
                }
            }
            (false, true, true) => {
                quote! {
                    #method_doc
                    async fn #name(#self_param, request: tonic::Request<#req_message>)
                        -> std::result::Result<tonic::Response<#box_stream<#res_message>>, tonic::Status> {
                        Err(tonic::Status::unimplemented("Not yet implemented"))
                    }
                }
            }
            (false, true, false) => {
                let stream = quote::format_ident!("{}Stream", method.identifier());
                let stream_doc = generate_doc_comment(format!(
                    " Server streaming response type for the {} method.",
                    method.identifier()
                ));

                quote! {
                    #stream_doc
                    type #stream: tonic::codegen::tokio_stream::Stream<Item = std::result::Result<#res_message, tonic::Status>> + #stream_bound;

                    #method_doc
                    async fn #name(#self_param, request: tonic::Request<#req_message>)
                        -> std::result::Result<tonic::Response<Self::#stream>, tonic::Status>;
                }
            }
            (true, true, true) => {
                quote! {
                    #method_doc
                    async fn #name(#self_param, request: tonic::Request<#streaming<#req_message>>)
                        -> std::result::Result<tonic::Response<#box_stream<#res_message>>, tonic::Status> {
                        Err(tonic::Status::unimplemented("Not yet implemented"))
                    }
                }
            }
            (true, true, false) => {
                let stream = quote::format_ident!("{}Stream", method.identifier());
                let stream_doc = generate_doc_comment(format!(
                    " Server streaming response type for the {} method.",
                    method.identifier()
                ));

                quote! {
                    #stream_doc
                    type #stream: tonic::codegen::tokio_stream::Stream<Item = std::result::Result<#res_message, tonic::Status>> + #stream_bound;

                    #method_doc
                    async fn #name(#self_param, request: tonic::Request<#streaming<#req_message>>)
                        -> std::result::Result<tonic::Response<Self::#stream>, tonic::Status>;
                }
            }
        };

        stream.extend(method);
    }

    stream
}

fn generate_named(server_service: &syn::Ident, service_name: &str) -> TokenStream {
    let service_name = syn::LitStr::new(service_name, proc_macro2::Span::call_site());
    let name_doc = generate_doc_comment(" Generated gRPC service name");

    quote! {
        #name_doc
        pub const SERVICE_NAME: &str = #service_name;

        impl<T> tonic::server::NamedService for #server_service<T> {
            const NAME: &'static str = SERVICE_NAME;
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn generate_methods<T: Service>(
    service: &T,
    emit_package: bool,
    proto_path: &str,
    compile_well_known_types: bool,
    use_arc_self: bool,
    generate_default_stubs: bool,
    local: bool,
) -> TokenStream {
    let mut stream = TokenStream::new();

    for method in service.methods() {
        let path = format_method_path(service, method, emit_package);
        let method_path = Lit::Str(LitStr::new(&path, Span::call_site()));
        let ident = quote::format_ident!("{}", method.name());
        let server_trait = quote::format_ident!("{}", service.name());

        let method_stream = match (method.client_streaming(), method.server_streaming()) {
            (false, false) => generate_unary(
                method,
                proto_path,
                compile_well_known_types,
                ident,
                server_trait,
                use_arc_self,
                local,
            ),

            (false, true) => generate_server_streaming(
                method,
                proto_path,
                compile_well_known_types,
                ident.clone(),
                server_trait,
                use_arc_self,
                generate_default_stubs,
                local,
            ),
            (true, false) => generate_client_streaming(
                method,
                proto_path,
                compile_well_known_types,
                ident.clone(),
                server_trait,
                use_arc_self,
                local,
            ),

            (true, true) => generate_streaming(
                method,
                proto_path,
                compile_well_known_types,
                ident.clone(),
                server_trait,
                use_arc_self,
                generate_default_stubs,
                local,
            ),
        };

        let method = quote! {
            #method_path => {
                #method_stream
            }
        };
        stream.extend(method);
    }

    stream
}

fn generate_unary<T: Method>(
    method: &T,
    proto_path: &str,
    compile_well_known_types: bool,
    method_ident: Ident,
    server_trait: Ident,
    use_arc_self: bool,
    local: bool,
) -> TokenStream {
    let codec_name = syn::parse_str::<syn::Path>(method.codec_path()).unwrap();

    let service_ident = quote::format_ident!("{}Svc", method.identifier());

    let (request, response) = method.request_response_name(proto_path, compile_well_known_types);

    let inner_arg = if use_arc_self {
        quote!(inner)
    } else {
        quote!(&inner)
    };

    let wrapper = wrapper_token(local);
    let box_future = box_future_token(local);
    let grpc = server_grpc_token(local);

    quote! {
        #[allow(non_camel_case_types)]
        struct #service_ident<T: #server_trait >(pub #wrapper<T>);

        impl<T: #server_trait> tonic::server::UnaryService<#request> for #service_ident<T> {
            type Response = #response;
            type Future = #box_future<tonic::Response<Self::Response>, tonic::Status>;

            fn call(&mut self, request: tonic::Request<#request>) -> Self::Future {
                let inner = #wrapper::clone(&self.0);
                let fut = async move {
                    <T as #server_trait>::#method_ident(#inner_arg, request).await
                };
                Box::pin(fut)
            }
        }

        let accept_compression_encodings = self.accept_compression_encodings;
        let send_compression_encodings = self.send_compression_encodings;
        let max_decoding_message_size = self.max_decoding_message_size;
        let max_encoding_message_size = self.max_encoding_message_size;
        let inner = self.inner.clone();
        let fut = async move {
            let method = #service_ident(inner);
            let codec = #codec_name::default();

            let mut grpc = #grpc::new(codec)
                .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

            let res = grpc.unary(method, req).await;
            Ok(res)
        };

        Box::pin(fut)
    }
}

#[allow(clippy::too_many_arguments)]
fn generate_server_streaming<T: Method>(
    method: &T,
    proto_path: &str,
    compile_well_known_types: bool,
    method_ident: Ident,
    server_trait: Ident,
    use_arc_self: bool,
    generate_default_stubs: bool,
    local: bool,
) -> TokenStream {
    let codec_name = syn::parse_str::<syn::Path>(method.codec_path()).unwrap();

    let service_ident = quote::format_ident!("{}Svc", method.identifier());

    let (request, response) = method.request_response_name(proto_path, compile_well_known_types);

    let box_stream = box_stream_token(local);
    let response_stream = if !generate_default_stubs {
        let stream = quote::format_ident!("{}Stream", method.identifier());
        quote!(type ResponseStream = T::#stream)
    } else {
        quote!(type ResponseStream = #box_stream<#response>)
    };

    let inner_arg = if use_arc_self {
        quote!(inner)
    } else {
        quote!(&inner)
    };

    let wrapper = wrapper_token(local);
    let box_future = box_future_token(local);
    let grpc = server_grpc_token(local);

    quote! {
        #[allow(non_camel_case_types)]
        struct #service_ident<T: #server_trait >(pub #wrapper<T>);

        impl<T: #server_trait> tonic::server::ServerStreamingService<#request> for #service_ident<T> {
            type Response = #response;
            #response_stream;
            type Future = #box_future<tonic::Response<Self::ResponseStream>, tonic::Status>;

            fn call(&mut self, request: tonic::Request<#request>) -> Self::Future {
                let inner = #wrapper::clone(&self.0);
                let fut = async move {
                    <T as #server_trait>::#method_ident(#inner_arg, request).await
                };
                Box::pin(fut)
            }
        }

        let accept_compression_encodings = self.accept_compression_encodings;
        let send_compression_encodings = self.send_compression_encodings;
        let max_decoding_message_size = self.max_decoding_message_size;
        let max_encoding_message_size = self.max_encoding_message_size;
        let inner = self.inner.clone();
        let fut = async move {
            let method = #service_ident(inner);
            let codec = #codec_name::default();

            let mut grpc = #grpc::new(codec)
                .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

            let res = grpc.server_streaming(method, req).await;
            Ok(res)
        };

        Box::pin(fut)
    }
}

fn generate_client_streaming<T: Method>(
    method: &T,
    proto_path: &str,
    compile_well_known_types: bool,
    method_ident: Ident,
    server_trait: Ident,
    use_arc_self: bool,
    local: bool,
) -> TokenStream {
    let service_ident = quote::format_ident!("{}Svc", method.identifier());

    let (request, response) = method.request_response_name(proto_path, compile_well_known_types);
    let codec_name = syn::parse_str::<syn::Path>(method.codec_path()).unwrap();

    let inner_arg = if use_arc_self {
        quote!(inner)
    } else {
        quote!(&inner)
    };

    let wrapper = wrapper_token(local);
    let box_future = box_future_token(local);
    let grpc = server_grpc_token(local);
    let streaming = streaming_type_token(local);
    let service_trait = if local {
        quote!(tonic::local::server::ClientStreamingService)
    } else {
        quote!(tonic::server::ClientStreamingService)
    };

    quote! {
        #[allow(non_camel_case_types)]
        struct #service_ident<T: #server_trait >(pub #wrapper<T>);

        impl<T: #server_trait> #service_trait<#request> for #service_ident<T>
        {
            type Response = #response;
            type Future = #box_future<tonic::Response<Self::Response>, tonic::Status>;

            fn call(&mut self, request: tonic::Request<#streaming<#request>>) -> Self::Future {
                let inner = #wrapper::clone(&self.0);
                let fut = async move {
                    <T as #server_trait>::#method_ident(#inner_arg, request).await
                };
                Box::pin(fut)
            }
        }

        let accept_compression_encodings = self.accept_compression_encodings;
        let send_compression_encodings = self.send_compression_encodings;
        let max_decoding_message_size = self.max_decoding_message_size;
        let max_encoding_message_size = self.max_encoding_message_size;
        let inner = self.inner.clone();
        let fut = async move {
            let method = #service_ident(inner);
            let codec = #codec_name::default();

            let mut grpc = #grpc::new(codec)
                .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

            let res = grpc.client_streaming(method, req).await;
            Ok(res)
        };

        Box::pin(fut)
    }
}

#[allow(clippy::too_many_arguments)]
fn generate_streaming<T: Method>(
    method: &T,
    proto_path: &str,
    compile_well_known_types: bool,
    method_ident: Ident,
    server_trait: Ident,
    use_arc_self: bool,
    generate_default_stubs: bool,
    local: bool,
) -> TokenStream {
    let codec_name = syn::parse_str::<syn::Path>(method.codec_path()).unwrap();

    let service_ident = quote::format_ident!("{}Svc", method.identifier());

    let (request, response) = method.request_response_name(proto_path, compile_well_known_types);

    let box_stream = box_stream_token(local);
    let response_stream = if !generate_default_stubs {
        let stream = quote::format_ident!("{}Stream", method.identifier());
        quote!(type ResponseStream = T::#stream)
    } else {
        quote!(type ResponseStream = #box_stream<#response>)
    };

    let inner_arg = if use_arc_self {
        quote!(inner)
    } else {
        quote!(&inner)
    };

    let wrapper = wrapper_token(local);
    let box_future = box_future_token(local);
    let grpc = server_grpc_token(local);
    let streaming = streaming_type_token(local);
    let service_trait = if local {
        quote!(tonic::local::server::StreamingService)
    } else {
        quote!(tonic::server::StreamingService)
    };

    quote! {
        #[allow(non_camel_case_types)]
        struct #service_ident<T: #server_trait>(pub #wrapper<T>);

        impl<T: #server_trait> #service_trait<#request> for #service_ident<T>
        {
            type Response = #response;
            #response_stream;
            type Future = #box_future<tonic::Response<Self::ResponseStream>, tonic::Status>;

            fn call(&mut self, request: tonic::Request<#streaming<#request>>) -> Self::Future {
                let inner = #wrapper::clone(&self.0);
                let fut = async move {
                    <T as #server_trait>::#method_ident(#inner_arg, request).await
                };
                Box::pin(fut)
            }
        }

        let accept_compression_encodings = self.accept_compression_encodings;
        let send_compression_encodings = self.send_compression_encodings;
        let max_decoding_message_size = self.max_decoding_message_size;
        let max_encoding_message_size = self.max_encoding_message_size;
        let inner = self.inner.clone();
        let fut = async move {
            let method = #service_ident(inner);
            let codec = #codec_name::default();

            let mut grpc = #grpc::new(codec)
                .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

            let res = grpc.streaming(method, req).await;
            Ok(res)
        };

        Box::pin(fut)
    }
}
