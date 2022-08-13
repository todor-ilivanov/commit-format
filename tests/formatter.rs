#[cfg(test)]
mod tests {
    use cli_commit_formatter::format_commit_body;

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
