# onepassword-cli

[![crate.io](https://img.shields.io/crates/v/onepassword-cli)](https://crates.io/crates/onepassword-cli)

A wrapper for [1password-cli](https://support.1password.com/command-line-getting-started/). It intent to offering a similar usage with the cli, make it easy-to-use.
For using this crate, you need to setup 1password-cli first.
Please see [1password-cli getting started](https://support.1password.com/command-line-getting-started/)
For now, only part of the cli utility have been implemented

- get
  - [x] account
  - [x] document
  - [x] item
  - [x] totp (one time password)
  - [ ] group
  - [x] user
  - [ ] vault
  - [ ] template
  - [ ] group
- list
  - [x] documents
  - [x] items
  - [ ] events
  - [ ] groups
  - [ ] templates
  - [x] users
  - [ ] vaults
- create
  - [x] document
  - [ ] group
  - [ ] item
  - [ ] user
  - [ ] vault
- add
  - [ ] group
  - [ ] user
- delete
  - [x] document
  - [ ] group
  - [x] item
  - [ ] trash
  - [ ] user
  - [ ] vault
- edit
  - [ ] document
  - [ ] group
  - [ ] item
  - [ ] user
  - [ ] vault
- encode
  - [ ] encode

# Installation

- Find on [crates.io](https://crates.io/crates/onepassword-cli)
- Use [cargo-edit](https://crates.io/crates/cargo-edit)

```sh
cargo add onepassword-cli
```

# How to use

- get account info

```rust
extern crate dotenv;
extern crate onepassword_cli;
use onepassword_cli::OpCLI;

dotenv::dotenv().unwrap();
let pass = dotenv::var("OP_PASS").unwrap();
let op_cli = OpCLI::new_with_pass("my", &pass)
    .await
    .unwrap();
let account = op_cli.get().account().run().await;
assert!(account.is_ok())
```

- get a login item include username password

```rust
extern crate dotenv;
extern crate onepassword_cli;
use onepassword_cli::OpCLI;

dotenv::dotenv().unwrap();
let pass = dotenv::var("OP_PASS").unwrap();
let op_cli = OpCLI::new_with_pass("my", &pass).await.unwrap();
let item_lite = op_cli.get().item_lite("facebook").run().await;
assert!(item_lite.is_ok());
println!("{:?}", &item_lite.unwrap().password);
```

- create a document

```rust
extern crate dotenv;
extern crate onepassword_cli;
use onepassword_cli::OpCLI;

dotenv::dotenv().unwrap();
let pass = dotenv::var("OP_PASS").unwrap();
let op_cli = OpCLI::new_with_pass("my", &pass)
    .await
    .unwrap();
let doc = op_cli.create().document("auth.log").run().await;
assert!(doc.is_ok())
```

- get one time password

```rust
extern crate dotenv;
extern crate onepassword_cli;
use onepassword_cli::OpCLI;

dotenv::dotenv().unwrap();
let pass = dotenv::var("OP_PASS").unwrap();
let op_cli = OpCLI::new_with_pass("my", &pass)
    .await
    .unwrap();
let otps = op_cli.get().totp("facebook").run().await;
assert!(otps.is_ok())
```
