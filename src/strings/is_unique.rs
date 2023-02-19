//! # Is Unique
//! Implement an algorithm to determine if a string has all unique characters.
//! What if you cannot use additional data structures?


use std::collections::HashSet;

/// Solution: Use a hashset to store the characters in the string.
/// If the character is already in the hashset, return false.
/// If the character is not in the hashset, add it to the hashset.
/// If the loop completes, return true.
/// ```
/// use algorithms::strings::is_unique::*;
/// let string = "abcdefg";
/// let result = is_unique(string);
/// assert_eq!(result, true);
/// ```
///
/// ```
/// use algorithms::strings::is_unique::*;
/// let string = "abcdefga";
/// let result = is_unique(string);
/// assert_eq!(result, false);
/// ```
pub fn is_unique(string: &str) -> bool {
    let mut hashset = HashSet::new();
    for character in string.chars() {
        if hashset.contains(&character) {
            return false;
        } else {
            hashset.insert(character);
        }
    }
    true
}