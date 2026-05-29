use std::collections::BTreeSet;

use super::Render;
use crate::packet::{Field, Packet};

pub enum Junctions {
    All,
    Boundaries,
}

pub struct Style {
    pub separator_fixed: &'static str,
    pub separator_variable: &'static str,
    pub horizontal: &'static str,

    pub top_left: &'static str,
    pub top_mid: &'static str,
    pub top_right: &'static str,

    pub mid_left: &'static str,
    pub mid_mid: &'static str,
    pub mid_right: &'static str,

    pub bot_left: &'static str,
    pub bot_mid: &'static str,
    pub bot_right: &'static str,

    pub junctions: Junctions,
}

impl Style {
    pub fn ascii() -> Self {
        Style {
            separator_fixed: "|",
            separator_variable: ":",
            horizontal: "-",
            top_left: "+",
            top_mid: "+",
            top_right: "+",
            mid_left: "+",
            mid_mid: "+",
            mid_right: "+",
            bot_left: "+",
            bot_mid: "+",
            bot_right: "+",
            junctions: Junctions::All,
        }
    }

    pub fn unicode() -> Self {
        Style {
            separator_fixed: "│",
            separator_variable: "┊",
            horizontal: "─",
            top_left: "╭",
            top_mid: "┬",
            top_right: "╮",
            mid_left: "├",
            mid_mid: "┼",
            mid_right: "┤",
            bot_left: "╰",
            bot_mid: "┴",
            bot_right: "╯",
            junctions: Junctions::Boundaries,
        }
    }
}

pub struct Terminal {
    cells: usize,
    style: Style,
    ruler: bool,
}

#[derive(Clone)]
struct Row {
    segments: Vec<Segment>,
    boundaries: BTreeSet<usize>,
}

impl Row {
    fn new() -> Row {
        Row {
            segments: vec![],
            boundaries: BTreeSet::new(),
        }
    }
}

impl Row {
    fn width(&self) -> usize {
        self.segments.iter().map(|s| s.width).sum()
    }
}

#[derive(Clone)]
struct Segment {
    label: String,
    width: usize,
    show_label: bool,
    open: bool,
}

const WHITESPACE: &str = " ";
impl Terminal {
    pub fn new(width: usize) -> Self {
        Terminal {
            cells: width,
            style: Style::ascii(),
            ruler: true,
        }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn set_ruler(mut self, show: bool) -> Self {
        self.ruler = show;
        self
    }

    fn layout_segments(&self, fields: Vec<Field>) -> Vec<Row> {
        let mut rows: Vec<Row> = vec![Row::new()];
        let mut current_row_length = 0;
        for field in fields {
            let mut remaining = match field {
                Field::Fixed { bits, .. } => bits,
                Field::Variable { .. } => self.cells - current_row_length,
            };
            let mut group: Vec<(usize, usize)> = vec![];
            while remaining > 0 {
                let capacity = self.cells.saturating_sub(current_row_length);
                let take = remaining.min(capacity);
                let start = current_row_length;
                let end = current_row_length + take;

                let row_idx = rows.len().saturating_sub(1);
                let row = rows.last_mut().expect("always one row");

                let segment = Segment {
                    label: field.label().to_string(),
                    width: take,
                    show_label: false,
                    open: match field {
                        Field::Fixed { .. } => false,
                        Field::Variable { .. } => true,
                    },
                };

                row.segments.push(segment);
                let seg_idx = row.segments.len().saturating_sub(1);

                row.boundaries.insert(start);
                row.boundaries.insert(end);

                group.push((row_idx, seg_idx));

                current_row_length += take;
                remaining -= take;

                if current_row_length == self.cells {
                    rows.push(Row::new());
                    current_row_length = 0;
                }
            }

            if let Some((best_row_idx, best_seg_idx)) =
                group.iter().fold(None, |best, (r_idx, s_idx)| match best {
                    None => Some((r_idx, s_idx)),
                    Some((best_r_idx, best_s_idx)) => {
                        let best_width = rows[*best_r_idx].segments[*best_s_idx].width;
                        let current_width = rows[*r_idx].segments[*s_idx].width;
                        if current_width > best_width {
                            Some((r_idx, s_idx))
                        } else {
                            Some((best_r_idx, best_s_idx))
                        }
                    }
                })
            {
                rows[*best_row_idx].segments[*best_seg_idx].show_label = true;
            };
        }
        if rows.last().is_some_and(|row| row.segments.is_empty()) {
            rows.pop();
        }
        rows
    }

    fn render_ruler(&self) -> String {
        let mut ruler = vec![];
        if self.cells >= 10 {
            ruler.push(self.render_dline());
        }
        ruler.push(self.render_bline());
        ruler.join("\n")
    }

    fn render_dline(&self) -> String {
        (0..self.cells)
            .map(|i| {
                if i % 10 == 0 {
                    format!(" {}", i / 10)
                } else {
                    format!("{}{}", WHITESPACE, WHITESPACE)
                }
            })
            .collect()
    }

    fn render_bline(&self) -> String {
        (0..self.cells).map(|i| format!(" {}", i % 10)).collect()
    }

    fn render_hline(&self, above: Option<&Row>, below: Option<&Row>) -> String {
        let (left, right, width) = match (above, below) {
            (None, None) => unreachable!(),
            // top row
            (None, Some(r)) => (self.style.top_left, self.style.top_right, r.width()),
            // last row
            (Some(r), None) => (self.style.bot_left, self.style.bot_right, r.width()),
            // middle row
            (Some(a), Some(b)) => (
                self.style.mid_left,
                if b.width() < self.cells {
                    self.style.bot_right
                } else {
                    self.style.mid_right
                },
                a.width(),
            ),
        };

        let junction_char = |i| match self.style.junctions {
            Junctions::All => self.style.mid_mid,
            Junctions::Boundaries => match (above, below) {
                (None, None) => unreachable!(),
                (None, Some(r)) => {
                    if r.boundaries.contains(&i) {
                        self.style.top_mid
                    } else {
                        self.style.horizontal
                    }
                }
                (Some(r), None) => {
                    if r.boundaries.contains(&i) {
                        self.style.bot_mid
                    } else {
                        self.style.horizontal
                    }
                }
                (Some(a), Some(b)) => {
                    let in_a = a.boundaries.contains(&i);
                    let in_b = b.boundaries.contains(&i);
                    match (in_a, in_b) {
                        (true, true) => self.style.mid_mid,
                        (true, false) => self.style.bot_mid,
                        (false, true) => self.style.top_mid,
                        (false, false) => self.style.horizontal,
                    }
                }
            },
        };

        let mut line = String::with_capacity(self.cells);
        line.push_str(left);

        for i in 1..=width {
            line.push_str(self.style.horizontal);

            if i == width {
                line.push_str(right)
            } else {
                line.push_str(junction_char(i))
            }
        }

        line
    }

    fn render_row(&self, row: &Row) -> String {
        let start_char = match row.segments.first() {
            Some(Segment { open: true, .. }) => self.style.separator_variable,
            Some(Segment { open: false, .. }) => self.style.separator_fixed,
            None => self.style.separator_fixed,
        };

        let inner = row
            .segments
            .iter()
            .map(|seg| self.render_segment(seg))
            .collect::<String>();

        format!("{}{}", start_char, inner)
    }

    fn render_segment(&self, segment: &Segment) -> String {
        let width = segment.width * 2 - 1;

        let label = if segment.show_label {
            &segment.label[..segment.label.len().min(width)]
        } else {
            WHITESPACE
        };
        let padding = width.saturating_sub(label.len());
        let pl = padding / 2;
        let pr = padding - pl;
        format!(
            "{}{}{}{}",
            WHITESPACE.repeat(pl),
            label,
            WHITESPACE.repeat(pr),
            if segment.open {
                self.style.separator_variable
            } else {
                self.style.separator_fixed
            }
        )
    }
}

impl Render<String> for Terminal {
    fn render(&self, packet: &Packet) -> String {
        let rows = self.layout_segments(packet.fields.clone());
        let mut output: Vec<String> = vec![];

        if self.ruler {
            output.push(self.render_ruler());
        }

        output.push(self.render_hline(None, Some(&rows[0])));

        for (above, below) in rows.iter().zip(rows.iter().skip(1)) {
            output.push(self.render_row(above));
            output.push(self.render_hline(Some(above), Some(below)));
        }

        let last = rows.last().expect("at least one row");
        output.push(self.render_row(last));
        output.push(self.render_hline(Some(last), None));
        output.join("\n")
    }
}
