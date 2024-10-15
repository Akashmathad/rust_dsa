pub fn count_substrings(s: String) -> i32 {
  let string_arr : Vec<char> = s.chars().collect();
  let mut count = 0;
  let l = string_arr.len();
  let mut dp = vec![vec![false; l]; l];

  for g in 0..l {
    for i in 0..dp.len() {
      let j = g + i;
      if j >= dp.len() {
          break;
      }
      if g == 0 {
        dp[i][j] = true;
      }else if g == 1 {
        dp[i][j] = string_arr[i] == string_arr[j];
      }else {
        dp[i][j] = string_arr[i] == string_arr[j] && dp[i + 1][j - 1];
      }
      if dp[i][j] {
        count += 1;
      }
  }
  }

  count  
}