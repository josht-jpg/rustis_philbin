mod questions;

use clap::{Arg, Command};
use questions::*;
use rand::{thread_rng, Rng};
use std::io;

pub struct Settings {
    include_easy: bool,
    include_medium: bool,
    include_hard: bool,
}

pub fn get_game_settings() -> Settings {
    let matches =
        Command::new("rustis_philbin")
            .version("0.1.0")
            .author("Joshua Taylor, joshtaylor361@gmail.com")
            .about("Who Wants to Be a Millionaire? (Rustacean edition)")
            .arg(
                Arg::new("easy")
                    .short('e')
                    .long("easy")
                    .help("Flags that you would like to include easy answers in your quiz"),
            )
            .arg(Arg::new("medium").short('m').long("medium").help(
                "Flags that you would like to include medium difficulty answers in your quiz",
            ))
            .arg(
                Arg::new("hard")
                    .short('h')
                    .long("hard")
                    .help("Flags that you would like to include hard answers in your quiz"),
            )
            .get_matches();

    let include_easy = matches.is_present("easy");
    let include_medium = matches.is_present("medium");
    let include_hard = matches.is_present("hard");

    let include_all_difficulties = [include_easy, include_medium, include_hard]
        .iter()
        .all(|flag| !flag);

    Settings {
        include_easy: include_easy || include_all_difficulties,
        include_medium: include_medium || include_all_difficulties,
        include_hard: include_hard || include_all_difficulties,
    }
}

pub fn run_game(settings: Settings) -> Result<(), io::Error> {
    println!("\nWho wants to be a millionaire?\nLet's start!\n");

    let questions = get_questions(settings);

    for (i, question) in questions.iter().enumerate() {
        println!("{}", question.prompt);

        let correct_answer_index: usize = thread_rng().gen_range(0..4);
        print_answers(question, correct_answer_index);

        let num_questions_left = questions.len() - i - 1;
        get_answer(correct_answer_index, num_questions_left)?;
    }

    println!("\nCongratulations, you won Who Wants to Be a Millionaire! (Rustacean edition)\n");
    Ok(())
}

fn get_questions(settings: Settings) -> Vec<Question> {
    let mut questions = vec![];

    if settings.include_easy {
        questions.extend(EASY_QUESTIONS);
    }
    if settings.include_medium {
        questions.extend(MEDIUM_QUESTIONS);
    }
    if settings.include_hard {
        questions.extend(HARD_QUESTIONS);
    }

    questions
}

fn print_answers(question: &Question, correct_answer_index: usize) {
    let mut answers = question.incorrect_answers.to_vec();
    answers.insert(correct_answer_index, question.correct_answer);

    for (i, answer) in answers.iter().enumerate() {
        println!("\n{})\t{}", i + 1, answer)
    }
}

fn get_answer(correct_answer_index: usize, num_questions_left: usize) -> Result<(), io::Error> {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input)?;

    loop {
        match input.trim().parse::<u8>() {
            Ok(input) if (1..=4).contains(&input) => {
                handle_valid_input(input, correct_answer_index, num_questions_left);
                break;
            }
            _ => {
                println!("Sorry, I wasn't able to understand that. Could you repeat your answer?");
                input.clear();
                stdin.read_line(&mut input)?;
            }
        }
    }

    Ok(())
}

fn handle_valid_input(input: u8, correct_answer_index: usize, num_questions_left: usize) {
    let is_correct_answer = input - 1 == correct_answer_index as u8;
    if is_correct_answer {
        if num_questions_left > 0 {
            println!(
                "\nCorrect, well done! {num_questions_left} {}.\n",
                if num_questions_left == 1 {
                    "question left"
                } else {
                    "questions more to go"
                }
            )
        }
    } else {
        println!("\nIncorrect! Try again anytime.");
        std::process::exit(0)
    }
}
