pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
  let mut l: usize = 0;
  let mut sum = 0;
  let mut ans = std::i32::MAX;

  for r in 0..nums.len() {
    sum += nums[r];

    if sum >= target {
      ans = std::cmp::min(ans, (r - l + 1) as i32);

      loop {
        sum -= nums[l];
        l += 1;

        if sum >= target {
          ans = std::cmp::min(ans, (r - l + 1) as i32);
        }else {
          break;
        }
      }
    
  }
    }
      

  if ans == std::i32::MAX {
    return 0;
  }
  ans
}