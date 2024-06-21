/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        let mut original = x;
        let mut reversed = 0;

        while original > reversed {
            reversed = reversed * 10 + original % 10;
            original /= 10;
        }

        // When the length of the number is odd, we can skip the middle digit by dividing it by 10
        original == reversed || original == reversed / 10
    }
}
// @lc code=end

