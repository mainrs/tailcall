use std::collections::VecDeque;

use crate::blueprint::from_config2::types::{Transform, TryTransform};
use crate::blueprint::from_config2::Ctx;
use crate::blueprint::{blueprint, Blueprint};
use crate::config;
use crate::config::Config;
use crate::valid::Valid;

/// Transforms the `@upstream` operator.
pub struct UpstreamTransform;

impl Transform for UpstreamTransform {
  type Input = Config;
  type Output = Blueprint;
  type Error = String;

  fn transform(self, input: &Self::Input, mut value: Self::Output, ctx: &dyn Ctx) -> Valid<Self::Output, Self::Error> {
    let upstream = TryTransform::transform_all(VecDeque::from([UpstreamValuesTransform])).transform(
      &input.upstream,
      value.upstream,
      ctx,
    )?;
    value.upstream = upstream;
    Ok(value)
  }
}

// This is a no-op transform, since all values are used as-is inside the blueprint.
// The struct is added for future extensibility.
struct UpstreamValuesTransform;

impl Transform for UpstreamValuesTransform {
  type Input = config::Upstream;
  type Output = blueprint::Upstream;
  type Error = String;

  fn transform(self, input: &Self::Input, value: Self::Output) -> Valid<Self::Output, Self::Error> {
    Ok(input.clone())
  }
}
