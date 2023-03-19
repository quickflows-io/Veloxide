# Components of Veloxide

Veloxide is comprised of the following components, the higher level of which we will cover in more detail in the following sections.

| Component                                                   | Crate(s) of Significance                                                                                                                                                                                                                                                      | Notes                                                                                                                         |
| --------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Web Server                                                | [Axum](https://docs.rs/axum/latest/axum/), [Tower](https://docs.rs/tower/0.4.13/tower/)                                                                                                                                                                                                                                     | The endpoint path and timestamp metadata for each issued command are captured and stored in the database in the events table. |
| GraphQL | [async-graphql](https://docs.rs/async-graphql/latest/async_graphql/), [async-graphql-axum](https://docs.rs/async-graphql-axum/5.0.6/async_graphql_axum/)  |
| OpenAPI Doc Generation                                    | [Utopia](https://docs.rs/utoipa/latest/utoipa/)                                                                                                                                                                                                                               | Serves interactive documentation at `/swagger-ui`                                                                             |
| Async Runtime                                             | [Tokio](https://docs.rs/tokio/latest/tokio/index.html)                                                                                                                                                                                                                        |                                                                                                                               |
| Tracing                                                   | [Tracing](https://docs.rs/tracing/latest/tracing/) & [Tracing OpenTelemetry](https://docs.rs/tracing-opentelemetry/latest/) & [OpenTelemetry-Jaeger](https://docs.rs/opentelemetry-jaeger/latest/) & [Tracing Log](https://docs.rs/tracing-log/latest/tracing_log/index.html) & [tracing-bunyan-formatter](https://docs.rs/tracing-bunyan-formatter/0.3.6/tracing_bunyan_formatter/) | Use the [`#[instrument]`](https://docs.rs/tracing/latest/tracing/attr.instrument.html) macro to automatically generate new spans whenever a function is called! Also, all logs are automatically embedded in Trace spans by default!                                                                               |
| Metrics                                                   | [Axum Prometheus](https://docs.rs/axum-prometheus/latest/axum_prometheus/)                                                                                                                                                                                                    | Metrics are pre-configured for collection at /metrics                                                                         |
| Serializing & Deserializing                               | [Serde](https://docs.rs/serde/latest/serde/index.html) ([yaml](https://docs.rs/serde_yaml/latest/serde_yaml/) & [json](https://docs.rs/serde_json/latest/serde_json/))                                                                                                        |                                                                                                                               |
| Command Query Responsibility Segregation & Event Sourcing | [cqrs-es](https://docs.rs/cqrs-es/latest/cqrs_es/)                                                                                                                                                                                                                            |                                                                                                                               |
| Async Database Driver (SQL)                             | [SQLx](https://docs.rs/sqlx/latest/sqlx/)                                                                                                                                                                                                                                     | SQL queries are checked against the database for validity _at compile time_                                                   |
| Database agnostic, declarative database schema and Object Relational Mapping                             | [prisma-rust-client](https://github.com/Brendonovich/prisma-client-rust)                                                                                                                                                                                                                                     |                                                    |
| Mocking                                                   | [mockall](https://docs.rs/mockall/latest/mockall/)                                                                                                                                                                                                                            | Leverage the power of Rust's macro system by using [`#[automock]`](https://docs.rs/mockall/latest/mockall/attr.automock.html) to automatically create mocks!                                                                                                                               |
| Error Handling                                            | [thiserror](https://docs.rs/thiserror/latest/thiserror/)                                                                                                                                                                                                                      |                                                                                                                               |
| Behavior Driven Development / Cucumber Testing            | [Cucumber](https://docs.rs/cucumber/latest/cucumber/)                                                                                                                                                                                                                         |                                                                                                                               |
| Loading env variables & .env file                         | [Dotenvy](https://docs.rs/dotenvy/latest/dotenvy/)                                                                                                                                                                                                                            |                                                                                                                               |
| Improved assertion difference identification              | [Pretty Assertions](https://docs.rs/pretty_assertions/latest/pretty_assertions/)                                                                                                                                                                                              | Highlights the difference in tests character by character                                                                     |
| Supercharged derive attributes                            | [Derivative](https://mcarton.github.io/rust-derivative/latest/index.html)                                                                                                                                                                                                     |
| Code coverage generation | [cargo-llmvm-cov](https://github.com/taiki-e/cargo-llvm-cov) | |
| Automatic Typescript Binding Generation | [ts-rs](https://docs.rs/ts-rs/latest/ts_rs) | Automatically generates TypeScript interfaces for your Rust view models! |
| Authentication using OpenID Connect | [openidconnect-rs](https://docs.rs/openidconnect/latest/openidconnect/) | Log in with Google (other providers to come) |