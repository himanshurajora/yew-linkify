### yew-linkify: A Yew component for automatic linkifying of your text content

yew-linkify = "0.0.1"
![Build Status](https://github.com/himanshurajora/yew-linkify/actions/workflows/ci.yml/badge.svg?branch=main)
![Crates.io](https://img.shields.io/crates/v/yew-linkify)

---

**A Yew component for automatically converting URLs and emails in your text into clickable links.**

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
yew-linkify = "0.0.2"
```

## Usage

Import and use the component in your Yew app:

```rust
use yew_linkify::Linkify;

html! {
          <Linkify text={"hi https://google.com".to_string()} />
}
```

## What to Expect
- All URLs and email addresses in the provided text will be automatically converted to clickable links.
- Supports both HTTP(S) links and email addresses.

---

For development, testing, and contribution instructions, see [DEVELOPERS.md](DEVELOPERS.md).

---

Made by Vedik Dev : Himanshu Jangid
