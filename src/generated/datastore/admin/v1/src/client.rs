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

/// Implements a client for the Cloud Datastore API.
///
/// # Service Description
///
/// Google Cloud Datastore Admin API
///
/// The Datastore Admin API provides several admin services for Cloud Datastore.
///
/// Concepts: Project, namespace, kind, and entity as defined in the Google Cloud
/// Datastore API.
///
/// Operation: An Operation represents work being performed in the background.
///
/// EntityFilter: Allows specifying a subset of entities in a project. This is
/// specified as a combination of kinds and namespaces (either or both of which
/// may be all).
///
/// Export/Import Service:
///
/// - The Export/Import service provides the ability to copy all or a subset of
///   entities to/from Google Cloud Storage.
/// - Exported data may be imported into Cloud Datastore for any Google Cloud
///   Platform project. It is not restricted to the export source project. It is
///   possible to export from one project and then import into another.
/// - Exported data can also be loaded into Google BigQuery for analysis.
/// - Exports and imports are performed asynchronously. An Operation resource is
///   created for each export/import. The state (including any errors encountered)
///   of the export/import may be queried via the Operation resource.
///
/// Index Service:
///
/// - The index service manages Cloud Datastore composite indexes.
/// - Index creation and deletion are performed asynchronously.
///   An Operation resource is created for each such asynchronous operation.
///   The state of the operation (including any errors encountered)
///   may be queried via the Operation resource.
///
/// Operation Service:
///
/// - The Operations collection provides a record of actions performed for the
///   specified project (including any operations in progress). Operations are not
///   created directly but through calls on other collections or resources.
/// - An operation that is not yet done may be cancelled. The request to cancel
///   is asynchronous and the operation may continue to run for some time after the
///   request to cancel is made.
/// - An operation that is done may be deleted so that it is no longer listed as
///   part of the Operation collection.
/// - ListOperations returns all pending operations, but not completed
///   operations.
/// - Operations are created by service DatastoreAdmin, but are accessed via
///   service google.longrunning.Operations.
///
/// # Configuration
///
/// `DatastoreAdmin` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `DatastoreAdmin` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `DatastoreAdmin` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct DatastoreAdmin {
    inner: Arc<dyn crate::stubs::dynamic::DatastoreAdmin>,
}

impl DatastoreAdmin {
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
        T: crate::stubs::DatastoreAdmin + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::DatastoreAdmin>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::DatastoreAdmin> {
        crate::transport::DatastoreAdmin::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::DatastoreAdmin> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::DatastoreAdmin::new)
    }

    /// Exports a copy of all or a subset of entities from Google Cloud Datastore
    /// to another storage system, such as Google Cloud Storage. Recent updates to
    /// entities may not be reflected in the export. The export occurs in the
    /// background and its progress can be monitored and managed via the
    /// Operation resource that is created. The output of an export may only be
    /// used once the associated operation is done. If an export operation is
    /// cancelled before completion it may leave partial data behind in Google
    /// Cloud Storage.
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
    /// # use gcp_sdk_datastore_admin_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::ExportEntitiesResponse, model::ExportEntitiesMetadata>
    /// ) -> Result<model::ExportEntitiesResponse> {
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
    /// # use gcp_sdk_datastore_admin_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::ExportEntitiesResponse, model::ExportEntitiesMetadata>
    /// ) -> Result<model::ExportEntitiesResponse> {
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
    /// field contains the final response, this will be a [crate::model::ExportEntitiesResponse] (as
    /// an [Any]), or the error (as a `Status`).
    ///
    /// If the `done` field is `false`, the operation has not completed.  The
    /// operation may also include a [crate::model::ExportEntitiesMetadata] value in the `metadata`
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
    /// [send()]: crate::builders::datastore_admin::ExportEntities::send
    /// [poller()]: crate::builders::datastore_admin::ExportEntities::poller
    /// [until_done()]: lro::Poller::until_done
    /// [PollingPolicy]: gax::polling_policy::PollingPolicy
    /// [PollingBackoffPolicy]: gax::polling_backoff_policy::PollingBackoffPolicy
    /// [get_operation]: Self::get_operation
    /// [metadata]: longrunning::model::Operation::result
    /// [name]: longrunning::model::Operation::name
    /// [Operation]: longrunning::model::Operation
    /// [result]: longrunning::model::Operation::result
    /// [Any]: wkt::Any
    pub fn export_entities(
        &self,
        project_id: impl Into<std::string::String>,
    ) -> crate::builders::datastore_admin::ExportEntities {
        crate::builders::datastore_admin::ExportEntities::new(self.inner.clone())
            .set_project_id(project_id.into())
    }

    /// Imports entities into Google Cloud Datastore. Existing entities with the
    /// same key are overwritten. The import occurs in the background and its
    /// progress can be monitored and managed via the Operation resource that is
    /// created. If an ImportEntities operation is cancelled, it is possible
    /// that a subset of the data has already been imported to Cloud Datastore.
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
    /// # use gcp_sdk_datastore_admin_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<wkt::Empty, model::ImportEntitiesMetadata>
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
    /// # use gcp_sdk_datastore_admin_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<wkt::Empty, model::ImportEntitiesMetadata>
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
    /// operation may also include a [crate::model::ImportEntitiesMetadata] value in the `metadata`
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
    /// [send()]: crate::builders::datastore_admin::ImportEntities::send
    /// [poller()]: crate::builders::datastore_admin::ImportEntities::poller
    /// [until_done()]: lro::Poller::until_done
    /// [PollingPolicy]: gax::polling_policy::PollingPolicy
    /// [PollingBackoffPolicy]: gax::polling_backoff_policy::PollingBackoffPolicy
    /// [get_operation]: Self::get_operation
    /// [metadata]: longrunning::model::Operation::result
    /// [name]: longrunning::model::Operation::name
    /// [Operation]: longrunning::model::Operation
    /// [result]: longrunning::model::Operation::result
    /// [Any]: wkt::Any
    pub fn import_entities(
        &self,
        project_id: impl Into<std::string::String>,
    ) -> crate::builders::datastore_admin::ImportEntities {
        crate::builders::datastore_admin::ImportEntities::new(self.inner.clone())
            .set_project_id(project_id.into())
    }

    /// Creates the specified index.
    /// A newly created index's initial state is `CREATING`. On completion of the
    /// returned [google.longrunning.Operation][google.longrunning.Operation], the
    /// state will be `READY`. If the index already exists, the call will return an
    /// `ALREADY_EXISTS` status.
    ///
    /// During index creation, the process could result in an error, in which
    /// case the index will move to the `ERROR` state. The process can be recovered
    /// by fixing the data that caused the error, removing the index with
    /// [delete][google.datastore.admin.v1.DatastoreAdmin.DeleteIndex], then
    /// re-creating the index with [create]
    /// [google.datastore.admin.v1.DatastoreAdmin.CreateIndex].
    ///
    /// Indexes with a single property cannot be created.
    ///
    /// [google.datastore.admin.v1.DatastoreAdmin.DeleteIndex]: crate::client::DatastoreAdmin::delete_index
    /// [google.longrunning.Operation]: longrunning::model::Operation
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
    /// # use gcp_sdk_datastore_admin_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::Index, model::IndexOperationMetadata>
    /// ) -> Result<model::Index> {
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
    /// # use gcp_sdk_datastore_admin_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::Index, model::IndexOperationMetadata>
    /// ) -> Result<model::Index> {
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
    /// field contains the final response, this will be a [crate::model::Index] (as
    /// an [Any]), or the error (as a `Status`).
    ///
    /// If the `done` field is `false`, the operation has not completed.  The
    /// operation may also include a [crate::model::IndexOperationMetadata] value in the `metadata`
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
    /// [send()]: crate::builders::datastore_admin::CreateIndex::send
    /// [poller()]: crate::builders::datastore_admin::CreateIndex::poller
    /// [until_done()]: lro::Poller::until_done
    /// [PollingPolicy]: gax::polling_policy::PollingPolicy
    /// [PollingBackoffPolicy]: gax::polling_backoff_policy::PollingBackoffPolicy
    /// [get_operation]: Self::get_operation
    /// [metadata]: longrunning::model::Operation::result
    /// [name]: longrunning::model::Operation::name
    /// [Operation]: longrunning::model::Operation
    /// [result]: longrunning::model::Operation::result
    /// [Any]: wkt::Any
    pub fn create_index(
        &self,
        project_id: impl Into<std::string::String>,
    ) -> crate::builders::datastore_admin::CreateIndex {
        crate::builders::datastore_admin::CreateIndex::new(self.inner.clone())
            .set_project_id(project_id.into())
    }

    /// Deletes an existing index.
    /// An index can only be deleted if it is in a `READY` or `ERROR` state. On
    /// successful execution of the request, the index will be in a `DELETING`
    /// [state][google.datastore.admin.v1.Index.State]. And on completion of the
    /// returned [google.longrunning.Operation][google.longrunning.Operation], the
    /// index will be removed.
    ///
    /// During index deletion, the process could result in an error, in which
    /// case the index will move to the `ERROR` state. The process can be recovered
    /// by fixing the data that caused the error, followed by calling
    /// [delete][google.datastore.admin.v1.DatastoreAdmin.DeleteIndex] again.
    ///
    /// [google.datastore.admin.v1.DatastoreAdmin.DeleteIndex]: crate::client::DatastoreAdmin::delete_index
    /// [google.datastore.admin.v1.Index.State]: crate::model::index::State
    /// [google.longrunning.Operation]: longrunning::model::Operation
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
    /// # use gcp_sdk_datastore_admin_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::Index, model::IndexOperationMetadata>
    /// ) -> Result<model::Index> {
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
    /// # use gcp_sdk_datastore_admin_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::Index, model::IndexOperationMetadata>
    /// ) -> Result<model::Index> {
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
    /// field contains the final response, this will be a [crate::model::Index] (as
    /// an [Any]), or the error (as a `Status`).
    ///
    /// If the `done` field is `false`, the operation has not completed.  The
    /// operation may also include a [crate::model::IndexOperationMetadata] value in the `metadata`
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
    /// [send()]: crate::builders::datastore_admin::DeleteIndex::send
    /// [poller()]: crate::builders::datastore_admin::DeleteIndex::poller
    /// [until_done()]: lro::Poller::until_done
    /// [PollingPolicy]: gax::polling_policy::PollingPolicy
    /// [PollingBackoffPolicy]: gax::polling_backoff_policy::PollingBackoffPolicy
    /// [get_operation]: Self::get_operation
    /// [metadata]: longrunning::model::Operation::result
    /// [name]: longrunning::model::Operation::name
    /// [Operation]: longrunning::model::Operation
    /// [result]: longrunning::model::Operation::result
    /// [Any]: wkt::Any
    pub fn delete_index(
        &self,
        project_id: impl Into<std::string::String>,
        index_id: impl Into<std::string::String>,
    ) -> crate::builders::datastore_admin::DeleteIndex {
        crate::builders::datastore_admin::DeleteIndex::new(self.inner.clone())
            .set_project_id(project_id.into())
            .set_index_id(index_id.into())
    }

    /// Gets an index.
    pub fn get_index(
        &self,
        project_id: impl Into<std::string::String>,
        index_id: impl Into<std::string::String>,
    ) -> crate::builders::datastore_admin::GetIndex {
        crate::builders::datastore_admin::GetIndex::new(self.inner.clone())
            .set_project_id(project_id.into())
            .set_index_id(index_id.into())
    }

    /// Lists the indexes that match the specified filters.  Datastore uses an
    /// eventually consistent query to fetch the list of indexes and may
    /// occasionally return stale results.
    pub fn list_indexes(
        &self,
        project_id: impl Into<std::string::String>,
    ) -> crate::builders::datastore_admin::ListIndexes {
        crate::builders::datastore_admin::ListIndexes::new(self.inner.clone())
            .set_project_id(project_id.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::datastore_admin::ListOperations {
        crate::builders::datastore_admin::ListOperations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::datastore_admin::GetOperation {
        crate::builders::datastore_admin::GetOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::datastore_admin::DeleteOperation {
        crate::builders::datastore_admin::DeleteOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::datastore_admin::CancelOperation {
        crate::builders::datastore_admin::CancelOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
