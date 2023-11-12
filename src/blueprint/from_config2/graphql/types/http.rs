use hyper::HeaderMap;
use hyper::http::{HeaderName, HeaderValue};

use crate::blueprint::FieldDefinition;
use crate::blueprint::from_config2::Ctx;
use crate::blueprint::from_config2::types::Transform;
use crate::config::Field;
use crate::endpoint::Endpoint;
use crate::lambda::Lambda;
use crate::request_template::RequestTemplate;
use crate::valid::{Valid, ValidationError, ValidStructCompatibility};

pub struct HttpTransform<'a> {
    base_url: Option<&'a str>,
}

impl<'a> Transform for HttpTransform<'a> {
    type Input = Field;
    type Output = FieldDefinition;
    type Error = String;

    fn transform(self, input: &Self::Input, mut value: Self::Output, _ctx: &dyn Ctx) -> Valid<Self::Output, Self::Error> {
        // We define a capturing lambda that contains our main logic. When calling the lambda, we can
        // now easily add a `trace` method call without having to add the call to each possible
        // return value branch.
        let inner = move || {
            // Early return for the case that there is no `@const` definition.
            let Some(http_field) = &input.http else {
                return Ok(value);
            };

            // Every `@http` directive needs a baseUrl. This can either bee the directive's `baseUrl`
            // attribute or the `@server`'s `baseUrl` attribute.
            let Some(base_url) = http_field.base_url.as_deref().or(self.base_url) else {
                return Valid::fail("No base URL defined".to_owned());
            };

            // Construct the final request URL by appending the `path` attribute value to the URL.
            let base_url = sanitize(base_url);
            let path = &http_field.path;
            let base_url = format!("{base_url}{path}");

            // Construct the needed [`HeaderMap`] instance.
            let headers_len = http_field.headers.len();
            let mut headers = HeaderMap::with_capacity(headers_len);

            for (key, value) in http_field.headers.iter() {
                let key = HeaderName::from_bytes(key.as_bytes()).map_err(|e| ValidationError::new(e.to_string()))?;
                let value = HeaderValue::from_bytes(value.as_bytes()).map_err(|e| ValidationError::new(e.to_string()))?;
                headers.insert(key, value);
            }

            let query = http_field.query.iter().map(|(a, b)| (a.clone(), b.clone())).collect();
            let template = RequestTemplate::try_from(
                Endpoint::new(base_url)
                    .method(http_field.method.clone())
                    .headers(headers)
                    .query(query)
                    .body(http_field.body.clone()),
            )
                .map_err(|e| ValidationError::new(e.to_string()))?;

            value.resolver = Some(Lambda::from_request_template(template).expression);
            Ok(value)
        };

        inner().trace("@http")
    }
}

// Removes trailing forward slashes (`/`).
fn sanitize(base_url: &str) -> &str {
    if !base_url.ends_with('/') {
        return base_url;
    }

    &base_url[0..base_url.len() - 1]
}
