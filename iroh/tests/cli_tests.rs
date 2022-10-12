#[tokio::test]
async fn lookup_cli_test() {
    trycmd::TestCases::new()
        .env("IROH_CTL_FIXTURE", "lookup")
        .case("tests/cmd/lookup.trycmd")
        .run();
}

#[tokio::test]
async fn get_success_cli_test() {
    trycmd::TestCases::new()
        .env("IROH_CTL_FIXTURE", "get")
        .case("tests/cmd/get_*_success.trycmd")
        .run();
}

#[tokio::test]
async fn get_failure_cli_test() {
    trycmd::TestCases::new()
        .env("IROH_CTL_FIXTURE", "get")
        .case("tests/cmd/get_*failure.trycmd")
        .run();
}

#[tokio::test]
async fn version_cli_test() {
    trycmd::TestCases::new()
        .case("tests/cmd/version.trycmd")
        .run();
}