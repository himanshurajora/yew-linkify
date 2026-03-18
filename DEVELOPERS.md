# Developer Guide for yew-linkify

## Running Tests

Run all tests:
```sh
cargo test
```

## Example App

To run the example app for manual testing:
1. Install [Trunk](https://trunkrs.dev/):
   ```sh
   cargo install trunk
   ```
2. Run the example:
   ```sh
   trunk serve --example basic
   ```
3. Open the provided local URL in your browser.

## Continuous Integration & Publishing

- All pull requests and pushes to `main` are automatically built and tested via GitHub Actions.
- Publishing to [crates.io](https://crates.io/crates/yew-linkify) is automated: any push to `main` that passes CI will trigger a publish (requires `CARGO_REGISTRY_TOKEN` secret).

## Contributing

Feel free to open issues or submit pull requests to improve the library!
