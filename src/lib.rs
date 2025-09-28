mod console;
mod fio;
mod http;
use futures::Stream;
use std::io::Error;

pub use console::{Stdin, Stdout};

pub trait Input<T> {
    fn into_stream(self) -> impl Stream<Item = T>;
}

pub trait Process<T, U> {
    fn process<S>(
        &self,
        stream: S,
    ) -> impl std::future::Future<Output = impl Stream<Item = U>> + Send
    where
        S: Stream<Item = T> + Send;
}

pub trait Output<T> {
    fn output<S>(&self, stream: S) -> impl std::future::Future<Output = Result<(), Error>> + Send
    where
        S: Stream<Item = T> + Send;
}
