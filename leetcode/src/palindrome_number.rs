use crate::Solution;

impl Solution {
    // assume the number uses decimal system
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            false
        } else {
            let mut i = x;
            let mut num = 0_i32;
            loop {
                let rem = i % 10;
                i = (i - rem) / 10;

                num = num * 10 + rem;

                if i == 0 {
                    break;
                }
            }

            num == x
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_number() {
        assert_eq!(true, Solution::is_palindrome(121));
        assert_eq!(false, Solution::is_palindrome(-121));
        assert_eq!(false, Solution::is_palindrome(10));
    }
}
