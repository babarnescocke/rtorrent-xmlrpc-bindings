//! Rtorrent d.* multicall operations

use crate::{multicall::raw, Server};
use std::borrow::Cow;
use std::marker::PhantomData;

super::op_type! {
    /// A `d.*` operation for multicalls
    DownloadMultiCallOp
}

/// `MultiBuilder` is a tool for building queries across many downloads
///
/// The constructed query is executed in a single XMLRPC call.  The query results are in convenient
/// Rust types.
///
/// ## Usage
///
/// Example: Print name, size, and upload ratio for all downloads in the "default" view.
///
/// ```no_run
/// use rtorrent_xmlrpc_bindings as rtorrent;
/// use rtorrent::multicall::d;
///
/// let my_handle = rtorrent::Server::new("http://1.2.3.4/RPC2");
///
/// d::MultiBuilder::new(&my_handle, "default")
///     .call(d::NAME)
///     .call(d::RATIO)
///     .call(d::SIZE_BYTES)
///     .invoke()?
///     .iter()
///     .for_each(|(name, ratio, bytes)| {
///         println!("{}: {} bytes, {} ratio", name, bytes, ratio);
///     });
/// # Ok::<(), rtorrent::Error>(())
/// ```
///
/// The `call()` method can be invoked repeatedly to add more columns to the query -- in the above
/// example, selecting the `NAME`, `RATIO`, and `SIZE_BYTES` columns.
pub struct MultiBuilder {
    pub(crate) inner: raw::MultiBuilder,
}

impl MultiBuilder {
    /// Start building a multicall over downloads in some specific `view` on `server`.
    ///
    /// Views usually include:
    /// * "main"
    /// * "default"
    /// * "name"
    /// * "active"
    /// * "started"
    /// * "stopped"
    /// * "complete"
    /// * "incomplete"
    /// * "hashing"
    /// * "seeding"
    /// * "leeching"
    pub fn new(server: &Server, view: &str) -> Self {
        Self {
            inner: raw::MultiBuilder::new(server, "d.multicall2", "", view),
        }
    }
}

macro_rules! define_builder {
    ( $(#[$meta:meta])* $prev: ident, $name: ident, $($phantoms:ident $ty:ident),* | $phantom_last:ident $ty_last:ident ) => {
        ops::define_builder!($(#[$meta])* DownloadMultiCallOp, $prev, $name, $($phantoms $ty),* | $phantom_last $ty_last);
    }
}
pub(crate) use define_builder;

macro_rules! d_op_const {
    ( $(#[$meta:meta])* $name: ident, $res: ty, $api: literal ) => {
        super::op_const!( $(#[$meta])* DownloadMultiCallOp, $name, $res, "d.", $api);
    };
}

d_op_const!(
    /// Infohash for this torrent.
    HASH, String, "hash");
d_op_const!(
    BASE_FILENAME, String, "base_filename");
d_op_const!(
    BASE_PATH, String, "base_path");
d_op_const!(
    DIRECTORY, String, "directory");
d_op_const!(
    DIRECTORY_BASE, String, "directory_base");
d_op_const!(
    /// The item's chunk size, in bytes (also known as "piece size").
    CHUNK_SIZE, i64, "chunk_size");
d_op_const!(
    /// Is the download complete (100%)?
    COMPLETE, bool, "complete");
d_op_const!(
    /// Is the download incomplete (less than 100%)?
    INCOMPLETE, bool, "incomplete");
d_op_const!(
    /// The number of completed bytes.
    COMPLETED_BYTES, i64, "completed_bytes");
d_op_const!(
    /// The number of completed chunks (pieces).
    COMPLETED_CHUNKS, i64, "completed_chunks");
d_op_const!(
    /// Get the download rate.
    DOWN_RATE, i64, "down.rate");
d_op_const!(
    /// Get the download total (bytes).
    DOWN_TOTAL, i64, "down.total");
d_op_const!(
    /// Is this torrent active?
    IS_ACTIVE, bool, "is_active");
d_op_const!(
    IS_OPEN, bool, "is_open");
d_op_const!(
    IS_CLOSED, bool, "is_closed");
d_op_const!(
    /// The metafile from which this download was created.
    LOADED_FILE, String, "loaded_file");
d_op_const!(
    /// Unstructured error messages, either generated by rtorrent, or forwarded from the
    /// tracker.
    MESSAGE, String, "message");
d_op_const!(
    /// Get the name of the torrent.
    NAME, String, "name");
d_op_const!(
    /// Get the upload/download ratio for this download.
    RATIO, f64, "ratio");
d_op_const!(
    /// Get the size, in bytes, of the torrent contents.
    SIZE_BYTES, i64, "size_bytes");
d_op_const!(
    /// Get the number of files associated with this download.
    SIZE_FILES, i64, "size_files");
d_op_const!(
    /// Get the state (`false` is stopped).
    STATE, bool, "state");
d_op_const!(
    /// Starts as the file the download was initially created from.
    TIED_TO_FILE, String, "tied_to_file");
d_op_const!(
    /// Get the number of trackers associated with this download.
    TRACKER_SIZE, i64, "tracker_size");
d_op_const!(
    /// Get the upload rate.
    UP_RATE, i64, "up.rate");
d_op_const!(
    /// Get the upload total (bytes).
    UP_TOTAL, i64, "up.total");
