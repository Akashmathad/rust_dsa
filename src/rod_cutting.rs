pub fn rod_cutting(price: Vec<i32>, n: i32) -> i32{
  let mut dp = vec![0; (n + 1) as usize];
  let l = dp.len();
  dp[1] = price[0];

  for i in 2..l {
    let mut max = price[i - 1];
    for j in 1..i {
      max = std::cmp::max(max, price[j - 1] + dp[i - j]);
    }
    dp[i] = max;
  }
  dp[l - 1]
}