use crate::{Input, Output};
use futures::stream::StreamExt;
use serde::Deserialize;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio_stream::wrappers::LinesStream;

#[derive(Debug, Deserialize)]
pub struct Stdin {}

impl Input<String> for Stdin {
    fn into_stream(self) -> impl futures::Stream<Item = String> {
        LinesStream::new(tokio::io::BufReader::new(tokio::io::stdin()).lines())
            .filter_map(|line| async move { line.ok() })
    }
}

#[derive(Debug, Deserialize)]
pub struct Stdout {}

impl<T: std::fmt::Display + Send> Output<T> for Stdout {
    fn output<S>(
        &self,
        stream: S,
    ) -> impl std::future::Future<Output = Result<(), std::io::Error>> + Send
    where
        S: futures::Stream<Item = T> + Send,
    {
        async move {
            let mut stdout = tokio::io::stdout();
            futures::pin_mut!(stream);
            while let Some(o) = stream.next().await {
                let s = format!("{o}\n");
                stdout.write(s.as_bytes()).await.unwrap();
            }
            Ok(())
        }
    }
}
