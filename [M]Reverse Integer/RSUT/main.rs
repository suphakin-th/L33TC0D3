impl Solution {
    pub fn reverse(x: i32) -> i32 {
      let mut x = x;
        let mut result: i32 = 0;
        let int_max: i32 = std::i32::MAX;
        let int_min: i32 = std::i32::MIN;
        while x != 0 {
            let digit = x % 10;
            x /= 10;
            if result > (int_max / 10) || (result == int_max / 10 && digit > int_max % 10) {
                return 0;
            }
            if result < (int_min / 10) || (result == int_min / 10 && digit < int_min % 10) {
                return 0;
            }

            // Update result
            result = result * 10 + digit;
        }

        result
    }
}