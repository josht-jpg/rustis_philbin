mod questions;

use clap::{App, Arg};
use questions::*;
// use questions::get_total_number_of_questions;
use rand::Rng;
use std::error::Error;
use std::io;
struct Settings {
    num_questions: u32,
    include_easy: bool,
    include_medium: bool,
    include_hard: bool,
}

fn parse_num_questions_argument(input: &str) -> Result<u32, Box<dyn Error>> {
    let total_num_questions = get_total_number_of_questions();

    match input.parse() {
        Ok(num_questions) if num_questions <= 0 => Err(From::from(input)),
        Ok(num_questions) if num_questions <= total_num_questions => Ok(num_questions),
        Ok(_) => {
            println!(
                "That a lot of questions, best I can do is {}, sorry. Have fun!",
                total_num_questions
            );
            Ok(total_num_questions)
        }
        _ => Err(From::from(input)),
    }
}

// TODO: change function name
fn get_args() -> Result<Settings, Box<dyn Error>> {
    let matches = App::new("Who wants to be a millionaire? (Rustacean edition)")
        .version("0.1.0")
        .author("Joshua Taylor joshtaylor361@gmail.com")
        .about("Who wants to be a millionaire? (Rustacean edition)")
        .arg(
            Arg::with_name("num_questions")
                .short("n")
                .long("questions")
                .value_name("NUM_QUESTIONS")
                .default_value("12")
                .help("Number of questions you'd like to be quized on"),
        )
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

    let num_questions = matches
        .value_of("num_questions")
        .map(parse_num_questions_argument)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;

    let include_easy = matches.is_present("easy");
    let include_medium = matches.is_present("medium");
    let include_hard = matches.is_present("hard");

    let include_all_difficulties = [include_easy, include_medium, include_hard]
        .iter()
        .all(|flag| !flag);

    Ok(Settings {
        num_questions: num_questions.unwrap(),
        include_easy: include_easy || include_all_difficulties,
        include_medium: include_medium || include_all_difficulties,
        include_hard: include_hard || include_all_difficulties,
    })
}

fn get_questions() -> Vec<questions::Question> {
    let mut questions = vec![];
    questions.extend(questions::EASY_QUESTIONS);
    return questions;
}

pub fn start() -> Result<(), io::Error> {
    println!("Who wants to be a rust millionaire?\n");
    println!("Let's start!\n");

    for question in get_questions() {
        println!("{}", question.prompt);

        let mut rng = rand::thread_rng();
        let corrent_answer_index: u8 = rng.gen_range(0..4);

        for i in 0..4 {
            println!(
                "{})\t{}",
                i + 1,
                if i == corrent_answer_index {
                    question.corrent_answer
                } else {
                    question.incorrect_answers[if i == 3 { 2 } else { i } as usize]
                }
            )
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().parse::<u8>() {
            Ok(input) if input >= 1 && input <= 4 => {
                if input - 1 == corrent_answer_index {
                    println!("\nCorret, well done! n more question to go.")
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

    println!("Congratulations, You won who wants to be a Rust millionaire!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn estimation_test() {}
}
