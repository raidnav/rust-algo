#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_fibonacci() {
    assert_eq!(fibonacci(0), 1);
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(2), 2);
    assert_eq!(fibonacci(3), 3);
    assert_eq!(fibonacci(4), 5);
    assert_eq!(fibonacci(5), 8);
    assert_eq!(fibonacci(10), 89);
    assert_eq!(fibonacci(20), 10946);
    assert_eq!(fibonacci(100), 573147844013817084101);
    assert_eq!(fibonacci(184), 205697230343233228174223751303346572685);
  }

  #[test]
  fn test_recursive_fibonacci() {
    assert_eq!(recursive_fibonacci(0), 1);
    assert_eq!(recursive_fibonacci(1), 1);
    assert_eq!(recursive_fibonacci(2), 2);
    assert_eq!(recursive_fibonacci(3), 3);
    assert_eq!(recursive_fibonacci(4), 5);
    assert_eq!(recursive_fibonacci(5), 8);
    assert_eq!(recursive_fibonacci(10), 89);
    assert_eq!(recursive_fibonacci(20), 10946);
    assert_eq!(recursive_fibonacci(100), 573147844013817084101);
    assert_eq!(
      recursive_fibonacci(184),
      205697230343233228174223751303346572685
    );
  }
}