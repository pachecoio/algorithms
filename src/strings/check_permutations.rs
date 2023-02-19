//! # Check Permutations
//! ## Problem
//! Given two strings, write a method to decide if one is a permutation of the other.
//!
//! ## Examples
//!
//! ```
//! use algorithms::strings::check_permutations::*;
//!
//! let s1 = "abc";
//! let s2 = "cba";
//! let result = check_permutations(s1, s2);
//! assert_eq!(result, true);
//! ```
//!
//! ```
//! use algorithms::strings::check_permutations::*;
//!
//! let s1 = "abc";
//! let s2 = "cbad";
//! let result = check_permutations(s1, s2);
//! assert_eq!(result, false);
//! ```

/// ## Solution
/// ### Approach
/// 1. Sort the strings
/// 2. Compare the sorted strings
///
/// ### Complexity
/// * Time: O(n log n)
/// * Space: O(n)
///
/// ### Code
pub fn check_permutations(s1: &str, s2: &str) -> bool {
    let mut s1_chars: Vec<char> = s1.chars().collect();
    let mut s2_chars: Vec<char> = s2.chars().collect();
    s1_chars.sort();
    s2_chars.sort();
    s1_chars == s2_chars
}