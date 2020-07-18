use std::collections::{HashMap, HashSet};

struct Solution {}

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adj_list = HashMap::new();
        for each in prerequisites {
            adj_list.entry(each[0]).or_insert(Vec::new()).push(each[1]);
        }

        let mut white: HashSet<i32> = (0..num_courses).collect();
        let mut gray = HashSet::new();
        let mut black = HashSet::new();
        let mut top_sort = Vec::new();

        for node in 0..num_courses {
            if black.contains(&node) {
                continue;
            } else if Solution::is_cycle_found(&adj_list, &mut white, &mut gray, &mut black, &mut top_sort, node) {
                return vec![];
            }
        }

        top_sort
    }

    fn is_cycle_found(adj_list: &HashMap<i32, Vec<i32>>,
                      white: &mut HashSet<i32>,
                      gray: &mut HashSet<i32>,
                      black: &mut HashSet<i32>,
                      top_sort: &mut Vec<i32>,
                      node: i32) -> bool {
        white.remove(&node);
        gray.insert(node);

        if let Some(adjs) = adj_list.get(&node)
        {
            for adj in adjs {
                if black.contains(adj) {
                    continue;
                } else if gray.contains(adj) {
                    return true;
                } else if Solution::is_cycle_found(adj_list, white, gray, black, top_sort, *adj) {
                    return true;
                }
            }
        }

        top_sort.push(node);

        gray.remove(&node);
        black.insert(node);
        false
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::find_order(2, vec![vec![1, 0]]), vec![0, 1]);
        assert_eq!(Solution::find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]), vec![0,1,2,3])
    }
}
