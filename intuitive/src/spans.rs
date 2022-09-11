//! Structures for working with styled text.

use std::ops::Deref;

use tui::text::{Span as TuiSpan, Spans as TuiSpans};

use crate::style::Style;

/// Text with a specific style.
#[derive(Default, Clone)]
pub struct Span {
  text: String,
  style: Style,
}

impl Span {
  pub fn new<Str: Into<String>, Sty: Into<Style>>(text: Str, style: Sty) -> Self {
    Self {
      text: text.into(),
      style: style.into(),
    }
  }
}

impl<S: Into<String>> From<S> for Span {
  fn from(s: S) -> Self {
    Self {
      text: s.into(),
      style: Style::default(),
    }
  }
}

impl<'a> From<&'a Span> for TuiSpan<'a> {
  fn from(span: &'a Span) -> Self {
    Self {
      content: (&span.text).into(),
      style: span.style.into(),
    }
  }
}

impl From<Span> for TuiSpan<'_> {
  fn from(span: Span) -> Self {
    Self {
      content: span.text.into(),
      style: span.style.into(),
    }
  }
}

/// Text with a variety of styles.
#[derive(Default, Clone)]
pub struct Spans(Vec<Span>);

impl From<Span> for Spans {
  fn from(span: Span) -> Self {
    Spans(vec![span])
  }
}

impl<S: Into<String>> From<S> for Spans {
  fn from(s: S) -> Self {
    Spans(vec![s.into().into()])
  }
}

impl<'a> From<&'a Spans> for TuiSpans<'a> {
  fn from(spans: &'a Spans) -> Self {
    TuiSpans(spans.0.iter().map(TuiSpan::from).collect())
  }
}

impl From<Spans> for TuiSpans<'_> {
  fn from(spans: Spans) -> Self {
    TuiSpans(spans.0.into_iter().map(TuiSpan::from).collect())
  }
}

impl Deref for Spans {
  type Target = Vec<Span>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}