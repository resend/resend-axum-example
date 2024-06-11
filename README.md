# Resend with Axum

This example shows how to use Resend with [Axum](https://crates.io/crates/axum).

## Prerequisites

To get the most out of this guide, you’ll need to:

* [Create an API key](https://resend.com/api-keys)
* [Verify your domain](https://resend.com/domains)

## Instructions

1. Replace `re_123456789` on `src/main.rs` with your API key.

2. Run the server:

    ```sh
    cargo run
    ```

3. Access the URL http://localhost:3000/ via your browser or run

    ```sh
    curl http://localhost:3000/
    ```

## License

MIT License