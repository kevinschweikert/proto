# proto

Render network packet field diagrams as ASCII art, Unicode art, or Mermaid.js.

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

**ASCII (32-bit width, the default):**

```bash
proto "SourcePort:16,DestPort:16,Length:16,Checksum:16,Data:*"
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
:                             Data                              :
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

**Unicode (32-bit width):**

```bash
proto --style unicode "SourcePort:16,DestPort:16,Length:16,Checksum:16,Data:*"
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

  udp    User Datagram Protocol
  tcp    Transmission Control Protocol
```

Use like this:

```bash
proto tcp
```
