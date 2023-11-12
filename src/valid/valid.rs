use crate::valid::Cause;

use super::ValidationError;

pub type Valid<A, E> = Result<A, ValidationError<E>>;

pub trait ValidStructCompatibility<A, E>: Sized {
  fn fail(error: E) -> Valid<A, E>;

  fn from_validation_err(error: ValidationError<E>) -> Self;

  fn from_vec_cause(causes: Vec<Cause<E>>) -> Self;

  fn succeed(value: A) -> Valid<A, E>;

  fn trace(self, message: &str) -> Valid<A, E>;

  fn when(self, f: impl FnOnce() -> bool) -> Valid<(), E> {
    if f() {
      self.unit()
    } else {
      Self::succeed(())
    }
  }

  fn map_to_unit(self) -> Valid<(), E>;
}

impl<A, E> ValidStructCompatibility<A, E> for Valid<A, E> {
  fn fail(error: E) -> Valid<A, E> {
    Err(ValidationError::new(error))
  }

  fn from_validation_err(error: ValidationError<E>) -> Self {
    Err(error)
  }

  fn from_vec_cause(causes: Vec<Cause<E>>) -> Self {
    Err(causes.into())
  }

  fn succeed(value: A) -> Valid<A, E> {
    Ok(value)
  }

  fn trace(self, message: &str) -> Valid<A, E> {
    self.map_err(|err| err.trace(message))
  }

  fn map_to_unit(self) -> Valid<(), E> {
    self.map(|_| ())
  }
}

#[cfg(test)]
mod tests {
  use crate::valid::valid::Valid;
  use crate::valid::{ValidStructCompatibility, ValidateAll};

  use super::{Cause, ValidationError};

  #[test]
  fn test_ok() {
    let result = Valid::<i32, ()>::succeed(1);
    assert_eq!(result, Valid::succeed(1));
  }

  #[test]
  fn test_fail() {
    let result = Valid::<(), i32>::fail(1);
    assert_eq!(result, Valid::fail(1));
  }

  #[test]
  fn test_validate_or_both_ok() {
    let result1 = Valid::<bool, i32>::succeed(true);
    let result2 = Valid::<u8, i32>::succeed(3);

    assert_eq!(result1.and(result2), Valid::succeed(3u8));
  }

  #[test]
  fn test_validate_or_first_fail() {
    let result1 = Valid::<bool, i32>::fail(-1);
    let result2 = Valid::<u8, i32>::succeed(3);

    assert_eq!(result1.and(result2), Valid::fail(-1));
  }

  #[test]
  fn test_validate_or_second_fail() {
    let result1 = Valid::<bool, i32>::succeed(true);
    let result2 = Valid::<u8, i32>::fail(-2);

    assert_eq!(result1.and(result2), Valid::fail(-2));
  }

  #[test]
  fn test_validate_all() {
    let input: Vec<i32> = [1, 2, 3].to_vec();
    let result: Valid<Vec<i32>, i32> = input.validate_all(|a| Valid::fail(a * 2));
    assert_eq!(result, Err(vec![Cause::new(2), Cause::new(4), Cause::new(6)]));
  }

  #[test]
  fn test_validate_all_ques() {
    let input: Vec<i32> = [1, 2, 3].to_vec();
    let result: Valid<Vec<i32>, i32> = input.validate_all(|a| Valid::fail(a * 2));
    assert_eq!(
      result,
      Valid::from_vec_cause(vec![Cause::new(2), Cause::new(4), Cause::new(6)])
    );
  }

  #[test]
  fn test_ok_ok_cause() {
    let option: Option<i32> = None;
    let result = Valid::from_option(option, 1);
    assert_eq!(result, Valid::from_vec_cause(vec![Cause::new(1)]));
  }

  #[test]
  fn test_trace() {
    let result = Valid::<(), i32>::fail(1).trace("A").trace("B").trace("C");
    let expected = Valid::from_vec_cause(vec![Cause {
      message: 1,
      description: None,
      trace: vec!["C".to_string(), "B".to_string(), "A".to_string()].into(),
    }]);
    assert_eq!(result, expected);
  }

  #[test]
  fn test_validate_fold_err() {
    let valid = Valid::<(), i32>::fail(1);
    let result = valid.fold(|_| Valid::<(), i32>::fail(2), Valid::<(), i32>::fail(3));
    assert_eq!(result, Valid::from_vec_cause(vec![Cause::new(1), Cause::new(3)]));
  }

  #[test]
  fn test_validate_fold_ok() {
    let valid = Valid::<i32, i32>::succeed(1);
    let result = valid.fold(Valid::<i32, i32>::fail, Valid::<i32, i32>::fail(2));
    assert_eq!(result, Valid::fail(1));
  }

  #[test]
  fn test_to_result() {
    let result = Valid::<(), i32>::fail(1).to_result().unwrap_err();
    assert_eq!(result, ValidationError::new(1));
  }

  #[test]
  fn test_validate_both_ok() {
    let result1 = Valid::<bool, i32>::succeed(true);
    let result2 = Valid::<u8, i32>::succeed(3);

    assert_eq!(result1.zip(result2), Valid::succeed((true, 3u8)));
  }

  #[test]
  fn test_validate_both_first_fail() {
    let result1 = Valid::<bool, i32>::fail(-1);
    let result2 = Valid::<u8, i32>::succeed(3);

    assert_eq!(result1.zip(result2), Valid::fail(-1));
  }

  #[test]
  fn test_validate_both_second_fail() {
    let result1 = Valid::<bool, i32>::succeed(true);
    let result2 = Valid::<u8, i32>::fail(-2);

    assert_eq!(result1.zip(result2), Valid::fail(-2));
  }

  #[test]
  fn test_validate_both_both_fail() {
    let result1 = Valid::<bool, i32>::fail(-1);
    let result2 = Valid::<u8, i32>::fail(-2);

    assert_eq!(
      result1.zip(result2),
      Valid::from_vec_cause(vec![Cause::new(-1), Cause::new(-2)])
    );
  }

  #[test]
  fn test_and_then_success() {
    let result = Valid::<i32, i32>::succeed(1).and_then(|a| Valid::succeed(a + 1));
    assert_eq!(result, Valid::succeed(2));
  }

  #[test]
  fn test_and_then_fail() {
    let result = Valid::<i32, i32>::succeed(1).and_then(|a| Valid::<i32, i32>::fail(a + 1));
    assert_eq!(result, Valid::fail(2));
  }
}
