/*
    Anagram Check
    Given two strings, check if they are anagrams of each other. 
    Anagrams are words or phrases formed by rearranging the letters of another, 
    using all the original letters exactly once. 
    The strings may contain spaces or punctuation, but you need to ignore them while checking.

    You need to implement the function `are_anagrams(s1: String, s2: String) -> bool`.
    The function should return `true` if the two strings are anagrams, and `false` otherwise.

    Hint: Consider normalizing the strings by removing non-alphabetical characters and converting to lowercase before checking.
*/

use std::fmt::{self, Display, Formatter};
// by github copilot agent
pub fn are_anagrams(s1: String, s2: String) -> bool {
    // Convert strings to lowercase and filter out non-alphabetic characters
    let s1_chars: Vec<char> = s1.chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    
    let s2_chars: Vec<char> = s2.chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    // If lengths are different after normalization, they can't be anagrams
    if s1_chars.len() != s2_chars.len() {
        return false;
    }

    // Count character frequencies
    let mut char_count = [0; 26];
    
    for c in s1_chars {
        char_count[(c as u8 - b'a') as usize] += 1;
    }
    
    for c in s2_chars {
        char_count[(c as u8 - b'a') as usize] -= 1;
    }

    // Check if all counts are zero
    char_count.iter().all(|&count| count == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram_1() {
        let s1 = "listen".to_string();
        let s2 = "silent".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_2() {
        let s1 = "evil".to_string();
        let s2 = "vile".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_3() {
        let s1 = "hello".to_string();
        let s2 = "world".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, false);
    }

    #[test]
    fn test_anagram_4() {
        let s1 = "Clint Eastwood".to_string();
        let s2 = "Old West Action".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_5() {
        let s1 = "Astronomer".to_string();
        let s2 = "Moon starer".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }
}
