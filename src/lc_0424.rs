pub fn character_replacement(s: String, k: i32) -> i32 {
  let string_arr: Vec<char> = s.chars().collect();
  let mut l: usize = 0;
  let mut max = 0;
  let mut ans = 0;
  let mut frequency = vec![0; 26];

  for r in 0..s.len() {
    let index = ((string_arr[r] as u8) - ('A' as u8)) as usize;
    frequency[index] += 1;
    max = std::cmp::max(max, frequency[index]);

    while ((r - l + 1) as i32) - max > k {
      frequency[((string_arr[l] as u8) - ('A' as u8)) as usize] -= 1;
      l += 1;
    }

    ans = std::cmp::max(ans, (r - l + 1) as i32);
  }
  ans
}
