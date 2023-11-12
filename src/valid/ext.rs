use crate::valid::{Valid, ValidationError};

pub trait ValidateAll<A, E> {
  fn validate_all<B>(self, validator: impl Fn(A) -> Valid<B, E>) -> Valid<Vec<B>, E>;
}

impl<A, E, I> ValidateAll<A, E> for I
where
  I: IntoIterator<Item = A>,
{
  fn validate_all<B>(self, validator: impl Fn(A) -> Valid<B, E>) -> Valid<Vec<B>, E> {
    // We cannot simply rely on the classical `collect::<Result<B>, E>` methodology,
    // since this will not allow us to combine the errors without a major refactor.
    // Thus, we work with the `Valid` wrapper type to make error combinations work.
    let mut results: Vec<B> = Vec::new();
    let mut error: ValidationError<E> = ValidationError::empty();

    // Iterate over each item and validate it. If there was an error during validation,
    // we make sure to combine it with any previously occurring errors.
    for item in self {
      let validation_result = validator(item);
      match validation_result {
        Ok(result) => results.push(result),
        Err(err) => error = error.combine(err),
      }
    }

    if !error.is_empty() {
      return Err(error);
    }

    Ok(results)
  }
}

pub trait ValidateBoth<A1, A2, E> {
  fn validate_both(self, other: Valid<A2, E>) -> Valid<(A1, A2), E>;
}

impl<A1, A2, E> ValidateBoth<A1, A2, E> for Valid<A1, E> {
  fn validate_both(self, other: Valid<A2, E>) -> Valid<(A1, A2), E> {
    match (self, other) {
      (Ok(a1), Ok(a2)) => Ok((a1, a2)),
      (Err(e1), Err(e2)) => Err(e1.combine(e2)),
      (Ok(_), Err(e)) => Err(e),
      (Err(e), Ok(_)) => Err(e),
    }
  }
}

pub trait ValidateThen<A, E> {
  fn validate_then(self, other: Valid<A, E>) -> Valid<A, E>;
}

impl<A1, A2, E> ValidateThen<A2, E> for Valid<A1, E> {
  fn validate_then(self, other: Valid<A2, E>) -> Valid<A2, E> {
    match (self, other) {
      (Err(e1), Err(e2)) => Err(e1.combine(e2)),
      (Err(e), Ok(_)) => Err(e),
      (Ok(_), other) => other,
    }
  }
}
