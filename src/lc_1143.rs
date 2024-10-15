pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
  let l1 = text1.len();
  let l2 = text2.len();
  let text1: Vec<char> = text1.chars().collect();
  let text2: Vec<char> = text2.chars().collect();

  let mut dp = vec![vec![0; l2 + 1]; l1 + 1];
  
  for i in (0..l1).rev() {
    for j in (0..l2).rev() {
        if text1[i] == text2[j] {
            dp[i][j] = dp[i + 1][j + 1] + 1;
        } else {
            dp[i][j] = std::cmp::max(dp[i + 1][j], dp[i][j + 1]);
        }
    }
}

  dp[0][0]
}