# \DefaultApi

All URIs are relative to *https://api.usps.com/oauth2/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**post_code**](DefaultApi.md#post_code) | **POST** /authorize | Generate an authorization code.
[**post_revoke**](DefaultApi.md#post_revoke) | **POST** /revoke | Invalidate OAuth tokens.
[**post_token**](DefaultApi.md#post_token) | **POST** /token | Generate OAuth tokens.



## post_code

> models::InlineResponse200 post_code(tintcyalf, client_id, response_type, redirect_uri, scope, state)
Generate an authorization code.

The client application identifier and redirect URI are input during client application registration.  The client application redirect URI specified here must match the one specified during client registration.  The Authorization Code request and response is the first leg of the three-legged Authorization Code flow stipulated here: - **Authorization Code Grant**, see [IETF 6749, section 4.1](https://datatracker.ietf.org/doc/html/rfc6749#section-4.1).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tintcyalf** | **String** | The cookie value must include the TINTCYALF token issued by Customer Registration subsequent to successful user sign-in. | [required] |
**client_id** | **String** | The unique identifier of the third-party client application. | [required] |
**response_type** | **String** | The type of response requested.  Must be set to 'code'. | [required] |[default to code]
**redirect_uri** | Option<**String**> | The authorization code redirect uri for the third-party application to receive the authorization code.  This is used to verify the identify of the requester. The request will not be redirected to this URI. |  |
**scope** | Option<**String**> | A client application may request a limited scope based on the resource owner's consent choices. |  |
**state** | Option<**String**> | An opaque value used by the client to maintain state between the request and callback.  The authorization server includes this value when redirecting the user-agent back to the client.  The parameter SHOULD be used for preventing cross-site request forgery.   The client application should provide sufficient information to complete the authorization process in their application.  It may also include a nonce value for security purposes.  |  |

### Return type

[**models::InlineResponse200**](inline_response_200.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_revoke

> post_revoke(token, token_type_hint)
Invalidate OAuth tokens.

Prevent tokens from being further used to access APIs. Dispose one or more OAuth tokens that are no longer needed. Based on the \\'OAuth 2.0 Token Revocation\\', IETF Draft RFC 7009, August 2013, see [IETF 7009](https://datatracker.ietf.org/doc/html/rfc7009).  Basic Authentication is used to access this resource, using the issued client Id and client secret.     ``` Authorization: Basic N0MyejJiS1FodDJUTEJjVTE2VmxlZUplQm1hdExiMjQ6TENtSE85RUFENXk0bUNURA== ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | The token (a hash value). | [required] |
**token_type_hint** | Option<**String**> | A hint to the type of the given token. See OAuth Token Types Hint registry, https://www.rfc-editor.org/rfc/rfc7009#section-4.1.2.1 |  |[default to refresh_token]

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_token

> models::InlineResponse2001 post_token(grant_type, scope, client_id, client_secret, code, redirect_uri, refresh_token)
Generate OAuth tokens.

Issue one or more OAuth tokens for a client application to use to make subsequent resource requests. Based on the _OAuth 2.0 Authorization Framework_, IETF Draft RFC 6749, October 2012, see [IETF 6749](https://datatracker.ietf.org/doc/html/rfc6749).  Note that the following OAuth grant types are supported: - **Authorization Code Grant**, see [IETF 6749, section 4.1](https://datatracker.ietf.org/doc/html/rfc6749#section-4.1). - **Client Credentials Grant**, see [IETF 6749, section 4.4](https://datatracker.ietf.org/doc/html/rfc6749#section-4.4). - **Refresh Token**, see [IETF 6749, section 6](https://datatracker.ietf.org/doc/html/rfc6749#section-6)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grant_type** | Option<**String**> | The OAuth standard flow being requested by the client application. |  |
**scope** | Option<**String**> | The OAuth scope being requested by the client application, specified as a list of space-delimited, case-sensitive strings.  If ommitted then the default scope configured for the client application will be used. |  |
**client_id** | Option<**String**> | The unique identifier issued to the client application making the request. Used for authenticating the client application. |  |
**client_secret** | Option<**String**> | The shared secret issued to the client application making the request. Used for authenticating the client application. |  |
**code** | Option<**String**> | To be done. |  |
**redirect_uri** | Option<**String**> | To be done. |  |
**refresh_token** | Option<**String**> | The refresh token value to be used to issue a new access token. |  |

### Return type

[**models::InlineResponse2001**](inline_response_200_1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

