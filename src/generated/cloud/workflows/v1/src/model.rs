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

#![allow(rustdoc::redundant_explicit_links)]
#![allow(rustdoc::broken_intra_doc_links)]
#![no_implicit_prelude]
extern crate async_trait;
extern crate bytes;
extern crate gax;
extern crate lazy_static;
extern crate location;
extern crate longrunning;
extern crate lro;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_with;
extern crate std;
extern crate tracing;
extern crate wkt;

/// Workflow program to be executed by Workflows.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct Workflow {
    /// The resource name of the workflow.
    /// Format: projects/{project}/locations/{location}/workflows/{workflow}
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// Description of the workflow provided by the user.
    /// Must be at most 1000 unicode characters long.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub description: std::string::String,

    /// Output only. State of the workflow deployment.
    pub state: crate::model::workflow::State,

    /// Output only. The revision of the workflow.
    /// A new revision of a workflow is created as a result of updating the
    /// following properties of a workflow:
    ///
    /// - [Service account][google.cloud.workflows.v1.Workflow.service_account]
    /// - [Workflow code to be
    ///   executed][google.cloud.workflows.v1.Workflow.source_contents]
    ///
    /// The format is "000001-a4d", where the first six characters define
    /// the zero-padded revision ordinal number. They are followed by a hyphen and
    /// three hexadecimal random characters.
    ///
    /// [google.cloud.workflows.v1.Workflow.service_account]: crate::model::Workflow::service_account
    /// [google.cloud.workflows.v1.Workflow.source_contents]: crate::model::Workflow::source_code
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub revision_id: std::string::String,

    /// Output only. The timestamp for when the workflow was created.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub create_time: std::option::Option<wkt::Timestamp>,

    /// Output only. The timestamp for when the workflow was last updated.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub update_time: std::option::Option<wkt::Timestamp>,

    /// Output only. The timestamp for the latest revision of the workflow's
    /// creation.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub revision_create_time: std::option::Option<wkt::Timestamp>,

    /// Labels associated with this workflow.
    /// Labels can contain at most 64 entries. Keys and values can be no longer
    /// than 63 characters and can only contain lowercase letters, numeric
    /// characters, underscores, and dashes. Label keys must start with a letter.
    /// International characters are allowed.
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub labels: std::collections::HashMap<std::string::String, std::string::String>,

    /// The service account associated with the latest workflow version.
    /// This service account represents the identity of the workflow and determines
    /// what permissions the workflow has.
    /// Format: projects/{project}/serviceAccounts/{account} or {account}
    ///
    /// Using `-` as a wildcard for the `{project}` or not providing one at all
    /// will infer the project from the account. The `{account}` value can be the
    /// `email` address or the `unique_id` of the service account.
    ///
    /// If not provided, workflow will use the project's default service account.
    /// Modifying this field for an existing workflow results in a new workflow
    /// revision.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub service_account: std::string::String,

    /// Optional. The resource name of a KMS crypto key used to encrypt or decrypt
    /// the data associated with the workflow.
    ///
    /// Format:
    /// projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{cryptoKey}
    ///
    /// Using `-` as a wildcard for the `{project}` or not providing one at all
    /// will infer the project from the account.
    ///
    /// If not provided, data associated with the workflow will not be
    /// CMEK-encrypted.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub crypto_key_name: std::string::String,

    /// Output only. Error regarding the state of the workflow. For example, this
    /// field will have error details if the execution data is unavailable due to
    /// revoked KMS key permissions.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub state_error: std::option::Option<crate::model::workflow::StateError>,

    /// Optional. Describes the level of platform logging to apply to calls and
    /// call responses during executions of this workflow. If both the workflow and
    /// the execution specify a logging level, the execution level takes
    /// precedence.
    pub call_log_level: crate::model::workflow::CallLogLevel,

    /// Optional. User-defined environment variables associated with this workflow
    /// revision. This map has a maximum length of 20. Each string can take up to
    /// 40KiB. Keys cannot be empty strings and cannot start with “GOOGLE” or
    /// “WORKFLOWS".
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub user_env_vars: std::collections::HashMap<std::string::String, std::string::String>,

    /// Required. Location of the workflow source code.
    /// Modifying this field for an existing workflow results in a new workflow
    /// revision.
    #[serde(flatten, skip_serializing_if = "std::option::Option::is_none")]
    pub source_code: std::option::Option<crate::model::workflow::SourceCode>,
}

impl Workflow {
    /// Sets the value of [name][crate::model::Workflow::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [description][crate::model::Workflow::description].
    pub fn set_description<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.description = v.into();
        self
    }

    /// Sets the value of [state][crate::model::Workflow::state].
    pub fn set_state<T: std::convert::Into<crate::model::workflow::State>>(mut self, v: T) -> Self {
        self.state = v.into();
        self
    }

    /// Sets the value of [revision_id][crate::model::Workflow::revision_id].
    pub fn set_revision_id<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.revision_id = v.into();
        self
    }

    /// Sets the value of [create_time][crate::model::Workflow::create_time].
    pub fn set_create_time<T: std::convert::Into<std::option::Option<wkt::Timestamp>>>(
        mut self,
        v: T,
    ) -> Self {
        self.create_time = v.into();
        self
    }

    /// Sets the value of [update_time][crate::model::Workflow::update_time].
    pub fn set_update_time<T: std::convert::Into<std::option::Option<wkt::Timestamp>>>(
        mut self,
        v: T,
    ) -> Self {
        self.update_time = v.into();
        self
    }

    /// Sets the value of [revision_create_time][crate::model::Workflow::revision_create_time].
    pub fn set_revision_create_time<T: std::convert::Into<std::option::Option<wkt::Timestamp>>>(
        mut self,
        v: T,
    ) -> Self {
        self.revision_create_time = v.into();
        self
    }

    /// Sets the value of [service_account][crate::model::Workflow::service_account].
    pub fn set_service_account<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.service_account = v.into();
        self
    }

    /// Sets the value of [crypto_key_name][crate::model::Workflow::crypto_key_name].
    pub fn set_crypto_key_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.crypto_key_name = v.into();
        self
    }

    /// Sets the value of [state_error][crate::model::Workflow::state_error].
    pub fn set_state_error<
        T: std::convert::Into<std::option::Option<crate::model::workflow::StateError>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.state_error = v.into();
        self
    }

    /// Sets the value of [call_log_level][crate::model::Workflow::call_log_level].
    pub fn set_call_log_level<T: std::convert::Into<crate::model::workflow::CallLogLevel>>(
        mut self,
        v: T,
    ) -> Self {
        self.call_log_level = v.into();
        self
    }

    /// Sets the value of [labels][crate::model::Workflow::labels].
    pub fn set_labels<T, K, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = (K, V)>,
        K: std::convert::Into<std::string::String>,
        V: std::convert::Into<std::string::String>,
    {
        use std::iter::Iterator;
        self.labels = v.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
        self
    }

    /// Sets the value of [user_env_vars][crate::model::Workflow::user_env_vars].
    pub fn set_user_env_vars<T, K, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = (K, V)>,
        K: std::convert::Into<std::string::String>,
        V: std::convert::Into<std::string::String>,
    {
        use std::iter::Iterator;
        self.user_env_vars = v.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
        self
    }

    /// Sets the value of `source_code`.
    pub fn set_source_code<
        T: std::convert::Into<std::option::Option<crate::model::workflow::SourceCode>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.source_code = v.into();
        self
    }

    /// The value of [source_code][crate::model::Workflow::source_code]
    /// if it holds a `SourceContents`, `None` if the field is not set or
    /// holds a different branch.
    pub fn get_source_contents(&self) -> std::option::Option<&std::string::String> {
        #[allow(unreachable_patterns)]
        self.source_code.as_ref().and_then(|v| match v {
            crate::model::workflow::SourceCode::SourceContents(v) => std::option::Option::Some(v),
            _ => std::option::Option::None,
        })
    }

    /// Sets the value of [source_code][crate::model::Workflow::source_code]
    /// to hold a `SourceContents`.
    ///
    /// Note that all the setters affecting `source_code` are
    /// mutually exclusive.
    pub fn set_source_contents<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.source_code =
            std::option::Option::Some(crate::model::workflow::SourceCode::SourceContents(v.into()));
        self
    }
}

impl wkt::message::Message for Workflow {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.workflows.v1.Workflow"
    }
}

/// Defines additional types related to Workflow
pub mod workflow {
    #[allow(unused_imports)]
    use super::*;

    /// Describes an error related to the current state of the workflow.
    #[serde_with::serde_as]
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(default, rename_all = "camelCase")]
    #[non_exhaustive]
    pub struct StateError {
        /// Provides specifics about the error.
        #[serde(skip_serializing_if = "std::string::String::is_empty")]
        pub details: std::string::String,

        /// The type of this state error.
        #[serde(rename = "type")]
        pub r#type: crate::model::workflow::state_error::Type,
    }

    impl StateError {
        /// Sets the value of [details][crate::model::workflow::StateError::details].
        pub fn set_details<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
            self.details = v.into();
            self
        }

        /// Sets the value of [r#type][crate::model::workflow::StateError::type].
        pub fn set_type<T: std::convert::Into<crate::model::workflow::state_error::Type>>(
            mut self,
            v: T,
        ) -> Self {
            self.r#type = v.into();
            self
        }
    }

    impl wkt::message::Message for StateError {
        fn typename() -> &'static str {
            "type.googleapis.com/google.cloud.workflows.v1.Workflow.StateError"
        }
    }

    /// Defines additional types related to StateError
    pub mod state_error {
        #[allow(unused_imports)]
        use super::*;

        /// Describes the possibled types of a state error.
        #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
        pub struct Type(std::borrow::Cow<'static, str>);

        impl Type {
            /// Creates a new Type instance.
            pub const fn new(v: &'static str) -> Self {
                Self(std::borrow::Cow::Borrowed(v))
            }

            /// Gets the enum value.
            pub fn value(&self) -> &str {
                &self.0
            }
        }

        /// Useful constants to work with [Type](Type)
        pub mod r#type {
            use super::Type;

            /// No type specified.
            pub const TYPE_UNSPECIFIED: Type = Type::new("TYPE_UNSPECIFIED");

            /// Caused by an issue with KMS.
            pub const KMS_ERROR: Type = Type::new("KMS_ERROR");
        }

        impl std::convert::From<std::string::String> for Type {
            fn from(value: std::string::String) -> Self {
                Self(std::borrow::Cow::Owned(value))
            }
        }
    }

    /// Describes the current state of workflow deployment.
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct State(std::borrow::Cow<'static, str>);

    impl State {
        /// Creates a new State instance.
        pub const fn new(v: &'static str) -> Self {
            Self(std::borrow::Cow::Borrowed(v))
        }

        /// Gets the enum value.
        pub fn value(&self) -> &str {
            &self.0
        }
    }

    /// Useful constants to work with [State](State)
    pub mod state {
        use super::State;

        /// Invalid state.
        pub const STATE_UNSPECIFIED: State = State::new("STATE_UNSPECIFIED");

        /// The workflow has been deployed successfully and is serving.
        pub const ACTIVE: State = State::new("ACTIVE");

        /// Workflow data is unavailable. See the `state_error` field.
        pub const UNAVAILABLE: State = State::new("UNAVAILABLE");
    }

    impl std::convert::From<std::string::String> for State {
        fn from(value: std::string::String) -> Self {
            Self(std::borrow::Cow::Owned(value))
        }
    }

    /// Describes the level of platform logging to apply to calls and call
    /// responses during workflow executions.
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct CallLogLevel(std::borrow::Cow<'static, str>);

    impl CallLogLevel {
        /// Creates a new CallLogLevel instance.
        pub const fn new(v: &'static str) -> Self {
            Self(std::borrow::Cow::Borrowed(v))
        }

        /// Gets the enum value.
        pub fn value(&self) -> &str {
            &self.0
        }
    }

    /// Useful constants to work with [CallLogLevel](CallLogLevel)
    pub mod call_log_level {
        use super::CallLogLevel;

        /// No call logging level specified.
        pub const CALL_LOG_LEVEL_UNSPECIFIED: CallLogLevel =
            CallLogLevel::new("CALL_LOG_LEVEL_UNSPECIFIED");

        /// Log all call steps within workflows, all call returns, and all exceptions
        /// raised.
        pub const LOG_ALL_CALLS: CallLogLevel = CallLogLevel::new("LOG_ALL_CALLS");

        /// Log only exceptions that are raised from call steps within workflows.
        pub const LOG_ERRORS_ONLY: CallLogLevel = CallLogLevel::new("LOG_ERRORS_ONLY");

        /// Explicitly log nothing.
        pub const LOG_NONE: CallLogLevel = CallLogLevel::new("LOG_NONE");
    }

    impl std::convert::From<std::string::String> for CallLogLevel {
        fn from(value: std::string::String) -> Self {
            Self(std::borrow::Cow::Owned(value))
        }
    }

    /// Required. Location of the workflow source code.
    /// Modifying this field for an existing workflow results in a new workflow
    /// revision.
    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    #[non_exhaustive]
    pub enum SourceCode {
        /// Workflow code to be executed. The size limit is 128KB.
        SourceContents(std::string::String),
    }
}

/// Request for the
/// [ListWorkflows][google.cloud.workflows.v1.Workflows.ListWorkflows]
/// method.
///
/// [google.cloud.workflows.v1.Workflows.ListWorkflows]: crate::client::Workflows::list_workflows
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListWorkflowsRequest {
    /// Required. Project and location from which the workflows should be listed.
    /// Format: projects/{project}/locations/{location}
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub parent: std::string::String,

    /// Maximum number of workflows to return per call. The service might return
    /// fewer than this value even if not at the end of the collection. If a value
    /// is not specified, a default value of 500 is used. The maximum permitted
    /// value is 1000 and values greater than 1000 are coerced down to 1000.
    pub page_size: i32,

    /// A page token, received from a previous `ListWorkflows` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListWorkflows` must
    /// match the call that provided the page token.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub page_token: std::string::String,

    /// Filter to restrict results to specific workflows.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub filter: std::string::String,

    /// Comma-separated list of fields that specify the order of the results.
    /// Default sorting order for a field is ascending. To specify descending order
    /// for a field, append a "desc" suffix.
    /// If not specified, the results are returned in an unspecified order.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub order_by: std::string::String,
}

impl ListWorkflowsRequest {
    /// Sets the value of [parent][crate::model::ListWorkflowsRequest::parent].
    pub fn set_parent<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.parent = v.into();
        self
    }

    /// Sets the value of [page_size][crate::model::ListWorkflowsRequest::page_size].
    pub fn set_page_size<T: std::convert::Into<i32>>(mut self, v: T) -> Self {
        self.page_size = v.into();
        self
    }

    /// Sets the value of [page_token][crate::model::ListWorkflowsRequest::page_token].
    pub fn set_page_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.page_token = v.into();
        self
    }

    /// Sets the value of [filter][crate::model::ListWorkflowsRequest::filter].
    pub fn set_filter<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.filter = v.into();
        self
    }

    /// Sets the value of [order_by][crate::model::ListWorkflowsRequest::order_by].
    pub fn set_order_by<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.order_by = v.into();
        self
    }
}

impl wkt::message::Message for ListWorkflowsRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.workflows.v1.ListWorkflowsRequest"
    }
}

/// Response for the
/// [ListWorkflows][google.cloud.workflows.v1.Workflows.ListWorkflows]
/// method.
///
/// [google.cloud.workflows.v1.Workflows.ListWorkflows]: crate::client::Workflows::list_workflows
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListWorkflowsResponse {
    /// The workflows that match the request.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub workflows: std::vec::Vec<crate::model::Workflow>,

    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub next_page_token: std::string::String,

    /// Unreachable resources.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub unreachable: std::vec::Vec<std::string::String>,
}

impl ListWorkflowsResponse {
    /// Sets the value of [next_page_token][crate::model::ListWorkflowsResponse::next_page_token].
    pub fn set_next_page_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.next_page_token = v.into();
        self
    }

    /// Sets the value of [workflows][crate::model::ListWorkflowsResponse::workflows].
    pub fn set_workflows<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<crate::model::Workflow>,
    {
        use std::iter::Iterator;
        self.workflows = v.into_iter().map(|i| i.into()).collect();
        self
    }

    /// Sets the value of [unreachable][crate::model::ListWorkflowsResponse::unreachable].
    pub fn set_unreachable<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<std::string::String>,
    {
        use std::iter::Iterator;
        self.unreachable = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for ListWorkflowsResponse {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.workflows.v1.ListWorkflowsResponse"
    }
}

#[cfg(feature = "unstable-stream")]
impl gax::paginator::PageableResponse for ListWorkflowsResponse {
    type PageItem = crate::model::Workflow;

    fn items(self) -> std::vec::Vec<Self::PageItem> {
        self.workflows
    }

    fn next_page_token(&self) -> std::string::String {
        gax::paginator::extract_token(&self.next_page_token)
    }
}

/// Request for the
/// [GetWorkflow][google.cloud.workflows.v1.Workflows.GetWorkflow] method.
///
/// [google.cloud.workflows.v1.Workflows.GetWorkflow]: crate::client::Workflows::get_workflow
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct GetWorkflowRequest {
    /// Required. Name of the workflow for which information should be retrieved.
    /// Format: projects/{project}/locations/{location}/workflows/{workflow}
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// Optional. The revision of the workflow to retrieve. If the revision_id is
    /// empty, the latest revision is retrieved.
    /// The format is "000001-a4d", where the first six characters define
    /// the zero-padded decimal revision number. They are followed by a hyphen and
    /// three hexadecimal characters.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub revision_id: std::string::String,
}

impl GetWorkflowRequest {
    /// Sets the value of [name][crate::model::GetWorkflowRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [revision_id][crate::model::GetWorkflowRequest::revision_id].
    pub fn set_revision_id<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.revision_id = v.into();
        self
    }
}

impl wkt::message::Message for GetWorkflowRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.workflows.v1.GetWorkflowRequest"
    }
}

/// Request for the
/// [CreateWorkflow][google.cloud.workflows.v1.Workflows.CreateWorkflow]
/// method.
///
/// [google.cloud.workflows.v1.Workflows.CreateWorkflow]: crate::client::Workflows::create_workflow
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct CreateWorkflowRequest {
    /// Required. Project and location in which the workflow should be created.
    /// Format:  projects/{project}/locations/{location}
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub parent: std::string::String,

    /// Required. Workflow to be created.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub workflow: std::option::Option<crate::model::Workflow>,

    /// Required. The ID of the workflow to be created. It has to fulfill the
    /// following requirements:
    ///
    /// * Must contain only letters, numbers, underscores and hyphens.
    /// * Must start with a letter.
    /// * Must be between 1-64 characters.
    /// * Must end with a number or a letter.
    /// * Must be unique within the customer project and location.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub workflow_id: std::string::String,
}

impl CreateWorkflowRequest {
    /// Sets the value of [parent][crate::model::CreateWorkflowRequest::parent].
    pub fn set_parent<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.parent = v.into();
        self
    }

    /// Sets the value of [workflow][crate::model::CreateWorkflowRequest::workflow].
    pub fn set_workflow<T: std::convert::Into<std::option::Option<crate::model::Workflow>>>(
        mut self,
        v: T,
    ) -> Self {
        self.workflow = v.into();
        self
    }

    /// Sets the value of [workflow_id][crate::model::CreateWorkflowRequest::workflow_id].
    pub fn set_workflow_id<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.workflow_id = v.into();
        self
    }
}

impl wkt::message::Message for CreateWorkflowRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.workflows.v1.CreateWorkflowRequest"
    }
}

/// Request for the
/// [DeleteWorkflow][google.cloud.workflows.v1.Workflows.DeleteWorkflow]
/// method.
///
/// [google.cloud.workflows.v1.Workflows.DeleteWorkflow]: crate::client::Workflows::delete_workflow
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct DeleteWorkflowRequest {
    /// Required. Name of the workflow to be deleted.
    /// Format: projects/{project}/locations/{location}/workflows/{workflow}
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,
}

impl DeleteWorkflowRequest {
    /// Sets the value of [name][crate::model::DeleteWorkflowRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }
}

impl wkt::message::Message for DeleteWorkflowRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.workflows.v1.DeleteWorkflowRequest"
    }
}

/// Request for the
/// [UpdateWorkflow][google.cloud.workflows.v1.Workflows.UpdateWorkflow]
/// method.
///
/// [google.cloud.workflows.v1.Workflows.UpdateWorkflow]: crate::client::Workflows::update_workflow
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct UpdateWorkflowRequest {
    /// Required. Workflow to be updated.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub workflow: std::option::Option<crate::model::Workflow>,

    /// List of fields to be updated. If not present, the entire workflow
    /// will be updated.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub update_mask: std::option::Option<wkt::FieldMask>,
}

impl UpdateWorkflowRequest {
    /// Sets the value of [workflow][crate::model::UpdateWorkflowRequest::workflow].
    pub fn set_workflow<T: std::convert::Into<std::option::Option<crate::model::Workflow>>>(
        mut self,
        v: T,
    ) -> Self {
        self.workflow = v.into();
        self
    }

    /// Sets the value of [update_mask][crate::model::UpdateWorkflowRequest::update_mask].
    pub fn set_update_mask<T: std::convert::Into<std::option::Option<wkt::FieldMask>>>(
        mut self,
        v: T,
    ) -> Self {
        self.update_mask = v.into();
        self
    }
}

impl wkt::message::Message for UpdateWorkflowRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.workflows.v1.UpdateWorkflowRequest"
    }
}

/// Represents the metadata of the long-running operation.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct OperationMetadata {
    /// The time the operation was created.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub create_time: std::option::Option<wkt::Timestamp>,

    /// The time the operation finished running.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub end_time: std::option::Option<wkt::Timestamp>,

    /// Server-defined resource path for the target of the operation.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub target: std::string::String,

    /// Name of the verb executed by the operation.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub verb: std::string::String,

    /// API version used to start the operation.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub api_version: std::string::String,
}

impl OperationMetadata {
    /// Sets the value of [create_time][crate::model::OperationMetadata::create_time].
    pub fn set_create_time<T: std::convert::Into<std::option::Option<wkt::Timestamp>>>(
        mut self,
        v: T,
    ) -> Self {
        self.create_time = v.into();
        self
    }

    /// Sets the value of [end_time][crate::model::OperationMetadata::end_time].
    pub fn set_end_time<T: std::convert::Into<std::option::Option<wkt::Timestamp>>>(
        mut self,
        v: T,
    ) -> Self {
        self.end_time = v.into();
        self
    }

    /// Sets the value of [target][crate::model::OperationMetadata::target].
    pub fn set_target<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.target = v.into();
        self
    }

    /// Sets the value of [verb][crate::model::OperationMetadata::verb].
    pub fn set_verb<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.verb = v.into();
        self
    }

    /// Sets the value of [api_version][crate::model::OperationMetadata::api_version].
    pub fn set_api_version<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.api_version = v.into();
        self
    }
}

impl wkt::message::Message for OperationMetadata {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.workflows.v1.OperationMetadata"
    }
}
