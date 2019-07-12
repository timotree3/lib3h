//! This module provides the api definition for working with lib3h

extern crate holochain_persistence_api;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde;
// extern crate lib3h;

pub mod data_types;
pub mod network_engine;
pub mod protocol_client;
pub mod protocol_server;

/// string encoded address type
pub type Address = holochain_persistence_api::hash::HashString;

/// type name for a bool indicating if work was done during a `process()`
pub type DidWork = bool;

// /// TODO: To replace with our own custom Error handling
// use failure::Error;
// pub type Lib3hResult<T> = Result<T, Error>;
pub mod error;
// pub use error::
// use error::Lib3hProtocolResult as Lib3hResult;
