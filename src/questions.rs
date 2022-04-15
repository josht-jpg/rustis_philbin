pub struct Question {
    pub prompt: &'static str,
    pub incorrect_answers: [&'static str; 3],
    pub corrent_answer: &'static str,
}

type QuestionSet<'a> = [Question; 4];

pub const EASY_QUESTIONS: QuestionSet = [
    Question {
        prompt: "What does a lifetime annoation do?",
        incorrect_answers: ["_", "_", "_"],
        corrent_answer: "Specifies the scope for which a reference is valid",
    },
    Question {
        prompt: "What are the varients of Rust's Result type?",
        incorrect_answers: ["Some(T), None", "_", "_"],
        corrent_answer: "Ok(T), Err(E)",
    },
    Question {
        prompt: "What does a lifetime annoation do?",
        incorrect_answers: ["_", "_", "_"],
        corrent_answer: "Specifies the scope for which a reference is valid",
    },
    Question {
        prompt: "What does a lifetime annoation do?",
        incorrect_answers: ["_", "_", "_"],
        corrent_answer: "Specifies the scope for which a reference is valid",
    },
];

pub fn get_total_number_of_questions() -> u32 {
    EASY_QUESTIONS.len() as u32
}
