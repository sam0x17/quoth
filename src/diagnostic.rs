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
    span: Span,
    context_name: Option<String>,
    children: Vec<Diagnostic>,
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

    pub fn level(&self) -> DiagnosticLevel {
        self.level
    }

    pub fn message(&self) -> &str {
        &self.message
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

    pub fn merged_span(&self) -> Result<Span, SpanJoinError> {
        let mut merged_span = self.span.clone();
        for child in &self.children {
            merged_span = merged_span.join(&child.merged_span()?)?;
        }
        Ok(merged_span)
    }
}

impl Spanned for Diagnostic {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let level = self.level;
        let message = &self.message;
        write!(f, "{level}: {message}\n")?;
        let span = self.span();
        let LineCol { line, col } = span.line_col();
        let num_width = if line == 0 {
            1
        } else {
            (line as f64).log10() as usize + 1
        };
        for _ in 1..num_width {
            write!(f, " ")?;
        }
        write!(f, " --> ")?;
        match span.source_path() {
            Some(path) => write!(f, "{}", path.display())?,
            None => write!(f, "{}", self.context_name())?,
        }
        let real_line = line + 1;
        write!(f, ":{real_line}:{col}\n")?;
        for _ in 0..num_width {
            write!(f, " ")?;
        }
        write!(f, " |\n")?;
        let mut last_line_range = 0..0;
        for (i, (lin, range)) in span.source_lines().into_iter().enumerate() {
            let num = i + line + 1;
            write!(f, "{num} | {lin}\n")?;
            last_line_range = range;
        }
        for _ in 0..num_width {
            write!(f, " ")?;
        }
        write!(f, "   ")?;
        for _ in 0..last_line_range.start {
            write!(f, " ")?;
        }
        for _ in last_line_range {
            write!(f, "^")?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}

#[cfg(test)]
use std::rc::Rc;

#[test]
fn test_diagnostic_display() {
    let diag = Diagnostic {
        level: DiagnosticLevel::Error,
        message: "this is an error".to_string(),
        span: Span::new(Rc::new(Source::from_str("this is a triumph")), 5..7),
        context_name: Some("the thing".to_string()),
        children: Vec::new(),
    };
    println!("{}", diag.to_string());
    assert_eq!(diag.to_string(), include_str!("samples/diagnostic_01.txt"));
}
