# Rust Snippets

This repository contains multiple small Rust projects. Each directory within the repository represents a separate project.

## Prerequisites

Before you can run the projects, make sure you have the following prerequisites installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Running a Project

To run a project, navigate to the project's directory and use the `cargo run` command:

```sh
cd path/to/project_directory
cargo run
```

## Environment Variables

Each project directory includes a `.env.example` file with sample environment variables. Before running a project, ensure that you create a `.env` file in the same directory and set the environment variables according to the variables listed in the `.env.example` file.

```sh
# Create a .env file based on the .env.example file
cp .env.example .env
# Edit the .env file to set the appropriate values
nano .env
```

## Sample Scripts

Here are a few sample scripts you can use to get started:

### Navigating to a Project Directory

```sh
cd examples/project1
```

### Running a Project

```sh
cargo run
```

### Setting Up Environment Variables

```sh
cp .env.example .env
nano .env
```

## Contributing

If you'd like to contribute to this repository, feel free to submit a pull request. 

## License

This repository is licensed under the [MIT License](LICENSE).

Happy coding!