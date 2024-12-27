# bedrock-rs

_Universal toolkit for Minecraft Bedrock Edition in Rust_

An easy-to-use universal library for Minecraft Bedrock written in Rust, that aims to provide:

- [X] Standards
- [X] Common implementations
- [X] An easy-to-use api

We also have a community discord, feel free to join it to learn more about our future and get help when
needed: https://discord.com/invite/VCVcrvt3JC

## Crates:

We have split up bedrock-rs into multiple crates to better manage its size, all crates are accessible via the normal
bedrock-rs crate.
We also provide a ton of different features which you can enable

- [Shared](https://github.com/bedrock-crustaceans/bedrockrs/tree/main/crates/shared):
    - [X] Common datatypes that can use derive macros defined in other crates and are used between the crates.

- [Proto](https://github.com/bedrock-crustaceans/bedrockrs/tree/main/crates/proto):
    - [X] Full implementation of the Bedrock protocol.
    - [X] Support for both Server and Client side intended.
    - [X] Multi protocol support

- [World](https://github.com/bedrock-crustaceans/bedrockrs/tree/main/crates/world):
    - [X] Level data structures.
    - [X] Implementation of the Bedrock level format using rusty leveldb.

- [Server](https://github.com/bedrock-crustaceans/bedrockrs/tree/main/crates/server):
    - [X] Extremly light weight server software foundation.
    - [X] Built-in multi protocol support.
    - [X] Asynchronously build via Tokio

- [Addon](https://github.com/bedrock-crustaceans/bedrockrs/tree/main/crates/addon):
    - [X] Datatypes defining the structure of Addons.
    - [X] Serialization and Deserialization of addons.
    - [X] Easy way to create addons programmatically.

- [Form](https://github.com/bedrock-crustaceans/bedrockrs/tree/main/crates/form):
    - [X] Implementation of the JSON form format used in Minecraft Bedrock.

## Contributing:

Feel free to join in at any time. Your contributions are highly valued, and a big thank you to all who participate. We
recommend you familiarize yourself with the bedrock-rs codebase. Whether it's tackling existing issues, adding new
features,
or even introducing entirely fresh modules, your creativity is always welcome!

And If you like bedrock-rs, remember to give it a Star
