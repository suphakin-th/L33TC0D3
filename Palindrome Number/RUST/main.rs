impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let str_x = x.to_string();
        let chars = str_x.chars().collect::<Vec<_>>();
        for i in 0..str_x.len() / 2 {
            if chars[i] != chars[str_x.len() - i - 1] {
                return false;
            }
        }
        true
    }
}