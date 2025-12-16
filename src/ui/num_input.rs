/// Handles numeric inputs with normal `text_input`
pub fn num_input(previous_state: u64, input: &str) -> u64 {
    if input.is_empty() {
        0
    } else {
        input.parse().unwrap_or(previous_state)
    }
}
