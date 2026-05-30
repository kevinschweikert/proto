# proto

Render network packet field diagrams as ASCII art, Unicode art, or Mermaid.js.

```
> proto udp --style unicode
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

## Installation

```bash
$ cargo install --git https://github.com/kevinschweikert/proto.git
```

## Synopsis

```
$ proto --help
Usage: proto [OPTIONS] <DEFINITION>

Arguments:
  <DEFINITION>
          Protocol name or inline definition

Options:
  -s, --style <STYLE>
          Output style

          Possible values:
          - ascii:   RFC Style ASCII characters
          - unicode: Modern unicode characters
          - mermaid: Mermaid spec
          
          [default: ascii]

  -b, --bits-per-row <B>
          number of bits per row
          
          [default: 32]

  -n, --no-ruler
          omit the bit number header

  -h, --help
          Print help (see a summary with '-h')
```

## Packet Definitions

A packet definition is a comma-separated list of `label:bits` pairs:

```
"SourcePort:16,DestPort:16,Length:16,Checksum:16,Data:*"
```

## Examples

**ASCII:**

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

**Unicode:**

```bash
proto --style unicode "SourcePort:16,DestPort:16,Length:16,Checksum:16,Chunk:64,Data:*"
```

Output:

```
 0                   1                   2                   3  
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
╭───────────────────────────────┬───────────────────────────────╮
│          SourcePort           │           DestPort            │
├───────────────────────────────┼───────────────────────────────┤
│            Length             │           Checksum            │
├───────────────────────────────┴───────────────────────────────┤
│                             Chunk                             │
├                                                               ┤
│                                                               │
├───────────────────────────────────────────────────────────────┤
┊                             Data                              ┊
╰───────────────────────────────────────────────────────────────╯
```

**Mermaid.js:**

```bash
proto --style mermaid "SourcePort:16,DestPort:16,Length:16,Checksum:16,Data:*"
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

**Built-in shortcuts:**

```bash
$ proto --list
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

Use like this:

```bash
proto tcp
```
