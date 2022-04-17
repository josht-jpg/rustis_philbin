pub struct Question {
    pub prompt: &'static str,
    pub incorrect_answers: [&'static str; 3],
    pub corrent_answer: &'static str,
}

const NUM_QUESTIONS_PER_SET: usize = 4;

type QuestionSet<'a> = [Question; NUM_QUESTIONS_PER_SET];

pub const EASY_QUESTIONS: QuestionSet = [
    Question {
        prompt: "What type is this: str?",
        incorrect_answers: ["Stir fry", "String", "Software Test Report"],
        corrent_answer: "String slice",
    },
    Question {
        prompt: "",
        incorrect_answers: ["Stir fry", "String", "Software Test Report"],
        corrent_answer: "String slice",
    },
    Question {
        prompt: "WWhat statement best describes T in this function:\tfn largest<T>(list: &[T]) -> T { /*...*/ }\t?",
        incorrect_answers: ["T is a String type", "String", "Software Test Report"],
        corrent_answer: "T is a Generic type",
    },
    Question {
        prompt: "What are the varients of Rust's Result type?",
        incorrect_answers: [
            "Some(T), None",
            "Some(T), Err(E)",
            "LooksGoodMate(T), HeadsUpCheifWeGotIssues(E)",
        ],
        corrent_answer: "Ok(T), Err(E)",
    },
];

pub const MEDIUM_QUESTIONS: QuestionSet = [
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

pub const HARD_QUESTIONS: QuestionSet = [
    Question {
        prompt: "What would cause the size of the usize type to change?",
        incorrect_answers: ["_", "_", "_"],
        corrent_answer: "Switching from a 32-bit platform to a 64-bit platform",
    },
    Question {
        prompt: "What would cause the size of the usize type to change?",
        incorrect_answers: ["_", "_", "_"],
        corrent_answer: "Switching from a 32-bit platform to a 64-bit platform",
    },
    Question {
        prompt: "What would cause the size of the usize type to change?",
        incorrect_answers: ["_", "_", "_"],
        corrent_answer: "Switching from a 32-bit platform to a 64-bit platform",
    },
    Question {
        prompt: "What would cause the size of the usize type to change?",
        incorrect_answers: ["_", "_", "_"],
        corrent_answer: "Switching from a 32-bit platform to a 64-bit platform",
    },
];
