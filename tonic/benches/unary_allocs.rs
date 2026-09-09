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
//! Allocations per in-process unary RPC.
//!
//! A counting global allocator cannot share a process with a timed benchmark:
//! it adds two atomic read-modify-writes to every allocation. So this census
//! lives in its own `harness = false` target and reports no timings. Wall-clock
//! cost is measured by the separate `unary` target.

mod unary_support;

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

use unary_support::{SIZES, client, one_rpc, payload, runtime};

/// RPCs run before the counters are read, so steady-state buffer growth is not
/// charged to the measured window.
const WARMUP_RPCS: usize = 200;

/// RPCs spanned by the measured window.
const MEASURED_RPCS: usize = 1000;

static COUNT: AtomicUsize = AtomicUsize::new(0);
static BYTES: AtomicUsize = AtomicUsize::new(0);

struct Counting;

#[global_allocator]
static ALLOCATOR: Counting = Counting;

// Allocation counts stay separate from deallocation counts, so each RPC delta
// is independent of when its response is dropped.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        COUNT.fetch_add(1, Ordering::Relaxed);
        BYTES.fetch_add(layout.size(), Ordering::Relaxed);
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        COUNT.fetch_add(1, Ordering::Relaxed);
        BYTES.fetch_add(layout.size(), Ordering::Relaxed);
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        COUNT.fetch_add(1, Ordering::Relaxed);
        BYTES.fetch_add(new_size, Ordering::Relaxed);
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

/// Totals for one payload size. Per-RPC figures are integer quotients, so the
/// totals are reported alongside them to keep a fractional count visible.
struct Census {
    size: usize,
    allocs: usize,
    bytes: usize,
}

fn census(size: usize) -> Census {
    let payload = payload(size);
    let mut client = client();
    let runtime = runtime();

    let response = runtime.block_on(one_rpc(&mut client, &payload));
    assert_eq!(
        response, payload,
        "census must measure a working round trip"
    );

    for _ in 0..WARMUP_RPCS {
        let _ = runtime.block_on(one_rpc(&mut client, &payload));
    }

    let allocs_before = COUNT.load(Ordering::Relaxed);
    let bytes_before = BYTES.load(Ordering::Relaxed);

    for _ in 0..MEASURED_RPCS {
        let _ = runtime.block_on(one_rpc(&mut client, &payload));
    }

    Census {
        size,
        allocs: COUNT.load(Ordering::Relaxed) - allocs_before,
        bytes: BYTES.load(Ordering::Relaxed) - bytes_before,
    }
}

fn main() {
    for size in SIZES {
        let Census {
            size,
            allocs,
            bytes,
        } = census(size);
        println!(
            "unary/{size}: {allocs_per_rpc} allocs/rpc, {bytes_per_rpc} bytes/rpc \
             ({allocs} allocs and {bytes} bytes over {MEASURED_RPCS} rpcs)",
            allocs_per_rpc = allocs / MEASURED_RPCS,
            bytes_per_rpc = bytes / MEASURED_RPCS,
        );
    }
}
