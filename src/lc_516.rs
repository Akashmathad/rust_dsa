pub fn longest_palindrome_subseq(s: String) -> i32 {
  let l = s.len();
  let mut dp = vec![vec![0; l]; l];
  let string_arr: Vec<char> = s.chars().collect();

  for g in 0..l {
    for i in 0..l {
      let j = i + g;
      if j >= l {
        break;
      }

      if g == 0 {
        dp[i][j] = 1;
      }else if g == 1 && string_arr[i] == string_arr[j] {
        dp[i][j] = 2;
      }else {
        if string_arr[i] == string_arr[j] {
          dp[i][j] = 2 + dp[i + 1][j - 1];
        }else {
          dp[i][j] = std::cmp::max(dp[i + 1][j], dp[i][j - 1]);
        }
      }
    }
  }
  dp[0][l - 1]
}