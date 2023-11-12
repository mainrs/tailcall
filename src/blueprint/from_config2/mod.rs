use std::collections::VecDeque;

use crate::blueprint::Blueprint;
use crate::blueprint::from_config2::graphql::GraphQLTransform;
use crate::blueprint::from_config2::graphql::schema::SchemaTransform;
use crate::blueprint::from_config2::server::ServerTransform;
use crate::blueprint::from_config2::types::{Transform, TryTransform};
use crate::config::{Config, Type};
use crate::valid::Valid;

mod ctx;
mod graphql;
mod server;
mod types;
mod upstream;

pub trait Ctx {
    fn is_scalar(&self, type_name: &str) -> bool {
        const SCALARS: [&str; 6] = ["Boolean", "Float", "ID", "Int", "JSON", "String"];
        SCALARS.contains(&type_name)
    }

    fn is_known_type(&self, type_name: &str) -> bool;

    fn find_type(&self, type_name: &str) -> Option<&Type>;
}

pub(crate) fn create_graphql_blueprint(config: &Config) -> Valid<Blueprint, String> {
    let transformers = &[GraphQLTransform, SchemaTransform, ServerTransform];
    let value =
        TryTransform::transform_all(VecDeque::from(transformers)).transform(config, Blueprint::default(), config)?;
    Ok(value)
}
