//! Fixed-form persistent-state recognition and validation fixtures.

use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;

use crate::fixed_form::{logical_statements, physical_lines};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct StateFinding {
    pub line: usize,
    pub routine: String,
    pub origin: String,
    pub storage: String,
    pub layout: String,
    pub statement: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CompilerOracle {
    pub common_blocks: BTreeSet<String>,
    pub persistent_variables: BTreeSet<String>,
}

/// Scans complete fixed-form logical statements, retaining program-unit scope.
pub fn scan(bytes: &[u8]) -> Vec<StateFinding> {
    let lines = physical_lines(bytes);
    let mut current = "UNKNOWN_PROGRAM_UNIT".to_owned();
    let mut output = Vec::new();
    for statement in logical_statements(&lines) {
        let text = lexical_compact(&statement.normalized_statement_text);
        if let Some(name) = routine_name(&text) {
            current = name;
        }
        let line = statement.physical_line_start as usize;
        let evidence = statement.normalized_statement_text.clone();
        for (block, layout) in common_blocks(&text) {
            output.push(StateFinding {
                line,
                routine: current.clone(),
                origin: "COMMON".to_owned(),
                storage: block,
                layout,
                statement: evidence.clone(),
            });
        }
        for storage in data_variables(&text) {
            output.push(simple(line, &current, "DATA", storage, &evidence));
        }
        for storage in save_variables(&text) {
            output.push(simple(line, &current, "SAVE", storage, &evidence));
        }
        if declaration_initialization(&text) {
            for storage in declaration_variables(&text) {
                output.push(simple(
                    line,
                    &current,
                    "DECLARATION_INITIALIZATION",
                    storage,
                    &evidence,
                ));
            }
        }
        for (prefix, origin) in [
            ("BLOCKDATA", "BLOCK_DATA"),
            ("EQUIVALENCE", "EQUIVALENCE"),
            ("ENTRY", "ENTRY"),
            ("INCLUDE", "INCLUDE"),
        ] {
            if text.starts_with(prefix) {
                output.push(simple(
                    line,
                    &current,
                    origin,
                    "program_unit_construct".to_owned(),
                    &evidence,
                ));
            }
        }
        if io_statement(&text) {
            output.push(simple(
                line,
                &current,
                "FORTRAN_IO",
                "fortran_runtime_units".to_owned(),
                &evidence,
            ));
        }
        if xerror_call(&text) {
            output.push(simple(
                line,
                &current,
                "XERROR_CALL",
                "xerror_runtime_state".to_owned(),
                &evidence,
            ));
        }
    }
    output.sort();
    output.dedup();
    output
}

pub fn program_units(bytes: &[u8]) -> Vec<String> {
    let lines = physical_lines(bytes);
    logical_statements(&lines)
        .into_iter()
        .filter_map(|statement| {
            routine_name(&lexical_compact(&statement.normalized_statement_text))
        })
        .collect()
}

fn simple(
    line: usize,
    routine: &str,
    origin: &str,
    storage: String,
    statement: &str,
) -> StateFinding {
    StateFinding {
        line,
        routine: routine.to_owned(),
        origin: origin.to_owned(),
        layout: storage.clone(),
        storage,
        statement: statement.to_owned(),
    }
}

/// Runs GNU Fortran's parse-tree dump as an independent validation oracle.
pub fn compiler_oracle(compiler: &Path, source: &Path) -> Result<CompilerOracle, String> {
    let output = Command::new(compiler)
        .args([
            "-x",
            "f77",
            "-std=legacy",
            "-ffixed-line-length-none",
            "-fdump-parse-tree",
            "-fsyntax-only",
        ])
        .arg(source)
        .output()
        .map_err(|error| format!("start compiler oracle: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "compiler oracle failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&output.stderr));
    Ok(parse_compiler_oracle(&text))
}

pub fn parse_compiler_oracle(text: &str) -> CompilerOracle {
    let mut output = CompilerOracle::default();
    let mut symbol = None;
    let mut in_common = BTreeSet::new();
    let mut named_common_members = BTreeSet::new();
    for raw in text.lines() {
        let line = raw.trim();
        if let Some(rest) = line.strip_prefix("symtree: '") {
            symbol = rest.split('\'').next().map(canonical_variable);
        } else if let Some(rest) = line.strip_prefix("common:") {
            if let Some(named) = rest.trim().strip_prefix('/') {
                if let Some((block, members)) = named.split_once('/') {
                    output.common_blocks.insert(block.to_ascii_uppercase());
                    named_common_members.extend(
                        members
                            .split(',')
                            .map(canonical_variable)
                            .filter(|value| !value.is_empty()),
                    );
                }
            }
        } else if line.starts_with("attributes:") {
            if line.contains("IN-COMMON") {
                if let Some(symbol) = symbol.as_ref() {
                    in_common.insert(symbol.clone());
                }
            }
            if (line.contains(" DATA")
                || line.contains("EXPLICIT-SAVE")
                || line.contains("IMPLICIT-SAVE"))
                && !line.contains("PARAMETER")
            {
                if let Some(symbol) = symbol.as_ref() {
                    output.persistent_variables.insert(symbol.clone());
                }
            }
        }
    }
    if in_common
        .iter()
        .any(|symbol| !named_common_members.contains(symbol))
    {
        output.common_blocks.insert("(blank)".to_owned());
    }
    output
}

pub fn scanner_oracle(findings: &[StateFinding]) -> CompilerOracle {
    CompilerOracle {
        common_blocks: findings
            .iter()
            .filter(|finding| finding.origin == "COMMON")
            .map(|finding| finding.storage.clone())
            .collect(),
        persistent_variables: findings
            .iter()
            .filter(|finding| {
                matches!(
                    finding.origin.as_str(),
                    "DATA" | "SAVE" | "DECLARATION_INITIALIZATION"
                ) && !finding.storage.starts_with('/')
                    && finding.storage != "all_locals"
            })
            .map(|finding| canonical_variable(&finding.storage))
            .collect(),
    }
}

fn lexical_compact(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut quote = None;
    for byte in input.bytes() {
        if let Some(current) = quote {
            result.push(byte as char);
            if byte == current {
                quote = None;
            }
        } else if matches!(byte, b'\'' | b'"') {
            quote = Some(byte);
            result.push(byte as char);
        } else if !byte.is_ascii_whitespace() {
            result.push((byte as char).to_ascii_uppercase());
        }
    }
    result
}

fn routine_name(text: &str) -> Option<String> {
    if let Some(rest) = text.strip_prefix("BLOCKDATA") {
        let name = identifier(rest);
        return Some(if name.is_empty() {
            "BLOCK_DATA".to_owned()
        } else {
            name
        });
    }
    for keyword in ["SUBROUTINE", "FUNCTION"] {
        if let Some(index) = text.find(keyword) {
            let prefix = &text[..index];
            if keyword == "FUNCTION"
                && !matches!(
                    prefix,
                    "" | "INTEGER" | "REAL" | "DOUBLEPRECISION" | "LOGICAL" | "COMPLEX"
                )
                && !prefix.starts_with("CHARACTER")
            {
                continue;
            }
            let name = identifier(&text[index + keyword.len()..]);
            if !name.is_empty() {
                return Some(name);
            }
        }
    }
    text.strip_prefix("PROGRAM")
        .map(identifier)
        .filter(|name| !name.is_empty())
}

fn identifier(input: &str) -> String {
    input
        .chars()
        .take_while(|character| character.is_ascii_alphanumeric() || *character == '_')
        .collect()
}

fn common_blocks(text: &str) -> Vec<(String, String)> {
    let Some(mut rest) = text.strip_prefix("COMMON") else {
        return Vec::new();
    };
    let mut output = Vec::new();
    while !rest.is_empty() {
        rest = rest.trim_start_matches(',');
        let (block, after_name) = if let Some(named) = rest.strip_prefix('/') {
            let Some(end) = named.find('/') else {
                break;
            };
            (
                named[..end].to_ascii_uppercase(),
                &named[end.saturating_add(1)..],
            )
        } else {
            ("(blank)".to_owned(), rest)
        };
        let next = next_common_block(after_name).unwrap_or(after_name.len());
        let layout = after_name[..next].trim_matches(',').to_owned();
        output.push((block, layout));
        rest = &after_name[next..];
        if next == after_name.len() {
            break;
        }
    }
    output
}

fn next_common_block(text: &str) -> Option<usize> {
    let mut depth = 0_usize;
    for (index, character) in text.char_indices() {
        match character {
            '(' => depth += 1,
            ')' => depth = depth.saturating_sub(1),
            '/' if depth == 0 => return Some(index),
            _ => {}
        }
    }
    None
}

fn data_variables(text: &str) -> Vec<String> {
    let Some(mut rest) = text.strip_prefix("DATA") else {
        return Vec::new();
    };
    let mut output = Vec::new();
    while let Some(slash) = rest.find('/') {
        let names = rest[..slash].trim_matches(',');
        output.extend(
            split_top_level(names)
                .into_iter()
                .map(canonical_variable)
                .filter(|name| !name.is_empty()),
        );
        rest = &rest[slash + 1..];
        let Some(end) = rest.find('/') else { break };
        rest = &rest[end + 1..];
    }
    output
}

fn save_variables(text: &str) -> Vec<String> {
    let Some(rest) = text.strip_prefix("SAVE") else {
        return Vec::new();
    };
    if rest.contains('=')
        || rest
            .bytes()
            .next()
            .is_some_and(|byte| byte.is_ascii_digit() || matches!(byte, b'(' | b'='))
    {
        return Vec::new();
    }
    if rest.is_empty() {
        return vec!["all_locals".to_owned()];
    }
    split_top_level(rest.trim_matches(','))
        .into_iter()
        .map(|value| {
            if value.starts_with('/') && value.ends_with('/') {
                value.to_ascii_uppercase()
            } else {
                canonical_variable(value)
            }
        })
        .filter(|value| !value.is_empty())
        .collect()
}

fn declaration_initialization(text: &str) -> bool {
    declaration_prefix(text).is_some()
        && (text.contains("::") && text.contains('=') || text.contains('/'))
        && !text.starts_with("DATA")
}

fn declaration_variables(text: &str) -> Vec<String> {
    let Some(offset) = declaration_prefix(text) else {
        return Vec::new();
    };
    let rest = text[offset..].trim_start_matches(':');
    split_top_level(rest)
        .into_iter()
        .filter(|item| item.contains('=') || item.contains('/'))
        .map(|item| canonical_variable(item.split(['=', '/']).next().unwrap_or(item)))
        .filter(|item| !item.is_empty())
        .collect()
}

fn declaration_prefix(text: &str) -> Option<usize> {
    [
        "DOUBLEPRECISION",
        "DOUBLECOMPLEX",
        "INTEGER",
        "LOGICAL",
        "CHARACTER",
        "COMPLEX",
        "REAL",
    ]
    .into_iter()
    .find_map(|prefix| {
        text.strip_prefix(prefix)
            .map(|rest| text.len() - rest.len())
    })
}

fn split_top_level(text: &str) -> Vec<&str> {
    let mut output = Vec::new();
    let mut depth = 0_usize;
    let mut start = 0_usize;
    for (index, character) in text.char_indices() {
        match character {
            '(' => depth += 1,
            ')' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                output.push(&text[start..index]);
                start = index + 1;
            }
            _ => {}
        }
    }
    output.push(&text[start..]);
    output
}

fn canonical_variable(input: &str) -> String {
    identifier(input.trim().trim_start_matches('/')).to_ascii_uppercase()
}

fn io_statement(text: &str) -> bool {
    [
        "OPEN",
        "CLOSE",
        "READ",
        "WRITE",
        "REWIND",
        "BACKSPACE",
        "ENDFILE",
        "INQUIRE",
    ]
    .into_iter()
    .any(|keyword| text.starts_with(keyword))
}

fn xerror_call(text: &str) -> bool {
    [
        "XERMSG", "XERRWV", "XERPRN", "XERSVE", "XGETF", "XSETF", "J4SAVE",
    ]
    .into_iter()
    .any(|name| text.contains(&format!("CALL{name}")) || text.contains(&format!("{name}(")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    struct Fixture {
        name: &'static str,
        source: &'static str,
        origins: &'static [&'static str],
        commons: &'static [&'static str],
    }

    const FIXTURES: &[Fixture] = &[
        Fixture {
            name: "named_common",
            source: "      SUBROUTINE A\n      COMMON /STATE/ X,Y\n      END\n",
            origins: &["COMMON"],
            commons: &["STATE"],
        },
        Fixture {
            name: "blank_common",
            source: "      SUBROUTINE A\n      COMMON X,Y\n      END\n",
            origins: &["COMMON"],
            commons: &["(blank)"],
        },
        Fixture {
            name: "multiple_common",
            source: "      SUBROUTINE A\n      COMMON /ONE/ X, /TWO/ Y\n      END\n",
            origins: &["COMMON"],
            commons: &["ONE", "TWO"],
        },
        Fixture {
            name: "continued_common",
            source: "      SUBROUTINE A\n      COMMON /LONG/ X,\n     1 Y,Z\n      END\n",
            origins: &["COMMON"],
            commons: &["LONG"],
        },
        Fixture {
            name: "spaced_keyword_lowercase",
            source: "      subroutine a\n      c o m m o n /s p a c e/ x\n      end\n",
            origins: &["COMMON"],
            commons: &["SPACE"],
        },
        Fixture {
            name: "comments_literals_hollerith",
            source: "      SUBROUTINE A\nC     COMMON /NOPE/ X\n      CHARACTER*20 S\n      S='COMMON /NOPE/ X !'\n      PRINT 10, 16HCOMMON /NOPE/ X\n   10 FORMAT(A)\n      END\n",
            origins: &[],
            commons: &[],
        },
        Fixture {
            name: "labelled_save",
            source: "      SUBROUTINE A\n  100 SAVE I\n      END\n",
            origins: &["SAVE"],
            commons: &[],
        },
        Fixture {
            name: "identifier_starting_with_save_is_not_a_save_statement",
            source: "      SUBROUTINE A\n      SAVEDT = .FALSE.\n      END\n",
            origins: &[],
            commons: &[],
        },
        Fixture {
            name: "save_block",
            source: "      SUBROUTINE A\n      COMMON /STATE/ X\n      SAVE /STATE/\n      END\n",
            origins: &["COMMON", "SAVE"],
            commons: &["STATE"],
        },
        Fixture {
            name: "data",
            source: "      SUBROUTINE A\n      DATA I,J /1,2/\n      END\n",
            origins: &["DATA"],
            commons: &[],
        },
        Fixture {
            name: "declaration_initialization",
            source: "      SUBROUTINE A\n      INTEGER :: I=1\n      END\n",
            origins: &["DECLARATION_INITIALIZATION"],
            commons: &[],
        },
        Fixture {
            name: "old_style_declaration_initialization",
            source: "      SUBROUTINE A\n      INTEGER I /1/\n      END\n",
            origins: &["DECLARATION_INITIALIZATION"],
            commons: &[],
        },
        Fixture {
            name: "block_data_equivalence",
            source: "      BLOCK DATA B\n      COMMON /CSTATE/ X\n      DATA X /1.0/\n      END\n      SUBROUTINE A\n      EQUIVALENCE (X,Y)\n      END\n",
            origins: &["BLOCK_DATA", "COMMON", "DATA", "EQUIVALENCE"],
            commons: &["CSTATE"],
        },
        Fixture {
            name: "continued_dimension",
            source: "      SUBROUTINE A\n      INTEGER X(2,\n     1 3)\n      SAVE X\n      END\n",
            origins: &["SAVE"],
            commons: &[],
        },
        Fixture {
            name: "multiple_units_entry",
            source: "      SUBROUTINE A\n      SAVE I\n      ENTRY ALT()\n      END\n      SUBROUTINE B\n      DATA J /0/\n      END\n",
            origins: &["SAVE", "ENTRY", "DATA"],
            commons: &[],
        },
        Fixture {
            name: "include_statement",
            source: "      SUBROUTINE A\n      INCLUDE 'fixture.inc'\n      END\n",
            origins: &["INCLUDE"],
            commons: &[],
        },
    ];

    #[test]
    fn fixtures_cover_fixed_form_state_constructs() {
        for fixture in FIXTURES {
            let findings = scan(fixture.source.as_bytes());
            let origins = findings
                .iter()
                .map(|finding| finding.origin.as_str())
                .collect::<BTreeSet<_>>();
            for origin in fixture.origins {
                assert!(origins.contains(origin), "{} lacks {origin}", fixture.name);
            }
            assert_eq!(
                scanner_oracle(&findings).common_blocks,
                fixture
                    .commons
                    .iter()
                    .map(|value| (*value).to_owned())
                    .collect(),
                "{}",
                fixture.name
            );
        }
    }

    #[test]
    fn compiler_oracle_agrees_when_gfortran_is_configured() {
        let Some(compiler) = std::env::var_os("SLATEC_GFORTRAN") else {
            return;
        };
        let temp = tempfile::tempdir().unwrap();
        fs::write(temp.path().join("fixture.inc"), "      INTEGER INCLUDED\n").unwrap();
        for fixture in FIXTURES {
            let path = temp.path().join(format!("{}.f", fixture.name));
            fs::write(&path, fixture.source).unwrap();
            let compiler = compiler_oracle(Path::new(&compiler), &path).unwrap();
            let scanner = scanner_oracle(&scan(fixture.source.as_bytes()));
            assert_eq!(
                compiler.common_blocks, scanner.common_blocks,
                "{}",
                fixture.name
            );
            assert_eq!(
                compiler.persistent_variables, scanner.persistent_variables,
                "{}",
                fixture.name
            );
        }
    }

    #[test]
    fn multiple_program_units_keep_their_own_scope() {
        let source = FIXTURES
            .iter()
            .find(|fixture| fixture.name == "multiple_units_entry")
            .unwrap()
            .source;
        let findings = scan(source.as_bytes());
        assert!(
            findings
                .iter()
                .any(|finding| finding.routine == "A" && finding.storage == "I")
        );
        assert!(
            findings
                .iter()
                .any(|finding| finding.routine == "B" && finding.storage == "J")
        );
    }
}
