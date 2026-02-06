// LOCAL
pub struct Solution;
// next better iteration should be [Manacher's Algorithm](https://www.geeksforgeeks.org/dsa/manachers-algorithm-linear-time-longest-palindromic-substring-part-1/)
// LOCAL END

fn expand_from_center(
    s: &[u8],
    mut cur_start: isize,
    mut cur_end: isize,
    start: &mut usize,
    end: &mut usize,
) {
    while cur_start >= 0
        && cur_end < s.len() as isize
        && s[cur_start as usize] == s[cur_end as usize]
    {
        if (cur_end - cur_start) as usize > *end - *start {
            *start = cur_start as usize;
            *end = cur_end as usize;
        }
        cur_start -= 1;
        cur_end += 1;
    }
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() < 2 {
            return s;
        }

        let b = s.as_bytes();
        let mut start = 0;
        let mut end = 0;

        for i in 0..b.len() {
            expand_from_center(b, i as isize, i as isize, &mut start, &mut end);
            expand_from_center(b, i as isize, i as isize + 1, &mut start, &mut end);
        }

        s[start..=end].to_owned()
    }
}
