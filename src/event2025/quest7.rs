use std::collections::{HashMap, HashSet};

pub fn solve_part_1(input: &str) -> String {
    let (names, rules) = input.split_once("\n\n").unwrap();
    let names: Vec<&str> = names.split(",").collect();
    let rules: HashMap<char, HashSet<char>> = rules
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" > ").unwrap();
            let to = to.split(",");
            (
                from.chars().next().unwrap(),
                to.map(|s| s.chars().next().unwrap()).collect(),
            )
        })
        .collect();
    for name in names {
        let mut allowed_chars = rules.get(&name.chars().next().unwrap());
        let mut acceptable = true;
        for ch in name.chars().skip(1) {
            match allowed_chars {
                Some(allowed) => {
                    if !allowed.contains(&ch) {
                        acceptable = false;
                        break;
                    }
                    allowed_chars = rules.get(&ch);
                }
                None => {
                    panic!("no rules for letter {ch} in name {name}");
                }
            }
        }
        if acceptable {
            return name.to_string();
        }
    }
    panic!("all names bad");
}

pub fn solve_part_2(input: &str) -> String {
    let (names, rules) = input.split_once("\n\n").unwrap();
    let names: Vec<&str> = names.split(",").collect();
    let rules: HashMap<char, HashSet<char>> = rules
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" > ").unwrap();
            let to = to.split(",");
            (
                from.chars().next().unwrap(),
                to.map(|s| s.chars().next().unwrap()).collect(),
            )
        })
        .collect();
    let mut sum_of_indices = 0;
    for (i, name) in names.into_iter().enumerate() {
        let mut allowed_chars = rules.get(&name.chars().next().unwrap());
        let mut acceptable = true;
        for ch in name.chars().skip(1) {
            match allowed_chars {
                Some(allowed) => {
                    if !allowed.contains(&ch) {
                        acceptable = false;
                        break;
                    }
                    allowed_chars = rules.get(&ch);
                }
                None => {
                    panic!("no rules for letter {ch} in name {name}");
                }
            }
        }
        if acceptable {
            sum_of_indices += 1 + i;
        }
    }
    sum_of_indices.to_string()
}

fn gen_names_with_prefix(
    prefix: &str,
    rules: &HashMap<char, HashSet<char>>,
    result: &mut HashSet<String>,
) {
    if prefix.len() >= 7 {
        result.insert(prefix.to_string());
    }
    if prefix.len() == 11 {
        return;
    }
    let last_char = prefix.chars().last().unwrap();
    if let Some(next_chars) = rules.get(&last_char) {
        for next_char in next_chars {
            let new_prefix = format!("{prefix}{next_char}");
            gen_names_with_prefix(new_prefix.as_str(), rules, result);
        }
    }
}

pub fn solve_part_3(input: &str) -> String {
    let (prefix, rules) = input.split_once("\n\n").unwrap();
    let prefixes: Vec<_> = prefix.split(",").collect();
    let rules: HashMap<char, HashSet<char>> = rules
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" > ").unwrap();
            let to = to.split(",");
            (
                from.chars().next().unwrap(),
                to.map(|s| s.chars().next().unwrap()).collect(),
            )
        })
        .collect();
    let mut results: HashSet<String> = HashSet::new();
    prefixes
        .into_iter()
        .filter(|&name| {
            let mut allowed_chars = rules.get(&name.chars().next().unwrap());
            let mut acceptable = true;
            for ch in name.chars().skip(1) {
                match allowed_chars {
                    Some(allowed) => {
                        if !allowed.contains(&ch) {
                            acceptable = false;
                            break;
                        }
                        allowed_chars = rules.get(&ch);
                    }
                    None => {
                        panic!("no rules for letter {ch} in name {name}");
                    }
                }
            }
            acceptable
        })
        .for_each(|prefix| gen_names_with_prefix(prefix, &rules, &mut results));
    results.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_log::test;

    #[test]
    fn test_solve_part_1() {
        assert_eq!(
            "Oroneth",
            solve_part_1(
                "Oronris,Urakris,Oroneth,Uraketh

r > a,i,o
i > p,w
n > e,r
o > n,m
k > f,r
a > k
U > r
e > t
O > r
t > h"
            )
        )
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(
            "23",
            solve_part_2(
                "Xanverax,Khargyth,Nexzeth,Helther,Braerex,Tirgryph,Kharverax

r > v,e,a,g,y
a > e,v,x,r
e > r,x,v,t
h > a,e,v
g > r,y
y > p,t
i > v,r
K > h
v > e
B > r
t > h
N > e
p > h
H > e
l > t
z > e
X > a
n > v
x > z
T > i"
            )
        );
    }

    #[test]
    fn test_solve_part_3() {
        assert_eq!(
            "25",
            solve_part_3(
                "Xaryt

X > a,o
a > r,t
r > y,e,a
h > a,e,v
t > h
v > e
y > p,t"
            )
        );
        assert_eq!(
            "1154",
            solve_part_3(
                "Khara,Xaryt,Noxer,Kharax

r > v,e,a,g,y
a > e,v,x,r,g
e > r,x,v,t
h > a,e,v
g > r,y
y > p,t
i > v,r
K > h
v > e
B > r
t > h
N > e
p > h
H > e
l > t
z > e
X > a
n > v
x > z
T > i"
            )
        );
    }
}
