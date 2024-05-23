See [USPS OAuth API documenation](https://developer.usps.com/api/81) for latest.
```sh
curl -O https://developer.usps.com/sites/default/files/apidoc_specs/US_Postal_Service-OAuth2-3.0.1-resolved%20%281%29.yaml

openapi-generator-cli generate -i US_Postal_Service-OAuth2-3.0.1-resolved%20%281%29.yaml -g rust -o usps-oauth-sdk
```