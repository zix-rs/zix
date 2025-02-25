#[cfg(test)]
mod tests {
    use zix_core::grid::out;
    use colored::Colorize;

    #[test]
    fn test_empty_input() {
        let result = out::<String>(vec![]);
        assert_eq!(result, "There are no items to display");
    }

    #[test]
    fn test_single_item() {
        let result = out(vec!["hello"]);
        assert_eq!(result.trim(), "hello");
    }

    #[test]
    fn test_ansi_codes() {
        let result = out(vec!["\x1b[31mred\x1b[0m", "\x1b[32mgreen\x1b[0m", "\x1b[34mblue\x1b[0m"]);
        assert!(result.contains("red"));
        assert!(result.contains("green"));
        assert!(result.contains("blue"));
    }

    #[test]
    fn test_unicode() {
        let result = out(vec!["こんにちは", "안녕하세요", "你好"]);
        assert!(result.contains("こんにちは"));
        assert!(result.contains("안녕하세요"));
        assert!(result.contains("你好"));
    }

    #[test]
    fn test_varying_lengths() {
        let result = out(vec!["short", "medium length", "very long string"]);
        assert!(result.contains("short"));
        assert!(result.contains("medium length"));
        assert!(result.contains("very long string"));
    }

    #[test]
    fn test_large_input() {
        let items: Vec<String> = (0..50).map(|i| format!("item {}", i)).collect();
        let result = out(items);
        assert!(result.lines().count() > 1); // Ensure multiple rows are created
    }
}
