// let numbers = vec![3, 5, 9, 1, 6];
// println!("{}", list_largest(numbers)); 9

// let nums = vec![];
// println!("{}", list_largest(nums)); 0

fn list_largest(nums : Vec<i32>) -> i32 {
  if nums.is_empty(){
    0
  } else {
    nums.into_iter().max().unwrap()
  }
}
