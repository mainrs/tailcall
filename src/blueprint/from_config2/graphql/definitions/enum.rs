use std::collections::BTreeSet;

use crate::blueprint::{Blueprint, Definition, EnumTypeDefinition, EnumValueDefinition};
use crate::blueprint::from_config2::Ctx;
use crate::blueprint::from_config2::types::Transform;
use crate::config;
use crate::valid::{Valid, ValidStructCompatibility};

pub struct EnumTransform;

impl Transform for EnumTransform {
    type Input = config::GraphQL;
    type Output = Blueprint;
    type Error = String;

    fn transform(self, input: &Self::Input, mut output: Self::Output, _ctx: &dyn Ctx) -> Valid<Self::Output, Self::Error> {
        let create_enum_def = |name: &str, variants: &BTreeSet<String>| -> Valid<Blueprint, String> {
            let enum_values = variants
                .into_iter()
                .map(|variant| EnumValueDefinition { description: None, name: variant.clone(), directives: Vec::new() })
                .collect();
            let enum_type_def =
                EnumTypeDefinition { name: name.to_owned(), directives: Vec::new(), description: None, enum_values };

            output.definitions.push(enum_type_def.into());
            Ok(output)
        };

        for (name, r#type) in &input.types {
            // First check that the type we received is actually an enum with variants.
            let Some(enum_variants) = &r#type.variants else {
                return Valid::fail(format!("No variants in {}", name));
            };

            output = create_enum_def(name, enum_variants)?;
        }

        Ok(output)
    }
}

impl From<EnumTypeDefinition> for Definition {
    fn from(value: EnumTypeDefinition) -> Self {
        Self::EnumTypeDefinition(value)
    }
}
