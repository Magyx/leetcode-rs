// LOCAL
pub struct Solution;
// LOCAL END

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() < 2 {
            return s.len() as i32;
        }

        let s = s.as_bytes();
        let mut arr = [0usize; 256];
        let mut longest = 0;
        let mut left = 0;

        for (right, &byte) in s.iter().enumerate() {
            if arr[byte as usize] >= left {
                left = arr[byte as usize];
            }
            arr[byte as usize] = right + 1;
            longest = longest.max(right - left + 1);

            #[cfg(debug_assertions)]
            println!(
                "window: {:?}",
                s[left..=right]
                    .iter()
                    .filter_map(|&b| char::from_u32(b as u32))
                    .collect::<Vec<char>>()
            );
        }

        longest as i32
    }
}
