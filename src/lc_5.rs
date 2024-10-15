pub fn longest_palindrome(s: String) -> String {
  let  string_arr: Vec<char> = s.chars().collect();
  let  l = s.len();
  let mut dp = vec![vec![false; l]; l];
  let mut ans = String::from("");

  for g in 0..l{
    for i in 0..l {
      let  j = g + i;
      if j >= l {
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
        let check = &s[i..j+1];
        if ans.len() < check.len() {
          ans = String::from(check);
        }
      }
    }
  }

  ans
}