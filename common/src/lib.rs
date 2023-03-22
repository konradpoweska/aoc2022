pub fn lines_from_stdin() -> impl Iterator<Item = String> {
    std::io::stdin().lines().map(|e| e.expect("Invalid line"))
}
