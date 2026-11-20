//! Throwaway sandbox crate for Kael's end-to-end loop tests (Phase 1, Task 16).
//! Kael breaks and fixes these on `kael/*` branches; nothing here matters.

/// Adds two numbers.
pub fn add(a: i64, b: i64) -> i64 {
    a + b
}

/// Classic FizzBuzz for one number.
pub fn fizzbuzz(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => n.to_string(),
    }
}

/// Returns true when `s` reads the same backwards (ASCII, case-insensitive).
pub fn is_palindrome(s: &str) -> bool {
    let c: Vec<u8> = s
        .bytes()
        .filter(u8::is_ascii_alphanumeric)
        .map(|b| b.to_ascii_lowercase())
        .collect();
    c.iter().eq(c.iter().rev())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-4, 4), 0);
    }

    #[test]
    fn fizzbuzz_cases() {
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(5), "Buzz");
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(7), "7");
    }

    #[test]
    fn palindromes() {
        assert!(is_palindrome("A man, a plan, a canal: Panama"));
        assert!(!is_palindrome("kael"));
    }
}
