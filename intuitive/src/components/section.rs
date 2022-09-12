use tui::{
  text::Spans as TuiSpans,
  widgets::{Block, Borders},
};

use crate::{
  components::{children::Children, Component},
  element::{Any as AnyElement, Element},
  event::{KeyEvent, KeyHandler},
  spans::Spans,
  style::Style,
  terminal::{Frame, Rect},
};

/// A component with a border and a title.
///
/// `Section` is used to wrap a component with a border and a title.
/// For example,
/// ```rust
/// use intuitive::{component, components::{Section, Text}, render};
///
/// #[component(Root)]
/// fn render() {
///   render! {
///     Section(title: "Input Box") {
///       Text(text: "Hi there!")
///     }
///   }
/// }
/// ```
/// Will render the following:
///
/// ![section](https://raw.githubusercontent.com/enricozb/intuitive/main/assets/section.png)
///
/// `Section` also optionally accepts a color, which will be applied to the title and border.
#[derive(Clone, Default)]
pub struct Section {
  pub title: Spans,
  pub border: Style,

  pub children: Children<1>,
  pub on_key: KeyHandler,
}

impl Component for Section {
  fn render(&self) -> AnyElement {
    AnyElement::new(Frozen {
      title: self.title.clone(),
      border: self.border,

      content: self.children[0].render(),
      on_key: self.on_key.clone(),
    })
  }
}

struct Frozen {
  title: Spans,
  border: Style,

  content: AnyElement,
  on_key: KeyHandler,
}

impl Element for Frozen {
  fn on_key(&self, event: KeyEvent) {
    self.on_key.handle_or(event, |event| self.content.on_key(event));
  }

  fn draw(&self, rect: Rect, frame: &mut Frame) {
    let block = Block::default()
      .title::<TuiSpans>((&self.title).into())
      .borders(Borders::ALL)
      .border_style(self.border.into());

    self.content.draw(block.inner(rect), frame);
    frame.render_widget(block, rect);
  }
}
