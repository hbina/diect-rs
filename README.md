# diect-rs - Di(e)ctionary

A simple Redis-like dictionary containing values that will die (or expires).

# Usage

There are two types of storage supported.

## Transient

In this mode, the service will store the values in the RAM.
Therefore, it will be gone if the service restarts or whatever.

## Persistent

If you want something that have more guarantees that it will remain valid, you can use the persistent mode.
This mode will be slower, but it will be backed by a database.
Therefore, it is less prone to being gone.

## Watching Development

Use this command to watch,

`systemfd --no-pid -s http::5000 -- cargo watch -x run`

## TODO

1. Improve performance of queries by hashing the values instead of using raw `String`s.
