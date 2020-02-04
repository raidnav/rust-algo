pub fn bubble_sort<T: ord>(arr: &mut [T]) {
  for i in 0..arr.len() {
    for j in 0..arr.len() - 1 - i {
      if arr[i] > arr[j + 1] {
        arr.swap(i, j + 1)
      }
    }
  }
}