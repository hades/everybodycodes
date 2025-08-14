use std::collections::HashSet;

use aho_corasick::AhoCorasick;
use interval::interval_set::IntervalSet;
use interval::interval_set::ToIntervalSet;
use interval::prelude::Bounded;
use interval::prelude::Empty;
use interval::prelude::Union;
use log::debug;
use trie_rs::Trie;
use trie_rs::TrieBuilder;
use trie_rs::inc_search::Answer;

pub fn solve_part_1(input: &str) -> String {
    let lines: Vec<&str> = input.split("\n").collect();
    let words: Vec<&str> = lines[0]["WORDS:".len()..].split(",").collect();
    let ac = AhoCorasick::new(words).unwrap();
    let matches_count = ac.find_overlapping_iter(lines[2]).count();
    matches_count.to_string()
}

pub fn solve_part_2(input: &str) -> String {
    let lines: Vec<&str> = input.split("\n").collect();
    // For each word, add the word and its reverse.
    let words: Vec<String> = lines[0]["WORDS:".len()..]
        .split(",")
        .flat_map(|word| [word.to_string(), word.chars().rev().collect::<String>()])
        .collect();
    let ac = AhoCorasick::new(words).unwrap();
    let symbols_count: usize = lines[2..]
        .iter()
        .map(|line| {
            let rune_intervals = ac.find_overlapping_iter(line).fold(
                IntervalSet::<usize>::empty(),
                |interval_set, mat| {
                    let range = mat.range();
                    interval_set.union(&(range.start, range.end - 1).to_interval_set())
                },
            );
            rune_intervals
                .iter()
                .map(|interval| interval.upper() - interval.lower() + 1)
                .sum::<usize>()
        })
        .sum();
    symbols_count.to_string()
}

fn find_words_starting_at<F>(
    rune_lines: &Vec<Vec<char>>,
    words: &Trie<char>,
    mut i: usize,
    mut j: usize,
    advance: F,
) -> Vec<(usize, usize)>
where
    F: Fn((usize, usize)) -> Option<(usize, usize)>,
{
    let mut inc_search = words.inc_search();
    let mut result_matching = vec![];
    let mut result_pending = vec![];
    while let Some(answer) = inc_search.query(&rune_lines[i][j]) {
        debug!("{} {} {:?}", i, j, answer);
        match answer {
            Answer::Match | Answer::PrefixAndMatch => {
                result_pending.push((i, j));
                result_matching.append(&mut result_pending);
            }
            Answer::Prefix => {
                result_pending.push((i, j));
            }
        }
        match advance((i, j)) {
            Some((ni, nj)) => {
                (i, j) = (ni, nj);
            }
            None => {
                break;
            }
        };
    }
    result_matching
}

fn trie_from_strs<'a, T: Iterator<Item = &'a str>>(strs: T) -> Trie<char> {
    let mut builder = TrieBuilder::new();
    for line in strs {
        let chars: Vec<char> = line.chars().collect();
        builder.push(chars);
    }
    builder.build()
}

pub fn solve_part_3(input: &str) -> String {
    let lines: Vec<&str> = input.split("\n").collect();
    let words = trie_from_strs(lines[0]["WORDS:".len()..].split(","));
    let rune_lines: Vec<Vec<char>> = lines[2..]
        .iter()
        .map(|line| line.chars().collect())
        .collect();
    let width = rune_lines[0].len();
    let mut scales_with_runic_words: HashSet<(usize, usize)> = HashSet::new();
    // Loop over every possible starting position of a word.
    for start_i in 0..rune_lines.len() {
        for start_j in 0..width {
            debug!("{} {} {:?}", start_i, start_j, scales_with_runic_words);
            find_words_starting_at(&rune_lines, &words, start_i, start_j, |(i, j)| {
                Some((i, (j + 1) % width))
            })
            .into_iter()
            .for_each(|c| {
                scales_with_runic_words.insert(c);
            });
            find_words_starting_at(&rune_lines, &words, start_i, start_j, |(i, j)| {
                Some((i, (j + width - 1) % width))
            })
            .into_iter()
            .for_each(|c| {
                scales_with_runic_words.insert(c);
            });
            find_words_starting_at(&rune_lines, &words, start_i, start_j, |(i, j)| {
                if i + 1 >= rune_lines.len() {
                    None
                } else {
                    Some((i + 1, j))
                }
            })
            .into_iter()
            .for_each(|c| {
                scales_with_runic_words.insert(c);
            });
            find_words_starting_at(&rune_lines, &words, start_i, start_j, |(i, j)| {
                if i == 0 { None } else { Some((i - 1, j)) }
            })
            .into_iter()
            .for_each(|c| {
                scales_with_runic_words.insert(c);
            });
        }
    }
    scales_with_runic_words.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_log::test;

    #[test]
    fn test_solve_part_1() {
        assert_eq!(
            "4",
            solve_part_1(
                "WORDS:THE,OWE,MES,ROD,HER

AWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE"
            )
        );
        assert_eq!(
            "3",
            solve_part_1(
                "WORDS:THE,OWE,MES,ROD,HER

THE FLAME SHIELDED THE HEART OF THE KINGS"
            )
        );
        assert_eq!(
            "2",
            solve_part_1(
                "WORDS:THE,OWE,MES,ROD,HER

POWE PO WER P OWE R"
            )
        );
        assert_eq!(
            "3",
            solve_part_1(
                "WORDS:THE,OWE,MES,ROD,HER

THERE IS THE END"
            )
        );
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(
            "42",
            solve_part_2(
                "WORDS:THE,OWE,MES,ROD,HER,QAQ

AWAKEN THE POWE ADORNED WITH THE FLAMES BRIGHT IRE
THE FLAME SHIELDED THE HEART OF THE KINGS
POWE PO WER P OWE R
THERE IS THE END
QAQAQ"
            )
        );
    }

    #[test]
    fn test_solve_part_3() {
        assert_eq!(
            "10",
            solve_part_3(
                "WORDS:THE,OWE,MES,ROD,RODEO

HELWORLT
ENIGWDXL
TRODEOAL"
            )
        );
    }

    #[test]
    fn test_solve_part_3_one_letter() {
        assert_eq!(
            "1",
            solve_part_3(
                "WORDS:H

HELWORLT
ENIGWDXL
TRODEOAL"
            )
        );
    }
}
