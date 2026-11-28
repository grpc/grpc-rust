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

use integration_tests::pb::{test_client::TestClient, test_server, Input, Output};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::{future::Future, time::Duration};
use tokio::{net::TcpListener, sync::oneshot};
use tokio_stream::wrappers::TcpListenerStream;
use tonic::{
    transport::{channel::Change, Channel, Endpoint, Server},
    Request, Response, Status,
};

#[tokio::test]
async fn idle_channel_consumes_discovery_updates() {
    let (_channel, tx) = Channel::balance_channel(1);
    tokio::time::timeout(Duration::from_secs(1), async {
        tx.send(Change::Insert(
            0,
            Endpoint::from_static("http://127.0.0.1:1"),
        ))
        .await
        .unwrap();
        tx.send(Change::Remove(0)).await.unwrap();
        tx.send(Change::Insert(
            1,
            Endpoint::from_static("http://127.0.0.1:2"),
        ))
        .await
        .unwrap();
        let _permit = tx.reserve().await.unwrap();
    })
    .await
    .expect("idle channel must drain discovery updates");
}

struct Svc(Arc<AtomicUsize>);

#[tonic::async_trait]
impl test_server::Test for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Ok(Response::new(Output {}))
    }
}

#[tokio::test]
async fn discovery_updates_route_to_current_endpoints() {
    let mut servers = Vec::new();
    let mut shutdowns = Vec::new();
    let mut endpoints = Vec::new();
    let counts = [Arc::new(AtomicUsize::new(0)), Arc::new(AtomicUsize::new(0))];
    for count in &counts {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        endpoints.push(
            Endpoint::from_shared(format!("http://{}", listener.local_addr().unwrap())).unwrap(),
        );
        let (tx, rx) = oneshot::channel();
        shutdowns.push(tx);
        let svc = test_server::TestServer::new(Svc(count.clone()));
        servers.push(tokio::spawn(async move {
            Server::builder()
                .add_service(svc)
                .serve_with_incoming_shutdown(TcpListenerStream::new(listener), async {
                    let _ = rx.await;
                })
                .await
                .unwrap();
        }));
    }

    tokio::time::timeout(Duration::from_secs(5), async {
        let (channel, tx) = Channel::balance_channel(1);
        let mut client = TestClient::new(channel);
        tx.send(Change::Insert(0, endpoints[0].clone()))
            .await
            .unwrap();
        client.unary_call(Input {}).await.unwrap();
        assert_eq!(counts[0].load(Ordering::SeqCst), 1);

        // Capacity one plus a reservation proves the last update was consumed.
        tx.send(Change::Insert(1, endpoints[1].clone()))
            .await
            .unwrap();
        tx.send(Change::Remove(0)).await.unwrap();
        drop(tx.reserve().await.unwrap());
        for _ in 0..3 {
            client.unary_call(Input {}).await.unwrap();
        }
        assert_eq!(counts[0].load(Ordering::SeqCst), 1);
        assert_eq!(counts[1].load(Ordering::SeqCst), 3);

        tx.send(Change::Remove(1)).await.unwrap();
        tx.send(Change::Remove(99)).await.unwrap();
        drop(tx.reserve().await.unwrap());
        {
            let call = client.unary_call(Input {});
            tokio::pin!(call);
            std::future::poll_fn(|cx| {
                assert!(call.as_mut().poll(cx).is_pending());
                std::task::Poll::Ready(())
            })
            .await;
            tokio::task::yield_now().await;
            tx.send(Change::Insert(1, endpoints[1].clone()))
                .await
                .unwrap();
            call.await.unwrap();
        }

        let calls = async {
            for _ in 0..10 {
                client.unary_call(Input {}).await.unwrap();
            }
        };
        let updates = async {
            for _ in 0..10 {
                tx.send(Change::Insert(1, endpoints[0].clone()))
                    .await
                    .unwrap();
                tx.send(Change::Remove(1)).await.unwrap();
                tx.send(Change::Insert(1, endpoints[1].clone()))
                    .await
                    .unwrap();
            }
            drop(tx.reserve().await.unwrap());
        };
        tokio::join!(calls, updates);
        // Evict any ready A service left by the burst before checking routing.
        tx.send(Change::Remove(1)).await.unwrap();
        tx.send(Change::Insert(1, endpoints[1].clone()))
            .await
            .unwrap();
        drop(tx.reserve().await.unwrap());
        let a_calls = counts[0].load(Ordering::SeqCst);
        drop(tx);
        for _ in 0..3 {
            client.unary_call(Input {}).await.unwrap();
        }
        assert_eq!(counts[0].load(Ordering::SeqCst), a_calls);
    })
    .await
    .expect("discovery updates and calls must make progress");

    for shutdown in shutdowns {
        shutdown.send(()).unwrap();
    }
    for server in servers {
        tokio::time::timeout(Duration::from_secs(5), server)
            .await
            .unwrap()
            .unwrap();
    }
}
