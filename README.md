# USPS OAuth SDK in Rust

See [USPS OAuth API documentation](https://developer.usps.com/api/81).

OAuth access tokens are used to grant authorized access to USPS&#174; APIs. Access tokens will expire, requiring applications to periodically check the expiration time and get new tokens.

The following __OAuth 2.0__ grant types are supported:
- **Authorization Code**, the token request exchanges an authorization code previously received for access and refresh tokens. User (Resource Owner) authentication and consent is prerequisite for authorization code generation. The authorization code is validated and must not have expired.

- **Client Credentials**, the token request exchanges the client Id and secret to get an access token. The client Id and secret are the credentials for your client application and are validated.

- **Refresh Token**, the refresh token is exchanged to get a new access token and an optional refresh token. The refresh token is validated and must not have expired or been revoked.

Other OAuth flows may become supported in future releases.

You will need to add an app to get a client Id and secret. These are the _**Consumer Key**_ and _**Consumer Secret**_ values in the API developer portal.



Each API will stipulate the level of authentication assurance required to access its resources, either *Client Application* or *Resource Owner* credentials.  The access token value is placed in the *Authorization* header in accordance with the *Bearer* token authentication scheme.

 ```
 Authorization: Bearer <your-token>

 ```

Each API will validate the access token, its expiration in addition to its OAuth scope for example. There may be further validations required which are specific to the resource being accessed.

You will need to get a new access token once the one you have has expired. It is best practice to get a new access token before expiration if further access to resources is needed.
You may also revoke a refresh token which you suspect has been disclosed or dispose it when it is no longer needed.
  


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 3.0.1
- Package version: 3.0.1
- Generator version: 7.6.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `usps-oauth-sdk` and add the following to `Cargo.toml` under `[dependencies]`:

```
usps-oauth-sdk = { path = "./usps-oauth-sdk" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.usps.com/oauth2/v3*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DefaultApi* | [**post_code**](docs/DefaultApi.md#post_code) | **POST** /authorize | Generate an authorization code.
*DefaultApi* | [**post_revoke**](docs/DefaultApi.md#post_revoke) | **POST** /revoke | Invalidate OAuth tokens.
*DefaultApi* | [**post_token**](docs/DefaultApi.md#post_token) | **POST** /token | Generate OAuth tokens.


## Documentation For Models

 - [AccessTokenResponse](docs/AccessTokenResponse.md)
 - [AuthorizationCodeCredentials](docs/AuthorizationCodeCredentials.md)
 - [AuthorizationCodeRequest](docs/AuthorizationCodeRequest.md)
 - [ClientCredentials](docs/ClientCredentials.md)
 - [InlineResponse200](docs/InlineResponse200.md)
 - [InlineResponse2001](docs/InlineResponse2001.md)
 - [ProviderAccessTokenResponse](docs/ProviderAccessTokenResponse.md)
 - [ProviderTokensResponse](docs/ProviderTokensResponse.md)
 - [RefreshTokenCredentials](docs/RefreshTokenCredentials.md)
 - [StandardErrorResponse](docs/StandardErrorResponse.md)
 - [TokenBody1](docs/TokenBody1.md)
 - [TokenRevokeRequest](docs/TokenRevokeRequest.md)
 - [TokensResponse](docs/TokensResponse.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



