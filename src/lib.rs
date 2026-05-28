pub mod packet;
pub mod render;

pub use packet::{Field, Packet};
pub use render::Render;
pub use render::ascii::{Style, Terminal};

pub fn render_terminal(packet: &Packet, width: usize, style: Option<Style>) -> String {
    let style = match style {
        Some(style) => style,
        None => Style::ascii(),
    };
    Terminal::new(width).with_style(style).render(packet)
}
