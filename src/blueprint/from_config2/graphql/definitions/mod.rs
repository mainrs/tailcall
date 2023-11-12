use crate::blueprint::Blueprint;
use crate::blueprint::from_config2::Ctx;
use crate::blueprint::from_config2::graphql::definitions::r#enum::EnumTransform;
use crate::blueprint::from_config2::graphql::definitions::scalar::ScalarTransform;
use crate::blueprint::from_config2::graphql::definitions::union::UnionTransform;
use crate::blueprint::from_config2::types::{Transform, TryTransform};
use crate::config::Config;
use crate::valid::{Valid, ValidateAll, ValidStructCompatibility};

pub mod r#enum;
pub mod object;
pub mod scalar;
pub mod union;

pub struct DefinitionsTransform;

impl Transform for DefinitionsTransform {
    type Input = Config;
    type Output = Blueprint;
    type Error = String;

    fn transform(self, input: &Self::Input, output: Self::Output, ctx: &dyn Ctx) -> Valid<Self::Output, Self::Error> {
        // let unions = input.graphql.unions.iter()
        //     .map(|(name, union)| {
        //         let input = UnionTransformInput::from((name.as_ref(), union));
        //         let transform = UnionTransform::new();
        //         transform.transform(&input, output, ctx)
        //     });
        //
        // craete for transforms that check doubles, do the folding on unon and stuff and merge them
        // with trytransform i think.

        // let mut definition_transformers: Vec<Valid<dyn Transform<_, _, _>, _>> = input.graphql.types.iter()
        //     .validate_all(|(name, r#type)| {
        //         // Check if the type is a scalar type.
        //         if r#type.scalar {
        //             let transform = ScalarTransform { name };
        //             return transform.transform(input, output, ctx);
        //         }
        //
        //         if let Some(variants) = &r#type.variants {
        //             // Enums are required to have at least one variant.
        //             if variants.is_empty() {
        //                 return Valid::fail("No variants found for enum.".to_owned());
        //             }
        //
        //             return Ok(EnumTransform { name, r#type }.into());
        //         }
        //
        //         // In any other case, we have a normal GraphQL object that we need to transform.
        //         Ok(ObjectTransform { name, r#type }.into())
        //     })?;
        //
        // // We transform our scalars, unions, objects and enums together into a single blueprint.
        // definition_transformers.extend(unions);
        // let Some(Ok(transform)) = definition_transformers.into_iter().reduce(|t1, t2| t1.and(t2)) else {
        //     return Valid::fail("".to_owned());
        // };
        //
        // transform.transform(input, output, ctx)

        let transformer = NoDoubleUsageTransform;
        let blueprint = transformer.transform(input, output, ctx)?;

        let transformers = vec![EnumTransform, ScalarTransform, UnionTransform];
        TryTransform::transform_all(transformers).transform(&input.graphql, blueprint, ctx)
    }
}

struct NoDoubleUsageTransform;

impl Transform for NoDoubleUsageTransform {
    type Input = Config;
    type Output = Blueprint;
    type Error = String;

    fn transform(self, input: &Self::Input, output: Self::Output, ctx: &dyn Ctx) -> Valid<Self::Output, Self::Error> {
        let inputs = input.input_types();
        let outputs = input.output_types();

        for (name, _) in input.graphql.types {
            if inputs.contains(&name) && outputs.contains(&name) {
                return Valid::fail("Type is used as input and output.".to_owned()).trace(&name);
            }
        }

        Ok(output)
    }
}