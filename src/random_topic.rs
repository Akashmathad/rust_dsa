use rand::seq::SliceRandom;

pub fn pick_random_topic() -> &'static str {
  let topics = [
    "Arrays & Hashing",
    "Two Pointers",
    "Sliding Window",
    "Stack",
    "Binary Search",
    "Linked List",
    "Trees",
    "Heap",
    "Backtracking",
    "Graphs",
    "1-D",
    "2-D",
    "Greedy",
    "Math",
    "Bit Manipulation",
  ];

  let mut rng = rand::thread_rng();
  topics.choose(&mut rng).unwrap()
}
