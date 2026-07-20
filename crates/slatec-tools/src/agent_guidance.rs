//! Offline validation for repository navigation and Codex guidance.
//!
//! The checks intentionally use a few explicit markers for cross-cutting
//! invariants. Ordinary explanatory prose remains free to evolve.

use crate::error::{CorpusError, Result};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path, PathBuf};

const ROOT_RULES: &[&str] = &[
    "sys-provider-neutral",
    "all-declaration-only",
    "generated-no-hand-edit",
    "link-granularity",
    "honest-native-validation",
];
const GENERATED_RULES: &[&str] = &[
    "generated-no-hand-edit",
    "deterministic-regeneration",
    "generated-no-native-artifacts",
];
const ROOT_NAVIGATION: &[&str] = &[
    "docs/architecture/README.md",
    "docs/agent/REPOSITORY-MAP.md",
    "docs/agent/TASK-CHECKLIST.md",
];
const ARCHITECTURE_LINKS: &[&str] = &[
    "slatec-sys-public-raw-api.md",
    "../api/family-features-and-backends.md",
    "native-link-granularity.md",
    "../agent/REPOSITORY-MAP.md",
    "../agent/TASK-CHECKLIST.md",
    "../agent/USING-CODEX.md",
];
const SKIPPED_DIRECTORIES: &[&str] = &[".git", ".worktrees", "target"];

#[derive(Debug)]
struct Link {
    line: usize,
    target: String,
}

/// Validates the authored guidance rooted at `root` without acquiring inputs
/// or invoking a native toolchain.
pub fn validate(root: &Path) -> Result<()> {
    let root = root.canonicalize()?;
    let members = workspace_members(&root)?;
    let mut errors = Vec::new();

    let root_guidance_path = root.join("AGENTS.md");
    let root_guidance = read_required(&root_guidance_path, &mut errors);
    let generated_guidance_path = root.join("generated/AGENTS.md");
    let generated_guidance = read_required(&generated_guidance_path, &mut errors);
    let architecture_path = root.join("docs/architecture/README.md");
    let architecture = read_required(&architecture_path, &mut errors);
    let repository_map_path = root.join("docs/agent/REPOSITORY-MAP.md");
    let repository_map = read_required(&repository_map_path, &mut errors);

    if let Some(contents) = root_guidance.as_deref() {
        validate_markers(
            contents,
            &root_guidance_path,
            ROOT_RULES,
            "required root invariant",
            &mut errors,
        );
        validate_required_links(contents, &root_guidance_path, ROOT_NAVIGATION, &mut errors);
        validate_workspace_guidance(&root, &members, contents, &mut errors);
    }
    if let Some(contents) = generated_guidance.as_deref() {
        validate_markers(
            contents,
            &generated_guidance_path,
            GENERATED_RULES,
            "generated-tree policy",
            &mut errors,
        );
    }
    if let Some(contents) = architecture.as_deref() {
        let mut required = ARCHITECTURE_LINKS.to_vec();
        if root
            .join("docs/architecture/safe-facade-link-granularity.md")
            .is_file()
        {
            required.push("safe-facade-link-granularity.md");
        }
        validate_required_links(contents, &architecture_path, &required, &mut errors);
    }
    if let Some(contents) = repository_map.as_deref() {
        validate_repository_map(contents, &members, &repository_map_path, &mut errors);
    }

    for source in guidance_files(&root)? {
        let contents = match fs::read_to_string(&source) {
            Ok(contents) => contents,
            Err(error) => {
                errors.push(format!(
                    "{}: could not read file: {error}",
                    display(&root, &source)
                ));
                continue;
            }
        };
        if source != root_guidance_path
            && source.file_name().is_some_and(|name| name == "AGENTS.md")
        {
            validate_scope(&root, &source, &contents, &mut errors);
        }
        for link in markdown_links(&contents) {
            validate_link(&root, &source, &link, &mut errors);
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(CorpusError::Verification(format!(
            "agent guidance validation failed:\n- {}",
            errors.join("\n- ")
        )))
    }
}

fn workspace_members(root: &Path) -> Result<Vec<String>> {
    let manifest = root.join("Cargo.toml");
    let contents = fs::read_to_string(&manifest)?;
    let value: toml::Value = toml::from_str(&contents)?;
    let members = value
        .get("workspace")
        .and_then(|workspace| workspace.get("members"))
        .and_then(toml::Value::as_array)
        .ok_or_else(|| {
            CorpusError::Policy("workspace manifest has no [workspace].members array".to_owned())
        })?;
    let mut paths = Vec::new();
    for member in members {
        let member = member.as_str().ok_or_else(|| {
            CorpusError::Policy("workspace member is not a string path".to_owned())
        })?;
        paths.push(member.replace('\\', "/"));
    }
    Ok(paths)
}

fn validate_workspace_guidance(
    root: &Path,
    members: &[String],
    root_guidance: &str,
    errors: &mut Vec<String>,
) {
    let exemptions = guidance_exemptions(root_guidance);
    for member in members {
        let guidance = root.join(member).join("AGENTS.md");
        if !guidance.is_file() && !exemptions.contains(member) {
            errors.push(format!(
                "{}: workspace member `{member}` has no crate-local AGENTS.md and no documented exemption",
                display(root, &root.join("Cargo.toml"))
            ));
        }
    }
}

fn guidance_exemptions(contents: &str) -> BTreeSet<String> {
    contents
        .lines()
        .filter_map(|line| line.trim().strip_prefix("<!-- agent-guidance-exemption: "))
        .filter_map(|line| line.strip_suffix(" -->"))
        .map(|path| path.trim().replace('\\', "/"))
        .collect()
}

fn validate_repository_map(
    contents: &str,
    members: &[String],
    path: &Path,
    errors: &mut Vec<String>,
) {
    for member in members {
        let marker = format!("`{member}`");
        if !contents.contains(&marker) {
            errors.push(format!(
                "{}: repository map omits workspace path `{member}`",
                path.display()
            ));
        }
    }
    for area in ["generated", "docs"] {
        let marker = format!("`{area}`");
        if !contents.contains(&marker) {
            errors.push(format!(
                "{}: repository map omits major path `{area}`",
                path.display()
            ));
        }
    }
}

fn validate_markers(
    contents: &str,
    path: &Path,
    markers: &[&str],
    category: &str,
    errors: &mut Vec<String>,
) {
    for marker in markers {
        let marker = format!("<!-- agent-rule: {marker} -->");
        if !contents.contains(&marker) {
            errors.push(format!(
                "{}: missing {category} marker `{marker}`",
                path.display()
            ));
        }
    }
}

fn validate_required_links(
    contents: &str,
    path: &Path,
    required: &[&str],
    errors: &mut Vec<String>,
) {
    let targets = markdown_links(contents)
        .into_iter()
        .map(|link| local_target(&link.target))
        .collect::<BTreeSet<_>>();
    for target in required {
        if !targets.contains(*target) {
            errors.push(format!(
                "{}: missing required architecture/navigation link `{target}`",
                path.display()
            ));
        }
    }
}

fn validate_scope(root: &Path, path: &Path, contents: &str, errors: &mut Vec<String>) {
    let declared = contents.lines().find_map(|line| {
        let after_scope = line.trim().strip_prefix("Scope:")?.trim();
        let start = after_scope.find('`')? + 1;
        let end = after_scope[start..].find('`')? + start;
        Some(
            after_scope[start..end]
                .trim()
                .trim_end_matches('/')
                .to_owned(),
        )
    });
    let Some(declared) = declared else {
        errors.push(format!(
            "{}: missing recognizable `Scope: `...`` declaration",
            display(root, path)
        ));
        return;
    };
    let declared = declared.strip_suffix("/**").unwrap_or(&declared);
    let expected = path
        .parent()
        .and_then(|parent| parent.strip_prefix(root).ok())
        .map(path_string)
        .unwrap_or_default();
    if declared.replace('\\', "/") != expected {
        errors.push(format!(
            "{}: scope declares `{declared}/**`, but its subtree is `{expected}/**`",
            display(root, path)
        ));
    }
}

fn guidance_files(root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = BTreeSet::new();
    collect_agent_files(root, &mut files)?;
    files.insert(root.join("docs/architecture/README.md"));
    let agent_dir = root.join("docs/agent");
    if agent_dir.is_dir() {
        for entry in fs::read_dir(agent_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_file()
                && entry
                    .path()
                    .extension()
                    .is_some_and(|extension| extension == "md")
            {
                files.insert(entry.path());
            }
        }
    }
    Ok(files.into_iter().collect())
}

fn collect_agent_files(directory: &Path, files: &mut BTreeSet<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        if file_type.is_symlink() {
            continue;
        }
        if file_type.is_dir() {
            if !SKIPPED_DIRECTORIES.contains(&entry.file_name().to_string_lossy().as_ref()) {
                collect_agent_files(&path, files)?;
            }
        } else if file_type.is_file() && entry.file_name() == "AGENTS.md" {
            files.insert(path);
        }
    }
    Ok(())
}

fn markdown_links(contents: &str) -> Vec<Link> {
    let mut links = Vec::new();
    for (line_index, line) in contents.lines().enumerate() {
        let mut remaining = line;
        while let Some(start) = remaining.find("](") {
            let after_open = &remaining[start + 2..];
            let Some(end) = after_open.find(')') else {
                break;
            };
            let target = after_open[..end].trim();
            if !target.is_empty() {
                links.push(Link {
                    line: line_index + 1,
                    target: target
                        .trim_matches('<')
                        .trim_matches('>')
                        .split_whitespace()
                        .next()
                        .unwrap_or_default()
                        .to_owned(),
                });
            }
            remaining = &after_open[end + 1..];
        }
    }
    links
}

fn validate_link(root: &Path, source: &Path, link: &Link, errors: &mut Vec<String>) {
    if is_external(&link.target) || link.target.starts_with('#') {
        return;
    }
    let target = local_target(&link.target);
    if target.is_empty() {
        return;
    }
    let target_path = Path::new(&target);
    if target_path.is_absolute() {
        errors.push(link_error(
            root,
            source,
            link,
            target_path,
            "absolute local paths are not allowed",
        ));
        return;
    }
    let candidate = lexical_normalize(&source.parent().unwrap_or(root).join(target_path));
    if !candidate.starts_with(root) {
        errors.push(link_error(
            root,
            source,
            link,
            &candidate,
            "link resolves outside the repository",
        ));
        return;
    }
    if !candidate.exists() {
        errors.push(link_error(
            root,
            source,
            link,
            &candidate,
            "referenced local file or directory does not exist",
        ));
        return;
    }
    if let Ok(canonical) = candidate.canonicalize() {
        if !canonical.starts_with(root) {
            errors.push(link_error(
                root,
                source,
                link,
                &canonical,
                "link follows outside the repository",
            ));
        }
    }
}

fn is_external(target: &str) -> bool {
    let target = target.to_ascii_lowercase();
    target.starts_with("http://")
        || target.starts_with("https://")
        || target.starts_with("mailto:")
        || target.starts_with("//")
}

fn local_target(target: &str) -> String {
    target
        .split('#')
        .next()
        .unwrap_or_default()
        .split('?')
        .next()
        .unwrap_or_default()
        .replace('\\', "/")
}

fn lexical_normalize(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            Component::Normal(part) => normalized.push(part),
            Component::RootDir | Component::Prefix(_) => normalized.push(component.as_os_str()),
        }
    }
    normalized
}

fn link_error(root: &Path, source: &Path, link: &Link, resolved: &Path, reason: &str) -> String {
    format!(
        "{}:{}: link `{}` resolved to `{}`: {reason}",
        display(root, source),
        link.line,
        link.target,
        display(root, resolved)
    )
}

fn read_required(path: &Path, errors: &mut Vec<String>) -> Option<String> {
    match fs::read_to_string(path) {
        Ok(contents) => Some(contents),
        Err(error) => {
            errors.push(format!(
                "{}: could not read required guidance file: {error}",
                path.display()
            ));
            None
        }
    }
}

fn display(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .map(path_string)
        .unwrap_or_else(|_| path.display().to_string())
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn current_repository_is_valid() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        validate(&root).unwrap();
    }

    #[test]
    fn rejects_missing_crate_guidance() {
        let fixture = Fixture::new();
        fs::remove_file(fixture.root().join("crates/slatec/AGENTS.md")).unwrap();
        fixture.assert_invalid("workspace member `crates/slatec` has no crate-local AGENTS.md");
    }

    #[test]
    fn rejects_broken_relative_link() {
        let fixture = Fixture::new();
        append(
            &fixture.root().join("AGENTS.md"),
            "\n[missing](docs/agent/missing.md)\n",
        );
        fixture.assert_invalid(
            "AGENTS.md:10: link `docs/agent/missing.md` resolved to `docs/agent/missing.md`: referenced local file or directory does not exist",
        );
    }

    #[test]
    fn rejects_incorrect_scope() {
        let fixture = Fixture::new();
        write(
            &fixture.root().join("crates/slatec/AGENTS.md"),
            "Scope: `crates/elsewhere/**`.\n",
        );
        fixture.assert_invalid("scope declares `crates/elsewhere/**`");
    }

    #[test]
    fn rejects_workspace_member_missing_from_repository_map() {
        let fixture = Fixture::new();
        append(&fixture.root().join("Cargo.toml"), "\n# fixture member\n");
        let manifest = fixture.root().join("Cargo.toml");
        let contents = fs::read_to_string(&manifest).unwrap();
        write(
            &manifest,
            &contents.replace(
                "\"crates/slatec-tools\",\n]",
                "\"crates/slatec-tools\",\n    \"crates/new\",\n]",
            ),
        );
        write(
            &fixture.root().join("crates/new/AGENTS.md"),
            "Scope: `crates/new/**`.\n",
        );
        fixture.assert_invalid("repository map omits workspace path `crates/new`");
    }

    #[test]
    fn rejects_missing_generated_policy_marker() {
        let fixture = Fixture::new();
        write(
            &fixture.root().join("generated/AGENTS.md"),
            "<!-- agent-rule: generated-no-hand-edit -->\n<!-- agent-rule: deterministic-regeneration -->\n",
        );
        fixture.assert_invalid("missing generated-tree policy marker");
    }

    #[test]
    fn rejects_missing_architecture_link() {
        let fixture = Fixture::new();
        let path = fixture.root().join("docs/architecture/README.md");
        let contents = fs::read_to_string(&path).unwrap();
        write(
            &path,
            &contents.replace("native-link-granularity.md", "removed.md"),
        );
        fixture.assert_invalid(
            "missing required architecture/navigation link `native-link-granularity.md`",
        );
    }

    #[test]
    fn rejects_missing_root_rule_marker() {
        let fixture = Fixture::new();
        let path = fixture.root().join("AGENTS.md");
        let contents = fs::read_to_string(&path).unwrap();
        write(
            &path,
            &contents.replace("<!-- agent-rule: link-granularity -->\n", ""),
        );
        fixture.assert_invalid("missing required root invariant marker");
    }

    struct Fixture {
        directory: TempDir,
    }

    impl Fixture {
        fn new() -> Self {
            let directory = tempfile::tempdir().unwrap();
            let root = directory.path();
            write(
                &root.join("Cargo.toml"),
                "[workspace]\nmembers = [\n    \"crates/slatec-sys\",\n    \"crates/slatec-src\",\n    \"crates/slatec-core\",\n    \"crates/slatec\",\n    \"crates/slatec-tools\",\n]\n",
            );
            write(
                &root.join("AGENTS.md"),
                "<!-- agent-rule: sys-provider-neutral -->\n<!-- agent-rule: all-declaration-only -->\n<!-- agent-rule: generated-no-hand-edit -->\n<!-- agent-rule: link-granularity -->\n<!-- agent-rule: honest-native-validation -->\n[architecture](docs/architecture/README.md)\n[map](docs/agent/REPOSITORY-MAP.md)\n[checklist](docs/agent/TASK-CHECKLIST.md)\n",
            );
            for crate_name in [
                "slatec-sys",
                "slatec-src",
                "slatec-core",
                "slatec",
                "slatec-tools",
            ] {
                write(
                    &root.join(format!("crates/{crate_name}/AGENTS.md")),
                    &format!("Scope: `crates/{crate_name}/**`.\n"),
                );
            }
            write(
                &root.join("generated/AGENTS.md"),
                "Scope: `generated/**`.\n<!-- agent-rule: generated-no-hand-edit -->\n<!-- agent-rule: deterministic-regeneration -->\n<!-- agent-rule: generated-no-native-artifacts -->\n",
            );
            write(
                &root.join("docs/architecture/README.md"),
                "[raw](slatec-sys-public-raw-api.md)\n[features](../api/family-features-and-backends.md)\n[native](native-link-granularity.md)\n[safe](safe-facade-link-granularity.md)\n[map](../agent/REPOSITORY-MAP.md)\n[checklist](../agent/TASK-CHECKLIST.md)\n[codex](../agent/USING-CODEX.md)\n",
            );
            for path in [
                "docs/architecture/slatec-sys-public-raw-api.md",
                "docs/architecture/native-link-granularity.md",
                "docs/architecture/safe-facade-link-granularity.md",
                "docs/api/family-features-and-backends.md",
                "docs/agent/TASK-CHECKLIST.md",
                "docs/agent/USING-CODEX.md",
            ] {
                write(&root.join(path), "fixture\n");
            }
            write(
                &root.join("docs/agent/REPOSITORY-MAP.md"),
                "`crates/slatec-sys`\n`crates/slatec-src`\n`crates/slatec-core`\n`crates/slatec`\n`crates/slatec-tools`\n`generated`\n`docs`\n",
            );
            Self { directory }
        }

        fn root(&self) -> &Path {
            self.directory.path()
        }

        fn assert_invalid(&self, expected: &str) {
            let error = validate(self.root()).unwrap_err().to_string();
            assert!(error.contains(expected), "{error}");
        }
    }

    fn write(path: &Path, contents: &str) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, contents).unwrap();
    }

    fn append(path: &Path, contents: &str) {
        let mut current = fs::read_to_string(path).unwrap();
        current.push_str(contents);
        write(path, &current);
    }
}
