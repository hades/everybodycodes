use std::collections::HashMap;
use std::fs;

use serde::Deserialize;
use serde::Serialize;

use crate::ecclient::AnswerResponse;
use crate::types::PuzzleKey;

const FILE: &str = "results.toml";

#[derive(Debug, Default, Deserialize, Serialize)]
struct PuzzleLogEntry {
    rejected_answers: Vec<String>,
    accepted_answer: Option<String>,
    rejected_first_characters: Vec<String>,
    rejected_answer_lengths: Vec<usize>,
    correct_first_character: Option<String>,
    correct_answer_length: Option<usize>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct Log {
    answers: HashMap<String, PuzzleLogEntry>,
}

fn puzzle_key_string(key: &PuzzleKey) -> String {
    format!(
        "event{}quest{}part{}",
        key.event,
        key.quest,
        key.part.as_u8()
    )
}

fn read_submission_log() -> Option<Log> {
    let log = fs::read_to_string(FILE).ok()?;
    let log: Log = toml::from_str(log.as_str()).ok()?;
    Some(log)
}

fn write_submission_log(log: &Log) {
    fs::write(FILE, toml::to_string_pretty(log).unwrap().as_str()).unwrap();
}

pub struct SubmissionResult {
    pub is_answer_correct: Option<bool>,
    pub is_first_character_correct: Option<bool>,
    pub is_length_correct: Option<bool>,
    pub details: Option<AnswerResponse>,
    pub cached: bool,
}

fn check_submission_log(key: &PuzzleKey, answer: &str) -> SubmissionResult {
    let mut result = SubmissionResult {
        details: None,
        is_answer_correct: None,
        is_first_character_correct: None,
        is_length_correct: None,
        cached: true,
    };
    let log = read_submission_log();
    if log.is_none() {
        return result;
    }
    let log = log.unwrap();
    if let Some(entry) = log.answers.get(puzzle_key_string(key).as_str()) {
        if entry.rejected_answer_lengths.contains(&answer.len()) {
            result.is_answer_correct = Some(false);
            result.is_length_correct = Some(false);
        } else if let Some(accepted_length) = &entry.correct_answer_length {
            result.is_length_correct = Some(*accepted_length == answer.len());
        }
        if let Some(first_character) = answer.chars().next() {
            let first_character = String::from(first_character);
            if entry.rejected_first_characters.contains(&first_character) {
                result.is_answer_correct = Some(false);
                result.is_first_character_correct = Some(false);
            } else if let Some(accepted_first_character) = &entry.correct_first_character {
                result.is_first_character_correct =
                    Some(*accepted_first_character == first_character);
            }
        }
        if result.is_answer_correct.is_some() {
            return result;
        }
        if entry.rejected_answers.iter().any(|a| a == answer) {
            result.is_answer_correct = Some(false);
        } else if let Some(accepted_answer) = &entry.accepted_answer {
            result.is_answer_correct = Some(accepted_answer == answer);
        }
    }
    result
}

fn record_submission_log(key: &PuzzleKey, answer: &str, result: &SubmissionResult) {
    let mut log = read_submission_log()
        .or_else(|| Some(Log::default()))
        .unwrap();
    let key = puzzle_key_string(key);
    if !log.answers.contains_key(&key) {
        log.answers.insert(key.clone(), PuzzleLogEntry::default());
    }
    let entry = log.answers.get_mut(&key).unwrap();
    match result.is_answer_correct {
        Some(true) => {
            entry.accepted_answer = Some(answer.to_string());
            entry.correct_answer_length = Some(answer.len());
            entry.correct_first_character = answer.chars().next().map(String::from);
        }
        Some(false) => {
            entry.rejected_answers.push(answer.to_string());
            if let Some(false) = result.is_length_correct
                && !entry.rejected_answer_lengths.contains(&answer.len())
            {
                entry.rejected_answer_lengths.push(answer.len());
            }
            if let (Some(false), Some(first_char)) =
                (result.is_first_character_correct, answer.chars().next())
            {
                let first_char = String::from(first_char);
                if !entry.rejected_first_characters.contains(&first_char) {
                    entry.rejected_first_characters.push(first_char);
                }
            }
        }
        _ => {}
    }
    write_submission_log(&log);
}

pub fn submit_with_cache<F>(key: &PuzzleKey, answer: &str, submit_fn: F) -> SubmissionResult
where
    F: FnOnce(&PuzzleKey, &str) -> AnswerResponse,
{
    let submission_log = check_submission_log(key, answer);
    if submission_log.is_answer_correct.is_some() {
        return submission_log;
    }
    let result = submit_fn(key, answer);
    let submission_result = SubmissionResult {
        is_answer_correct: Some(result.correct),
        is_first_character_correct: Some(result.first_correct),
        is_length_correct: Some(result.length_correct),
        details: Some(result),
        cached: false,
    };
    record_submission_log(key, answer, &submission_result);
    submission_result
}
