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

/// Implements a client for the BeyondCorp API.
///
/// # Service Description
///
/// API Overview:
///
/// The `beyondcorp.googleapis.com` service implements the Google Cloud
/// BeyondCorp API.
///
/// Data Model:
///
/// The AppConnectorsService exposes the following resource:
///
/// * AppConnectors, named as follows:
///   `projects/{project_id}/locations/{location_id}/appConnectors/{app_connector_id}`.
///
/// The AppConnectorsService provides methods to manage
/// (create/read/update/delete) BeyondCorp AppConnectors.
///
/// # Configuration
///
/// `AppConnectorsService` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `AppConnectorsService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `AppConnectorsService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct AppConnectorsService {
    inner: Arc<dyn crate::stubs::dynamic::AppConnectorsService>,
}

impl AppConnectorsService {
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
        T: crate::stubs::AppConnectorsService + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::AppConnectorsService>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::AppConnectorsService> {
        crate::transport::AppConnectorsService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::AppConnectorsService> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::AppConnectorsService::new)
    }

    /// Lists AppConnectors in a given project and location.
    pub fn list_app_connectors(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::app_connectors_service::ListAppConnectors {
        crate::builders::app_connectors_service::ListAppConnectors::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single AppConnector.
    pub fn get_app_connector(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::app_connectors_service::GetAppConnector {
        crate::builders::app_connectors_service::GetAppConnector::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a new AppConnector in a given project and location.
    ///
    /// # Long running operations
    ///
    /// Calling [poller()] on the resulting builder returns an implementation of
    /// the [lro::Poller] trait. You need to call `Poller::poll` on this
    /// `Poller` at least once to start the LRO. You may periodically poll this
    /// object to find the status of the operation. The poller automatically
    /// extract the final response value and any intermediate metadata values.
    ///
    /// Calling [send()] on the resulting builder starts a LRO (long-Running
    /// Operation). LROs run in the background, and the application may poll
    /// them periodically to find out if they have succeeded, or failed. See
    /// below for instructions on how to manually use the resulting [Operation].
    /// We recommend `poller()` in favor of `send()`.
    ///
    /// ## Polling until completion
    ///
    /// Applications that do not care about intermediate results in a
    /// long-running operation may use the [until_done()] function:
    ///
    /// ```
    /// # use gax::Result;
    /// # use google_cloud_beyondcorp_appconnectors_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::AppConnector, model::AppConnectorOperationMetadata>
    /// ) -> Result<model::AppConnector> {
    ///     poller.until_done().await
    /// }
    /// ```
    ///
    /// This will wait until the LRO completes (successfully or with an error).
    /// Applications can set the [PollingPolicy] and [PollingBackoffPolicy] to
    /// control for how long the function runs.
    ///
    /// ## Polling with detailed metadata updates
    ///
    /// Using the result of [poller()] follows a common pattern:
    ///
    /// ```
    /// # use gax::Result;
    /// # use google_cloud_beyondcorp_appconnectors_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::AppConnector, model::AppConnectorOperationMetadata>
    /// ) -> Result<model::AppConnector> {
    ///     while let Some(p) = poller.poll().await {
    ///         match p {
    ///             lro::PollingResult::Completed(r) => { return r; },
    ///             lro::PollingResult::InProgress(m) => { println!("in progress {m:?}"); },
    ///             lro::PollingResult::PollingError(_) => { /* ignored */ },
    ///         }
    ///         tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    ///     }
    ///     Err(gax::error::Error::other("LRO never completed"))
    /// }
    /// ```
    ///
    /// ## Manually polling long-running operations
    ///
    /// If you call [send()], you need to examine the contents of the resulting
    /// [Operation][longrunning::model::Operation] to determine the result of
    /// the operation.
    ///
    /// If the `done` field is `true`, the operation has completed. The `result`
    /// field contains the final response, this will be a [crate::model::AppConnector] (as
    /// an [Any]), or the error (as a `Status`).
    ///
    /// If the `done` field is `false`, the operation has not completed.  The
    /// operation may also include a [crate::model::AppConnectorOperationMetadata] value in the `metadata`
    /// field. This value would also be encoded as an [Any]. The metadata may
    /// include information about how much progress the LRO has made.
    ///
    /// To find out if the operation has completed, use the [get_operation]
    /// method and repeat the steps outlined above.
    ///
    /// Note that most errors on [get_operation] do not indicate that the
    /// long-running operation failed. Long-running operation failures return
    /// the error status in the [result] field.
    ///
    /// [send()]: crate::builders::app_connectors_service::CreateAppConnector::send
    /// [poller()]: crate::builders::app_connectors_service::CreateAppConnector::poller
    /// [until_done()]: lro::Poller::until_done
    /// [PollingPolicy]: gax::polling_policy::PollingPolicy
    /// [PollingBackoffPolicy]: gax::polling_backoff_policy::PollingBackoffPolicy
    /// [get_operation]: Self::get_operation
    /// [metadata]: longrunning::model::Operation::result
    /// [name]: longrunning::model::Operation::name
    /// [Operation]: longrunning::model::Operation
    /// [result]: longrunning::model::Operation::result
    /// [Any]: wkt::Any
    pub fn create_app_connector(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::app_connectors_service::CreateAppConnector {
        crate::builders::app_connectors_service::CreateAppConnector::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates the parameters of a single AppConnector.
    ///
    /// # Long running operations
    ///
    /// Calling [poller()] on the resulting builder returns an implementation of
    /// the [lro::Poller] trait. You need to call `Poller::poll` on this
    /// `Poller` at least once to start the LRO. You may periodically poll this
    /// object to find the status of the operation. The poller automatically
    /// extract the final response value and any intermediate metadata values.
    ///
    /// Calling [send()] on the resulting builder starts a LRO (long-Running
    /// Operation). LROs run in the background, and the application may poll
    /// them periodically to find out if they have succeeded, or failed. See
    /// below for instructions on how to manually use the resulting [Operation].
    /// We recommend `poller()` in favor of `send()`.
    ///
    /// ## Polling until completion
    ///
    /// Applications that do not care about intermediate results in a
    /// long-running operation may use the [until_done()] function:
    ///
    /// ```
    /// # use gax::Result;
    /// # use google_cloud_beyondcorp_appconnectors_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::AppConnector, model::AppConnectorOperationMetadata>
    /// ) -> Result<model::AppConnector> {
    ///     poller.until_done().await
    /// }
    /// ```
    ///
    /// This will wait until the LRO completes (successfully or with an error).
    /// Applications can set the [PollingPolicy] and [PollingBackoffPolicy] to
    /// control for how long the function runs.
    ///
    /// ## Polling with detailed metadata updates
    ///
    /// Using the result of [poller()] follows a common pattern:
    ///
    /// ```
    /// # use gax::Result;
    /// # use google_cloud_beyondcorp_appconnectors_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::AppConnector, model::AppConnectorOperationMetadata>
    /// ) -> Result<model::AppConnector> {
    ///     while let Some(p) = poller.poll().await {
    ///         match p {
    ///             lro::PollingResult::Completed(r) => { return r; },
    ///             lro::PollingResult::InProgress(m) => { println!("in progress {m:?}"); },
    ///             lro::PollingResult::PollingError(_) => { /* ignored */ },
    ///         }
    ///         tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    ///     }
    ///     Err(gax::error::Error::other("LRO never completed"))
    /// }
    /// ```
    ///
    /// ## Manually polling long-running operations
    ///
    /// If you call [send()], you need to examine the contents of the resulting
    /// [Operation][longrunning::model::Operation] to determine the result of
    /// the operation.
    ///
    /// If the `done` field is `true`, the operation has completed. The `result`
    /// field contains the final response, this will be a [crate::model::AppConnector] (as
    /// an [Any]), or the error (as a `Status`).
    ///
    /// If the `done` field is `false`, the operation has not completed.  The
    /// operation may also include a [crate::model::AppConnectorOperationMetadata] value in the `metadata`
    /// field. This value would also be encoded as an [Any]. The metadata may
    /// include information about how much progress the LRO has made.
    ///
    /// To find out if the operation has completed, use the [get_operation]
    /// method and repeat the steps outlined above.
    ///
    /// Note that most errors on [get_operation] do not indicate that the
    /// long-running operation failed. Long-running operation failures return
    /// the error status in the [result] field.
    ///
    /// [send()]: crate::builders::app_connectors_service::UpdateAppConnector::send
    /// [poller()]: crate::builders::app_connectors_service::UpdateAppConnector::poller
    /// [until_done()]: lro::Poller::until_done
    /// [PollingPolicy]: gax::polling_policy::PollingPolicy
    /// [PollingBackoffPolicy]: gax::polling_backoff_policy::PollingBackoffPolicy
    /// [get_operation]: Self::get_operation
    /// [metadata]: longrunning::model::Operation::result
    /// [name]: longrunning::model::Operation::name
    /// [Operation]: longrunning::model::Operation
    /// [result]: longrunning::model::Operation::result
    /// [Any]: wkt::Any
    pub fn update_app_connector(
        &self,
        app_connector: impl Into<crate::model::AppConnector>,
    ) -> crate::builders::app_connectors_service::UpdateAppConnector {
        crate::builders::app_connectors_service::UpdateAppConnector::new(self.inner.clone())
            .set_app_connector(app_connector.into())
    }

    /// Deletes a single AppConnector.
    ///
    /// # Long running operations
    ///
    /// Calling [poller()] on the resulting builder returns an implementation of
    /// the [lro::Poller] trait. You need to call `Poller::poll` on this
    /// `Poller` at least once to start the LRO. You may periodically poll this
    /// object to find the status of the operation. The poller automatically
    /// extract the final response value and any intermediate metadata values.
    ///
    /// Calling [send()] on the resulting builder starts a LRO (long-Running
    /// Operation). LROs run in the background, and the application may poll
    /// them periodically to find out if they have succeeded, or failed. See
    /// below for instructions on how to manually use the resulting [Operation].
    /// We recommend `poller()` in favor of `send()`.
    ///
    /// ## Polling until completion
    ///
    /// Applications that do not care about intermediate results in a
    /// long-running operation may use the [until_done()] function:
    ///
    /// ```
    /// # use gax::Result;
    /// # use google_cloud_beyondcorp_appconnectors_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<wkt::Empty, model::AppConnectorOperationMetadata>
    /// ) -> Result<wkt::Empty> {
    ///     poller.until_done().await
    /// }
    /// ```
    ///
    /// This will wait until the LRO completes (successfully or with an error).
    /// Applications can set the [PollingPolicy] and [PollingBackoffPolicy] to
    /// control for how long the function runs.
    ///
    /// ## Polling with detailed metadata updates
    ///
    /// Using the result of [poller()] follows a common pattern:
    ///
    /// ```
    /// # use gax::Result;
    /// # use google_cloud_beyondcorp_appconnectors_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<wkt::Empty, model::AppConnectorOperationMetadata>
    /// ) -> Result<wkt::Empty> {
    ///     while let Some(p) = poller.poll().await {
    ///         match p {
    ///             lro::PollingResult::Completed(r) => { return r; },
    ///             lro::PollingResult::InProgress(m) => { println!("in progress {m:?}"); },
    ///             lro::PollingResult::PollingError(_) => { /* ignored */ },
    ///         }
    ///         tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    ///     }
    ///     Err(gax::error::Error::other("LRO never completed"))
    /// }
    /// ```
    ///
    /// ## Manually polling long-running operations
    ///
    /// If you call [send()], you need to examine the contents of the resulting
    /// [Operation][longrunning::model::Operation] to determine the result of
    /// the operation.
    ///
    /// If the `done` field is `true`, the operation has completed. The `result`
    /// field contains the final response, this will be a [wkt::Empty] (as
    /// an [Any]), or the error (as a `Status`).
    ///
    /// If the `done` field is `false`, the operation has not completed.  The
    /// operation may also include a [crate::model::AppConnectorOperationMetadata] value in the `metadata`
    /// field. This value would also be encoded as an [Any]. The metadata may
    /// include information about how much progress the LRO has made.
    ///
    /// To find out if the operation has completed, use the [get_operation]
    /// method and repeat the steps outlined above.
    ///
    /// Note that most errors on [get_operation] do not indicate that the
    /// long-running operation failed. Long-running operation failures return
    /// the error status in the [result] field.
    ///
    /// [send()]: crate::builders::app_connectors_service::DeleteAppConnector::send
    /// [poller()]: crate::builders::app_connectors_service::DeleteAppConnector::poller
    /// [until_done()]: lro::Poller::until_done
    /// [PollingPolicy]: gax::polling_policy::PollingPolicy
    /// [PollingBackoffPolicy]: gax::polling_backoff_policy::PollingBackoffPolicy
    /// [get_operation]: Self::get_operation
    /// [metadata]: longrunning::model::Operation::result
    /// [name]: longrunning::model::Operation::name
    /// [Operation]: longrunning::model::Operation
    /// [result]: longrunning::model::Operation::result
    /// [Any]: wkt::Any
    pub fn delete_app_connector(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::app_connectors_service::DeleteAppConnector {
        crate::builders::app_connectors_service::DeleteAppConnector::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Report status for a given connector.
    ///
    /// # Long running operations
    ///
    /// Calling [poller()] on the resulting builder returns an implementation of
    /// the [lro::Poller] trait. You need to call `Poller::poll` on this
    /// `Poller` at least once to start the LRO. You may periodically poll this
    /// object to find the status of the operation. The poller automatically
    /// extract the final response value and any intermediate metadata values.
    ///
    /// Calling [send()] on the resulting builder starts a LRO (long-Running
    /// Operation). LROs run in the background, and the application may poll
    /// them periodically to find out if they have succeeded, or failed. See
    /// below for instructions on how to manually use the resulting [Operation].
    /// We recommend `poller()` in favor of `send()`.
    ///
    /// ## Polling until completion
    ///
    /// Applications that do not care about intermediate results in a
    /// long-running operation may use the [until_done()] function:
    ///
    /// ```
    /// # use gax::Result;
    /// # use google_cloud_beyondcorp_appconnectors_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::AppConnector, model::AppConnectorOperationMetadata>
    /// ) -> Result<model::AppConnector> {
    ///     poller.until_done().await
    /// }
    /// ```
    ///
    /// This will wait until the LRO completes (successfully or with an error).
    /// Applications can set the [PollingPolicy] and [PollingBackoffPolicy] to
    /// control for how long the function runs.
    ///
    /// ## Polling with detailed metadata updates
    ///
    /// Using the result of [poller()] follows a common pattern:
    ///
    /// ```
    /// # use gax::Result;
    /// # use google_cloud_beyondcorp_appconnectors_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::AppConnector, model::AppConnectorOperationMetadata>
    /// ) -> Result<model::AppConnector> {
    ///     while let Some(p) = poller.poll().await {
    ///         match p {
    ///             lro::PollingResult::Completed(r) => { return r; },
    ///             lro::PollingResult::InProgress(m) => { println!("in progress {m:?}"); },
    ///             lro::PollingResult::PollingError(_) => { /* ignored */ },
    ///         }
    ///         tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    ///     }
    ///     Err(gax::error::Error::other("LRO never completed"))
    /// }
    /// ```
    ///
    /// ## Manually polling long-running operations
    ///
    /// If you call [send()], you need to examine the contents of the resulting
    /// [Operation][longrunning::model::Operation] to determine the result of
    /// the operation.
    ///
    /// If the `done` field is `true`, the operation has completed. The `result`
    /// field contains the final response, this will be a [crate::model::AppConnector] (as
    /// an [Any]), or the error (as a `Status`).
    ///
    /// If the `done` field is `false`, the operation has not completed.  The
    /// operation may also include a [crate::model::AppConnectorOperationMetadata] value in the `metadata`
    /// field. This value would also be encoded as an [Any]. The metadata may
    /// include information about how much progress the LRO has made.
    ///
    /// To find out if the operation has completed, use the [get_operation]
    /// method and repeat the steps outlined above.
    ///
    /// Note that most errors on [get_operation] do not indicate that the
    /// long-running operation failed. Long-running operation failures return
    /// the error status in the [result] field.
    ///
    /// [send()]: crate::builders::app_connectors_service::ReportStatus::send
    /// [poller()]: crate::builders::app_connectors_service::ReportStatus::poller
    /// [until_done()]: lro::Poller::until_done
    /// [PollingPolicy]: gax::polling_policy::PollingPolicy
    /// [PollingBackoffPolicy]: gax::polling_backoff_policy::PollingBackoffPolicy
    /// [get_operation]: Self::get_operation
    /// [metadata]: longrunning::model::Operation::result
    /// [name]: longrunning::model::Operation::name
    /// [Operation]: longrunning::model::Operation
    /// [result]: longrunning::model::Operation::result
    /// [Any]: wkt::Any
    pub fn report_status(
        &self,
        app_connector: impl Into<std::string::String>,
    ) -> crate::builders::app_connectors_service::ReportStatus {
        crate::builders::app_connectors_service::ReportStatus::new(self.inner.clone())
            .set_app_connector(app_connector.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::app_connectors_service::ListLocations {
        crate::builders::app_connectors_service::ListLocations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::app_connectors_service::GetLocation {
        crate::builders::app_connectors_service::GetLocation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Sets the access control policy on the specified resource. Replaces
    /// any existing policy.
    ///
    /// Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED`
    /// errors.
    pub fn set_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> crate::builders::app_connectors_service::SetIamPolicy {
        crate::builders::app_connectors_service::SetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Gets the access control policy for a resource. Returns an empty policy
    /// if the resource exists and does not have a policy set.
    pub fn get_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> crate::builders::app_connectors_service::GetIamPolicy {
        crate::builders::app_connectors_service::GetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Returns permissions that a caller has on the specified resource. If the
    /// resource does not exist, this will return an empty set of
    /// permissions, not a `NOT_FOUND` error.
    ///
    /// Note: This operation is designed to be used for building
    /// permission-aware UIs and command-line tools, not for authorization
    /// checking. This operation may "fail open" without warning.
    pub fn test_iam_permissions(
        &self,
        resource: impl Into<std::string::String>,
    ) -> crate::builders::app_connectors_service::TestIamPermissions {
        crate::builders::app_connectors_service::TestIamPermissions::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::app_connectors_service::ListOperations {
        crate::builders::app_connectors_service::ListOperations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::app_connectors_service::GetOperation {
        crate::builders::app_connectors_service::GetOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::app_connectors_service::DeleteOperation {
        crate::builders::app_connectors_service::DeleteOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::app_connectors_service::CancelOperation {
        crate::builders::app_connectors_service::CancelOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
