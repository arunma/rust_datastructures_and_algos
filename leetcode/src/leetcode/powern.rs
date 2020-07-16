struct Solution {}

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut n: i64 = n as i64;
        let mut x = x;
        if n < 0 {
            n = -n;
            x = 1.0 / x;
        }
        let mut ans = 1.0;
        let mut curr = x;
        let mut i = n;
        while i > 0 {
            if i % 2 == 1 {
                ans = ans * curr;
            }
            curr = curr * curr;
            i = i / 2
        }
        ans
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::my_pow(2.0000, 10), 1024.00000);
        assert_eq!(Solution::my_pow(2.1000, 3), 9.261000000000001);
        assert_eq!(Solution::my_pow(2.0000, -2), 0.25000);
        assert_eq!(Solution::my_pow(1.00000, -2147483648), 1.0);
        assert_eq!(Solution::my_pow(2.00000, -2147483648), 0.0000);
    }
}
