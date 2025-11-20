use std::collections::HashMap;

use bit_set::BitSet;
use itertools::{Itertools, izip};
use log::debug;

type BitSetBase = u32;

fn is_child(child: &str, parent1: &str, parent2: &str) -> bool {
    izip!(child.chars(), parent1.chars(), parent2.chars()).all(|(c, a, b)| c == a || c == b)
}

fn similarity(a: &str, b: &str) -> usize {
    izip!(a.chars(), b.chars()).filter(|(a, b)| a == b).count()
}

fn unequality_bitset(a: &str, b: &str) -> BitSet<BitSetBase> {
    izip!(a.chars(), b.chars())
        .enumerate()
        .filter_map(|(i, (a_ch, b_ch))| if a_ch == b_ch { None } else { Some(i) })
        .collect()
}

pub fn solve_part_1(input: &str) -> String {
    let dnas = input
        .lines()
        .map(|l| {
            let (_, dna) = l.split_once(":").unwrap();
            dna
        })
        .collect::<Vec<_>>();
    let (child, parenta, parentb) = if is_child(dnas[0], dnas[1], dnas[2]) {
        (dnas[0], dnas[1], dnas[2])
    } else if is_child(dnas[1], dnas[0], dnas[2]) {
        (dnas[1], dnas[0], dnas[2])
    } else {
        (dnas[2], dnas[0], dnas[1])
    };
    (similarity(child, parenta) * similarity(child, parentb)).to_string()
}

pub fn solve_part_2(input: &str) -> String {
    let dnas = input
        .lines()
        .map(|l| {
            let (_, dna) = l.split_once(":").unwrap();
            dna
        })
        .collect::<Vec<_>>();
    let inequalities = dnas
        .iter()
        .enumerate()
        .cartesian_product(dnas.iter().enumerate())
        .map(|((a_id, a), (b_id, b))| ((a_id, b_id), unequality_bitset(a, b)))
        .collect::<HashMap<_, _>>();
    let mut total_similarities = 0;
    'outer: for (child_id, &child_string) in dnas.iter().enumerate() {
        for (parent_a_id, &parent_a_string) in dnas.iter().enumerate() {
            if parent_a_id == child_id {
                continue;
            }
            for (parent_b_id, &parent_b_string) in dnas.iter().enumerate() {
                if parent_b_id == child_id || parent_b_id == parent_a_id {
                    continue;
                }
                debug!("{child_id} {parent_a_id} {parent_b_id}");
                if inequalities[&(child_id, parent_a_id)]
                    .intersection(&inequalities[&(child_id, parent_b_id)])
                    .count()
                    == 0
                {
                    total_similarities += similarity(child_string, parent_a_string)
                        * similarity(child_string, parent_b_string);
                    continue 'outer;
                }
            }
        }
    }
    total_similarities.to_string()
}

pub fn solve_part_3(input: &str) -> String {
    let dnas = input
        .lines()
        .map(|l| {
            let (_, dna) = l.split_once(":").unwrap();
            dna
        })
        .collect::<Vec<_>>();
    let inequalities = dnas
        .iter()
        .enumerate()
        .cartesian_product(dnas.iter().enumerate())
        .map(|((a_id, a), (b_id, b))| ((a_id, b_id), unequality_bitset(a, b)))
        .collect::<HashMap<_, _>>();
    let mut parents: HashMap<usize, (usize, usize)> = HashMap::new();
    'outer: for (child_id, _) in dnas.iter().enumerate() {
        for (parent_a_id, _) in dnas.iter().enumerate() {
            if parent_a_id == child_id {
                continue;
            }
            for (parent_b_id, _) in dnas.iter().enumerate() {
                if parent_b_id == child_id || parent_b_id == parent_a_id {
                    continue;
                }
                debug!("{child_id} {parent_a_id} {parent_b_id}");
                if inequalities[&(child_id, parent_a_id)]
                    .intersection(&inequalities[&(child_id, parent_b_id)])
                    .count()
                    == 0
                {
                    parents.insert(child_id, (parent_a_id, parent_b_id));
                    continue 'outer;
                }
            }
        }
    }
    let mut family_id = (0usize..dnas.len()).collect::<Vec<_>>();
    for (child, (parent_a, parent_b)) in parents.drain() {
        let destination_family_id = family_id[child];
        let merge_families = (family_id[parent_a], family_id[parent_b]);
        for candidate_family in family_id.iter_mut() {
            if *candidate_family == merge_families.0 || *candidate_family == merge_families.1 {
                *candidate_family = destination_family_id;
            }
        }
    }
    let families = family_id
        .iter()
        .enumerate()
        .map(|(member_idx, &family_idx)| (family_idx, member_idx))
        .into_group_map();
    families
        .values()
        .max_by_key(|f| f.len())
        .unwrap()
        .iter()
        .map(|member_id| member_id + 1)
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_log::test;

    #[test]
    fn test_solve_part_1() {
        assert_eq!(
            "414",
            solve_part_1(
                "1:CAAGCGCTAAGTTCGCTGGATGTGTGCCCGCG
2:CTTGAATTGGGCCGTTTACCTGGTTTAACCAT
3:CTAGCGCTGAGCTGGCTGCCTGGTTGACCGCG"
            )
        );
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(
            "1245",
            solve_part_2(
                "1:GCAGGCGAGTATGATACCCGGCTAGCCACCCC
2:TCTCGCGAGGATATTACTGGGCCAGACCCCCC
3:GGTGGAACATTCGAAAGTTGCATAGGGTGGTG
4:GCTCGCGAGTATATTACCGAACCAGCCCCTCA
5:GCAGCTTAGTATGACCGCCAAATCGCGACTCA
6:AGTGGAACCTTGGATAGTCTCATATAGCGGCA
7:GGCGTAATAATCGGATGCTGCAGAGGCTGCTG"
            )
        );
    }

    #[test]
    fn test_solve_part_3() {
        assert_eq!(
            "36",
            solve_part_3(
                "1:GCAGGCGAGTATGATACCCGGCTAGCCACCCC
2:TCTCGCGAGGATATTACTGGGCCAGACCCCCC
3:GGTGGAACATTCGAAAGTTGCATAGGGTGGTG
4:GCTCGCGAGTATATTACCGAACCAGCCCCTCA
5:GCAGCTTAGTATGACCGCCAAATCGCGACTCA
6:AGTGGAACCTTGGATAGTCTCATATAGCGGCA
7:GGCGTAATAATCGGATGCTGCAGAGGCTGCTG
8:GGCGTAAAGTATGGATGCTGGCTAGGCACCCG"
            )
        );
    }
}
