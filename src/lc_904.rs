use std::collections::HashMap;

pub fn total_fruit(fruits: Vec<i32>) -> i32 {
  let mut map:HashMap<i32, i32> = HashMap::new();
  let mut l:usize = 0;
  let mut ans = 0;
  
  for r in 0..fruits.len() {
    if let Some(&val) = map.get(&fruits[r]){
      map.insert(fruits[r], val + 1);
    }else {
      map.insert(fruits[r], 1);
    }

    if map.len() <= 2 {
      let nans = (r - l + 1) as i32;
      if nans > ans {
        ans = nans;
      }
    }else {
      if let Some(&val) = map.get(&fruits[l]){
        if val == 1 {
          map.remove(&fruits[l]);
        }else {
          map.insert(fruits[l], val - 1);
        }
      }
      l -= 1;
    }
  }
  ans
}