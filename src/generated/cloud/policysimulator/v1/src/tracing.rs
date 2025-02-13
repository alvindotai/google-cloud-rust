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

/// Implements a [Simulator](crate::stubs::Simulator) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct Simulator<T>
where
    T: crate::stubs::Simulator + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> Simulator<T>
where
    T: crate::stubs::Simulator + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::stubs::Simulator for Simulator<T>
where
    T: crate::stubs::Simulator + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn get_replay(
        &self,
        req: crate::model::GetReplayRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Replay> {
        self.inner.get_replay(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_replay(
        &self,
        req: crate::model::CreateReplayRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_replay(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_replay_results(
        &self,
        req: crate::model::ListReplayResultsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListReplayResultsResponse> {
        self.inner.list_replay_results(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::ListOperationsResponse> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
    }

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_policy::PollingPolicy> {
        self.inner.get_polling_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}
