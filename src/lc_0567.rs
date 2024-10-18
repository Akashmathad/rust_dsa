use std::collections::HashMap;

pub fn check_inclusion(s1: String, s2: String) -> bool {
  if s1.len() > s2.len() {
    return false;
  }
  let mut map: HashMap<char, i32> = HashMap::new();
  let mut count = 0;
  let mut l: usize = 0;
  let arr1: Vec<char> = s1.chars().collect();
  let arr2: Vec<char> = s2.chars().collect();
  let l1 = s1.len();
  let l2 = s2.len();

  for i in 0..l1 - 1 {
    if let Some(&val) = map.get(&arr1[i]) {
      map.insert(arr1[i], val + 1);
    } else {
      map.insert(arr1[i], 1);
    }
  }

  for i in 0..l1 - 1 {
    let ch = arr2[i];

    if let Some(&val) = map.get(&ch) {
      if val > 0 {
        count += 1;
      }
      map.insert(ch, val - 1);
    } else {
      map.insert(ch, -1);
    }
  }

  for r in l1 - 1..l2 {
    let mut ch = arr2[r];

    if let Some(&val) = map.get(&ch) {
      if val > 0 {
        count += 1;
      }
      map.insert(ch, val - 1);
    } else {
      map.insert(ch, -1);
    }

    if count == l1 {
      return true;
    }

    ch = arr2[l];
    l += 1;

    if let Some(&val) = map.get(&ch) {
      if val + 1 > 0 {
        count -= 1;
      }
      map.insert(ch, val + 1);
    }
  }

  false
}
