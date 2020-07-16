pub struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace()
            .map(|each| each.trim())
            .rev()
            .collect::<Vec<&str>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::reverse_words(String::from("the sky is     blue")), "blue is sky the");
        assert_eq!(Solution::reverse_words(String::from("  hello world!  ")), "world! hello");
    }
}
