use tui::{
  style::{Color, Style},
  widgets::{Block, Borders},
};

use crate::{
  components::{Any as AnyComponent, Component},
  event::KeyEvent,
  terminal::{Frame, Rect},
};

#[derive(Clone)]
pub struct Section {
  pub title: String,
  pub color: Color,
  pub children: [AnyComponent; 1],
}

impl Component for Section {
  fn on_key(&self, event: KeyEvent) {
    self.children[0].on_key(event)
  }

  fn render(&self) -> AnyComponent {
    Clone::clone(self).into()
  }

  fn draw(&self, rect: Rect, frame: &mut Frame) {
    let block = Block::default()
      .title(self.title.as_ref())
      .borders(Borders::ALL)
      .border_style(Style::default().fg(self.color));

    self.children[0].draw(block.inner(rect), frame);
    frame.render_widget(block, rect);
  }
}

impl Default for Section {
  fn default() -> Self {
    Self {
      title: String::default(),
      color: Color::White,
      children: [Default::default()],
    }
  }
}