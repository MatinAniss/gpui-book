## Create GPUI App

[Create GPUI App](https://github.com/zed-industries/create-gpui-app) is a official CLI tool that lets you quickly setup and get started with a GPUI application.

### Build and Install

```
cargo install create-gpui-app
```

### Usage

#### Monolithic Structure

```
create-gpui-app --name my-app
cd my-app
```

```
my-app
├── src
│   └── main.rs
├── Cargo.toml
└── README.md
```

#### Workspace Structure

```
create-gpui-app --workspace --name my-app
cd my-app
```

```
my-app
├── Cargo.toml
├── crates
│   └── my-app
│       ├── Cargo.toml
│       └── src
│           └── main.rs
└── README.md
```
