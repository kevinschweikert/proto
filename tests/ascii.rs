use proto::{Field, Packet, Render, Style, Terminal};

fn render_ascii(fields: Vec<Field>) -> String {
    let packet = Packet { fields: fields };
    let renderer = Terminal::new(8);
    renderer.render(&packet)
}

fn render_unicode(fields: Vec<Field>) -> String {
    let packet = Packet { fields: fields };
    let renderer = Terminal::new(8).with_style(Style::unicode());
    renderer.render(&packet)
}

fn f(bits: usize, label: &str) -> Field {
    Field {
        bits: bits,
        label: label.to_string(),
    }
}

#[test]
fn udp() {
    let packet = Packet {
        fields: vec![
            f(16, "Source Port"),
            f(16, "Destination Port"),
            f(16, "Length"),
            f(16, "Checksum"),
            f(32, "Data"),
        ],
    };

    let renderer = Terminal::new(32);
    insta::assert_snapshot!(renderer.render(&packet));

    let renderer = Terminal::new(32).with_style(Style::unicode());
    insta::assert_snapshot!(renderer.render(&packet))
}

#[test]
fn test_single_field() {
    insta::assert_snapshot!( { render_ascii( vec![f(8,"A")]) }
        , @"
     0 1 2 3 4 5 6 7
    +-+-+-+-+-+-+-+-+
    |       A       |
    +-+-+-+-+-+-+-+-+
    ")
}

#[test]
fn test_single_field_unicode() {
    insta::assert_snapshot!( { render_unicode( vec![f(8,"A")]) }
        , @"
     0 1 2 3 4 5 6 7
    ╭───────────────╮
    │       A       │
    ╰───────────────╯
    ")
}

#[test]
fn test_multi_field() {
    insta::assert_snapshot!( { render_ascii( vec![f(4,"A"), f(4, "B")]) }
        , @"
     0 1 2 3 4 5 6 7
    +-+-+-+-+-+-+-+-+
    |   A   |   B   |
    +-+-+-+-+-+-+-+-+
    ")
}

#[test]
fn test_multi_field_unicode() {
    insta::assert_snapshot!( { render_unicode( vec![f(4,"A"), f(4, "B")]) }
        , @"
     0 1 2 3 4 5 6 7
    ╭───────┬───────╮
    │   A   │   B   │
    ╰───────┴───────╯
    ")
}

#[test]
fn test_two_rows() {
    insta::assert_snapshot!( { render_ascii( vec![f(4,"A"), f(4, "B"), f(8, "C")]) }
        , @"
     0 1 2 3 4 5 6 7
    +-+-+-+-+-+-+-+-+
    |   A   |   B   |
    +-+-+-+-+-+-+-+-+
    |       C       |
    +-+-+-+-+-+-+-+-+
    ")
}

#[test]
fn test_two_rows_unicode() {
    insta::assert_snapshot!(  render_unicode( vec![f(4,"A"), f(4, "B"), f(8, "C")]) 
        , @"
     0 1 2 3 4 5 6 7
    ╭───────┬───────╮
    │   A   │   B   │
    ├───────┴───────┤
    │       C       │
    ╰───────────────╯
    ")
}

#[test]
fn split_into_two_rows() {
    insta::assert_snapshot!(render_ascii(vec![f(16, "LONG")]), @"
     0 1 2 3 4 5 6 7
    +-+-+-+-+-+-+-+-+
    |     LONG      |
    +-+-+-+-+-+-+-+-+
    |               |
    +-+-+-+-+-+-+-+-+
    ")
}

#[test]
fn put_label_into_longest_segment() {
    insta::assert_snapshot!(render_ascii(vec![f(7, "PRE"),f(10, "LONG"), f(7, "POST")]), @"
     0 1 2 3 4 5 6 7
    +-+-+-+-+-+-+-+-+
    |     PRE     | |
    +-+-+-+-+-+-+-+-+
    |     LONG      |
    +-+-+-+-+-+-+-+-+
    | |    POST     |
    +-+-+-+-+-+-+-+-+
    ")
}

#[test]
fn put_label_into_longest_segment_unicode() {
    insta::assert_snapshot!(render_unicode(vec![f(7, "PRE"),f(10, "LONG"), f(7, "POST")]), @"
     0 1 2 3 4 5 6 7
    ╭─────────────┬─╮
    │     PRE     │ │
    ├─────────────┴─┤
    │     LONG      │
    ├─┬─────────────┤
    │ │    POST     │
    ╰─┴─────────────╯
    ")
}
