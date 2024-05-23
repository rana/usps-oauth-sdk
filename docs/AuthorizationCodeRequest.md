# AuthorizationCodeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | **String** | The unique identifier of the third-party client application. | 
**response_type** | **String** | The type of response requested.  Must be set to 'code'. | [default to Code]
**redirect_uri** | Option<**String**> | The authorization code redirect uri for the third-party application to receive the authorization code.  This is used to verify the identify of the requester. The request will not be redirected to this URI. | [optional]
**scope** | Option<**String**> | A client application may request a limited scope based on the resource owner's consent choices. | [optional]
**state** | Option<**String**> | An opaque value used by the client to maintain state between the request and callback.  The authorization server includes this value when redirecting the user-agent back to the client.  The parameter SHOULD be used for preventing cross-site request forgery.   The client application should provide sufficient information to complete the authorization process in their application.  It may also include a nonce value for security purposes.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


