use std::any::TypeId;
use std::collections::VecDeque;

use bytes::Buf;
use bytes::Bytes;

use crate::core::MessageType;
use crate::core::RecvMessage;
use crate::core::SendMessage;

/// An immutable value-type struct representing an incoming raw gRPC message.
pub(crate) struct IncomingRawMessage {
    buf: Box<dyn Buf + Send + Sync>,
    compressed: bool,
}

impl IncomingRawMessage {
    /// Constructs a new `IncomingRawMessage` initialized with a cheap empty buffer.
    pub(crate) fn new() -> Self {
        Self {
            buf: Box::new(Bytes::new()),
            compressed: false,
        }
    }

    /// Destructures the message by value into its raw payload buffer and compression flag.
    pub(crate) fn into_parts(self) -> (Box<dyn Buf + Send + Sync>, bool) {
        (self.buf, self.compressed)
    }

    /// Safely sets the per-message compression flag.
    pub(crate) fn set_compressed(&mut self, compressed: bool) {
        self.compressed = compressed;
    }
}

impl Default for IncomingRawMessage {
    fn default() -> Self {
        Self::new()
    }
}

impl MessageType for IncomingRawMessage {
    type Target<'a> = IncomingRawMessage;
}

impl RecvMessage for IncomingRawMessage {
    fn decode(&mut self, data: &mut dyn Buf) -> Result<(), String> {
        // Directly updates the immutable value-type container's inner buffer
        self.buf = Box::new(data.copy_to_bytes(data.remaining()));
        Ok(())
    }

    unsafe fn _ptr_for(&mut self, id: TypeId) -> Option<*mut ()> {
        if id == TypeId::of::<IncomingRawMessage>() {
            Some(self as *mut IncomingRawMessage as *mut ())
        } else {
            None
        }
    }
}

/// A custom `Buf` implementation that streams sequentially through a deque of `Bytes` chunks.
struct ChunkedBuf {
    chunks: VecDeque<Bytes>,
}

impl Buf for ChunkedBuf {
    fn remaining(&self) -> usize {
        self.chunks.iter().map(|b| b.len()).sum()
    }

    fn chunk(&self) -> &[u8] {
        self.chunks.front().map(|b| b.chunk()).unwrap_or(&[])
    }

    fn advance(&mut self, mut cnt: usize) {
        while cnt > 0 {
            if let Some(front) = self.chunks.front_mut() {
                let len = front.len();
                if cnt >= len {
                    cnt -= len;
                    self.chunks.pop_front();
                } else {
                    front.advance(cnt);
                    break;
                }
            } else {
                break;
            }
        }
    }
}

/// A raw outgoing message usable for configuring SendOptions cleanly.
/// Stores data as a hybrid enum to allow zero-copy outbound serialization
/// without allocating a `VecDeque` for standard contiguous messages.
pub(crate) enum RawMessage {
    Contiguous(Bytes),
    Chunks(VecDeque<Bytes>),
}

impl RawMessage {
    pub(crate) fn from_buf(mut buf: impl Buf) -> Self {
        let remaining = buf.remaining();
        if buf.chunk().len() == remaining {
            RawMessage::Contiguous(buf.copy_to_bytes(remaining))
        } else {
            let mut chunks = VecDeque::new();
            while buf.has_remaining() {
                let chunk_len = buf.chunk().len();
                chunks.push_back(buf.copy_to_bytes(chunk_len));
            }
            RawMessage::Chunks(chunks)
        }
    }
}

impl SendMessage for RawMessage {
    fn encode(&self) -> Result<Box<dyn Buf + Send + Sync>, String> {
        match self {
            RawMessage::Contiguous(bytes) => Ok(Box::new(bytes.clone())),
            RawMessage::Chunks(chunks) => {
                // `Bytes` clones are cheap $O(1)$ reference bumps, preserving idempotency safely.
                Ok(Box::new(ChunkedBuf {
                    chunks: chunks.clone(),
                }))
            }
        }
    }
}
