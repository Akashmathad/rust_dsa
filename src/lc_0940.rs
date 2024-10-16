use std::collections::HashMap;
pub fn distinct_subseq_ii(s: String) -> i32 {
  let string_arr: Vec<char> = s.chars().collect();
  let l = s.len();
  let mut dp = vec![0; l + 1];
  dp[0] = 1;
  let MOD = 1000000007;
  let mut map: HashMap<char, usize> = HashMap::new();

  for i in 1..=l {
    dp[i] = (2 * dp[i - 1]) % MOD;

    if let Some(&j) = map.get(&string_arr[i - 1]) {
      dp[i] = (dp[i] - dp[j] + MOD) % MOD;
    }

    map.insert(string_arr[i - 1], i - 1);
  }

  (dp[l] - 1 + MOD) % MOD
}
