use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Defines a security scheme that can be used by the operations.
/// Supported schemes are HTTP authentication, an API key (either as a
/// header or as a query parameter), OAuth2's common flows (implicit, password,
/// application and access code) as defined in RFC6749, and OpenID Connect
/// Discovery.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum SecurityScheme {
    #[serde(rename = "apiKey")]
    APIKey {
        #[serde(rename = "in")]
        location: APIKeyLocation,
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// Inline extensions to this object.
        #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
        extensions: IndexMap<String, serde_json::Value>,
    },
    #[serde(rename = "http")]
    HTTP {
        scheme: String,
        #[serde(rename = "bearerFormat")]
        bearer_format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// Inline extensions to this object.
        #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
        extensions: IndexMap<String, serde_json::Value>,
    },
    #[serde(rename = "oauth2")]
    OAuth2 {
        flows: OAuth2Flows,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// Inline extensions to this object.
        #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
        extensions: IndexMap<String, serde_json::Value>,
    },
    #[serde(rename = "openIdConnect")]
    OpenIDConnect {
        #[serde(rename = "openIdConnectUrl")]
        open_id_connect_url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// Inline extensions to this object.
        #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
        extensions: IndexMap<String, serde_json::Value>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum APIKeyLocation {
    Query,
    Header,
    Cookie,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OAuth2Flows {
    #[serde(flatten)]
    pub implicit: Option<OAuth2Flow>,
    #[serde(flatten)]
    pub password: Option<OAuth2Flow>,
    #[serde(flatten)]
    pub client_credentials: Option<OAuth2Flow>,
    #[serde(flatten)]
    pub authorization_code: Option<OAuth2Flow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum OAuth2Flow {
    #[serde(rename_all = "camelCase")]
    Implicit {
        authorization_url: String,
        refresh_url: Option<String>,
        #[serde(default)]
        scopes: IndexMap<String, String>,
    },
    #[serde(rename_all = "camelCase")]
    Password {
        refresh_url: Option<String>,
        token_url: String,
        #[serde(default)]
        scopes: IndexMap<String, String>,
    },
    #[serde(rename_all = "camelCase")]
    ClientCredentials {
        refresh_url: Option<String>,
        token_url: String,
        #[serde(default)]
        scopes: IndexMap<String, String>,
    },
    #[serde(rename_all = "camelCase")]
    AuthorizationCode {
        authorization_url: String,
        token_url: String,
        refresh_url: Option<String>,
        #[serde(default)]
        scopes: IndexMap<String, String>,
    },
}

#[cfg(feature = "conversions")]
mod conversions {
    use super::*;
    use crate::v3_1;

    impl From<v3_1::ApiKeyLocation> for APIKeyLocation {
        fn from(s: v3_1::ApiKeyLocation) -> Self {
            match s {
                v3_1::ApiKeyLocation::Query => APIKeyLocation::Query,
                v3_1::ApiKeyLocation::Header => APIKeyLocation::Header,
                v3_1::ApiKeyLocation::Cookie => APIKeyLocation::Cookie,
            }
        }
    }

    impl From<v3_1::OAuth2Flows> for OAuth2Flows {
        fn from(s: v3_1::OAuth2Flows) -> Self {
            OAuth2Flows {
                implicit: s.implicit.map(Into::into),
                password: s.password.map(Into::into),
                client_credentials: s.client_credentials.map(Into::into),
                authorization_code: s.authorization_code.map(Into::into),
            }
        }
    }

    impl From<v3_1::OAuth2Flow> for OAuth2Flow {
        fn from(s: v3_1::OAuth2Flow) -> Self {
            match s {
                v3_1::OAuth2Flow::Implicit {
                    authorization_url,
                    refresh_url,
                    scopes,
                } => OAuth2Flow::Implicit {
                    authorization_url,
                    refresh_url,
                    scopes,
                },
                v3_1::OAuth2Flow::Password {
                    refresh_url,
                    token_url,
                    scopes,
                } => OAuth2Flow::Password {
                    refresh_url,
                    token_url,
                    scopes,
                },
                v3_1::OAuth2Flow::ClientCredentials {
                    refresh_url,
                    token_url,
                    scopes,
                } => OAuth2Flow::ClientCredentials {
                    refresh_url,
                    token_url,
                    scopes,
                },
                v3_1::OAuth2Flow::AuthorizationCode {
                    authorization_url,
                    token_url,
                    refresh_url,
                    scopes,
                } => OAuth2Flow::AuthorizationCode {
                    authorization_url,
                    token_url,
                    refresh_url,
                    scopes,
                },
            }
        }
    }

    impl From<v3_1::SecurityScheme> for SecurityScheme {
        fn from(s: v3_1::SecurityScheme) -> Self {
            match s {
                v3_1::SecurityScheme::ApiKey {
                    location,
                    name,
                    description,
                    extensions,
                } => SecurityScheme::APIKey {
                    location: location.into(),
                    name,
                    description,
                    extensions,
                },
                v3_1::SecurityScheme::Http {
                    scheme,
                    bearer_format,
                    description,
                    extensions,
                } => SecurityScheme::HTTP {
                    scheme,
                    bearer_format,
                    description,
                    extensions,
                },
                v3_1::SecurityScheme::OAuth2 {
                    flows,
                    description,
                    extensions,
                } => SecurityScheme::OAuth2 {
                    flows: flows.into(),
                    description,
                    extensions,
                },
                v3_1::SecurityScheme::OpenIdConnect {
                    open_id_connect_url,
                    description,
                    extensions,
                } => SecurityScheme::OpenIDConnect {
                    open_id_connect_url,
                    description,
                    extensions,
                },
                v3_1::SecurityScheme::MutualTls { .. } => {
                    panic!("MutualTLS is not supported in Open API 3.0.x")
                }
            }
        }
    }
}
