# proto

Render network packet field diagrams as ASCII art, Unicode art, or Mermaid.js.

## Synopsis

```
$ proto --help
Usage: proto [OPTIONS] <DEFINITION>

Arguments:
  <DEFINITION>  

Options:
  -a, --ascii    
  -u, --unicode  
  -m, --mermaid  
  -b, --b <B>    [default: 32]
  -h, --help     Print help
```

## Packet Definitions

A packet definition is a comma-separated list of `label:bits` pairs:

```
"SourcePort:16,DestPort:16,Length:16,Checksum:16,Data:32"
```

## Examples

**ASCII (32-bit width, the default):**

```bash
proto "SourcePort:16,DestPort:16,Length:16,Checksum:16,Data:32"
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
|                             Data                              |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

**Unicode (32-bit width):**

```bash
proto --unicode "SourcePort:16,DestPort:16,Length:16,Checksum:16,Data:32"
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
│                             Data                              │
╰───────────────────────────────────────────────────────────────╯
```

**Mermaid.js:**

```bash
proto --mermaid "SourcePort:16,DestPort:16,Length:16,Checksum:16,Data:32"
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

**Built-in shortcut — `udp`:**

```bash
proto udp
```

**Custom width (16 bits):**

```bash
proto -b 16 "SourcePort:16,DestPort:16,Length:16,Checksum:16,Data:32"
```
