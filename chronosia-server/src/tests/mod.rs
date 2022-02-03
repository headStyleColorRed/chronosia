pub mod generic;
pub mod user;
pub mod punch;
pub mod tests;

pub mod utils {
    #[allow(dead_code)]
    pub fn format_error(expected: &str, received: &str) -> String {
        format!("\n\n- Expected: {}\n- Received: {}\n\n", expected, received)
    }
}