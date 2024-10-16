pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
  let mut ans = 0;
  let mut l: usize = 0;
  let mut count = 0;

  for r in 0..nums.len() {
    if nums[r] == 0 {
      count += 1;
    }

    if count <= k {
      let nans = (r - l + 1) as i32;
      if nans > ans {
        ans = nans;
      }
    } else {
      if nums[l] == 0 {
        count -= 1;
      }
      l += 1;
    }
  }
  ans
}
