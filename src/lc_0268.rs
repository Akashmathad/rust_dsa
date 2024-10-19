pub fn missing_number(nums: Vec<i32>) -> i32 {
  let mut ans: i32 = 0;

  for i in 0..=nums.len() {
    ans ^= i as i32;
  }

  for num in nums {
    ans ^= num;
  }
  ans
}
