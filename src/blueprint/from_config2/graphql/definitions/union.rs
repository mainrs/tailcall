use crate::blueprint::{Blueprint, Definition, UnionTypeDefinition};
use crate::blueprint::from_config2::Ctx;
use crate::blueprint::from_config2::types::Transform;
use crate::config;
use crate::config::Union;
use crate::valid::Valid;

pub struct UnionTransform;

impl Transform for UnionTransform {
    type Input = config::GraphQL;
    type Output = Blueprint;
    type Error = String;

    fn transform(self, input: &Self::Input, mut output: Self::Output, _ctx: &dyn Ctx) -> Valid<Self::Output, Self::Error> {
        let create_union_def = move |name: &str, union: &Union| -> Valid<Blueprint, String> {
            let union_def = UnionTypeDefinition {
                name: name.to_owned(),
                description: union.doc.clone(),
                directives: Vec::new(),
                types: union.types.clone(),
            };

            output.definitions.push(union_def.into());
            Ok(output)
        };

        for (name, union) in &input.unions {
            output = create_union_def(name, union)?;
        }

        Ok(output)
    }
}

impl From<UnionTypeDefinition> for Definition {
    fn from(value: UnionTypeDefinition) -> Self {
        Self::UnionTypeDefinition(value)
    }
}
