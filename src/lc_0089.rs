pub fn gray_code(n: i32) -> Vec<i32> {
  let mut ans = vec![0];
  for i in 0..n {
    let size = ans.len();
    for j in (0..size).rev() {
      ans.push(ans[j] | (1 << i));
    }
  }
  ans
}
