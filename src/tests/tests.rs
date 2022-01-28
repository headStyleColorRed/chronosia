#[cfg(test)]

mod tests {
    use crate::tests::user::user_tests::user_tests::user_flow;
    use crate::tests::punch::punch_tests::punch_tests::punch_flow;

    #[tokio::main]
    #[test]
    async fn run_tests() {
        user_flow().await;
        punch_flow().await;
    }
}