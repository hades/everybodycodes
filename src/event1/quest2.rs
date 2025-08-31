use std::mem::swap;

use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Node {
    id: i64,
    key: i64,
    symbol: char,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

#[derive(Debug)]
struct Tree {
    root: Option<Box<Node>>,
}

fn insert(tree: &mut Tree, id: i64, key: i64, symbol: char) {
    let mut node = &mut tree.root;
    while let Some(next_node) = node {
        if next_node.key > key {
            node = &mut next_node.left;
        } else {
            node = &mut next_node.right;
        }
    }
    *node = Some(Box::new(Node {
        id,
        key,
        symbol,
        left: None,
        right: None,
    }));
}

fn find_node_mut(tree: &mut Tree, id: i64) -> &mut Node {
    let mut stack: Vec<_> = tree.root.iter_mut().map(|n| n.as_mut()).collect();
    while let Some(node) = stack.pop() {
        if node.id == id {
            return node;
        }
        if let Some(ref mut next_node) = node.left {
            stack.push(next_node.as_mut());
        }
        if let Some(ref mut next_node) = node.right {
            stack.push(next_node.as_mut());
        }
    }
    unreachable!()
}

fn swap_values(left: &mut Tree, right: &mut Tree, id: i64) {
    let left_node = find_node_mut(left, id);
    let right_node = find_node_mut(right, id);
    (left_node.key, right_node.key) = (right_node.key, left_node.key);
    (left_node.symbol, right_node.symbol) = (right_node.symbol, left_node.symbol);
}

fn find_nodes_get_parent(tree: &mut Option<Box<Node>>, id: i64) -> Vec<&mut Option<Box<Node>>> {
    let mut stack: Vec<_> = vec![tree];
    let mut res = vec![];
    while let Some(node_holder) = stack.pop() {
        if node_holder.is_none() {
            continue;
        }
        let node_id = node_holder.as_ref().unwrap().id;
        if node_id == id {
            res.push(node_holder);
            continue;
        }
        let node = node_holder.as_mut().unwrap();
        stack.push(&mut node.left);
        stack.push(&mut node.right);
    }
    res
}

fn swap_with_detach(left: &mut Tree, right: &mut Tree, id: i64) {
    let mut left_holders = find_nodes_get_parent(&mut left.root, id);
    let mut right_holders = find_nodes_get_parent(&mut right.root, id);
    let (left, right) = match (left_holders.len(), right_holders.len()) {
        (0, 2) => (right_holders.pop(), right_holders.pop()),
        (1, 1) => (left_holders.pop(), right_holders.pop()),
        (2, 0) => (left_holders.pop(), left_holders.pop()),
        _ => unreachable!(),
    };
    swap(left.unwrap(), right.unwrap());
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
    let rank_refs = as_rank_refs(&tree);
    let largest_rank_size = rank_refs.iter().map(|r| r.len()).max().unwrap();
    rank_refs
        .into_iter()
        .filter(|r| r.len() == largest_rank_size)
        .next()
        .unwrap()
        .into_iter()
        .map(|node| node.symbol)
        .join("")
}

pub fn solve_part_1(input: &str) -> String {
    let re = Regex::new(r"ADD id=([0-9]+) left=\[([0-9]+),(.)\] right=\[([0-9]+),(.)\]").unwrap();
    let mut left = Tree { root: None };
    let mut right = Tree { root: None };
    for (_, [_node_id, left_key, left_symbol, right_key, right_symbol]) in
        re.captures_iter(input).map(|c| c.extract())
    {
        insert(
            &mut left,
            0,
            left_key.parse::<i64>().unwrap(),
            left_symbol.chars().next().unwrap(),
        );
        insert(
            &mut right,
            0,
            right_key.parse::<i64>().unwrap(),
            right_symbol.chars().next().unwrap(),
        );
    }
    format!(
        "{}{}",
        largest_rank_symbols(&left),
        largest_rank_symbols(&right)
    )
}

pub fn solve_part_2(input: &str) -> String {
    let add_re =
        Regex::new(r"ADD id=([0-9]+) left=\[([0-9]+),(.)\] right=\[([0-9]+),(.)\]").unwrap();
    let swap_re = Regex::new(r"SWAP ([0-9]+)").unwrap();
    let mut left = Tree { root: None };
    let mut right = Tree { root: None };
    for line in input.lines() {
        if let Some((_, [node_id, left_key, left_symbol, right_key, right_symbol])) =
            add_re.captures(line).map(|c| c.extract())
        {
            insert(
                &mut left,
                node_id.parse::<i64>().unwrap(),
                left_key.parse::<i64>().unwrap(),
                left_symbol.chars().next().unwrap(),
            );
            insert(
                &mut right,
                node_id.parse::<i64>().unwrap(),
                right_key.parse::<i64>().unwrap(),
                right_symbol.chars().next().unwrap(),
            );
            continue;
        }
        if let Some((_, [node_id])) = swap_re.captures(line).map(|c| c.extract()) {
            swap_values(&mut left, &mut right, node_id.parse::<i64>().unwrap());
        }
    }
    format!(
        "{}{}",
        largest_rank_symbols(&left),
        largest_rank_symbols(&right)
    )
}

pub fn solve_part_3(input: &str) -> String {
    let add_re =
        Regex::new(r"ADD id=([0-9]+) left=\[([0-9]+),(.)\] right=\[([0-9]+),(.)\]").unwrap();
    let swap_re = Regex::new(r"SWAP ([0-9]+)").unwrap();
    let mut left = Tree { root: None };
    let mut right = Tree { root: None };
    for line in input.lines() {
        if let Some((_, [node_id, left_key, left_symbol, right_key, right_symbol])) =
            add_re.captures(line).map(|c| c.extract())
        {
            insert(
                &mut left,
                node_id.parse::<i64>().unwrap(),
                left_key.parse::<i64>().unwrap(),
                left_symbol.chars().next().unwrap(),
            );
            insert(
                &mut right,
                node_id.parse::<i64>().unwrap(),
                right_key.parse::<i64>().unwrap(),
                right_symbol.chars().next().unwrap(),
            );
            continue;
        }
        if let Some((_, [node_id])) = swap_re.captures(line).map(|c| c.extract()) {
            swap_with_detach(&mut left, &mut right, node_id.parse::<i64>().unwrap());
        }
    }
    format!(
        "{}{}",
        largest_rank_symbols(&left),
        largest_rank_symbols(&right)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_part_1() {
        assert_eq!(
            "CFGNLK",
            solve_part_1(
                "ADD id=1 left=[10,A] right=[30,H]
ADD id=2 left=[15,D] right=[25,I]
ADD id=3 left=[12,F] right=[31,J]
ADD id=4 left=[5,B] right=[27,L]
ADD id=5 left=[3,C] right=[28,M]
ADD id=6 left=[20,G] right=[32,K]
ADD id=7 left=[4,E] right=[21,N]"
            )
        );
        assert_eq!(
            "EVERYBODYCODES",
            solve_part_1(
                "ADD id=1 left=[160,E] right=[175,S]
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
ADD id=20 left=[278,A] right=[169,C]"
            )
        );
    }

    #[test]
    fn test_part_3() {
        assert_eq!(
            "DJMGL",
            solve_part_3(
                "ADD id=1 left=[10,A] right=[30,H]
ADD id=2 left=[15,D] right=[25,I]
ADD id=3 left=[12,F] right=[31,J]
ADD id=4 left=[5,B] right=[27,L]
ADD id=5 left=[3,C] right=[28,M]
SWAP 1
SWAP 5
ADD id=6 left=[20,G] right=[32,K]
ADD id=7 left=[4,E] right=[21,N]
SWAP 2"
            )
        );
        assert_eq!(
            "DJCGL",
            solve_part_3(
                "ADD id=1 left=[10,A] right=[30,H]
ADD id=2 left=[15,D] right=[25,I]
ADD id=3 left=[12,F] right=[31,J]
ADD id=4 left=[5,B] right=[27,L]
ADD id=5 left=[3,C] right=[28,M]
SWAP 1
SWAP 5
ADD id=6 left=[20,G] right=[32,K]
ADD id=7 left=[4,E] right=[21,N]
SWAP 2
SWAP 5"
            )
        );
    }
}
