mod questions;

use clap::{App, Arg};
use questions::*;
use rand::Rng;
// use std::error::Error;
use std::io;
pub struct Settings {
    include_easy: bool,
    include_medium: bool,
    include_hard: bool,
}

// TODO: change function name
pub fn get_args() -> Settings {
    let matches = App::new("Who wants to be a millionaire? (Rustacean edition)")
        .version("0.1.0")
        .author("Joshua Taylor joshtaylor361@gmail.com")
        .about("Who wants to be a millionaire? (Rustacean edition)")
        .arg(
            Arg::with_name("easy")
                .short("e")
                .long("easy")
                .takes_value(false)
                .help("Flags that you would like to include easy answers in your quiz"),
        )
        .arg(
            Arg::with_name("medium")
                .short("m")
                .long("medium")
                .takes_value(false)
                .help("Flags that you would like to include medium answers in your quiz"),
        )
        .arg(
            Arg::with_name("hard")
                .short("h")
                .long("hard")
                .takes_value(false)
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

// TODO: more rustic way to do this?
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

pub fn start_game(settings: Settings) -> Result<(), io::Error> {
    println!("\nWho wants to be a rust millionaire?\n");
    println!("Let's start!\n");

    let questions = get_questions(settings);

    for (i, question) in questions.iter().enumerate() {
        println!("{}", question.prompt);

        let mut rng = rand::thread_rng();
        let corrent_answer_index: u8 = rng.gen_range(0..4);

        let incorrect_answers_index =
            |i: u8| if corrent_answer_index > i { i } else { i - 1 } as usize;

        for i in 0..4 {
            println!(
                "{})\t{}",
                i + 1,
                if i == corrent_answer_index {
                    question.corrent_answer
                } else {
                    question.incorrect_answers[incorrect_answers_index(i)]
                }
            )
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().parse::<u8>() {
            Ok(input) if input >= 1 && input <= 4 => {
                if input - 1 == corrent_answer_index {
                    let num_questions_left = questions.len() - i - 1;
                    if num_questions_left > 0 {
                        println!(
                            "\nCorrect, well done! {} more question to go.\n",
                            num_questions_left
                        )
                    }
                } else {
                    println!("\nIncorrect! Try again anytime.");
                    // break;
                    std::process::exit(0)
                }
            }
            _ => {
                println!("Sorry, I wasn't able to understand that. Could you repeat your answer?");
                io::stdin().read_line(&mut input)?;
            }
        }
    }

    println!("\nCongratulations, You won who wants to be a Rust millionaire!\n");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn estimation_test() {}
}
