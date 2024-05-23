# ProviderTokensResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_token** | **String** | The access token issued to use to acess protected resources. | 
**expires_in** | **i32** | The expiration time of the issued access token, in seconds. | 
**token_type** | **String** | The access token type provides the client with the information required to successfully utilize the access token to make a protected resource request (along with type-specific attributes).  The client MUST NOT use an access token if it does not understand the token type. | 
**scope** | Option<**String**> | The OAuth scope being requested by the client application, specified as a list of space-delimited, case-sensitive strings.  If ommitted then the default scope configured for the client application will be used. | [optional]
**refresh_token** | **String** | The refresh token. | 
**refresh_token_issued_at** | Option<**i32**> | The date and time the refresh token was issued expressed in Unix epoch time in milliseconds. | [optional]
**refresh_token_expires_in** | Option<**i32**> | The refresh token expiration, in seconds. | [optional]
**refresh_count** | Option<**i32**> | The number of times the refresh token operation has been used. | [optional]
**refresh_token_status** | Option<**String**> | The current state of the refresh token. | [optional]
**issued_at** | Option<**i32**> | The date and time the access token was issued, expressed in Unix epoch time in milliseconds. | [optional]
**status** | Option<**String**> | The status of the access token. | [optional]
**issuer** | Option<**String**> | The authority that issued the token(s). | [optional]
**client_id** | Option<**String**> | The unique identifier for the client application. | [optional]
**application_name** | Option<**String**> | The name of the client application. | [optional]
**api_products** | Option<**String**> | The list of API products approved for use by the client application. | [optional]
**public_key** | Option<**String**> | The base64-encoded public cryptographic key used to validate the signature of the access token.  Validation ensures that the access token has not been tampered with and it originated from a known, trusted source. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


