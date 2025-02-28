#[cfg(test)]
mod setup {
    use test_utils::{ProgramTestFixtures, TestResult};

    #[tokio::test]
    async fn test_setup() -> TestResult {
        let test_fixtures = ProgramTestFixtures::setup();

        Ok(())
    }
}
