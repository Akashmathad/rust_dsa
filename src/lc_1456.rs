pub fn max_vowels(s: String, k: i32) -> i32 {
  let string_arr: Vec<char> = s.chars().collect();
  let mut vowels = 0;
  let mut ans = 0;
  let mut l = 0;

  for i in 0..k - 1 {
    let ch = string_arr[i as usize];

    if ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' {
      vowels += 1;
    }
  }

  for r in (k - 1) as usize..s.len() {
    let mut ch = string_arr[r];
    if ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' {
      vowels += 1;
    }

    ans = std::cmp::max(ans, vowels);

    ch = string_arr[l];
    if ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' {
      vowels -= 1;
    }
    l += 1;
  }

  ans
}
