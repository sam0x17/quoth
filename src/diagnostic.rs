use core::fmt::{self, Display};

use crate as quoth;

use super::*;

/// Represents the severity of a [`Diagnostic`].
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum DiagnosticLevel {
    /// Represents an error diagnostic.
    Error,
    /// Represents a warning diagnostic.
    Warning,
    /// Represents a note diagnostic.
    Note,
    /// Represents a help diagnostic.
    Help,
}

impl Display for DiagnosticLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiagnosticLevel::Error => write!(f, "error"),
            DiagnosticLevel::Warning => write!(f, "warning"),
            DiagnosticLevel::Note => write!(f, "note"),
            DiagnosticLevel::Help => write!(f, "help"),
        }
    }
}

/// Represents a diagnostic message that can be displayed to the user, typically indicating a
/// parsing error or highlighting some fact about a [`Span`] of input
///
/// A [`Diagnostic`] can have children, which are other [`Diagnostic`]s that are related to the
/// parent [`Diagnostic`]. For example, a [`Diagnostic`] about a missing semicolon might have a
/// child [`Diagnostic`] about a missing closing brace.
///
/// Note that the [`Display`] implementation for [`Diagnostic`] is designed to be
/// human-readable, and is how [`Diagnostic`]s are intended to be displayed to the user.
///
/// # Examples
///
/// ```
/// use quoth::*;
/// use std::rc::Rc;
///
/// let source = Rc::new(Source::from_str("Hello, world!"));
/// let span = Span::new(source.clone(), 0..5);
/// let diag = Diagnostic::new(
///     DiagnosticLevel::Error,
///     span,
///     "this is an error",
///     Option::<String>::None,
///     Vec::new(),
/// );
/// println!("{}", diag);
/// ```
#[derive(Clone, PartialEq, Eq, Debug, Hash, Spanned)]
pub struct Diagnostic {
    level: DiagnosticLevel,
    span: Span,
    message: String,
    context_name: Option<String>,
    children: Vec<Diagnostic>,
}

impl Diagnostic {
    /// Creates a new [`Diagnostic`] with the given level, span, message, context name, and children.
    ///
    /// The context name is the name of the input that the [`Diagnostic`] is associated with. If
    /// the context name is `None`, the context name will default to "input".
    pub fn new(
        level: DiagnosticLevel,
        span: Span,
        message: impl ToString,
        context_name: Option<impl ToString>,
        children: Vec<Diagnostic>,
    ) -> Diagnostic {
        Diagnostic {
            level,
            span,
            message: message.to_string(),
            context_name: context_name.map(|n| n.to_string()),
            children,
        }
    }

    /// Sets the level of this [`Diagnostic`] to the given level.
    pub fn set_level(&mut self, level: DiagnosticLevel) {
        self.level = level;
    }

    /// Sets the message of this [`Diagnostic`] to the given message.
    pub fn set_message(&mut self, message: impl Display) {
        self.message = message.to_string()
    }

    /// Sets the context name of this [`Diagnostic`] to the given name.
    pub fn set_context_name(&mut self, name: Option<impl AsRef<str>>) {
        self.context_name = name.map(|n| n.as_ref().to_string());
    }

    /// Returns the level of this [`Diagnostic`].
    pub fn level(&self) -> DiagnosticLevel {
        self.level
    }

    /// Returns the string message of this [`Diagnostic`].
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the name of the context that this [`Diagnostic`] is associated with.
    ///
    /// This is typically the name of the input that the [`Diagnostic`] is associated with, but
    /// can be overridden by setting the context name.
    pub fn context_name(&self) -> &str {
        match &self.context_name {
            Some(context_name) => context_name,
            None => "input",
        }
    }

    /// Returns a [`Vec`] of the children of this [`Diagnostic`].
    pub fn children(&self) -> &Vec<Diagnostic> {
        &self.children
    }

    /// Returns a [`Span`] that represents the range of the input that this [`Diagnostic`] is
    /// associated with.
    ///
    /// Identical to calling `self.span()` when the [`Diagnostic`] has no children.
    pub fn merged_span(&self) -> core::result::Result<Span, SpanJoinError> {
        let mut merged_span = self.span.clone();
        for child in &self.children {
            merged_span = merged_span.join(&child.merged_span()?)?;
        }
        Ok(merged_span)
    }
}

impl Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level = self.level;
        let message = &self.message;
        writeln!(f, "{level}: {message}")?;
        let span = self.span();
        let LineCol { line, col } = span.start();
        let mut num_width = 1;
        let mut temp = line;
        while temp >= 10 {
            num_width += 1;
            temp /= 10;
        }
        for _ in 1..num_width {
            write!(f, " ")?;
        }
        write!(f, " --> ")?;
        #[cfg(feature = "std")]
        match span.source_path() {
            Some(path) => write!(f, "{}", path.display())?,
            None => write!(f, "{}", self.context_name())?,
        }
        #[cfg(not(feature = "std"))]
        write!(f, "{}", self.context_name())?;
        let real_line = line + 1;
        writeln!(f, ":{real_line}:{col}")?;
        for _ in 0..num_width {
            write!(f, " ")?;
        }
        writeln!(f, " |")?;
        for (i, (lin, range)) in span.source_lines().enumerate() {
            let num = i + line + 1;
            writeln!(f, "{num} | {lin}")?;
            for _ in 0..num_width {
                write!(f, " ")?;
            }
            write!(f, "   ")?;
            for _ in 0..range.start {
                write!(f, " ")?;
            }
            let chars = lin.chars();
            let mut prev = false;
            for i in range {
                let Some(char) = chars.get(i) else {
                    write!(f, " ")?;
                    prev = true;
                    continue;
                };
                let current = char.is_whitespace();
                let next = if i + 1 < chars.len() {
                    chars[i + 1].is_whitespace()
                } else {
                    false
                };
                if current && (next || prev) {
                    write!(f, " ")?;
                } else {
                    write!(f, "^")?;
                }
                prev = current;
            }
            writeln!(f)?;
        }
        for child in &self.children {
            write!(f, "{child}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
use crate::Rc;

#[test]
fn test_diagnostic_display_single_line() {
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

#[test]
fn test_diagnostic_display_two_line() {
    let diag = Diagnostic {
        level: DiagnosticLevel::Warning,
        message: "this is a warning".to_string(),
        span: Span::new(
            Rc::new(Source::from_str(include_str!("samples/code_02.rs"))),
            20..36,
        ),
        context_name: None,
        children: Vec::new(),
    };
    println!("{}", diag.to_string());
    assert_eq!(diag.to_string(), include_str!("samples/diagnostic_02.txt"));
}

#[test]
fn test_diagnostic_display_three_line() {
    let diag = Diagnostic {
        level: DiagnosticLevel::Warning,
        message: "this is a warning".to_string(),
        span: Span::new(
            Rc::new(Source::from_str(include_str!("samples/code_03.rs"))),
            38..106,
        ),
        context_name: None,
        children: Vec::new(),
    };
    println!("{}", diag.to_string());
    assert_eq!(diag.to_string(), include_str!("samples/diagnostic_03.txt"));
}

#[test]
fn test_diagnostic_display_with_children() {
    let source = Rc::new(Source::from_str(include_str!("samples/code_04.rs")));
    let mut diag = Diagnostic {
        level: DiagnosticLevel::Warning,
        message: "this is a warning".to_string(),
        span: Span::new(source.clone(), 38..106),
        context_name: None,
        children: Vec::new(),
    };
    diag.children.push(Diagnostic {
        level: DiagnosticLevel::Warning,
        message: "this is a warning".to_string(),
        span: Span::new(source.clone(), 108..127),
        context_name: None,
        children: Vec::new(),
    });
    println!("{}", diag.to_string());
    assert_eq!(diag.to_string(), include_str!("samples/diagnostic_05.txt"));
}
