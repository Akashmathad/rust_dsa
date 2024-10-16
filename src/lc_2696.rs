pub fn min_length(s: String) -> i32 {
  let arr: Vec<char> = s.chars().collect();
  let mut stack: Vec<char> = Vec::new();

  for i in 0..arr.len() {
    let ch: char = arr[i];

    if !stack.is_empty() {
      if let Some(&top) = stack.last() {
        if (ch == 'B' && top == 'A') || (ch == 'D' && top == 'C') {
          stack.pop();
          continue;
        }
      }
    }

    stack.push(ch);
  }
  return stack.len() as i32;
}
