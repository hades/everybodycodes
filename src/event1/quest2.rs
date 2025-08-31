use itertools::Itertools;
use regex::Regex;

struct Node {
    key: i64,
    symbol: char,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

struct Tree {
    root: Option<Box<Node>>,
}

fn insert(tree: &mut Tree, key: i64, symbol: char) {
    let mut node = &mut tree.root;
    while let Some(next_node) = node {
        if next_node.key > key {
            node = &mut next_node.left;
        } else {
            node = &mut next_node.right;
        }
    }
    *node = Some(Box::new(Node{key, symbol, left: None, right: None}));
}

fn as_rank_refs(tree: &Tree) -> Vec<Vec<&Node>> {
    let mut ranks = vec![];
    let mut next_rank = vec![];
    if let Some(ref node) = tree.root {
        next_rank.push(node.as_ref());
    }
    while next_rank.len() > 0 {
        ranks.push(next_rank.clone());
        next_rank.clear();
        for node in ranks.last().unwrap() {
            if let Some(ref left) = node.left {
                next_rank.push(left);
            }
            if let Some(ref right) = node.right {
                next_rank.push(right);
            }
        }
    }
    return ranks;
}

fn largest_rank_symbols(tree: &Tree) -> String {
    let largest_rank = as_rank_refs(&tree).into_iter().max_by_key(|rank| rank.len()).unwrap();
    largest_rank.into_iter().map(|node| node.symbol).join("")
}

pub fn solve_part_1(input: &str) -> String {
    let re = Regex::new(r"ADD id=([0-9]+) left=\[([0-9]+),(.)\] right=\[([0-9]+),(.)\]").unwrap();
    let mut left = Tree { root: None };
    let mut right = Tree { root: None };
    for (_, [_node_id, left_key, left_symbol, right_key, right_symbol]) in re.captures_iter(input).map(|c| c.extract()) {
        insert(&mut left, left_key.parse::<i64>().unwrap(), left_symbol.chars().next().unwrap());
        insert(&mut right, right_key.parse::<i64>().unwrap(), right_symbol.chars().next().unwrap());
    }
    format!("{}{}", largest_rank_symbols(&left), largest_rank_symbols(&right))
}

pub fn solve_part_2(input: &str) -> String { unimplemented!()}
pub fn solve_part_3(input: &str) -> String { unimplemented!()}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_part_1() {
        assert_eq!("CFGNLK", solve_part_1("ADD id=1 left=[10,A] right=[30,H]
ADD id=2 left=[15,D] right=[25,I]
ADD id=3 left=[12,F] right=[31,J]
ADD id=4 left=[5,B] right=[27,L]
ADD id=5 left=[3,C] right=[28,M]
ADD id=6 left=[20,G] right=[32,K]
ADD id=7 left=[4,E] right=[21,N]"));
        assert_eq!("EVERYBODYCODES", solve_part_1("ADD id=1 left=[160,E] right=[175,S]
ADD id=2 left=[140,W] right=[224,D]
ADD id=3 left=[122,U] right=[203,F]
ADD id=4 left=[204,N] right=[114,G]
ADD id=5 left=[136,V] right=[256,H]
ADD id=6 left=[147,G] right=[192,O]
ADD id=7 left=[232,I] right=[154,K]
ADD id=8 left=[118,E] right=[125,Y]
ADD id=9 left=[102,A] right=[210,D]
ADD id=10 left=[183,Q] right=[254,E]
ADD id=11 left=[146,E] right=[148,C]
ADD id=12 left=[173,Y] right=[299,S]
ADD id=13 left=[190,B] right=[277,B]
ADD id=14 left=[124,T] right=[142,N]
ADD id=15 left=[153,R] right=[133,M]
ADD id=16 left=[252,D] right=[276,M]
ADD id=17 left=[258,I] right=[245,P]
ADD id=18 left=[117,O] right=[283,!]
ADD id=19 left=[212,O] right=[127,R]
ADD id=20 left=[278,A] right=[169,C]"));
    }
}