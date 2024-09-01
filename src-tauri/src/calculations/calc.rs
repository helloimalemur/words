use crate::calculations;

pub fn run_calculations(file_contents: String) {
    // get calculcations
    let white_space = calculations::counts::get_ws_count(file_contents.clone());
    let paragraph_count =
        calculations::counts::get_paragraph_count(file_contents.clone());
    let word_count = calculations::counts::get_word_count(file_contents.clone());
    let unique_word_count =
        calculations::counts::get_unique_words(file_contents.clone());
    let char_count = calculations::counts::get_char_count(file_contents.clone());
    let _char_count_no_ws = char_count - white_space;

    let one_most_used =
        calculations::counts::get_top_used_word(file_contents.clone());
    let _two_most_used =
        calculations::counts::get_top_used_word(file_contents.clone());
    let _third_most_used =
        calculations::counts::get_top_used_word(file_contents.clone());
}
