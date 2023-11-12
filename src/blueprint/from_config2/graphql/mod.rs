use crate::blueprint::Blueprint;
use crate::blueprint::from_config2::Ctx;
use crate::blueprint::from_config2::graphql::definitions::DefinitionsTransform;
use crate::blueprint::from_config2::types::Transform;
use crate::config::Config;
use crate::valid::Valid;

pub mod schema;
pub mod types;
mod definitions;

pub struct GraphQLTransform;

impl Transform for GraphQLTransform {
    type Input = Config;
    type Output = Blueprint;
    type Error = String;

    fn transform(self, input: &Self::Input, output: Self::Output, ctx: &dyn Ctx) -> Valid<Self::Output, Self::Error> {
        let transformer = DefinitionsTransform;
        transformer.transform(input, output, ctx)
    }
}
