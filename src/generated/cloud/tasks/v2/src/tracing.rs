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
use crate::Result;

/// Implements a [CloudTasks](crate::stubs::) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct CloudTasks<T>
where
    T: crate::stubs::CloudTasks + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> CloudTasks<T>
where
    T: crate::stubs::CloudTasks + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::stubs::CloudTasks for CloudTasks<T>
where
    T: crate::stubs::CloudTasks + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_queues(
        &self,
        req: crate::model::ListQueuesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListQueuesResponse> {
        self.inner.list_queues(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_queue(
        &self,
        req: crate::model::GetQueueRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Queue> {
        self.inner.get_queue(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_queue(
        &self,
        req: crate::model::CreateQueueRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Queue> {
        self.inner.create_queue(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_queue(
        &self,
        req: crate::model::UpdateQueueRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Queue> {
        self.inner.update_queue(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_queue(
        &self,
        req: crate::model::DeleteQueueRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_queue(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn purge_queue(
        &self,
        req: crate::model::PurgeQueueRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Queue> {
        self.inner.purge_queue(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn pause_queue(
        &self,
        req: crate::model::PauseQueueRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Queue> {
        self.inner.pause_queue(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn resume_queue(
        &self,
        req: crate::model::ResumeQueueRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Queue> {
        self.inner.resume_queue(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::TestIamPermissionsResponse> {
        self.inner.test_iam_permissions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_tasks(
        &self,
        req: crate::model::ListTasksRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListTasksResponse> {
        self.inner.list_tasks(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_task(
        &self,
        req: crate::model::GetTaskRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Task> {
        self.inner.get_task(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_task(
        &self,
        req: crate::model::CreateTaskRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Task> {
        self.inner.create_task(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_task(
        &self,
        req: crate::model::DeleteTaskRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_task(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn run_task(
        &self,
        req: crate::model::RunTaskRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Task> {
        self.inner.run_task(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::ListLocationsResponse> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::Location> {
        self.inner.get_location(req, options).await
    }
}
