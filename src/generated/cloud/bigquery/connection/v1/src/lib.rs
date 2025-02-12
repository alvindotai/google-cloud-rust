// Copyright 2025 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Code generated by sidekick. DO NOT EDIT.

//! Google Cloud Client Libraries for Rust - BigQuery Connection API
//!
//! **WARNING:** this crate is under active development. We expect multiple
//! breaking changes in the upcoming releases. Testing is also incomplete, we do
//! **not** recommend that you use this crate in production. We welcome feedback
//! about the APIs, documentation, missing features, bugs, etc.
//!
//! This crate contains traits, types, and functions to interact with BigQuery Connection API
//! Most applications will use the structs defined in the [client] module.
//! More specifically:
//!
//! * [ConnectionService](client/struct.ConnectionService.html)

/// The messages and enums that are part of this client library.
#[allow(clippy::module_inception)]
pub mod model;

pub use gax::error::Error;
pub use gax::Result;

#[allow(rustdoc::invalid_html_tags)]
#[allow(rustdoc::redundant_explicit_links)]
pub mod stubs;

/// Concrete implementations of this client library traits.
pub mod client;

/// Request builders.
pub mod builders;

#[doc(hidden)]
pub(crate) mod tracing;

#[doc(hidden)]
pub(crate) mod transport;

/// The default host used by the service.
const DEFAULT_HOST: &str = "https://bigqueryconnection.googleapis.com/";

pub(crate) mod info {
    const NAME: &str = env!("CARGO_PKG_NAME");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    lazy_static::lazy_static! {
        pub(crate) static ref X_GOOG_API_CLIENT_HEADER: String = {
            let ac = gax::api_header::XGoogApiClient{
                name:          NAME,
                version:       VERSION,
                library_type:  gax::api_header::GAPIC,
            };
            ac.header_value()
        };
    }
}
