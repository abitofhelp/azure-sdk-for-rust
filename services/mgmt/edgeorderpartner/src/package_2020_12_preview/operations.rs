#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use super::{models, API_VERSION};
#[derive(Clone)]
pub struct Client {
    endpoint: String,
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    endpoint: Option<String>,
    scopes: Option<Vec<String>>,
}
pub const DEFAULT_ENDPOINT: &str = azure_core::resource_manager_endpoint::AZURE_PUBLIC_CLOUD;
impl ClientBuilder {
    pub fn new(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
        }
    }
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self.scopes.unwrap_or_else(|| vec![format!("{}/", endpoint)]);
        Client::new(endpoint, self.credential, scopes)
    }
}
impl Client {
    pub(crate) fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub(crate) fn token_credential(&self) -> &dyn azure_core::auth::TokenCredential {
        self.credential.as_ref()
    }
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(&self, request: impl Into<azure_core::Request>) -> Result<azure_core::Response, azure_core::Error> {
        let mut context = azure_core::Context::default();
        let mut request = request.into();
        self.pipeline.send(&mut context, &mut request).await
    }
    pub fn new(
        endpoint: impl Into<String>,
        credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
        scopes: Vec<String>,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            azure_core::ClientOptions::default(),
            Vec::new(),
            Vec::new(),
        );
        Self {
            endpoint,
            credential,
            scopes,
            pipeline,
        }
    }
}
#[non_exhaustive]
#[derive(Debug, thiserror :: Error)]
#[allow(non_camel_case_types)]
pub enum Error {
    #[error(transparent)]
    ListOperationsPartner(#[from] list_operations_partner::Error),
    #[error(transparent)]
    ManageInventoryMetadata(#[from] manage_inventory_metadata::Error),
    #[error(transparent)]
    ManageLink(#[from] manage_link::Error),
    #[error(transparent)]
    SearchInventories(#[from] search_inventories::Error),
}
impl Client {
    pub fn list_operations_partner(&self) -> list_operations_partner::Builder {
        list_operations_partner::Builder { client: self.clone() }
    }
    #[doc = "API for updating inventory metadata and inventory configuration"]
    pub fn manage_inventory_metadata(
        &self,
        family_identifier: impl Into<String>,
        subscription_id: impl Into<String>,
        location: impl Into<String>,
        serial_number: impl Into<String>,
        manage_inventory_metadata_request: impl Into<models::ManageInventoryMetadataRequest>,
    ) -> manage_inventory_metadata::Builder {
        manage_inventory_metadata::Builder {
            client: self.clone(),
            family_identifier: family_identifier.into(),
            subscription_id: subscription_id.into(),
            location: location.into(),
            serial_number: serial_number.into(),
            manage_inventory_metadata_request: manage_inventory_metadata_request.into(),
        }
    }
    #[doc = "API for linking management resource with inventory"]
    pub fn manage_link(
        &self,
        family_identifier: impl Into<String>,
        subscription_id: impl Into<String>,
        location: impl Into<String>,
        serial_number: impl Into<String>,
        manage_link_request: impl Into<models::ManageLinkRequest>,
    ) -> manage_link::Builder {
        manage_link::Builder {
            client: self.clone(),
            family_identifier: family_identifier.into(),
            subscription_id: subscription_id.into(),
            location: location.into(),
            serial_number: serial_number.into(),
            manage_link_request: manage_link_request.into(),
        }
    }
    #[doc = "API for Search inventories"]
    pub fn search_inventories(
        &self,
        subscription_id: impl Into<String>,
        search_inventories_request: impl Into<models::SearchInventoriesRequest>,
    ) -> search_inventories::Builder {
        search_inventories::Builder {
            client: self.clone(),
            subscription_id: subscription_id.into(),
            search_inventories_request: search_inventories_request.into(),
        }
    }
}
pub mod list_operations_partner {
    use super::{models, API_VERSION};
    #[derive(Debug, thiserror :: Error)]
    pub enum Error {
        #[error("HTTP status code {}", status_code)]
        DefaultResponse {
            status_code: http::StatusCode,
            value: models::ErrorResponse,
        },
        #[error("Failed to parse request URL: {0}")]
        ParseUrl(url::ParseError),
        #[error("Failed to build request: {0}")]
        BuildRequest(http::Error),
        #[error("Failed to serialize request body: {0}")]
        Serialize(serde_json::Error),
        #[error("Failed to get access token: {0}")]
        GetToken(azure_core::Error),
        #[error("Failed to execute request: {0}")]
        SendRequest(azure_core::Error),
        #[error("Failed to get response bytes: {0}")]
        ResponseBytes(azure_core::StreamError),
        #[error("Failed to deserialize response: {0}, body: {1:?}")]
        Deserialize(serde_json::Error, bytes::Bytes),
    }
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<models::OperationListResult, Error>> {
            Box::pin(async move {
                let url_str = &format!("{}/providers/Microsoft.EdgeOrderPartner/operations", self.client.endpoint(),);
                let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                let mut req_builder = http::request::Builder::new();
                req_builder = req_builder.method(http::Method::GET);
                let credential = self.client.token_credential();
                let token_response = credential
                    .get_token(&self.client.scopes().join(" "))
                    .await
                    .map_err(Error::GetToken)?;
                req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
                let req_body = azure_core::EMPTY_BODY;
                req_builder = req_builder.uri(url.as_str());
                let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                match rsp_status {
                    http::StatusCode::OK => {
                        let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                        let rsp_value: models::OperationListResult =
                            serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                        Ok(rsp_value)
                    }
                    status_code => {
                        let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                        let rsp_value: models::ErrorResponse =
                            serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                        Err(Error::DefaultResponse {
                            status_code,
                            value: rsp_value,
                        })
                    }
                }
            })
        }
    }
}
pub mod manage_inventory_metadata {
    use super::{models, API_VERSION};
    #[derive(Debug)]
    pub enum Response {
        Ok200,
        Accepted202,
        NoContent204,
    }
    #[derive(Debug, thiserror :: Error)]
    pub enum Error {
        #[error("HTTP status code {}", status_code)]
        DefaultResponse {
            status_code: http::StatusCode,
            value: models::ErrorResponse,
        },
        #[error("Failed to parse request URL: {0}")]
        ParseUrl(url::ParseError),
        #[error("Failed to build request: {0}")]
        BuildRequest(http::Error),
        #[error("Failed to serialize request body: {0}")]
        Serialize(serde_json::Error),
        #[error("Failed to get access token: {0}")]
        GetToken(azure_core::Error),
        #[error("Failed to execute request: {0}")]
        SendRequest(azure_core::Error),
        #[error("Failed to get response bytes: {0}")]
        ResponseBytes(azure_core::StreamError),
        #[error("Failed to deserialize response: {0}, body: {1:?}")]
        Deserialize(serde_json::Error, bytes::Bytes),
    }
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) family_identifier: String,
        pub(crate) subscription_id: String,
        pub(crate) location: String,
        pub(crate) serial_number: String,
        pub(crate) manage_inventory_metadata_request: models::ManageInventoryMetadataRequest,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<Response, Error>> {
            Box::pin(async move {
                let url_str = & format ! ("{}/subscriptions/{}/providers/Microsoft.EdgeOrderPartner/locations/{}/productFamilies/{}/inventories/{}/manageInventoryMetadata" , self . client . endpoint () , & self . subscription_id , & self . location , & self . family_identifier , & self . serial_number) ;
                let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                let mut req_builder = http::request::Builder::new();
                req_builder = req_builder.method(http::Method::POST);
                let credential = self.client.token_credential();
                let token_response = credential
                    .get_token(&self.client.scopes().join(" "))
                    .await
                    .map_err(Error::GetToken)?;
                req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
                req_builder = req_builder.header("content-type", "application/json");
                let req_body = azure_core::to_json(&self.manage_inventory_metadata_request).map_err(Error::Serialize)?;
                req_builder = req_builder.uri(url.as_str());
                let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                match rsp_status {
                    http::StatusCode::OK => Ok(Response::Ok200),
                    http::StatusCode::ACCEPTED => Ok(Response::Accepted202),
                    http::StatusCode::NO_CONTENT => Ok(Response::NoContent204),
                    status_code => {
                        let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                        let rsp_value: models::ErrorResponse =
                            serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                        Err(Error::DefaultResponse {
                            status_code,
                            value: rsp_value,
                        })
                    }
                }
            })
        }
    }
}
pub mod manage_link {
    use super::{models, API_VERSION};
    #[derive(Debug)]
    pub enum Response {
        Ok200,
        NoContent204,
    }
    #[derive(Debug, thiserror :: Error)]
    pub enum Error {
        #[error("HTTP status code {}", status_code)]
        DefaultResponse {
            status_code: http::StatusCode,
            value: models::ErrorResponse,
        },
        #[error("Failed to parse request URL: {0}")]
        ParseUrl(url::ParseError),
        #[error("Failed to build request: {0}")]
        BuildRequest(http::Error),
        #[error("Failed to serialize request body: {0}")]
        Serialize(serde_json::Error),
        #[error("Failed to get access token: {0}")]
        GetToken(azure_core::Error),
        #[error("Failed to execute request: {0}")]
        SendRequest(azure_core::Error),
        #[error("Failed to get response bytes: {0}")]
        ResponseBytes(azure_core::StreamError),
        #[error("Failed to deserialize response: {0}, body: {1:?}")]
        Deserialize(serde_json::Error, bytes::Bytes),
    }
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) family_identifier: String,
        pub(crate) subscription_id: String,
        pub(crate) location: String,
        pub(crate) serial_number: String,
        pub(crate) manage_link_request: models::ManageLinkRequest,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<Response, Error>> {
            Box::pin(async move {
                let url_str = &format!(
                    "{}/subscriptions/{}/providers/Microsoft.EdgeOrderPartner/locations/{}/productFamilies/{}/inventories/{}/manageLink",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.location,
                    &self.family_identifier,
                    &self.serial_number
                );
                let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                let mut req_builder = http::request::Builder::new();
                req_builder = req_builder.method(http::Method::POST);
                let credential = self.client.token_credential();
                let token_response = credential
                    .get_token(&self.client.scopes().join(" "))
                    .await
                    .map_err(Error::GetToken)?;
                req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
                req_builder = req_builder.header("content-type", "application/json");
                let req_body = azure_core::to_json(&self.manage_link_request).map_err(Error::Serialize)?;
                req_builder = req_builder.uri(url.as_str());
                let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                match rsp_status {
                    http::StatusCode::OK => Ok(Response::Ok200),
                    http::StatusCode::NO_CONTENT => Ok(Response::NoContent204),
                    status_code => {
                        let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                        let rsp_value: models::ErrorResponse =
                            serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                        Err(Error::DefaultResponse {
                            status_code,
                            value: rsp_value,
                        })
                    }
                }
            })
        }
    }
}
pub mod search_inventories {
    use super::{models, API_VERSION};
    #[derive(Debug, thiserror :: Error)]
    pub enum Error {
        #[error("HTTP status code {}", status_code)]
        DefaultResponse {
            status_code: http::StatusCode,
            value: models::ErrorResponse,
        },
        #[error("Failed to parse request URL: {0}")]
        ParseUrl(url::ParseError),
        #[error("Failed to build request: {0}")]
        BuildRequest(http::Error),
        #[error("Failed to serialize request body: {0}")]
        Serialize(serde_json::Error),
        #[error("Failed to get access token: {0}")]
        GetToken(azure_core::Error),
        #[error("Failed to execute request: {0}")]
        SendRequest(azure_core::Error),
        #[error("Failed to get response bytes: {0}")]
        ResponseBytes(azure_core::StreamError),
        #[error("Failed to deserialize response: {0}, body: {1:?}")]
        Deserialize(serde_json::Error, bytes::Bytes),
    }
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) subscription_id: String,
        pub(crate) search_inventories_request: models::SearchInventoriesRequest,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<models::PartnerInventoryList, Error>> {
            Box::pin(async move {
                let url_str = &format!(
                    "{}/subscriptions/{}/providers/Microsoft.EdgeOrderPartner/searchInventories",
                    self.client.endpoint(),
                    &self.subscription_id
                );
                let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                let mut req_builder = http::request::Builder::new();
                req_builder = req_builder.method(http::Method::POST);
                let credential = self.client.token_credential();
                let token_response = credential
                    .get_token(&self.client.scopes().join(" "))
                    .await
                    .map_err(Error::GetToken)?;
                req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
                req_builder = req_builder.header("content-type", "application/json");
                let req_body = azure_core::to_json(&self.search_inventories_request).map_err(Error::Serialize)?;
                req_builder = req_builder.uri(url.as_str());
                let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                match rsp_status {
                    http::StatusCode::OK => {
                        let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                        let rsp_value: models::PartnerInventoryList =
                            serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                        Ok(rsp_value)
                    }
                    status_code => {
                        let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                        let rsp_value: models::ErrorResponse =
                            serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                        Err(Error::DefaultResponse {
                            status_code,
                            value: rsp_value,
                        })
                    }
                }
            })
        }
    }
}
