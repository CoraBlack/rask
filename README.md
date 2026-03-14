# rask -- A vscode-like task json file runner in rust

It just my improv-coding work... But I will tell you how to use it

### Build it with cargo

```sh
cargo build
```

### Create a json file in a directory

```json5
{
    "name": "my task",
    "command": "echo",
    "args": [
        "Hello World"
    ]
}
```

### Run it with rask

```sh
rask ./your-task.json
```
