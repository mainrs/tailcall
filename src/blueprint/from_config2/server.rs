use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

use crate::{blueprint, config};
use crate::blueprint::Blueprint;
use crate::blueprint::from_config2::Ctx;
use crate::blueprint::from_config2::types::{Transform, TryTransform};
use crate::config::Config;
use crate::valid::{Valid, ValidateAll, ValidateBoth, ValidationError, ValidStructCompatibility};

/// Transforms the `@server` operator.
pub struct ServerTransform;

impl Transform for ServerTransform {
    type Input = Config;
    type Output = Blueprint;
    type Error = String;

    fn transform(self, input: &Self::Input, mut value: Self::Output, ctx: &dyn Ctx) -> Valid<Self::Output, Self::Error> {
        let transformers: Vec<Box<dyn Transform<Error=Self::Error, Input=config::Server, Output=blueprint::Server>>> = vec![
            Box::new(ServerResponseHeaderTransform),
            Box::new(ServerValuesTransform),
        ];
        let server = TryTransform::transform_all(transformers)
            .transform(&input.server, value.server, ctx)?;
        value.server = server;
        Ok(value)
    }
}

// Transforms all fields that do not need any further value processing.
struct ServerValuesTransform;

impl Transform for ServerValuesTransform {
    type Input = config::Server;
    type Output = blueprint::Server;
    type Error = String;

    fn transform(self, input: &Self::Input, value: Self::Output, _ctx: &dyn Ctx) -> Valid<Self::Output, Self::Error> {
        // FIXME: weird getter naming on `input`.
        let server = value
            .enable_apollo_tracing(input.enable_apollo_tracing())
            .enable_cache_control_header(input.enable_cache_control())
            .enable_graphiql(input.enable_graphiql())
            .enable_introspection(input.enable_introspection())
            .enable_query_validation(input.enable_query_validation())
            .enable_response_validation(input.enable_http_validation())
            .global_response_timeout(input.get_global_response_timeout())
            .port(input.get_port())
            .vars(input.get_vars());
        Ok(server)
    }
}

// Transforms the HTTP header fields, checking that only valid HTTP headers have been specified.
struct ServerResponseHeaderTransform;

impl Transform for ServerResponseHeaderTransform {
    type Input = config::Server;
    type Output = blueprint::Server;
    type Error = String;

    fn transform(self, input: &Self::Input, value: Self::Output, _ctx: &dyn Ctx) -> Valid<Self::Output, Self::Error> {
        let validate_key = |key: &str| -> Valid<HeaderName, _> {
            HeaderName::from_bytes(key.as_bytes())
                .map_err(|err| ValidationError::new(format!("Parsing failed because of {err}")))
        };
        let validate_value = |value: &str| -> Valid<HeaderValue, _> {
            HeaderValue::from_bytes(value.as_bytes())
                .map_err(|err| ValidationError::new(format!("Parsing failed because of {err}")))
        };
        let validator = |(k, v): (String, String)| validate_key(&k).validate_both(validate_value(&v));

        let headers = input
            .response_headers
            .0
            .validate_all(validator)
            .trace("responseHeaders")
            .trace("@server")
            .trace("schema")?;
        let mut header_map = HeaderMap::new();
        header_map.extend(headers);
        Ok(value.response_headers(header_map))
    }
}
