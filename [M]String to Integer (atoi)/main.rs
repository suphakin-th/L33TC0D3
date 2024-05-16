impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let int_max: i32 = 2_147_483_647;
        let int_min: i32 = -2_147_483_648;

        let bytes = s.as_bytes();
        let mut i = 0;
        let n = bytes.len();
        while i < n && bytes[i] == b' ' {
            i += 1;
        }
        let mut sign = 1;
        if i < n {
            if bytes[i] == b'-' {
                sign = -1;
                i += 1;
            } else if bytes[i] == b'+' {
                i += 1;
            }
        }
        let mut result: i64 = 0;
        while i < n && bytes[i].is_ascii_digit() {
            let digit = (bytes[i] - b'0') as i64;
            result = result * 10 + digit;
            if result * sign < int_min as i64 {
                return int_min;
            }
            if result * sign > int_max as i64 {
                return int_max;
            }
            
            i += 1;
        }
        (result * sign) as i32
    }
}