pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
  let mut sum = 0;
  let mut count = 0;
  let mut l: usize = 0;

  for i in 0..k - 1 {
    sum += arr[i as usize];
  }

  for r in (k - 1) as usize..arr.len() {
    sum += arr[r];

    let avg = sum / k;

    if avg >= threshold {
      count += 1;
    }

    sum -= arr[l];
    l += 1;
  }

  count
}
