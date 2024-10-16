pub fn number_of_substrings(s: String) -> i32 {
  let mut a: i32 = -1;
  let mut b: i32 = -1;
  let mut c: i32 = -1;
  let string_arr: Vec<char> = s.chars().collect();
  let mut ans = 0;

  for i in 0..s.len() {
    let ch = string_arr[i];

    if ch == 'a' {
      a = i as i32;
    } else if ch == 'b' {
      b = i as i32;
    } else {
      c = i as i32;
    }

    if a != -1 && b != -1 && c != -1 {
      ans += std::cmp::min(a, std::cmp::min(b, c)) + 1;
    }
  }
  ans
}
