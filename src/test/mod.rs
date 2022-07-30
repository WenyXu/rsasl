//! Test utilities for rsasl
//!
//! This module contains several utility functions and data types to test SASL handling in
//! protocol implementations and user code

mod mechanism;
mod config;

use std::sync::Arc;
pub use config::{EmptyCallback, client_config, server_config};
use crate::channel_bindings::NoChannelBindings;
use crate::config::SASLConfig;
use crate::registry::Mechanism;
use crate::sasl::SASL;
use crate::session::{Session, Side};
use crate::validate::NoValidation;

pub fn test_client_session(config: Arc<SASLConfig>, mechanism: &Mechanism) -> Session<NoValidation, NoChannelBindings> {
    let mech = mechanism.client(&config, &[mechanism.mechanism])
                        .unwrap().unwrap();
    let sasl = SASL {
        config,
        cb: NoChannelBindings,
        validation: None,
    };
    Session::new(sasl, Side::Client, mech, *mechanism)
}