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

/// A dyn-compatible, crate-private version of [super::IdentityAwareProxyAdminService].
#[async_trait::async_trait]
pub trait IdentityAwareProxyAdminService: std::fmt::Debug + Send + Sync {
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse>;

    async fn get_iap_settings(
        &self,
        req: crate::model::GetIapSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::IapSettings>;

    async fn update_iap_settings(
        &self,
        req: crate::model::UpdateIapSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::IapSettings>;

    async fn list_tunnel_dest_groups(
        &self,
        req: crate::model::ListTunnelDestGroupsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListTunnelDestGroupsResponse>;

    async fn create_tunnel_dest_group(
        &self,
        req: crate::model::CreateTunnelDestGroupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::TunnelDestGroup>;

    async fn get_tunnel_dest_group(
        &self,
        req: crate::model::GetTunnelDestGroupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::TunnelDestGroup>;

    async fn delete_tunnel_dest_group(
        &self,
        req: crate::model::DeleteTunnelDestGroupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn update_tunnel_dest_group(
        &self,
        req: crate::model::UpdateTunnelDestGroupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::TunnelDestGroup>;
}

/// All implementations of [crate::stubs::IdentityAwareProxyAdminService] also implement [IdentityAwareProxyAdminService].
#[async_trait::async_trait]
impl<T: crate::stubs::IdentityAwareProxyAdminService> IdentityAwareProxyAdminService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::set_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::get_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse> {
        T::test_iam_permissions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_iap_settings(
        &self,
        req: crate::model::GetIapSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::IapSettings> {
        T::get_iap_settings(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_iap_settings(
        &self,
        req: crate::model::UpdateIapSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::IapSettings> {
        T::update_iap_settings(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_tunnel_dest_groups(
        &self,
        req: crate::model::ListTunnelDestGroupsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListTunnelDestGroupsResponse> {
        T::list_tunnel_dest_groups(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_tunnel_dest_group(
        &self,
        req: crate::model::CreateTunnelDestGroupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::TunnelDestGroup> {
        T::create_tunnel_dest_group(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_tunnel_dest_group(
        &self,
        req: crate::model::GetTunnelDestGroupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::TunnelDestGroup> {
        T::get_tunnel_dest_group(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_tunnel_dest_group(
        &self,
        req: crate::model::DeleteTunnelDestGroupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_tunnel_dest_group(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_tunnel_dest_group(
        &self,
        req: crate::model::UpdateTunnelDestGroupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::TunnelDestGroup> {
        T::update_tunnel_dest_group(self, req, options).await
    }
}

/// A dyn-compatible, crate-private version of [super::IdentityAwareProxyOAuthService].
#[async_trait::async_trait]
pub trait IdentityAwareProxyOAuthService: std::fmt::Debug + Send + Sync {
    async fn list_brands(
        &self,
        req: crate::model::ListBrandsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListBrandsResponse>;

    async fn create_brand(
        &self,
        req: crate::model::CreateBrandRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Brand>;

    async fn get_brand(
        &self,
        req: crate::model::GetBrandRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Brand>;

    async fn create_identity_aware_proxy_client(
        &self,
        req: crate::model::CreateIdentityAwareProxyClientRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::IdentityAwareProxyClient>;

    async fn list_identity_aware_proxy_clients(
        &self,
        req: crate::model::ListIdentityAwareProxyClientsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListIdentityAwareProxyClientsResponse>;

    async fn get_identity_aware_proxy_client(
        &self,
        req: crate::model::GetIdentityAwareProxyClientRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::IdentityAwareProxyClient>;

    async fn reset_identity_aware_proxy_client_secret(
        &self,
        req: crate::model::ResetIdentityAwareProxyClientSecretRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::IdentityAwareProxyClient>;

    async fn delete_identity_aware_proxy_client(
        &self,
        req: crate::model::DeleteIdentityAwareProxyClientRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;
}

/// All implementations of [crate::stubs::IdentityAwareProxyOAuthService] also implement [IdentityAwareProxyOAuthService].
#[async_trait::async_trait]
impl<T: crate::stubs::IdentityAwareProxyOAuthService> IdentityAwareProxyOAuthService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_brands(
        &self,
        req: crate::model::ListBrandsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListBrandsResponse> {
        T::list_brands(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_brand(
        &self,
        req: crate::model::CreateBrandRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Brand> {
        T::create_brand(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_brand(
        &self,
        req: crate::model::GetBrandRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Brand> {
        T::get_brand(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_identity_aware_proxy_client(
        &self,
        req: crate::model::CreateIdentityAwareProxyClientRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::IdentityAwareProxyClient> {
        T::create_identity_aware_proxy_client(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_identity_aware_proxy_clients(
        &self,
        req: crate::model::ListIdentityAwareProxyClientsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListIdentityAwareProxyClientsResponse> {
        T::list_identity_aware_proxy_clients(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_identity_aware_proxy_client(
        &self,
        req: crate::model::GetIdentityAwareProxyClientRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::IdentityAwareProxyClient> {
        T::get_identity_aware_proxy_client(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn reset_identity_aware_proxy_client_secret(
        &self,
        req: crate::model::ResetIdentityAwareProxyClientSecretRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::IdentityAwareProxyClient> {
        T::reset_identity_aware_proxy_client_secret(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_identity_aware_proxy_client(
        &self,
        req: crate::model::DeleteIdentityAwareProxyClientRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_identity_aware_proxy_client(self, req, options).await
    }
}
