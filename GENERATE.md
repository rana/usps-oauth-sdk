# USPS OAuth SDK in Rust

See [USPS OAuth API documentation](https://developer.usps.com/api/81).

See [OpenAPI Generator Rust documentation](https://openapi-generator.tech/docs/generators/rust/).

Step 1. Manually update `yaml` file for proper functioning.
- Add `format: int64` to all "Unix epoch time" fields.
```yaml
issued_at:
    format: int64
refresh_token_issued_at:
    format: int64
```

Step 2. Generate SDK.
```sh
> pwd
/home/rana/prj/usps-oauth-sdk

curl -O https://developer.usps.com/sites/default/files/apidoc_specs/US_Postal_Service-OAuth2-3.0.1-resolved%20%281%29.yaml

openapi-generator-cli generate -i US_Postal_Service-OAuth2-3.0.1-resolved%20%281%29.yaml -g rust --additional-properties=packageName=usps-oauth-sdk,preferUnsignedInt=true
```

Step 3. Manually update Cargo.toml.
- `reqwest`
    - Add `default-features = false`
    - Add `rustls-tls`
```toml
reqwest = { version = "^0.12", default-features = false, features = [
    "json",
    "multipart",
    "rustls-tls",
] }
```