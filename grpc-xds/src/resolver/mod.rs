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

//! The xDS resolver (gRFC A27, A28, A31). So far, only the config selector it
//! hands to the channel.

use std::fmt::Debug;
use std::sync::Arc;

use grpc::client::CallOptions;

mod config_selector;

/// The cluster the config selector routed an RPC to. The config selector's
/// cluster interceptors store an `Arc<PickedCluster>` in the RPC's
/// `CallOptions`, keyed by its type, where it tells the cluster manager picker
/// which child to pick from.
#[derive(Debug)]
struct PickedCluster {
    /// Name of the cluster manager child for the cluster.
    child_name: String,
    /// Keeps the cluster in use, i.e. subscribed and in the resolver's
    /// service config, while this pick is alive.
    // TODO: Get this from the Dependency Manager.
    _cluster_ref: Arc<()>,
}

/// Returns the name of the cluster manager child that the RPC with
/// `call_options` was routed to, or `None` if the config selector didn't
/// route it.
pub(crate) fn picked_child_name(call_options: &CallOptions) -> Option<&str> {
    call_options
        .get_option::<Arc<PickedCluster>>()
        .map(|picked| picked.child_name.as_str())
}

/// Routes the RPC with `call_options` to the cluster manager child named
/// `child_name`, as the config selector would. The pick's cluster ref isn't
/// tracked by anything.
#[cfg(test)]
pub(crate) fn set_picked_child_for_testing(call_options: &mut CallOptions, child_name: &str) {
    call_options.set_option(Arc::new(PickedCluster {
        child_name: child_name.to_owned(),
        _cluster_ref: Arc::new(()),
    }));
}
