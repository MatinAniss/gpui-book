## Create GPUI App

[Create GPUI App](https://github.com/zed-industries/create-gpui-app) is a official CLI tool that lets you quickly setup and get started with a GPUI application.

### Build and Install

```properties
cargo install create-gpui-app
```

### Usage

#### Monolithic Structure

```properties
create-gpui-app --name my-app
cd my-app
```

```properties
my-app
├── src
│   └── main.rs
├── Cargo.toml
└── README.md
```

#### Workspace Structure

```properties
create-gpui-app --workspace --name my-app
cd my-app
```

```properties
my-app
├── Cargo.toml
├── crates
│   └── my-app
│       ├── Cargo.toml
│       └── src
│           └── main.rs
└── README.md
```
