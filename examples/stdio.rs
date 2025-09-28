use data_proc::{Input, Output, Stdin, Stdout};

#[tokio::main]
async fn main() {
    let stdin = Stdin {};
    let stdout = Stdout {};
    stdout.output(stdin.into_stream()).await.unwrap();
}
