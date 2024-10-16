use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
  let mut max = 0;
  let string_arr: Vec<char> = s.chars().collect();
  let mut map: HashMap<char, usize> = HashMap::new();
  let mut l: usize = 0;

  for r in 0..string_arr.len() {
    if let Some(&val) = map.get(&string_arr[r]) {
      l = std::cmp::max(l, val + 1);
    }
    map.insert(string_arr[r], r);
    max = std::cmp::max(max, (r - l + 1) as i32);
  }

  max
}
