use crate::calculations;

pub struct CalcResults {
    pub white_space: i64,
    pub paragraph_count: i64,
    pub word_count: usize,
    pub unique_word_count: usize,
    pub char_count: i64,
    pub char_count_no_ws: i64,
    pub first_most_used: String,
    pub second_most_used: String,
    pub third_most_used: String
}

pub fn run_calculations(file_contents: String) -> CalcResults {
    // get calculcations
    let white_space = calculations::counts::get_ws_count(file_contents.clone());
    let paragraph_count =
        calculations::counts::get_paragraph_count(file_contents.clone());
    let word_count = calculations::counts::get_word_count(file_contents.clone());
    let unique_word_count =
        calculations::counts::get_unique_words(file_contents.clone());
    let char_count = calculations::counts::get_char_count(file_contents.clone());
    let char_count_no_ws = char_count - white_space;

    let first_most_used =
        calculations::counts::get_top_used_word(file_contents.clone(), 0usize);
    let second_most_used =
        calculations::counts::get_top_used_word(file_contents.clone(), 1usize);
    let third_most_used =
        calculations::counts::get_top_used_word(file_contents.clone(), 2usize);

    CalcResults {
        white_space,
        paragraph_count,
        word_count,
        unique_word_count,
        char_count,
        char_count_no_ws,
        first_most_used,
        second_most_used,
        third_most_used,
    }
}
