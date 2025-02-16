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

/// A dyn-compatible, crate-private version of [super::ServiceHealth].
#[async_trait::async_trait]
pub trait ServiceHealth: std::fmt::Debug + Send + Sync {
    async fn list_events(
        &self,
        req: crate::model::ListEventsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListEventsResponse>;

    async fn get_event(
        &self,
        req: crate::model::GetEventRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Event>;

    async fn list_organization_events(
        &self,
        req: crate::model::ListOrganizationEventsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListOrganizationEventsResponse>;

    async fn get_organization_event(
        &self,
        req: crate::model::GetOrganizationEventRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::OrganizationEvent>;

    async fn list_organization_impacts(
        &self,
        req: crate::model::ListOrganizationImpactsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListOrganizationImpactsResponse>;

    async fn get_organization_impact(
        &self,
        req: crate::model::GetOrganizationImpactRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::OrganizationImpact>;

    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::ListLocationsResponse>;

    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::Location>;
}

/// All implementations of [crate::stubs::ServiceHealth] also implement [ServiceHealth].
#[async_trait::async_trait]
impl<T: crate::stubs::ServiceHealth> ServiceHealth for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_events(
        &self,
        req: crate::model::ListEventsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListEventsResponse> {
        T::list_events(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_event(
        &self,
        req: crate::model::GetEventRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Event> {
        T::get_event(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_organization_events(
        &self,
        req: crate::model::ListOrganizationEventsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListOrganizationEventsResponse> {
        T::list_organization_events(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_organization_event(
        &self,
        req: crate::model::GetOrganizationEventRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::OrganizationEvent> {
        T::get_organization_event(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_organization_impacts(
        &self,
        req: crate::model::ListOrganizationImpactsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListOrganizationImpactsResponse> {
        T::list_organization_impacts(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_organization_impact(
        &self,
        req: crate::model::GetOrganizationImpactRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::OrganizationImpact> {
        T::get_organization_impact(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::ListLocationsResponse> {
        T::list_locations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::Location> {
        T::get_location(self, req, options).await
    }
}
