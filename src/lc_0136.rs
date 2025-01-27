pub fn single_number(nums: Vec<i32>) -> i32 {
  let mut ans = 0;

  for num in nums {
    ans ^= num;
  }
  ans
}
