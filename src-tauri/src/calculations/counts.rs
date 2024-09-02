pub fn get_word_count(input: String) -> usize {
    let remove_em_dashes = input.replace('\u{2014}', " ").to_lowercase();
    words_count::count(remove_em_dashes.to_lowercase()).words
}

pub fn get_char_count(input: String) -> i64 {
    words_count::count(input.replace(['\n', '\r'], "")).characters as i64
}

pub fn get_paragraph_count(input: String) -> i64 {
    input.split("\n\n\n\n").count() as i64
}

pub fn _get_line_count(input: String) -> i64 {
    (input.split("\n\n").count() - 1) as i64
}

pub fn get_ws_count(input: String) -> i64 {
    words_count::count(input).whitespaces as i64
}

// pub fn get_para_count(_path: String) {
//     todo!()
// }

pub fn get_top_used_word(input: String, nth_size: usize) -> String {
    let binding = input.to_lowercase();

    let mut nth_string = String::new();
    // let mut nth_size = 0usize;
    // let top = words_count::count_separately(binding.as_str())
    //     .into_iter()
    //     .max_by_key(|entry| entry.1);

    let mut top: Vec<(&str, usize)> = words_count::count_separately(binding.as_str())
        .into_iter()
        .filter(|a| {!a.0.is_empty()})
        .collect();

    top.sort_by_key(|ent| { ent.1 });
    top.reverse();

    // println!("{:?}", top);

    let mut pos= 0usize;
    if ((top.len() >= nth_size) && (nth_size > 0usize)) {
        pos = top.len() - nth_size;
    }

    // let mut rev_top = Vec::new();
    // for i in (0..top.len()).rev() {
    //     rev_top.push(top.get(i).unwrap())
    // }

    if let Some(s) = top.get(pos) {
        let (top_string, mut nth_size) = s;
        nth_string = top_string.to_string();
        // println!("toplen: {}, nths: {}, pos: {} -- {}", top.len(), nth_size, pos, nth_string);
        nth_size = nth_size;
    }

    nth_string.to_string()
}

pub fn get_unique_words(input: String) -> usize {
    let binding = input.to_lowercase();
    let result = words_count::count_separately(binding.as_str());
    result.len()
}
