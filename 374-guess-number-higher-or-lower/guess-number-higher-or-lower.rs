/** 
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let (mut left, mut right) = (1, n);
        loop {
            let middle = left + (right - left) / 2;
            match guess(middle) {
                -1 => right = middle - 1,
                1 => left = middle + 1,
                _ => return middle,
            }
        }
    }
}