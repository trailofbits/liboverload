# liboverload

`liboverload` is a configurable Rust library that can be injected to processes
to replace them with other processes based on input from environment variables
or files.

## Features

* Support for `LD_PRELOAD` and `LD_AUDIT` injection: Hooks into a running
  process by dynamically loading itself as a shared library.
* Multiple configuration methods: Reads command-line arguments from the
  environment or configuration files to execute commands.

## Building

To build the project, simply run:

```sh
cargo build --release
```

This will compile the project into a dynamic library stored in
`target/release/liboverload.so`.

## Usage

When the library is compiled, it can be loaded via the `LD_PRELOAD` environment
variable. This will allow it to hook into the process and alter its behavior.
You can use it as follows:

```sh
export OVERLOAD_CMD="echo hello world"
export LD_PRELOAD=target/release/liboverload.so
./your_target_program
```

The library will print the banner and execute the command provided via the
environment or configuration files.

You may also use `LD_AUDIT` to load the library:

```sh
echo "echo hello world" > commands
export OVERLOAD_CMD_FILE=commands
export LD_AUDIT=target/release/liboverload.so
./your_target_program
```

## Logging

`liboverload` uses the `env_logger` crate to handle logging. By default, it logs
at the INFO level. You can customize the log level by setting the `OVERLOAD_LOG`
environment variable. For example, the following will enable debug-level logs
for the library:

```sh
export OVERLOAD_LOG=debug
```

## Stdio redirection

`liboverload` lets you redirect the file descriptors for stdin, stdout, and
stderr. By default, the existing file descriptors are reused, but you might
redirect them to a file or pipe by setting `OVERLOAD_STDIN`, `OVERLOAD_STDOUT`
or `OVERLOAD_STDERR` to a path.

```sh
export OVERLOAD_STDIN=/dev/null     # stdin will be read from /dev/null
export OVERLOAD_STDOUT=captured_out # stdout will be written to a file named `capture_out`
export OVERLOAD_STDERR=inherit      # stderr will be kept unchanged
```

## Testing

You can run tests for the `liboverload` functionality using Cargo:

```sh
cargo test
```
