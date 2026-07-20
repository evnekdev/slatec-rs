//! Reviewed real Airy-function raw API policy and rendering.
//!
//! The eight FNLIB scalar functions have a simple, compiler-observed ABI: one
//! real scalar passed by address and a real scalar returned directly. Complex
//! Amos Airy drivers and Airy subsidiaries are deliberately reported here but
//! are outside this review policy.

use crate::hash;
use serde_json::{Value, json};
use std::collections::BTreeMap;

/// The complete real FNLIB Airy driver set promoted by this review.
pub const REAL_DRIVERS: &[&str] = &["AI", "AIE", "BI", "BIE", "DAI", "DAIE", "DBI", "DBIE"];

/// All catalogue identities directly named as Airy routines, plus the two
/// real Airy asymptotic helpers used by the FNLIB drivers.
const AIRY_RELATED: &[&str] = &[
    "AI", "AIE", "BI", "BIE", "CAIRY", "CBIRY", "D9AIMP", "DAI", "DAIE", "DBI", "DBIE", "DJAIRY",
    "DYAIRY", "JAIRY", "R9AIMP", "YAIRY", "ZAIRY", "ZBIRY",
];

/// Returns whether the routine is one of the reviewed real scalar drivers.
pub fn is_real_driver(routine: &str) -> bool {
    REAL_DRIVERS.contains(&routine)
}

/// Returns the public and provider feature for a reviewed real Airy driver.
pub fn feature(routine: &str) -> Option<&'static str> {
    is_real_driver(routine).then_some("special-airy")
}

/// Returns the real scalar precision established by the reviewed source pair.
pub fn precision(routine: &str) -> Option<&'static str> {
    spec(routine).map(|spec| spec.precision)
}

/// Source-hash guard input for the real Airy review policy.
pub fn source_manifest(records: &[Value]) -> String {
    let mut entries = records
        .iter()
        .filter(|record| is_real_driver(&field(record, "routine")))
        .filter(|record| field(record, "historical_role") == "user_callable")
        .filter(|record| field(record, "generated_declaration_status") == "generated_abi_validated")
        .map(|record| {
            format!(
                "{}\0{}\0{}\n",
                field(record, "routine"),
                field(record, "source_hash"),
                field(record, "canonical_provider"),
            )
        })
        .collect::<Vec<_>>();
    entries.sort();
    hash::bytes(entries.concat().as_bytes())
}

/// Renders the canonical reviewed `slatec_sys::special::airy` module.
pub fn render_module(records: &[Value]) -> String {
    let mut output = String::from(
        "/// Reviewed real scalar Airy-function declarations.\n///\n/// The complex Amos Airy interfaces and Airy implementation subsidiaries are\n/// intentionally not re-exported here; see the generated Airy family report.\n#[cfg(any(feature = \"special-airy\", feature = \"raw-family-special-airy\"))]\npub mod airy {\n",
    );
    let mut items = records
        .iter()
        .filter(|record| is_real_driver(&field(record, "routine")))
        .filter(|record| field(record, "reviewed_declaration_status") == "reviewed_public_driver")
        .collect::<Vec<_>>();
    items.sort_by_key(|record| field(record, "routine"));
    assert_eq!(
        items.len(),
        REAL_DRIVERS.len(),
        "reviewed Airy policy did not render every real scalar driver"
    );
    for record in items {
        render_routine(&mut output, record);
    }
    if output.ends_with("\n\n") {
        output.pop();
    }
    output.push_str("}\n");
    output
}

/// Creates a complete, explicit Airy-family promotion and deferment report.
pub fn family_report(records: &[Value]) -> Value {
    let mut rows = records
        .iter()
        .filter(|record| AIRY_RELATED.contains(&field(record, "routine").as_str()))
        .map(|record| {
            let routine = field(record, "routine");
            let reviewed = field(record, "reviewed_declaration_status") == "reviewed_public_driver";
            let scope = is_real_driver(&routine);
            let disposition = if reviewed {
                "reviewed real scalar public driver"
            } else if matches!(routine.as_str(), "CAIRY" | "CBIRY" | "ZAIRY" | "ZBIRY") {
                "deferred ABI-sensitive complex Airy driver"
            } else {
                "deferred Airy subsidiary"
            };
            let reason = if reviewed {
                "none"
            } else if matches!(routine.as_str(), "CAIRY" | "CBIRY" | "ZAIRY" | "ZBIRY") {
                "complex argument, result, scaling, and status-code contracts are outside the reviewed real-scalar ABI policy"
            } else {
                "internal Airy helper or Amos subsidiary; no independent public-contract review is claimed"
            };
            json!({
                "routine":routine,
                "report_scope":"Airy identity inventory",
                "in_real_scalar_promotion_scope":scope,
                "historical_role":field(record,"historical_role"),
                "program_unit_kind":field(record,"program_unit_kind"),
                "precision":field(record,"precision"),
                "canonical_provider":field(record,"canonical_provider"),
                "source_hash":field(record,"source_hash"),
                "source_file":field(record,"source_file"),
                "native_symbol":field(record,"native_symbol"),
                "generated_declaration_status":field(record,"generated_declaration_status"),
                "reviewed_declaration_status":field(record,"reviewed_declaration_status"),
                "current_rust_paths":record.get("legacy_rust_paths").cloned().unwrap_or_else(|| json!([])),
                "canonical_rust_path":field(record,"canonical_rust_path"),
                "provider_feature":field(record,"provider_feature"),
                "documentation_status":field(record,"documentation_status"),
                "compile_test_status":field(record,"compile_test_status"),
                "link_test_status":field(record,"link_test_status"),
                "runtime_test_status":field(record,"runtime_test_status"),
                "safe_wrapper_status":field(record,"safe_wrapper_status"),
                "final_disposition":disposition,
                "exclusion_or_deferment_reason":reason,
            })
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left["routine"].as_str().cmp(&right["routine"].as_str()));
    let counts = rows
        .iter()
        .fold(BTreeMap::<String, usize>::new(), |mut counts, row| {
            *counts.entry(field(row, "final_disposition")).or_default() += 1;
            counts
        });
    json!({
        "schema_id":"slatec-sys.raw-api.airy-family-report",
        "schema_version":"1.0.0",
        "scope":"Every catalogue identity explicitly named as an Airy routine, plus R9AIMP and D9AIMP, which are direct FNLIB Airy asymptotic helpers.",
        "reviewed_real_driver_count":rows.iter().filter(|row| row["final_disposition"] == "reviewed real scalar public driver").count(),
        "disposition_counts":counts,
        "records":rows,
    })
}

fn render_routine(output: &mut String, record: &Value) {
    let routine = field(record, "routine");
    let spec = spec(&routine).expect("reviewed Airy routine has specification");
    let source_link = format!(
        "https://www.netlib.org/slatec/{}",
        field(record, "source_file")
    );
    output.push_str(&format!("    // raw-api-routine: {routine}\n"));
    doc_line(output, &format!("Original SLATEC routine `{routine}`."));
    doc_line(output, spec.purpose);
    doc_line(
        output,
        &format!("Mathematical operation: {}.", spec.operation),
    );
    doc_line(
        output,
        &format!(
            "Precision and return ABI: `{}` Fortran scalar function returned directly by the supported GNU MinGW ABI.",
            spec.precision
        ),
    );
    doc_line(
        output,
        &format!("Domain and exceptional behavior: {}", spec.domain),
    );
    doc_line(
        output,
        &format!(
            "Selected provider: `{}`; source: <{source_link}>; native symbol: `{}`.",
            field(record, "canonical_provider"),
            field(record, "native_symbol"),
        ),
    );
    doc_line(
        output,
        "Supported ABI profile: `ffi-profile-gnu-mingw-x86_64`; the real scalar input is passed by address.",
    );
    doc_line(output, "");
    doc_line(output, "# Arguments");
    doc_line(
        output,
        "- `X`: input real scalar, passed by address; null is not permitted, it is read only for the complete call, aliases no output because there is no output pointer, and the native routine does not retain it.",
    );
    doc_line(output, "");
    doc_line(output, "# Native behavior");
    doc_line(output, spec.error);
    doc_line(output, FNLIB_STATE);
    doc_line(output, "");
    doc_line(output, "# Safety");
    doc_line(
        output,
        "`X` must be non-null, correctly aligned, and readable as the documented scalar type for the complete call. The pointer is input-only and is not retained. The caller must link a provider built for the documented GNU MinGW ABI, must not permit a panic or other unwinding across this FFI boundary, and must avoid input ranges that trigger fatal legacy XERROR behavior. FNLIB initialization and legacy error handling are process-global; this raw declaration does not serialize calls, so callers must provide any required synchronization.",
    );
    output.push_str(&format!(
        "    #[doc(inline)]\n    pub use crate::families::special_airy::{};\n\n",
        routine.to_ascii_lowercase()
    ));
}

fn doc_line(output: &mut String, text: &str) {
    if text.is_empty() {
        output.push_str("    ///\n");
    } else {
        output.push_str("    /// ");
        output.push_str(text);
        output.push('\n');
    }
}

struct Spec {
    routine: &'static str,
    precision: &'static str,
    purpose: &'static str,
    operation: &'static str,
    domain: &'static str,
    error: &'static str,
}

const FNLIB_STATE: &str = "FNLIB initializes saved coefficients and machine constants on first use; this raw interface does not claim that initialization or the legacy error runtime is thread-safe.";
const AI_UNDERFLOW: &str = "For sufficiently large positive X the unscaled Ai implementation returns zero and reports the documented underflow through XERMSG.";
const BI_OVERFLOW: &str = "For sufficiently large positive X the unscaled Bi implementation reports its documented overflow through XERMSG.";
const SCALED_BEHAVIOR: &str = "The scaled implementation avoids the corresponding positive-real exponential range problem; its finite-input result and any machine-limit behavior remain those of the selected FNLIB source.";

const SPECS: &[Spec] = &[
    Spec {
        routine: "AI",
        precision: "f32",
        purpose: "Evaluates the Airy function Ai.",
        operation: "the real Airy Ai function",
        domain: "real X is accepted; large positive X reaches the documented underflow path.",
        error: AI_UNDERFLOW,
    },
    Spec {
        routine: "AIE",
        precision: "f32",
        purpose: "Evaluates Ai, exponentially scaled on the non-negative real axis.",
        operation: "Ai(X) for X <= 0 and exp(2 X^(3/2) / 3) Ai(X) for X >= 0",
        domain: "real X is accepted.",
        error: SCALED_BEHAVIOR,
    },
    Spec {
        routine: "BI",
        precision: "f32",
        purpose: "Evaluates the Bairy function Bi, the Airy function of the second kind.",
        operation: "the real Airy Bi function",
        domain: "real X is accepted; large positive X reaches the documented overflow path.",
        error: BI_OVERFLOW,
    },
    Spec {
        routine: "BIE",
        precision: "f32",
        purpose: "Evaluates Bi, exponentially scaled on the non-negative real axis.",
        operation: "Bi(X) for X <= 0 and exp(-2 X^(3/2) / 3) Bi(X) for X >= 0",
        domain: "real X is accepted.",
        error: SCALED_BEHAVIOR,
    },
    Spec {
        routine: "DAI",
        precision: "f64",
        purpose: "Evaluates the double-precision Airy function Ai.",
        operation: "the real double-precision Airy Ai function",
        domain: "real X is accepted; large positive X reaches the documented underflow path.",
        error: AI_UNDERFLOW,
    },
    Spec {
        routine: "DAIE",
        precision: "f64",
        purpose: "Evaluates double-precision Ai, exponentially scaled on the non-negative real axis.",
        operation: "Ai(X) for X <= 0 and exp(2 X^(3/2) / 3) Ai(X) for X >= 0",
        domain: "real X is accepted.",
        error: SCALED_BEHAVIOR,
    },
    Spec {
        routine: "DBI",
        precision: "f64",
        purpose: "Evaluates the double-precision Bairy function Bi.",
        operation: "the real double-precision Airy Bi function",
        domain: "real X is accepted; large positive X reaches the documented overflow path.",
        error: BI_OVERFLOW,
    },
    Spec {
        routine: "DBIE",
        precision: "f64",
        purpose: "Evaluates double-precision Bi, exponentially scaled on the non-negative real axis.",
        operation: "Bi(X) for X <= 0 and exp(-2 X^(3/2) / 3) Bi(X) for X >= 0",
        domain: "real X is accepted.",
        error: SCALED_BEHAVIOR,
    },
];

fn spec(routine: &str) -> Option<&'static Spec> {
    SPECS.iter().find(|spec| spec.routine == routine)
}

fn field(value: &Value, key: &str) -> String {
    value
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned()
}
