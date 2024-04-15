// 1. Implement a function that checks whether a given string is a palindrome or not.
fn is_palindrome(s: &str) -> bool {
    // Check if the string is equal to its reverse
    s.chars().eq(s.chars().rev())
}

// 2. Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.
fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    // Find the position of the first occurrence of the target number in the array
    arr.iter().position(|&x| x == target)
}

// 3. Given a string of words, implement a function that returns the shortest word in the string.
fn shortest_word(s: &str) -> &str {
    // Split the string into words and find the minimum length word
    s.split_whitespace()
        .min_by_key(|&word| word.len())
        .unwrap_or_default()
}

// 4. Implement a function that checks whether a given number is prime or not.
fn is_prime(n: u64) -> bool {
    // Check if the number is divisible by any number from 2 to its square root
    if n <= 1 {
        return false;
    }
    for i in 2..=n / 2 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// 5. Given a sorted array of integers, implement a function that returns the median of the array.
fn median(arr: &[i32]) -> f64 {
    // Calculate the median based on the array length
    let len = arr.len();
    if len % 2 == 0 {
        (arr[len / 2 - 1] + arr[len / 2]) as f64 / 2.0
    } else {
        arr[len / 2] as f64
    }
}

// 6. Implement a function that finds the longest common prefix of a given set of strings.
fn longest_common_prefix(strs: &[String]) -> String {
    // Find the longest common prefix by comparing each string
    if strs.is_empty() {
        return String::new();
    }
    let mut prefix = strs[0].clone();
    for s in strs.iter().skip(1) {
        while !s.starts_with(&prefix) {
            prefix.pop();
        }
    }
    prefix
}

// 7. Implement a function that returns the kth smallest element in a given array.
fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    // Sort the array and return the kth element
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();
    sorted_arr.get(k - 1).cloned()
}

// 8. Given a binary tree, implement a function that returns the maximum depth of the tree.
struct TreeNode {
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    // Constructor to create a new TreeNode
    fn new() -> Self {
        TreeNode {
            left: None,
            right: None,
        }
    }

    // Recursive function to find the maximum depth of the tree
    fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                1 + std::cmp::max(Self::max_depth(node.left), Self::max_depth(node.right))
            }
        }
    }
}

// 9. Reverse a string in Rust
fn reverse_string(s: &str) -> String {
    // Reverse the characters of the string
    s.chars().rev().collect()
}

// 11. Merge two sorted arrays in Rust
fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    // Merge two sorted arrays into one sorted array
    let mut result = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0);
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            result.push(arr1[i]);
            i += 1;
        } else {
            result.push(arr2[j]);
            j += 1;
        }
    }
    result.extend_from_slice(&arr1[i..]);
    result.extend_from_slice(&arr2[j..]);
    result
}

// 12. Find the maximum subarray sum in Rust
fn max_subarray_sum(arr: &[i32]) -> i32 {
    // Kadane's algorithm to find the maximum subarray sum
    let mut max_sum = arr[0];
    let mut current_sum = arr[0];
    for &num in arr.iter().skip(1) {
        current_sum = std::cmp::max(num, current_sum + num);
        max_sum = std::cmp::max(max_sum, current_sum);
    }
    max_sum
}

fn main() {
    // Testing the functions

    // 1. Palindrome check
    println!("Is 'radar' a palindrome? {}", is_palindrome("radar"));

    // 2. First occurrence of a number in sorted array
    println!(
        "Index of first occurrence of 3: {:?}",
        first_occurrence(&[1, 2, 3, 3, 4, 5], 3)
    );

    // 3. Shortest word in a string
    println!(
        "Shortest word in 'hello world': {}",
        shortest_word("hello world")
    );

    // 4. Prime number check
    println!("Is 13 prime? {}", is_prime(13));

    // 5. Median of a sorted array
    println!("Median of [1, 2, 3, 4, 5]: {}", median(&[1, 2, 3, 4, 5]));

    // 6. Longest common prefix of strings
    println!(
        "Longest common prefix: {}",
        longest_common_prefix(&vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string()
        ])
    );

    // 7. Kth smallest element in array
    println!(
        "The 3rd smallest element: {:?}",
        kth_smallest(&[3, 1, 4, 1, 5, 9, 2, 6, 5, 3], 3)
    );

    // 8. Max depth of binary tree
    let tree = Some(Box::new(TreeNode::new()));
    println!("Max depth of binary tree: {}", TreeNode::max_depth(tree));

    // 9. Reverse a string
    println!("Reverse 'rust': {}", reverse_string("rust"));

    // 10. Prime number check (alternative implementation)
    println!("Is 17 prime? {}", is_prime(17));

    // 11. Merge two sorted arrays
    println!(
        "Merged sorted arrays: {:?}",
        merge_sorted_arrays(&[1, 3, 5], &[2, 4, 6])
    );

    // 12. Maximum subarray sum
    println!(
        "Maximum subarray sum: {}",
        max_subarray_sum(&[-2, 1, -3, 4, -1, 2, 1, -5, 4])
    );
}
