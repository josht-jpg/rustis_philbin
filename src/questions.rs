pub struct Question {
    pub prompt: &'static str,
    pub incorrect_answers: [&'static str; 3],
    pub corrent_answer: &'static str,
}

const NUM_QUESTIONS_PER_SET: usize = 3;
type QuestionSet<'a> = [Question; NUM_QUESTIONS_PER_SET];

pub const EASY_QUESTIONS: QuestionSet = [
    Question {
        prompt: "What type is this: str?",
        incorrect_answers: ["Stir fry", "String", "Software Test Report"],
        corrent_answer: "String slice",
    },
    Question {
        prompt: "What is a trait?",
        incorrect_answers: [
            "A data type that helps structure code by packaging related values together",
            "A feature that allows you to enumerate all of a type's variants",
            "Stir Fry",
        ],
        corrent_answer: "A set of method signatures that can be implemented by multiple types",
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
        prompt: "Which of the following types does not implement the Copy trait",
        incorrect_answers: ["i32", "char", "bool"],
        corrent_answer: "Box",
    },
    Question {
        prompt: "When would we use Rust's Box type?",
        incorrect_answers: [
            "When we want to interact with and store data on the stack",
            "When we want to access or modify a mutable static variable",
            "When we want to enable multiple ownership",
        ],
        corrent_answer: "When we want to interact with and store data on the heap",
    },
    Question {
        prompt: "When is a value dropped?",
        incorrect_answers: [
            "When the program ends",
            "When the function it's declared in finishes execution",
            "And let all the fly skimmies feel the beat Hmmmmmmm, Drrrrrop!",
        ],
        corrent_answer: "When the variable that holds it goes out of scope",
    },
];

pub const HARD_QUESTIONS: QuestionSet = [
		Question {
        prompt: "When can a type T be Sync?",
	      incorrect_answers: [
          "When T can be transferred across thread boundaries", 
          "When T is a plumbing fixture where one can wash their hands or clean kitchenware", 
          "When T is safe to be sent to another thread"
        ],
        corrent_answer: "T can be Sync if and only if &T is Send",
    },
    Question {
        prompt: "What is the difference between these two loops?
a) for e in some_vec {
     ...
}

b) for e in some_vec.iter() {
    ...
}",
        incorrect_answers: ["They are equivolent", "Loop (a) will not work, loop (b) will work", "Loop (b) will not work, loop (a) will work"],
        corrent_answer: "loop (a) consumes some_vec, and loop (b) borrows some_vec",
    },
    Question {
        prompt: "What is the difference between Rc and Arc?",
        incorrect_answers: [
            "Arc is used for multiple ownership. Rc is also used for multiple ownership, but is threadsafe.", 
            "Rc is used for interior immutability. Arc is also used for interior immutability, but is threadsafe", 
            "Rc stands for \"Reference Counted\", and Arc stands for \"Async Reference Counted\""
        ],
        corrent_answer: "Rc is used for multiple ownership. Arc is also used for multiple ownership, but is threadsafe.",
    },

];
