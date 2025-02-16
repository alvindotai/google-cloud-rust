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
#![allow(rustdoc::broken_intra_doc_links)]

use crate::Result;
use std::sync::Arc;

/// Implements a client for the Service Health API.
///
/// # Service Description
///
/// Request service health events relevant to your Google Cloud project.
///
/// # Configuration
///
/// `ServiceHealth` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `ServiceHealth` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `ServiceHealth` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct ServiceHealth {
    inner: Arc<dyn crate::stubs::dynamic::ServiceHealth>,
}

impl ServiceHealth {
    /// Creates a new client with the default configuration.
    pub async fn new() -> Result<Self> {
        Self::new_with_config(gax::options::ClientConfig::default()).await
    }

    /// Creates a new client with the specified configuration.
    pub async fn new_with_config(conf: gax::options::ClientConfig) -> Result<Self> {
        let inner = Self::build_inner(conf).await?;
        Ok(Self { inner })
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is when mocking the
    /// client.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: crate::stubs::ServiceHealth + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::ServiceHealth>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::ServiceHealth> {
        crate::transport::ServiceHealth::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::ServiceHealth> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::ServiceHealth::new)
    }

    /// Lists events under a given project and location.
    pub fn list_events(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::service_health::ListEvents {
        crate::builders::service_health::ListEvents::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Retrieves a resource containing information about an event.
    pub fn get_event(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::service_health::GetEvent {
        crate::builders::service_health::GetEvent::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists organization events under a given organization and location.
    pub fn list_organization_events(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::service_health::ListOrganizationEvents {
        crate::builders::service_health::ListOrganizationEvents::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Retrieves a resource containing information about an event affecting an
    /// organization .
    pub fn get_organization_event(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::service_health::GetOrganizationEvent {
        crate::builders::service_health::GetOrganizationEvent::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists assets impacted by organization events under a given organization and
    /// location.
    pub fn list_organization_impacts(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::service_health::ListOrganizationImpacts {
        crate::builders::service_health::ListOrganizationImpacts::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Retrieves a resource containing information about impact to an asset under
    /// an organization affected by a service health event.
    pub fn get_organization_impact(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::service_health::GetOrganizationImpact {
        crate::builders::service_health::GetOrganizationImpact::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::service_health::ListLocations {
        crate::builders::service_health::ListLocations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::service_health::GetLocation {
        crate::builders::service_health::GetLocation::new(self.inner.clone()).set_name(name.into())
    }
}
