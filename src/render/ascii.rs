use std::collections::BTreeSet;

use super::Render;
use crate::packet::{Field, Packet};

pub enum Junctions {
    All,
    Boundaries,
}

pub struct Style {
    pub separator: &'static str,
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
            separator: "|",
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
            separator: "│",
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

#[derive(Clone)]
struct Segment {
    label: String,
    width: usize,
    show_label: bool,
}

const WHITESPACE: &str = " ";
impl Terminal {
    pub fn new(width: usize) -> Self {
        Terminal {
            cells: width,
            style: Style::ascii(),
        }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    fn layout_segments(&self, fields: Vec<Field>) -> Vec<Row> {
        let mut rows: Vec<Row> = vec![Row::new()];
        let mut current_row_length = 0;
        for field in fields {
            let mut remaining = field.bits;
            let mut group: Vec<(usize, usize)> = vec![];
            while remaining > 0 {
                let capacity = self.cells.saturating_sub(current_row_length);
                let take = remaining.min(capacity);
                let start = current_row_length;
                let end = current_row_length + take;

                let row_idx = rows.len().saturating_sub(1);
                let row = rows.last_mut().expect("always one row");

                let segment = Segment {
                    label: field.label.clone(),
                    width: take,
                    show_label: false,
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
        let (left, right) = match (above, below) {
            (None, None) => unreachable!(),
            // top row
            (None, Some(_)) => (self.style.top_left, self.style.top_right),
            // last row
            (Some(_), None) => (self.style.bot_left, self.style.bot_right),
            // middle row
            (Some(_), Some(_)) => (self.style.mid_left, self.style.mid_right),
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

        for i in 1..=self.cells {
            line.push_str(self.style.horizontal);

            if i == self.cells {
                line.push_str(right)
            } else {
                line.push_str(junction_char(i))
            }
        }

        line
    }

    fn render_row(&self, row: &Row) -> String {
        let inner = row
            .segments
            .iter()
            .map(|seg| self.render_segment(seg))
            .collect::<Vec<_>>()
            .join(self.style.separator);
        format!("{}{}{}", self.style.separator, inner, self.style.separator)
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
            "{}{}{}",
            WHITESPACE.repeat(pl),
            label,
            WHITESPACE.repeat(pr),
        )
    }
}

impl Render<String> for Terminal {
    fn render(&self, packet: &Packet) -> String {
        let rows = self.layout_segments(packet.fields.clone());
        let mut output: Vec<String> = vec![];
        if self.cells >= 10 {
            output.push(self.render_dline());
        }
        output.push(self.render_bline());
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
