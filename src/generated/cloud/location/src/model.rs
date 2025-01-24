// Copyright 2024 Google LLC
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

#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::redundant_explicit_links)]
#![allow(rustdoc::broken_intra_doc_links)]
#![no_implicit_prelude]
extern crate async_trait;
extern crate bytes;
extern crate gax;
extern crate lazy_static;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_with;
extern crate std;
extern crate tracing;
extern crate wkt;

/// The request message for [Locations.ListLocations][google.cloud.location.Locations.ListLocations].
///
/// [google.cloud.location.Locations.ListLocations]: crate::client::Locations::list_locations
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListLocationsRequest {
    /// The resource that owns the locations collection, if applicable.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// The standard list filter.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub filter: std::string::String,

    /// The standard list page size.
    pub page_size: i32,

    /// The standard list page token.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub page_token: std::string::String,
}

impl ListLocationsRequest {
    /// Sets the value of `name`.
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of `filter`.
    pub fn set_filter<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.filter = v.into();
        self
    }

    /// Sets the value of `page_size`.
    pub fn set_page_size<T: std::convert::Into<i32>>(mut self, v: T) -> Self {
        self.page_size = v.into();
        self
    }

    /// Sets the value of `page_token`.
    pub fn set_page_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.page_token = v.into();
        self
    }
}

impl wkt::message::Message for ListLocationsRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.location.ListLocationsRequest"
    }
}

/// The response message for [Locations.ListLocations][google.cloud.location.Locations.ListLocations].
///
/// [google.cloud.location.Locations.ListLocations]: crate::client::Locations::list_locations
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListLocationsResponse {
    /// A list of locations that matches the specified filter in the request.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub locations: std::vec::Vec<crate::model::Location>,

    /// The standard List next-page token.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub next_page_token: std::string::String,
}

impl ListLocationsResponse {
    /// Sets the value of `locations`.
    pub fn set_locations<T: std::convert::Into<std::vec::Vec<crate::model::Location>>>(
        mut self,
        v: T,
    ) -> Self {
        self.locations = v.into();
        self
    }

    /// Sets the value of `next_page_token`.
    pub fn set_next_page_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.next_page_token = v.into();
        self
    }
}

impl wkt::message::Message for ListLocationsResponse {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.location.ListLocationsResponse"
    }
}

#[cfg(feature = "unstable-stream")]
impl gax::paginator::PageableResponse for ListLocationsResponse {
    type PageItem = crate::model::Location;

    fn items(self) -> std::vec::Vec<Self::PageItem> {
        self.locations
    }

    fn next_page_token(&self) -> std::string::String {
        gax::paginator::extract_token(&self.next_page_token)
    }
}

/// The request message for [Locations.GetLocation][google.cloud.location.Locations.GetLocation].
///
/// [google.cloud.location.Locations.GetLocation]: crate::client::Locations::get_location
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct GetLocationRequest {
    /// Resource name for the location.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,
}

impl GetLocationRequest {
    /// Sets the value of `name`.
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }
}

impl wkt::message::Message for GetLocationRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.location.GetLocationRequest"
    }
}

/// A resource that represents Google Cloud Platform location.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct Location {
    /// Resource name for the location, which may vary between implementations.
    /// For example: `"projects/example-project/locations/us-east1"`
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// The canonical id for this location. For example: `"us-east1"`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub location_id: std::string::String,

    /// The friendly name for this location, typically a nearby city name.
    /// For example, "Tokyo".
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub display_name: std::string::String,

    /// Cross-service attributes for the location. For example
    ///
    /// ```norust
    /// {"cloud.googleapis.com/region": "us-east1"}
    /// ```
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub labels: std::collections::HashMap<std::string::String, std::string::String>,

    /// Service-specific metadata. For example the available capacity at the given
    /// location.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub metadata: std::option::Option<wkt::Any>,
}

impl Location {
    /// Sets the value of `name`.
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of `location_id`.
    pub fn set_location_id<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.location_id = v.into();
        self
    }

    /// Sets the value of `display_name`.
    pub fn set_display_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.display_name = v.into();
        self
    }

    /// Sets the value of `labels`.
    pub fn set_labels<
        T: std::convert::Into<std::collections::HashMap<std::string::String, std::string::String>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.labels = v.into();
        self
    }

    /// Sets the value of `metadata`.
    pub fn set_metadata<T: std::convert::Into<std::option::Option<wkt::Any>>>(
        mut self,
        v: T,
    ) -> Self {
        self.metadata = v.into();
        self
    }
}

impl wkt::message::Message for Location {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.location.Location"
    }
}
