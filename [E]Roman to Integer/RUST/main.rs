impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut data_dict = [0; 256];
        data_dict[b'I' as usize] = 1;
        data_dict[b'V' as usize] = 5;
        data_dict[b'X' as usize] = 10;
        data_dict[b'L' as usize] = 50;
        data_dict[b'C' as usize] = 100;
        data_dict[b'D' as usize] = 500;
        data_dict[b'M' as usize] = 1000;
        let mut pre_v = 0;
        let mut sum = 0;
        for c in s.bytes() {
            let current_val = data_dict[c as usize];
            
            if pre_v < current_val {
                sum += current_val - pre_v * 2;
            } else {
                sum += current_val;
            }
            pre_v = current_val;
        }
        sum
    }
}