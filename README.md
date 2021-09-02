# rpublish

## What it is

Is a blog like system to write and publish articles  
Keep in mind that this is a personal project to learn rust but i'm going to use this in my personal page to post articles
___

## How to build and run

You need to install the rust toolchain [Getting started](https://www.rust-lang.org/learn/get-started)

If you have rust toolchain installed then just compile and run

To run in debug mode (Is slower but easier to debug)
```
cargo run
```

To run in release mode (To run at full speed)
```
cargo run --release
```

In the first start you will be asked for your admin credentials
Fill them and you are ready to go

#### Editor dashboard
```
http//localhost::1337/dashboard
```

___
## Licences
This project is under LGPLv3 but it uses third party components with independent licences

### Editor.js
This project uses Editor.js as the editor for articles

Editor is under Apache-2.0 License 

[editorjs.io](https://editorjs.io/)  
[Github Repository](https://github.com/codex-team/editor.js)

### Moment.js
This project uses Moment.js to display fancy time related things like "1 minute ago"  
  
Moment.js is under MIT License 

[editorjs.io](https://momentjs.com/)  
[Github Repository](https://github.com/moment/moment/)  
[License](https://github.com/moment/moment/blob/develop/LICENSE)

### Rust crates
For crates used in this project please read Cargo.toml and check the license and more details in [crates.io](https://crates.io/) or [Lib.rs](https://lib.rs/about)