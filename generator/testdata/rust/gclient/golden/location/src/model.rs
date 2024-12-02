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

/// The request message for [Locations.ListLocations][google.cloud.location.Locations.ListLocations].
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListLocationsRequest {
    /// The resource that owns the locations collection, if applicable.
    pub name: String,

    /// The standard list filter.
    pub filter: String,

    /// The standard list page size.
    pub page_size: i32,

    /// The standard list page token.
    pub page_token: String,
}

impl ListLocationsRequest {
    /// Sets the value of `name`.
    pub fn set_name<T: Into<String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of `filter`.
    pub fn set_filter<T: Into<String>>(mut self, v: T) -> Self {
        self.filter = v.into();
        self
    }

    /// Sets the value of `page_size`.
    pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
        self.page_size = v.into();
        self
    }

    /// Sets the value of `page_token`.
    pub fn set_page_token<T: Into<String>>(mut self, v: T) -> Self {
        self.page_token = v.into();
        self
    }
}

/// The response message for [Locations.ListLocations][google.cloud.location.Locations.ListLocations].
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListLocationsResponse {
    /// A list of locations that matches the specified filter in the request.
    pub locations: Vec<crate::model::Location>,

    /// The standard List next-page token.
    pub next_page_token: String,
}

impl ListLocationsResponse {
    /// Sets the value of `locations`.
    pub fn set_locations<T: Into<Vec<crate::model::Location>>>(mut self, v: T) -> Self {
        self.locations = v.into();
        self
    }

    /// Sets the value of `next_page_token`.
    pub fn set_next_page_token<T: Into<String>>(mut self, v: T) -> Self {
        self.next_page_token = v.into();
        self
    }
}

/// The request message for [Locations.GetLocation][google.cloud.location.Locations.GetLocation].
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct GetLocationRequest {
    /// Resource name for the location.
    pub name: String,
}

impl GetLocationRequest {
    /// Sets the value of `name`.
    pub fn set_name<T: Into<String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
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
    pub name: String,

    /// The canonical id for this location. For example: `"us-east1"`.
    pub location_id: String,

    /// The friendly name for this location, typically a nearby city name.
    /// For example, "Tokyo".
    pub display_name: String,

    /// Cross-service attributes for the location. For example
    ///
    /// ```norust
    /// {"cloud.googleapis.com/region": "us-east1"}
    /// ```
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub labels: std::collections::HashMap<String, String>,

    /// Service-specific metadata. For example the available capacity at the given
    /// location.
    pub metadata: Option<wkt::Any>,
}

impl Location {
    /// Sets the value of `name`.
    pub fn set_name<T: Into<String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of `location_id`.
    pub fn set_location_id<T: Into<String>>(mut self, v: T) -> Self {
        self.location_id = v.into();
        self
    }

    /// Sets the value of `display_name`.
    pub fn set_display_name<T: Into<String>>(mut self, v: T) -> Self {
        self.display_name = v.into();
        self
    }

    /// Sets the value of `labels`.
    pub fn set_labels<T: Into<std::collections::HashMap<String, String>>>(mut self, v: T) -> Self {
        self.labels = v.into();
        self
    }

    /// Sets the value of `metadata`.
    pub fn set_metadata<T: Into<Option<wkt::Any>>>(mut self, v: T) -> Self {
        self.metadata = v.into();
        self
    }
}
