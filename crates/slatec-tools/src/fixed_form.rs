use crate::hash;

#[derive(Clone, Debug)]
pub struct RawSpan {
    pub start: u64,
    pub end: u64,
    pub line: u64,
    pub column_start: u16,
    pub column_end: u16,
}

#[derive(Clone, Debug)]
pub struct PhysicalLine {
    pub span: RawSpan,
    pub line_ending: &'static str,
    pub kind: LineKind,
    pub label: Option<String>,
    pub continuation: bool,
    pub statement_field: Vec<u8>,
    pub trailing_field: Vec<u8>,
    pub diagnostics: Vec<LexDiagnostic>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LineKind {
    Blank,
    Comment,
    Statement,
}

#[derive(Clone, Debug)]
pub struct LexDiagnostic {
    pub rule_id: &'static str,
    pub severity: &'static str,
    pub message: &'static str,
    pub span: RawSpan,
}

#[derive(Clone, Debug)]
pub struct LogicalStatement {
    pub raw_spans: Vec<RawSpan>,
    pub physical_line_start: u64,
    pub physical_line_end: u64,
    pub statement_label: Option<String>,
    pub raw_statement_text: Vec<u8>,
    pub normalized_statement_text: String,
    pub continuation_count: u32,
    pub classification: String,
    pub diagnostics: Vec<LexDiagnostic>,
}

pub fn physical_lines(bytes: &[u8]) -> Vec<PhysicalLine> {
    let mut result = Vec::new();
    let mut offset = 0_usize;
    let mut line = 1_u64;
    while offset < bytes.len() {
        let start = offset;
        while offset < bytes.len() && bytes[offset] != b'\n' && bytes[offset] != b'\r' {
            offset += 1;
        }
        let content_end = offset;
        let line_ending = if offset == bytes.len() {
            "none"
        } else if bytes[offset] == b'\r' && bytes.get(offset + 1) == Some(&b'\n') {
            offset += 2;
            "crlf"
        } else if bytes[offset] == b'\r' {
            offset += 1;
            "cr"
        } else {
            offset += 1;
            "lf"
        };
        let content = &bytes[start..content_end];
        let span = RawSpan {
            start: start as u64,
            end: offset as u64,
            line,
            column_start: 1,
            // Columns are one-based and may extend beyond the fixed-form text
            // field; retain their exact position for trailing/sequence evidence.
            column_end: content.len().saturating_add(1).min(u16::MAX as usize) as u16,
        };
        result.push(classify_line(content, span, line_ending));
        line += 1;
    }
    if bytes.is_empty() {
        result.push(classify_line(
            &[],
            RawSpan {
                start: 0,
                end: 0,
                line: 1,
                column_start: 1,
                column_end: 1,
            },
            "none",
        ));
    }
    result
}

fn classify_line(content: &[u8], span: RawSpan, line_ending: &'static str) -> PhysicalLine {
    let mut diagnostics = Vec::new();
    if content.iter().any(|byte| !byte.is_ascii()) {
        diagnostics.push(diag(
            "FF-NON-ASCII",
            "warning",
            "non-ASCII source bytes preserved without decoding",
            &span,
        ));
    }
    if content.contains(&b'\t') {
        diagnostics.push(diag(
            "FF-TAB-AMBIGUITY",
            "warning",
            "tab encountered; fixed-form column meaning is review-required",
            &span,
        ));
    }
    if content.is_empty() || content.iter().all(|byte| *byte == b' ' || *byte == b'\t') {
        return PhysicalLine {
            span,
            line_ending,
            kind: LineKind::Blank,
            label: None,
            continuation: false,
            statement_field: Vec::new(),
            trailing_field: Vec::new(),
            diagnostics,
        };
    }
    if matches!(content.first(), Some(b'C' | b'c' | b'*' | b'!')) {
        return PhysicalLine {
            span,
            line_ending,
            kind: LineKind::Comment,
            label: None,
            continuation: false,
            statement_field: Vec::new(),
            trailing_field: Vec::new(),
            diagnostics,
        };
    }
    let label_bytes = content.get(..content.len().min(5)).unwrap_or_default();
    let label = if label_bytes
        .iter()
        .all(|byte| byte.is_ascii_digit() || *byte == b' ')
    {
        let value = String::from_utf8_lossy(label_bytes).trim().to_owned();
        (!value.is_empty()).then_some(value)
    } else {
        diagnostics.push(diag(
            "FF-INVALID-LABEL-COLUMNS",
            "warning",
            "non-label bytes in fixed-form label columns",
            &span,
        ));
        None
    };
    let continuation = content
        .get(5)
        .is_some_and(|byte| !matches!(*byte, b' ' | b'0'));
    let statement_field = content
        .get(6..content.len().min(72))
        .unwrap_or_default()
        .to_vec();
    let trailing_field = content.get(72..).unwrap_or_default().to_vec();
    PhysicalLine {
        span,
        line_ending,
        kind: LineKind::Statement,
        label,
        continuation,
        statement_field,
        trailing_field,
        diagnostics,
    }
}

pub fn logical_statements(lines: &[PhysicalLine]) -> Vec<LogicalStatement> {
    let mut output = Vec::new();
    let mut current: Option<LogicalStatement> = None;
    for line in lines.iter().filter(|line| line.kind == LineKind::Statement) {
        if line.continuation {
            if let Some(statement) = current.as_mut() {
                statement
                    .raw_statement_text
                    .extend_from_slice(&strip_inline_comment(&line.statement_field));
                statement.raw_spans.push(line.span.clone());
                statement.physical_line_end = line.span.line;
                statement.continuation_count += 1;
                statement.diagnostics.extend(line.diagnostics.clone());
            } else {
                let mut diagnostics = line.diagnostics.clone();
                diagnostics.push(diag(
                    "FF-ORPHAN-CONTINUATION",
                    "warning",
                    "continuation line has no preceding statement",
                    &line.span,
                ));
                current = Some(new_statement(line, diagnostics));
            }
        } else {
            if let Some(statement) = current.take() {
                output.extend(split_semicolons(statement));
            }
            current = Some(new_statement(line, line.diagnostics.clone()));
        }
    }
    if let Some(statement) = current {
        output.extend(split_semicolons(statement));
    }
    for statement in &mut output {
        statement.normalized_statement_text = normalize(&statement.raw_statement_text);
        statement.classification =
            classify_statement(&statement.normalized_statement_text).to_owned();
    }
    output
}

fn new_statement(line: &PhysicalLine, diagnostics: Vec<LexDiagnostic>) -> LogicalStatement {
    LogicalStatement {
        raw_spans: vec![line.span.clone()],
        physical_line_start: line.span.line,
        physical_line_end: line.span.line,
        statement_label: line.label.clone(),
        raw_statement_text: strip_inline_comment(&line.statement_field),
        normalized_statement_text: String::new(),
        continuation_count: 0,
        classification: String::new(),
        diagnostics,
    }
}

fn split_semicolons(statement: LogicalStatement) -> Vec<LogicalStatement> {
    let positions = delimiters(&statement.raw_statement_text, b';');
    if positions.is_empty() {
        return vec![statement];
    }
    let mut output = Vec::new();
    let mut start = 0;
    for end in positions
        .into_iter()
        .chain(std::iter::once(statement.raw_statement_text.len()))
    {
        let raw = statement.raw_statement_text[start..end].to_vec();
        if !raw.iter().all(|byte| byte.is_ascii_whitespace()) {
            let mut part = statement.clone();
            part.raw_statement_text = raw;
            if start != 0 {
                part.statement_label = None;
            }
            output.push(part);
        }
        start = end + 1;
    }
    output
}

pub fn normalize(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_uppercase()
}

pub fn identifier_after(input: &str, keyword: &str) -> Option<String> {
    let remainder = input.strip_prefix(keyword)?.trim_start();
    let name: String = remainder
        .chars()
        .take_while(|character| {
            character.is_ascii_alphanumeric() || *character == '$' || *character == '_'
        })
        .collect();
    (!name.is_empty()).then_some(name)
}

fn classify_statement(input: &str) -> &'static str {
    if start_declaration(input).is_some() {
        "program_unit_start"
    } else if is_end(input) {
        "program_unit_end"
    } else if input.starts_with("ENTRY ") {
        "entry"
    } else {
        "other"
    }
}

#[derive(Clone, Debug)]
pub struct StartDeclaration {
    pub kind: String,
    pub name: Option<String>,
    pub declared_return_type: Option<String>,
}

pub fn start_declaration(input: &str) -> Option<StartDeclaration> {
    if let Some(name) = identifier_after(input, "SUBROUTINE") {
        return Some(StartDeclaration {
            kind: "subroutine".to_owned(),
            name: Some(name),
            declared_return_type: None,
        });
    }
    if let Some(name) = identifier_after(input, "PROGRAM") {
        return Some(StartDeclaration {
            kind: "program".to_owned(),
            name: Some(name),
            declared_return_type: None,
        });
    }
    if input == "BLOCK DATA" {
        return Some(StartDeclaration {
            kind: "block_data".to_owned(),
            name: None,
            declared_return_type: None,
        });
    }
    if let Some(name) = identifier_after(input, "BLOCK DATA") {
        return Some(StartDeclaration {
            kind: "block_data".to_owned(),
            name: Some(name),
            declared_return_type: None,
        });
    }
    let function = input
        .find(" FUNCTION ")
        .or_else(|| input.strip_prefix("FUNCTION ").map(|_| 0))?;
    let (prefix, name_at) = if function == 0 {
        (None, "FUNCTION".len())
    } else {
        (
            Some(input[..function].trim().to_owned()),
            function + " FUNCTION".len(),
        )
    };
    let name: String = input[name_at..]
        .trim_start()
        .chars()
        .take_while(|character| {
            character.is_ascii_alphanumeric() || *character == '$' || *character == '_'
        })
        .collect();
    if name.is_empty() {
        return None;
    }
    let valid_prefix = prefix.as_ref().is_none_or(|value| {
        matches!(
            value.as_str(),
            "INTEGER" | "REAL" | "DOUBLE PRECISION" | "COMPLEX" | "DOUBLE COMPLEX" | "LOGICAL"
        ) || value.starts_with("CHARACTER")
    });
    valid_prefix.then_some(StartDeclaration {
        kind: "function".to_owned(),
        name: Some(name),
        declared_return_type: prefix,
    })
}

pub fn is_end(input: &str) -> bool {
    matches!(
        input,
        "END" | "END SUBROUTINE" | "END FUNCTION" | "END PROGRAM" | "END BLOCK DATA"
    ) || input.starts_with("END SUBROUTINE ")
        || input.starts_with("END FUNCTION ")
        || input.starts_with("END PROGRAM ")
        || input.starts_with("END BLOCK DATA ")
}

pub fn entry_declaration(input: &str) -> Option<(String, Vec<String>)> {
    let name = identifier_after(input, "ENTRY")?;
    let args = input
        .split_once('(')
        .and_then(|(_, rest)| rest.split_once(')'))
        .map(|(inside, _)| {
            inside
                .split(',')
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(ToOwned::to_owned)
                .collect()
        })
        .unwrap_or_default();
    Some((name, args))
}

pub fn statement_hash(statement: &LogicalStatement) -> String {
    hash::bytes(&statement.raw_statement_text)
}

fn strip_inline_comment(bytes: &[u8]) -> Vec<u8> {
    let mut quote = None;
    let mut index = 0;
    while index < bytes.len() {
        let byte = bytes[index];
        if let Some(current) = quote {
            if byte == current {
                if bytes.get(index + 1) == Some(&current) {
                    index += 2;
                    continue;
                }
                quote = None;
            }
            index += 1;
            continue;
        }
        if byte == b'\'' || byte == b'"' {
            quote = Some(byte);
            index += 1;
            continue;
        }
        if byte == b'!' {
            return bytes[..index].to_vec();
        }
        if byte.is_ascii_digit() {
            let digits_start = index;
            while bytes.get(index).is_some_and(u8::is_ascii_digit) {
                index += 1;
            }
            if bytes
                .get(index)
                .is_some_and(|value| *value == b'H' || *value == b'h')
            {
                let length = std::str::from_utf8(&bytes[digits_start..index])
                    .ok()
                    .and_then(|value| value.parse::<usize>().ok())
                    .unwrap_or(0);
                index = index.saturating_add(1 + length);
                continue;
            }
            continue;
        }
        index += 1;
    }
    bytes.to_vec()
}

fn delimiters(bytes: &[u8], delimiter: u8) -> Vec<usize> {
    let mut quote = None;
    let mut output = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        let byte = bytes[index];
        if let Some(current) = quote {
            if byte == current {
                if bytes.get(index + 1) == Some(&current) {
                    index += 2;
                    continue;
                }
                quote = None;
            }
            index += 1;
            continue;
        }
        if byte == b'\'' || byte == b'"' {
            quote = Some(byte);
            index += 1;
            continue;
        }
        if byte == delimiter {
            output.push(index);
        }
        index += 1;
    }
    output
}

fn diag(
    rule_id: &'static str,
    severity: &'static str,
    message: &'static str,
    span: &RawSpan,
) -> LexDiagnostic {
    LexDiagnostic {
        rule_id,
        severity,
        message,
        span: span.clone(),
    }
}
