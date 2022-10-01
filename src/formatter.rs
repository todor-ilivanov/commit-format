pub fn format_commit_body(input: &str, line_length: usize) -> String {
    let words: Vec<String> = input.trim_end().split(' ').map(|w| w.to_string()).collect();
    let mut result: Vec<String> = Vec::new();

    for w in words {
        match result.last_mut() {
            Some(last) if len_if_appended(&w, last) < line_length => {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_formats_correctly() {
        let input = "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text.";
        let expected_result = "Lorem Ipsum is simply dummy text of the printing and typesetting\nindustry. Lorem Ipsum has been the industry's standard dummy text.";

        assert_eq!(format_commit_body(input, 72), expected_result)
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(format_commit_body("", 72), "")
    }

    #[test]
    fn test_long_word_is_unbroken() {
        let long_word = "thisisareallyreallyverylongword";
        assert_eq!(format_commit_body(long_word, 10), long_word)
    }
}
