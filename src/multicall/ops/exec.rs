/*! # rtorrent-xmlrpc-bindings

`rtorrent-xmlrpc-bindings` provides strongly-typed Rust bindings for the [rtorrent] [XMLRPC API].

The XMLRPC API allows a high degree of introspection and control over an rtorrent instance.

## Usage

The top-level structure representing an rtorrent instance is [`Server`].  All errors produced by
the crate are encapsulated by the [`Error`] type.

```no_run
use rtorrent_xmlrpc_bindings as rtorrent;

let my_handle = rtorrent::Server::new("http://1.2.3.4/RPC2");
println!("Hostname: {}", my_handle.hostname()?);

for download in my_handle.download_list()? {
    println!("Download: {}", download.name()?);
}
# Ok::<(), rtorrent::Error>(())
```

It can be more efficient to query multiple items at a time.  Rtorrent's XMLRPC API exposes an
interface for this called "multicalls."  In this crate, they are available through the
[`multicall`] submodule.

The following example queries the name and ratio of every torrent in rtorrent's "default" view and
prints the results.

```no_run
use rtorrent_xmlrpc_bindings as rtorrent;
use rtorrent::multicall::d;

let my_handle = rtorrent::Server::new("http://1.2.3.4/RPC2");

d::MultiBuilder::new(&my_handle, "default")
    .call(d::NAME)
    .call(d::RATIO)
    .invoke()?
    .iter()
    .for_each(|(name, ratio)| {
        println!("{}: ratio: {}", name, ratio);
    });
# Ok::<(), rtorrent::Error>(())
```

## Current Limitations

* Some XMLRPC APIs are not yet wrapped by this crate.

[rtorrent]: https://rakshasa.github.io/rtorrent/
[XMLRPC API]: https://rtorrent-docs.readthedocs.io/en/latest/cmd-ref.html

[`Error`]: crate::Error
[`multicall`]: crate::multicall
[`Server`]: crate::Server
!*/

use std::sync::Arc;
use xmlrpc::{Request, Value};

mod download;
mod file;
pub mod multicall;
mod peer;
mod tracker;
pub(crate) mod value_conversion;

pub use download::Download;
pub use file::File;
pub use peer::Peer;

pub use tracker::Tracker;
pub use value_conversion::TryFromValue;

/// The canonical [`Result`] for this crate (we return the same error type everywhere).
pub type Result<T> = std::result::Result<T, Error>;

/// The unified error type for this crate.
#[derive(Debug)]
pub enum Error {
    XmlRpc(xmlrpc::Error),
    UnexpectedStructure(String),
}

impl From<xmlrpc::Error> for Error {
    fn from(x: xmlrpc::Error) -> Self {
        Error::XmlRpc(x)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::XmlRpc(xe) => {
                write!(f, "XML-RPC: {}", xe)
            }
            Error::UnexpectedStructure(us) => {
                write!(f, "Unexpected XML structure: {}", us)
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::XmlRpc(xe) => Some(xe),
            _ => None,
        }
    }
}

macro_rules! exec_str_getter {
    ($(#[$meta:meta])* $method: ident) => {
        prim_getter!($(#[$meta])* "exec", $method, String);
    }
}
#[derive(Debug)]
struct ServerInner {
    endpoint: String,
}

/// `Server` represents a logical rtorrent instance
#[derive(Clone, Debug)]
pub struct Server {
    inner: Arc<ServerInner>,
}