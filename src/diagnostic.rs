use std::fmt::Display;

use super::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Note,
    Help,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Diagnostic {
    level: DiagnosticLevel,
    message: String,
    spans: Vec<Span>,
    children: Vec<Box<Diagnostic>>,
}

impl Diagnostic {
    pub fn set_level(&mut self, level: DiagnosticLevel) {
        self.level = level;
    }

    pub fn set_message(&mut self, message: impl Display) {
        self.message = message.to_string()
    }

    pub fn set_spans(&mut self, spans: impl MultiSpan) {
        self.spans = spans.into_spans();
    }
}
