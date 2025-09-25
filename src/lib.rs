use std::io::Error;

use futures::Stream;

pub trait Source<T> {
    fn into_stream(self) -> impl Stream<Item = T>;
}

pub trait Transformer<T, U> {
    fn proc<S>(&self, stream: S) -> impl std::future::Future<Output = impl Stream<Item = U>> + Send
    where
        S: Stream<Item = T> + Send;
}

pub trait Sink<T> {
    fn sink<S>(&self, stream: S) -> impl std::future::Future<Output = Result<(), Error>> + Send
    where
        S: Stream<Item = T> + Send;
}
