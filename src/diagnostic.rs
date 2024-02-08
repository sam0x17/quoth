use std::fmt::Display;

use super::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Note,
    Help,
}

impl Display for DiagnosticLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiagnosticLevel::Error => write!(f, "error"),
            DiagnosticLevel::Warning => write!(f, "warning"),
            DiagnosticLevel::Note => write!(f, "note"),
            DiagnosticLevel::Help => write!(f, "help"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Diagnostic {
    level: DiagnosticLevel,
    message: String,
    spans: Vec<Span>,
    children: Vec<Diagnostic>,
    context_name: Option<String>,
}

impl Diagnostic {
    pub fn set_level(&mut self, level: DiagnosticLevel) {
        self.level = level;
    }

    pub fn set_message(&mut self, message: impl Display) {
        self.message = message.to_string()
    }

    pub fn set_context_name(&mut self, name: Option<impl AsRef<str>>) {
        self.context_name = name.map(|n| n.as_ref().to_string());
    }

    pub fn set_spans(&mut self, spans: impl MultiSpan) {
        self.spans = spans.into_spans();
        debug_assert!(self.spans.len() > 0);
    }

    pub fn level(&self) -> DiagnosticLevel {
        self.level
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn spans(&self) -> &Vec<Span> {
        &self.spans
    }

    pub fn context_name(&self) -> &str {
        match &self.context_name {
            Some(context_name) => context_name,
            None => "input",
        }
    }

    pub fn children(&self) -> &Vec<Diagnostic> {
        &self.children
    }
}

impl Spanned for Diagnostic {
    fn span(&self) -> Span {
        if self.spans.len() > 1 {
            self.spans
                .first()
                .unwrap()
                .join(self.spans.last().unwrap())
                .unwrap()
        } else {
            self.spans.first().unwrap().clone()
        }
    }
}

impl Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let level = self.level;
        let message = &self.message;
        write!(f, "{level}: {message}\n")?;
        let span = self.span();
        let source_text = span.source_text();
        let LineCol { line, col } = span.line_col();
        let num_width = if line == 0 {
            1
        } else {
            (line as f64).log10() as usize + 1
        };
        for _ in 1..num_width {
            write!(f, " ")?;
        }
        write!(f, "--> ")?;
        match span.source_path() {
            Some(path) => write!(f, "{}", path.display())?,
            None => write!(f, "{}", self.context_name())?,
        }
        write!(f, ":{line}:{col}\n")?;
        for _ in 0..num_width {
            write!(f, " ")?;
        }
        write!(f, " |\n")?;
        // TODO: handle multi-line spans
        write!(f, "{line} | {source_text}\n")?;
        for _ in 0..num_width {
            write!(f, " ")?;
        }
        write!(f, " | ")?;
        Ok(())
    }
}
