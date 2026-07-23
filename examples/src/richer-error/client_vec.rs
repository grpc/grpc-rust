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

use tonic_types::{ErrorDetail, StatusExt};

use hello_world::HelloRequest;
use hello_world::greeter_client::GreeterClient;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        // Valid request
        // name: "Tonic".into(),
        // Name cannot be empty
        name: "".into(),
        // Name is too long
        // name: "some excessively long name".into(),
    });

    let response = match client.say_hello(request).await {
        Ok(response) => response,
        Err(status) => {
            println!(" Error status received. Extracting error details...\n");

            let err_details = status.get_error_details_vec();

            for (i, err_detail) in err_details.iter().enumerate() {
                println!("err_detail[{i}]");
                match err_detail {
                    ErrorDetail::BadRequest(bad_request) => {
                        // Handle bad_request details
                        println!(" {bad_request:?}");
                    }
                    ErrorDetail::Help(help) => {
                        // Handle help details
                        println!(" {help:?}");
                    }
                    ErrorDetail::LocalizedMessage(localized_message) => {
                        // Handle localized_message details
                        println!(" {localized_message:?}");
                    }
                    _ => {}
                }
            }

            println!();

            return Ok(());
        }
    };

    println!(" Successful response received.\n\n {response:?}\n");

    Ok(())
}
