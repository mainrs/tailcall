use std::collections::VecDeque;

use crate::blueprint::from_config2::types::{Transform, TryTransform};
use crate::blueprint::from_config2::Ctx;
use crate::blueprint::Blueprint;
use crate::config::{Config, Field};
use crate::valid::{Valid, ValidStructCompatibility, ValidateAll};
use crate::{blueprint, config};

pub struct SchemaTransform;

impl Transform for SchemaTransform {
  type Input = Config;
  type Output = Blueprint;
  type Error = String;

  fn transform(self, input: &Self::Input, mut value: Self::Output, ctx: &dyn Ctx) -> Valid<Self::Output, Self::Error> {
    let schema = TryTransform::transform_all(VecDeque::from([SchemaMutationTransform, SchemaQueryTransform]))
      .transform(&input.graphql, value.schema, ctx)?;
    // FIXME: logic might be wrong here... should work for now.

    value.schema = schema;
    Ok(value)
  }
}

fn validate_mutation(config: &Config) -> Valid<(), String> {
  let mutation_type_name = config.graphql.schema.mutation.as_ref();

  // Having no mutation is also a valid state.
  if mutation_type_name.is_none() {
    return Ok(());
  }

  // But if we have a mutation type defined, we also need it and its fields to be valid.
  // SAFETY: above if-statement.
  let mutation_type_name = mutation_type_name.unwrap();

  let Some(mutation) = config.find_type(mutation_type_name) else {
    return Valid::fail("Mutation type is not defined".to_owned());
  };

  mutation
    .fields
    .iter()
    .validate_all(|(_, field)| validate_field_has_resolver(field))
    .trace(mutation_type_name)
    .map_to_unit()
}

fn validate_query(config: &Config, query_root_name: &str) -> Valid<(), String> {
  let Some(query) = config.find_type(query_root_name) else {
    return Valid::fail("Query type is not defined".to_owned());
  };

  query
    .fields
    .iter()
    .validate_all(|(_, field)| validate_field_has_resolver(field))
    .trace(query_root_name)?;
  Ok(())
}

fn validate_field_has_resolver(field: &Field) -> Valid<(), String> {
  if !field.has_resolver() {
    return Valid::fail("No resolver has been found in the schema".to_owned());
  }

  Ok(())
}

struct SchemaMutationTransform;

impl Transform for SchemaMutationTransform {
  type Input = config::GraphQL;
  type Output = blueprint::SchemaDefinition;
  type Error = String;

  fn transform(self, input: &Self::Input, mut value: Self::Output, _ctx: &dyn Ctx) -> Valid<Self::Output, Self::Error> {
    let Some(mutation_type_name) = input.schema.mutation.as_ref() else {
      return Valid::fail("".to_owned());
    };
    let Some(mutation) = input.find_type(mutation_type_name) else {
      return Valid::fail("Mutation type is not defined".to_owned());
    };

    mutation
      .fields
      .iter()
      .validate_all(|(_, field)| validate_field_has_resolver(field))
      .trace(mutation_type_name)?;

    value.mutation = input.schema.mutation.clone();
    Ok(value)
  }
}

struct SchemaQueryTransform;

impl Transform for SchemaQueryTransform {
  type Input = config::GraphQL;
  type Output = blueprint::SchemaDefinition;
  type Error = String;

  fn transform(self, input: &Self::Input, mut value: Self::Output, _ctx: &dyn Ctx) -> Valid<Self::Output, Self::Error> {
    let Some(query_type_name) = input.schema.query.as_ref() else {
      return Valid::fail("Query root is missing".to_owned());
    };
    let Some(query) = input.find_type(query_type_name) else {
      return Valid::fail("Query type is not defined".to_owned());
    };

    query
      .fields
      .iter()
      .validate_all(|(_, field)| validate_field_has_resolver(field))
      .trace(query_type_name)?;

    value.query = query_type_name.to_owned();
    Ok(value)
  }
}
