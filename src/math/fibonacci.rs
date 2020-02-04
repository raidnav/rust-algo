mod math {
  pub fn fibonacci(n: u32) -> u128 {
    let mut a = 0;
    let mut b = 1;
    for _i in 0..n {
      let c = a + b;
      a = b;
      b = c;
    }
    b
  }

  pub fn recursive_fibonacci(n: u32) -> u128 {
    _recursive_fibonacci(n, 0, 1)
  }

  fn _recursive_fibonacci(n: u32, previous: u128, current: u128) -> u128 {
    if n == 0 {
      current
    } else {
      _recursive_fibonacci(n - 1, current, current + previous)
    }
  }

}