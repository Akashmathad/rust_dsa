pub fn hamming_weight(n: i32) -> i32 {
  let mut count = 0;
  let mut num = n;

  while num > 0 {
    let rsb = num & -num;
    num -= rsb;
    count += 1;
  }
  count
}
