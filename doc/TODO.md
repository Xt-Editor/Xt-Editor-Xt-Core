# TODO list

## Logging

Will be using `slog` for logging.

## RPC libraries

Will be using plain JSON for now.

`msgpack` support might come soon as an optional feature.

## Data structures

### Ropes

A similar project to mine, xi-editor, has developed it's own library
called 'rope', which stores large strings (ideal for large buffers!),
in a B-Tree data structure. NOTE: **not** documented well.

### Gapbuffer

Iota uses the Gapbuffer data storage model.
