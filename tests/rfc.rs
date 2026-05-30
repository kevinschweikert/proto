use proto::{Field, Packet, Render, RfcDiagram, Style};
mod common;
use common::{f, v};

fn render_ascii(fields: Vec<Field>) -> String {
    let packet = Packet {
        title: None,
        fields,
    };
    let renderer = RfcDiagram::new(8);
    renderer.render(&packet)
}

fn render_unicode(fields: Vec<Field>) -> String {
    let packet = Packet {
        title: None,
        fields,
    };
    let renderer = RfcDiagram::new(8).with_style(Style::unicode());
    renderer.render(&packet)
}

#[test]
fn udp() {
    let packet = Packet {
        title: Some("UDP Packet".to_string()),
        fields: vec![
            f(16, "Source Port"),
            f(16, "Destination Port"),
            f(16, "Length"),
            f(16, "Checksum"),
            v("Data"),
        ],
    };

    let renderer = RfcDiagram::new(32);
    insta::assert_snapshot!(renderer.render(&packet));

    let renderer = RfcDiagram::new(32).with_style(Style::unicode());
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
fn ruler_flag_respected() {
    let packet = Packet {
        title: None,
        fields: vec![f(8, "A")],
    };
    let renderer = RfcDiagram::new(8).with_ruler(false);
    insta::assert_snapshot!(      renderer.render(&packet) , @"
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
fn lasts_row_only_prints_available_bits() {
    insta::assert_snapshot!(render_ascii(vec![f(14, "SHORTENED")]), @"
     0 1 2 3 4 5 6 7
    +-+-+-+-+-+-+-+-+
    |   SHORTENED   |
    +-+-+-+-+-+-+-+-+
    |           |
    +-+-+-+-+-+-+
    ")
}

#[test]
fn variable_width_takes_full_row() {
    insta::assert_snapshot!(render_ascii(vec![f(8, "HEADER"), v("DATA")]), @"
     0 1 2 3 4 5 6 7
    +-+-+-+-+-+-+-+-+
    |    HEADER     |
    +-+-+-+-+-+-+-+-+
    :     DATA      :
    +-+-+-+-+-+-+-+-+
    ")
}

#[test]
fn variable_width_takes_full_row_unicode() {
    insta::assert_snapshot!(render_unicode(vec![f(8, "HEADER"), v("DATA")]), @"
     0 1 2 3 4 5 6 7
    ╭───────────────╮
    │    HEADER     │
    ├───────────────┤
    ┊     DATA      ┊
    ╰───────────────╯
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

#[test]
fn lasts_row_only_prints_available_bits_unicode() {
    insta::assert_snapshot!(render_unicode(vec![f(14, "SHORTENED")]), @"
     0 1 2 3 4 5 6 7
    ╭───────────────╮
    │   SHORTENED   │
    ├───────────┬───╯
    │           │
    ╰───────────╯
    ")
}
