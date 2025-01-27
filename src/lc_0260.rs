pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
  let mut ans = vec![0;2];
  let mut n = 0;

  for num in nums.iter() {
    n ^= *num;
  }

  for num in nums.iter() {
    if (num & (n & -n)) == 0 {
      ans[0] ^= num;
    } else {
      ans[1] ^= num;
    }
  }

  ans
}
