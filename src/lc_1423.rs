pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
  let mut sum = 0;
  let mut ans = std::i32::MAX;

  for i in 0..k{
    sum += card_points[i as usize];
  }
  ans = sum;
  let mut l = (k - 1) as usize;
  let mut r = card_points.len() - 1;
  let mut nk = k;

  while nk > 0 {
    sum -= card_points[l];
    sum += card_points[r];
    ans = std::cmp::max(ans, sum);
    l -= 1;
    r -= 1;
    nk -=1;
  }
  ans
}