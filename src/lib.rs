pub fn format_commit_body(input: &str, line_length: usize) -> String {
    let words: Vec<String> = input.trim_end().split(' ').map(|w| w.to_string()).collect();
    let mut result: Vec<String> = Vec::new();

    for w in words {
        match result.last_mut() {
            Some(last) if len_if_appended(&w, &last) < line_length => {
                let to_append = format!(" {}", w);
                last.push_str(&to_append)
            }
            _ => result.push(w),
        }
    }

    result.join("\n")
}

fn len_if_appended(new_word: &str, line: &str) -> usize {
    new_word.len() + line.len() + 1
}
