pub mod app;
pub mod backup;
pub mod daemon;
pub mod delete;
pub mod dir;
pub mod download;
pub mod export;
pub mod hw;
pub mod installer;
pub mod launcher;
pub mod loader;
pub mod logger;
pub mod node;
pub mod services;
pub mod signer;
pub mod utils;

use lianad::Version;

pub const VERSION: Version = Version {
    major: 10,
    minor: 0,
    patch: 0,
};
