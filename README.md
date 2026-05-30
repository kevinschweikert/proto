# proto

[![CI](https://github.com/kevinschweikert/proto/actions/workflows/ci.yml/badge.svg)](https://github.com/kevinschweikert/proto/actions/workflows/ci.yml)

Render network packet field diagrams as ASCII art, Unicode art, or Mermaid.js.

<!-- BEGIN CLI: udp --unicode --clean -->
<!-- AUTO-GENERATED: run `cargo -p xtask` -->

```bash
proto udp --unicode --clean
```

Output:

```
0                   1                   2                   3  
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
╭───────────────────────────────┬───────────────────────────────╮
│          Source Port          │       Destination Port        │
├───────────────────────────────┼───────────────────────────────┤
│            Length             │           Checksum            │
├───────────────────────────────┴───────────────────────────────┤
┊                             Data                              ┊
╰───────────────────────────────────────────────────────────────╯
```

<!-- END CLI -->

## Installation

```bash
cargo install --git https://github.com/kevinschweikert/proto.git
```

## Synopsis

<!-- BEGIN CLI: --help -->
<!-- AUTO-GENERATED: run `cargo -p xtask` -->

```bash
proto --help
```

Output:

```
Usage: proto [OPTIONS] [DEFINITION]

Arguments:
  [DEFINITION]  Protocol name or inline definition

Options:
  -u, --unicode                      Output in unicode style
  -m, --mermaid                      Output a mermaid.js spec
  -c, --clean                        remove crosses for each bit in a field for cleaner rendering
  -b, --bits-per-row <BITS_PER_ROW>  number of bits per row [default: 32]
  -n, --no-ruler                     omit the bit number header
  -l, --list                         list available protocols
  -f, --from-kaitai <FROM_KAITAI>    path to a kaitai spec file
  -h, --help                         Print help
```

<!-- END CLI -->

## Packet Definitions

A packet definition is a comma-separated list of `label:bits` pairs:

```
"SourcePort:16,DestPort:16,Length:16,Checksum:16,Data:*"
```

## Examples

**ASCII:**

<!-- BEGIN CLI: "SourcePort:16,DestPort:16,Length:16,Checksum:16,Chunk:64,Data:*" -->
<!-- AUTO-GENERATED: run `cargo -p xtask` -->

```bash
proto "SourcePort:16,DestPort:16,Length:16,Checksum:16,Chunk:64,Data:*"
```

Output:

```
0                   1                   2                   3  
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|          SourcePort           |           DestPort            |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|            Length             |           Checksum            |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                             Chunk                             |
+                                                               +
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
:                             Data                              :
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

<!-- END CLI -->

**Unicode:**

<!-- BEGIN CLI: --unicode "SourcePort:16,DestPort:16,Length:16,Checksum:16,Chunk:64,Data:*" -->
<!-- AUTO-GENERATED: run `cargo -p xtask` -->

```bash
proto --unicode "SourcePort:16,DestPort:16,Length:16,Checksum:16,Chunk:64,Data:*"
```

Output:

```
0                   1                   2                   3  
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
╭─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─╮
│          SourcePort           │           DestPort            │
├─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┤
│            Length             │           Checksum            │
├─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┤
│                             Chunk                             │
├                                                               ┤
│                                                               │
├─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┤
┊                             Data                              ┊
╰─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─╯
```

<!-- END CLI -->

**Mermaid.js:**

<!-- BEGIN CLI: --mermaid "SourcePort:16,DestPort:16,Length:16,Checksum:16,Data:*" -->
<!-- AUTO-GENERATED: run `cargo -p xtask` -->

```bash
proto --mermaid "SourcePort:16,DestPort:16,Length:16,Checksum:16,Data:*"
```

Output:

```
packet
+16: "SourcePort"
+16: "DestPort"
+16: "Length"
+16: "Checksum"
+32: "Data"
```

<!-- END CLI -->

**Built-in shortcuts:**

<!-- BEGIN CLI: --list -->
<!-- AUTO-GENERATED: run `cargo -p xtask` -->

```bash
proto --list
```

Output:

```
Available Protocols:

Layer 4 (Network):
  ip    Internet Protocol Version 4

Layer 4 (Transport):
  udp    User Datagram Protocol
  tcp    Transmission Control Protocol
```

<!-- END CLI -->

Use like this:

```bash
proto tcp
```

## Kaitai

> [!NOTE]
> Only a minimal subset of the spec will be parsed and supported

A [Kaitai Struct](https://kaitai.io/) specification can be loaded and visualized via:

```bash
proto --from-kaitai [PATH TO FILE]
```
