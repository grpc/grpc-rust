pub(crate) mod p2c;
pub(crate) mod ring_hash;

use indexmap::{IndexMap, IndexSet};

use crate::client::endpoint::EndpointAddress;

/// Trait for selecting a channel to handle a request.
///
/// Generic over `S` (the channel type in the ready set) and `Req` (the request).
/// The picker only needs to observe `S`'s load — it doesn't depend on any
/// specific channel state type.
pub(crate) trait ChannelPicker<S, Req> {
    fn pick<'a>(&self, req: &Req, ready: &'a IndexMap<EndpointAddress, S>) -> Option<&'a S>;

    /// Notify the picker that the cluster's member set changed.
    ///
    /// `members` is the full healthy-EDS membership, independent of connection
    /// or ejection state. Stateful pickers (e.g. ring-hash) rebuild their
    /// internal structure here so the ring stays stable across connection
    /// flaps and ejections (which do not change membership). Default: no-op
    /// (e.g. [`p2c::P2cPicker`], which is stateless).
    fn on_members_changed(&self, _members: &IndexSet<EndpointAddress>) {}
}
