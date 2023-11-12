use async_graphql_value::ConstValue;

use crate::blueprint::FieldDefinition;
use crate::blueprint::from_config2::Ctx;
use crate::blueprint::from_config2::types::Transform;
use crate::config::Field;
use crate::lambda::Expression::Literal;
use crate::valid::{Valid, ValidStructCompatibility};

pub struct ConstTransform;

impl Transform for ConstTransform {
    type Input = Field;
    type Output = FieldDefinition;
    type Error = String;

    fn transform(self, input: &Self::Input, mut output: Self::Output, ctx: &dyn Ctx) -> Valid<Self::Output, Self::Error> {
        // We define a capturing lambda that contains our main logic. When calling the lambda, we can
        // now easily add a `trace` method call without having to add the call to each possible
        // return value branch.
        let inner = move || {
            // Early return for the case that there is no `@const` definition.
            let Some(const_field) = &input.const_field else {
                return Ok(output);
            };

            let const_field_data = &const_field.data;
            let const_value = ConstValue::from_json(const_field_data.to_owned());

            // Check if the creation of the const value data was successful. `async-graphql` returns a
            // serde_json::Error if it failed to decode the data.
            let Ok(const_value) = const_value else {
                return Valid::fail(format!("Invalid JSON: {}", const_value.unwrap_err()));
            };

            // We validate that the resulting JSON object matches the expected type defined in GraphQL.
            // let validation_result = to_json_schema_for_field(input, config).validate(&const_value);
            // if let Err(e) = validation_result {
            //   return e.into();
            // }

            // From this point on, the value inside of `@const` is valid.
            output.resolver = Some(Literal(const_field_data.to_owned()));
            Ok(output)
        };

        inner().trace("@const")
    }
}
