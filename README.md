# data-proc

A lightweight, composable data processing pipeline framework for Rust with async support.

[![Crates.io](https://img.shields.io/crates/v/data-proc.svg)](https://crates.io/crates/data-proc)
[![Documentation](https://docs.rs/data-proc/badge.svg)](https://docs.rs/data-proc)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## Overview

`data-proc` provides a simple yet powerful abstraction for building data processing pipelines using Rust's async streams. The library is built around three core traits:

- **Source**: Produces a stream of data items
- **Transformer**: Processes a stream of input items into a stream of output items
- **Sink**: Consumes a stream of items

This design allows for composable, reusable components that can be combined to build complex data processing workflows.

## Features

- 🔄 **Stream-based**: Built on top of Rust's `futures::Stream` for efficient async processing
- 🧩 **Composable**: Mix and match sources, transformers, and sinks to build custom pipelines
- 🚀 **Concurrent**: Easily parallelize data processing with async/await
- 🔌 **Extensible**: Implement the traits for your own types to integrate with existing components

## Installation

Add `data-proc` to your `Cargo.toml`:

```toml
[dependencies]
data-proc = "0.1.0"
```

## Usage

### Basic Example

```rust
use data_proc::{Source, Transformer, Sink};
use futures::stream;
use futures::StreamExt;

// Define a simple source that produces numbers
struct NumberSource(Vec<i32>);

impl Source<i32> for NumberSource {
    fn into_stream(self) -> impl Stream<Item = i32> {
        stream::iter(self.0)
    }
}

// Define a transformer that doubles each number
struct Doubler;

impl Transformer<i32, i32> for Doubler {
    fn proc<S>(&self, stream: S) -> impl std::future::Future<Output = impl Stream<Item = i32>> + Send
    where
        S: Stream<Item = i32> + Send,
    {
        async move {
            stream.map(|n| n * 2)
        }
    }
}

// Define a sink that prints each number
struct PrintSink;

impl Sink<i32> for PrintSink {
    fn sink<S>(&self, stream: S) -> impl std::future::Future<Output = Result<(), std::io::Error>> + Send
    where
        S: Stream<Item = i32> + Send,
    {
        async move {
            futures::pin_mut!(stream);

            while let Some(item) = stream.next().await {
                println!("Got: {}", item);
            }

            Ok(())
        }
    }
}

// Use them together in a pipeline
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let source = NumberSource(vec![1, 2, 3, 4, 5]);
    let transformer = Doubler;
    let sink = PrintSink;

    // Connect the components
    sink.sink(transformer.proc(source.into_stream()).await).await
}
```

### Real-world Example

Check out the [parquet-elasticsearch example](examples/parquet-elasticsearch.rs) for a more complex use case that:

1. Reads Parquet files from a directory
2. Transforms the data into Elasticsearch bulk API format
3. Sends the data to an Elasticsearch cluster

## Core Traits

### Source

```rust
pub trait Source<T> {
    fn into_stream(self) -> impl Stream<Item = T>;
}
```

Implement this trait for types that produce data. The `into_stream` method consumes the source and returns a stream of items.

### Transformer

```rust
pub trait Transformer<T, U> {
    fn proc<S>(&self, stream: S) -> impl std::future::Future<Output = impl Stream<Item = U>> + Send
    where
        S: Stream<Item = T> + Send;
}
```

Implement this trait for types that transform data. The `proc` method takes a stream of input items and returns a future that resolves to a stream of output items.

### Sink

```rust
pub trait Sink<T> {
    fn sink<S>(&self, stream: S) -> impl std::future::Future<Output = Result<(), Error>> + Send
    where
        S: Stream<Item = T> + Send;
}
```

Implement this trait for types that consume data. The `sink` method takes a stream of items and returns a future that resolves to a result indicating success or failure.

## Common Patterns

### Parallel Processing

You can easily parallelize processing using the `buffer_unordered` and `for_each_concurrent` methods from the `futures` crate:

```rust
impl Transformer<Input, Output> for MyTransformer {
    fn proc<S>(&self, stream: S) -> impl std::future::Future<Output = impl Stream<Item = Output>> + Send
    where
        S: Stream<Item = Input> + Send,
    {
        async move {
            stream
                .map(|item| async move {
                    // Process each item asynchronously
                    process_item(item).await
                })
                .buffer_unordered(10) // Process up to 10 items concurrently
        }
    }
}
```

### Error Handling

For robust error handling, you can use the `Result` type in your stream items:

```rust
impl Source<Result<MyData, MyError>> for MySource {
    fn into_stream(self) -> impl Stream<Item = Result<MyData, MyError>> {
        // Implementation that can return errors
    }
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.