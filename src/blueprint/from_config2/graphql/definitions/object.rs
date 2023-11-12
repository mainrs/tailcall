use crate::blueprint::{Blueprint, Definition, ObjectTypeDefinition};
use crate::blueprint::from_config2::Ctx;
use crate::blueprint::from_config2::types::Transform;
use crate::config;
use crate::config::Type;
use crate::valid::Valid;

pub struct ObjectTransform;

impl Transform for ObjectTransform {
    type Input = config::GraphQL;
    type Output = Blueprint;
    type Error = String;

    fn transform(self, input: &Self::Input, mut output: Self::Output, _ctx: &dyn Ctx) -> Valid<Self::Output, Self::Error> {
        let create_scalar_def = |name: &str| -> Valid<Blueprint, String> {
            let object_type_def = ObjectTypeDefinition { name: name.to_owned(), description: None, fields: vec![], implements: Default::default() };
            output.definitions.push(object_type_def.into());
            Ok(output)
        };

        for (name, r#type) in &input.types {
            // First check that the type we received is actually a scalar.
            if r#type.is_object() {
                output = create_scalar_def(name)?;
            }
        }

        Ok(output)
    }
}

impl From<ObjectTypeDefinition> for Definition {
    fn from(value: ObjectTypeDefinition) -> Self {
        Self::ObjectTypeDefinition(value)
    }
}

impl Type {
    fn is_object(&self) -> bool {
        if let Some(variants) = &self.variants {
            if !variants.is_empty() {
                return false;
            }
        }

        if self.scalar {
            return false;
        }

        true
    }
}