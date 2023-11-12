use std::collections::VecDeque;

use crate::blueprint::from_config2::Ctx;
use crate::valid::{Valid, ValidateThen};

pub trait Transform: Sized {
  type Input;
  type Output: Clone;
  type Error;

  fn transform(self, input: &Self::Input, output: Self::Output, ctx: &dyn Ctx) -> Valid<Self::Output, Self::Error>;

  fn and<R: Transform>(self, other: R) -> And<Self, R> {
    And { left: self, right: other }
  }
}

pub struct And<L, R> {
  left: L,
  right: R,
}

impl<L: Transform, R: Transform> Transform for And<L, R> {
  type Input = L::Input;
  type Output = L::Output;
  type Error = L::Error;

  fn transform(self, input: &Self::Input, output: Self::Output, ctx: &dyn Ctx) -> Valid<Self::Output, Self::Error> {
    let result = self.left.transform(&input, output.clone(), ctx);
    match result {
      Ok(value) => self.right.transform(input, value, ctx),
      err => err.validate_then(self.transform(input, output, ctx)),
    }
  }
}

pub struct TryTransform<'a, I, O, E>(Box<dyn Fn(&I, O) -> Valid<O, E> + 'a>);

impl<'a, I, O, E> TryTransform<'a, I, O, E> {
  pub fn transform_all<A: Transform<Input = A, Output = O, Error = E>>(items: impl Into<VecDeque<A>>) -> Collect<A> {
    Collect(items.into())
  }
}

pub struct Collect<A>(VecDeque<A>);

impl<A: Transform> Transform for Collect<A> {
  type Input = A::Input;
  type Output = A::Output;
  type Error = A::Error;

  fn transform(self, input: &Self::Input, value: Self::Output, ctx: &dyn Ctx) -> Valid<Self::Output, Self::Error> {
    let mut items = self.0;
    let head = items.pop_front();

    if head.is_none() {
      return Ok(value);
    }

    // Safety: if-statement above.
    head
      .unwrap()
      .and(TryTransform::transform_all(items))
      .transform(input, value, ctx)
  }
}
