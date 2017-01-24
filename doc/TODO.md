# TODO list

## Logging

- `log4rs` crate.

## RPC libraries

Going to be using msgpack-rpc, which is similar to JSON-RPC, but using
msgpack for the encoding.

I also aim to make the backend for that modular, so users can use
msgpack-rpc or just plain JSON-RPC instead.

## Data structures

### Ropes

A similar project to mine, xi-editor, has developed it's own library
called 'rope', which stores large strings (ideal for large buffers!),
in a B-Tree data structure.

### Gapbuffer

Iota uses the Gapbuffer data storage model.
