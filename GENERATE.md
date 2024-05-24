# USPS OAuth SDK in Rust

Based on the USPS OpenAPI specification. 

See [USPS OAuth API documentation](https://developer.usps.com/api/81).

See [OpenAPI Generator Rust documentation](https://openapi-generator.tech/docs/generators/rust/).

Manually update `yaml` file for proper functioning.
- Add `format: int64` to all "Unix epoch time" fields.
```yaml
issued_at:
    format: int64
refresh_token_issued_at:
    format: int64
```

```sh
> pwd
/home/rana/prj/usps-oauth-sdk

curl -O https://developer.usps.com/sites/default/files/apidoc_specs/US_Postal_Service-OAuth2-3.0.1-resolved%20%281%29.yaml

openapi-generator-cli generate -i US_Postal_Service-OAuth2-3.0.1-resolved%20%281%29.yaml -g rust --additional-properties=packageName=usps-oauth-sdk,preferUnsignedInt=true
```