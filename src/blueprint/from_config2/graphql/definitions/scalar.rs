use crate::blueprint::{Blueprint, Definition, ScalarTypeDefinition};
use crate::blueprint::from_config2::Ctx;
use crate::blueprint::from_config2::types::Transform;
use crate::config;
use crate::valid::Valid;

pub struct ScalarTransform;

impl Transform for ScalarTransform {
    type Input = config::GraphQL;
    type Output = Blueprint;
    type Error = String;

    fn transform(self, input: &Self::Input, mut output: Self::Output, _ctx: &dyn Ctx) -> Valid<Self::Output, Self::Error> {
        let create_scalar_def = |name: &str| -> Valid<Blueprint, String> {
            let scalar_type_def = ScalarTypeDefinition { name: name.to_owned(), directive: Vec::new(), description: None };
            output.definitions.push(scalar_type_def.into());
            Ok(output)
        };

        for (name, r#type) in &input.types {
            // First check that the type we received is actually a scalar.
            if r#type.scalar {
                output = create_scalar_def(name)?;
            }
        }

        Ok(output)
    }
}

impl From<ScalarTypeDefinition> for Definition {
    fn from(value: ScalarTypeDefinition) -> Self {
        Self::ScalarTypeDefinition(value)
    }
}