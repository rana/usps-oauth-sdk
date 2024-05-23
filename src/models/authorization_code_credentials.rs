/*
 * OAuth 2.0
 *
 *  OAuth access tokens are used to grant authorized access to USPS&#174; APIs. Access tokens will expire, requiring applications to periodically check the expiration time and get new tokens.  The following __OAuth 2.0__ grant types are supported: - **Authorization Code**, the token request exchanges an authorization code previously received for access and refresh tokens. User (Resource Owner) authentication and consent is prerequisite for authorization code generation. The authorization code is validated and must not have expired.  - **Client Credentials**, the token request exchanges the client Id and secret to get an access token. The client Id and secret are the credentials for your client application and are validated.  - **Refresh Token**, the refresh token is exchanged to get a new access token and an optional refresh token. The refresh token is validated and must not have expired or been revoked.  Other OAuth flows may become supported in future releases.  You will need to add an app to get a client Id and secret. These are the _**Consumer Key**_ and _**Consumer Secret**_ values in the API developer portal.    Each API will stipulate the level of authentication assurance required to access its resources, either *Client Application* or *Resource Owner* credentials.  The access token value is placed in the *Authorization* header in accordance with the *Bearer* token authentication scheme.   ```  Authorization: Bearer eyJhbGciOiJSUzI1NiJ9.eyJpc3MiOiJ1c3BzLmNvbSIsInN1YiI6IjI0ODI4OTc2MTAwMSIsImF1ZCI6InM2QmhkUmtxdDMiLCJub25jZSI6Im4tMFM2X1d6QTJNaiIsImV4cCI6MTMxMTI4MTk3MCwiaWF0IjoxMzExMjgwOTcwLCJuYW1lIjoiSmFuZSBEb2UiLCJnaXZlbl9uYW1lIjoiSmFuZSIsImZhbWlseV9uYW1lIjoiRG9lIiwibG9jYWxlIjoiZW4tdXMiLCJhenAiOiJ1c3BzLmNvbSIsImFjciI6IkFBTDEiLCJhbXIiOiJwd2QifQ.qJ2SUGKn4TabFfMYODW1RLxirFmeeYPDyFvuJR0ywRVaRnoe7Rlk8yKM3v2fCBUi2lMo00whNhNWmqQktpGgvkVGWXGMNIlVxJCqt_aPFx3oOvkhKWGI49JI5NyXrpj4tfYD5pIYbrihkF7eMYG3XyqYMx1VLhhV0PmWhpq787K7_AGfRlNVQnD_WEHJt4SoEnsiw8vcwDWXcXr5yCzAEn8mfCSTlamqVBUyey1Fyg_xgQIRj_b9CO-O4kXsBM3vqo5CO2qET2tRd37niaQvV-g418sEpnw1iAtxWfcyU4IIjWlQa7AxAc3T4Vx6XOwn1CNI22ZhdaBskUtD-EexWQ   ```  Each API will validate the access token, its expiration in addition to its OAuth scope for example. There may be further validations required which are specific to the resource being accessed.  You will need to get a new access token once the one you have has expired. It is best practice to get a new access token before expiration if further access to resources is needed. You may also revoke a refresh token which you suspect has been disclosed or dispose it when it is no longer needed.   
 *
 * The version of the OpenAPI document: 3.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// AuthorizationCodeCredentials : The credentials of the client application that are verified by the authorization server.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationCodeCredentials {
    /// The OAuth standard flow being requested by the client application.
    #[serde(rename = "grant_type")]
    pub grant_type: GrantType,
    /// The OAuth scope being requested by the client application, specified as a list of space-delimited, case-sensitive strings.  If ommitted then the default scope configured for the client application will be used.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// The unique identifier issued to the client application making the request. Used for authenticating the client application.
    #[serde(rename = "client_id")]
    pub client_id: String,
    /// The shared secret issued to the client application making the request. Used for authenticating the client application.
    #[serde(rename = "client_secret")]
    pub client_secret: String,
    /// To be done.
    #[serde(rename = "code")]
    pub code: String,
    /// To be done.
    #[serde(rename = "redirect_uri")]
    pub redirect_uri: String,
}

impl AuthorizationCodeCredentials {
    /// The credentials of the client application that are verified by the authorization server.
    pub fn new(grant_type: GrantType, client_id: String, client_secret: String, code: String, redirect_uri: String) -> AuthorizationCodeCredentials {
        AuthorizationCodeCredentials {
            grant_type,
            scope: None,
            client_id,
            client_secret,
            code,
            redirect_uri,
        }
    }
}
/// The OAuth standard flow being requested by the client application.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GrantType {
    #[serde(rename = "authorization_code")]
    AuthorizationCode,
    #[serde(rename = "client_credentials")]
    ClientCredentials,
    #[serde(rename = "refresh_token")]
    RefreshToken,
    #[serde(rename = "password")]
    Password,
}

impl Default for GrantType {
    fn default() -> GrantType {
        Self::AuthorizationCode
    }
}

