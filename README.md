<!-- [![Build](https://github.com/mstephenp/post-rs/actions/workflows/build_status.yml/badge.svg)](https://github.com/mstephenp/post-rs/actions/workflows/build_status.yml) -->

# Sample Message Board Application
This server uses [axum](https://github.com/tokio-rs/axum/) to create a REST API for managing posts in a message board-like application.

If used inside VS Code, the [rest client](https://github.com/Huachao/vscode-restclient) extension can be used with calls listed in dev.http. This makes it easy to work with the API functionality.

There is also a client application (post-client) built with [yew](https://yew.rs/).

To run the server, from the top level run `cargo run -p post-server`

To run the client, from the post-client package, run `trunk serve` (requires [trunk](https://trunkrs.dev/) to be [setup](https://trunkrs.dev/#install) already).