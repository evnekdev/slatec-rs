//! Deterministic reconciliation evidence for the FISHPACK and ODE/DAE bulk milestone.

use std::fs;
use std::path::Path;

use serde_json::{Value, json};

use crate::error::{CorpusError, Result};
use crate::hash;

#[derive(Clone, Copy)]
struct FishpackRoutine {
    routine: &'static str,
    source: &'static str,
    hash: &'static str,
    coordinate_system: &'static str,
    grid: &'static str,
    safe_status: &'static str,
    safe_path: &'static str,
    provider_feature: &'static str,
    decision: &'static str,
}

const FISHPACK: &[FishpackRoutine] = &[
    FishpackRoutine {
        routine: "BLKTRI",
        source: "blktri.f",
        hash: "a6cd337d7100886e79d733f93d46a394798e815d3f9f45e94157a8bd21f0ae55",
        coordinate_system: "generic",
        grid: "block tridiagonal matrix",
        safe_status: "raw_only",
        safe_path: "",
        provider_feature: "fishpack-general",
        decision: "expert linear-system primitive; no PDE geometry contract",
    },
    FishpackRoutine {
        routine: "CBLKTR",
        source: "cblktr.f",
        hash: "34bdeeae64c2379ac2b5b47e9d9c47511eb68b4551da04e404aa6149901b9aba",
        coordinate_system: "generic complex",
        grid: "complex block tridiagonal matrix",
        safe_status: "raw_only",
        safe_path: "",
        provider_feature: "fishpack-general",
        decision: "complex expert primitive; no owned safe matrix contract",
    },
    FishpackRoutine {
        routine: "CMGNBN",
        source: "cmgnbn.f",
        hash: "d90dfb81074b2cf34af266ec88a8db535d14c663127e154f79a4d0461e0062d7",
        coordinate_system: "generic complex",
        grid: "complex centered elliptic system",
        safe_status: "raw_only",
        safe_path: "",
        provider_feature: "fishpack-general",
        decision: "complex generic solver; family-level safe representation deferred",
    },
    FishpackRoutine {
        routine: "GENBUN",
        source: "genbun.f",
        hash: "c54069e07f3b22162ab776f7a165213b6f21e67d482d38835888a6915b872af2",
        coordinate_system: "generic",
        grid: "centered two-dimensional elliptic system",
        safe_status: "subsidiary_raw_only",
        safe_path: "",
        provider_feature: "fishpack-general",
        decision: "reviewed transitive subsidiary for centered public drivers",
    },
    FishpackRoutine {
        routine: "HSTCRT",
        source: "hstcrt.f",
        hash: "85e104e06ec54fd77e3844395b3b8ff80d387f376727756b91aad1cca9c70cd7",
        coordinate_system: "Cartesian",
        grid: "staggered two-dimensional",
        safe_status: "raw_only",
        safe_path: "",
        provider_feature: "fishpack-general",
        decision: "source uses M-by-N staggered interior F(IDIMF,N), BDA/BDB length N, and BDC/BDD length M; its corner-free edge-vector model and 13*M+4*N+M*INT(log2(N)) workspace have not received an independent owned-grid/raw-parity review",
    },
    FishpackRoutine {
        routine: "HSTCSP",
        source: "hstcsp.f",
        hash: "ac706d1674ab5450c1932f30d9ee781fb8c869374a0447ecd545fe82c1a2c4b9",
        coordinate_system: "axisymmetric spherical",
        grid: "staggered two-dimensional",
        safe_status: "implemented",
        safe_path: "slatec::differential_equations::pde::StaggeredAxisymmetricSphericalHelmholtz2d::solve",
        provider_feature: "fishpack-spherical",
        decision: "owned staggered theta-r grid with source-specific pole and origin regularity checks",
    },
    FishpackRoutine {
        routine: "HSTCYL",
        source: "hstcyl.f",
        hash: "cc7ba2f159b5d63732b77e3ee2b201559520adec82a861e94084a0514400c87f",
        coordinate_system: "cylindrical",
        grid: "staggered two-dimensional",
        safe_status: "implemented",
        safe_path: "slatec::differential_equations::pde::StaggeredCylindricalHelmholtz2d::solve",
        provider_feature: "fishpack-cylindrical-polar",
        decision: "owned staggered r-z grid with source-specific axis and coefficient checks",
    },
    FishpackRoutine {
        routine: "HSTPLR",
        source: "hstplr.f",
        hash: "fc8d21942d0ef18365d2836e298f3764cf11e38c85ecc1193ad45cdee6e6341b",
        coordinate_system: "polar",
        grid: "staggered two-dimensional",
        safe_status: "implemented",
        safe_path: "slatec::differential_equations::pde::StaggeredPolarHelmholtz2d::solve",
        provider_feature: "fishpack-cylindrical-polar",
        decision: "owned staggered r-theta grid with source-specific origin and angular checks",
    },
    FishpackRoutine {
        routine: "HSTSSP",
        source: "hstssp.f",
        hash: "30244bf4964e343bc696482928c25e455180b2dee91442a83a17710762ed9596",
        coordinate_system: "unit-sphere surface",
        grid: "staggered two-dimensional",
        safe_status: "implemented",
        safe_path: "slatec::differential_equations::pde::StaggeredSphereSurfaceHelmholtz2d::solve",
        provider_feature: "fishpack-spherical",
        decision: "owned staggered theta-phi grid with source-exact pole regularity checks",
    },
    FishpackRoutine {
        routine: "HW3CRT",
        source: "hw3crt.f",
        hash: "0096a81afd97dbfe627ceb41dab17f3c33ecbf650456308ec625c3d09423da6d",
        coordinate_system: "Cartesian",
        grid: "centered three-dimensional",
        safe_status: "raw_only",
        safe_path: "",
        provider_feature: "fishpack-general",
        decision: "source owns F(LDIMF,MDIMF,N+1), BDXS/BDXF(MDIMF,N+1), BDYS/BDYF(LDIMF,N+1), and BDZS/BDZF(LDIMF,M+1); the six face-specific derivative/value orientations and 30+L+M+5*N+MAX(L,M,N)+7*(INT((L+1)/2)+INT((M+1)/2)) workspace need a dedicated safe/raw-parity design",
    },
    FishpackRoutine {
        routine: "HWSCRT",
        source: "hwscrt.f",
        hash: "9bcd5a3be9e6d63e7dcc33637eb37ef07ba10b727b74859d08cb4daa7f813202",
        coordinate_system: "Cartesian",
        grid: "centered two-dimensional",
        safe_status: "implemented",
        safe_path: "slatec::differential_equations::pde::CartesianHelmholtz2d::solve",
        provider_feature: "fishpack-cartesian-2d",
        decision: "existing owned rectangular-grid facade",
    },
    FishpackRoutine {
        routine: "HWSCSP",
        source: "hwscsp.f",
        hash: "2ebbe3c86deffcbe2f7a6af8dcf7590fbc9a8a60c0455e2b4f23f655b8353db9",
        coordinate_system: "axisymmetric spherical",
        grid: "centered two-dimensional",
        safe_status: "implemented",
        safe_path: "slatec::differential_equations::pde::AxisymmetricSphericalHelmholtz2d::solve",
        provider_feature: "fishpack-spherical",
        decision: "owned centered theta-r grid with source-specific pole and origin regularity checks",
    },
    FishpackRoutine {
        routine: "HWSCYL",
        source: "hwscyl.f",
        hash: "e8274ff7a4e394c74255f78d3ad0d659a4ffc578619fd40b9fb25df497d78f8d",
        coordinate_system: "cylindrical",
        grid: "centered two-dimensional",
        safe_status: "implemented",
        safe_path: "slatec::differential_equations::pde::CylindricalHelmholtz2d::solve",
        provider_feature: "fishpack-cylindrical-polar",
        decision: "owned centered r-z grid with checked axis and boundary modes",
    },
    FishpackRoutine {
        routine: "HWSPLR",
        source: "hwsplr.f",
        hash: "fc3fc0e8ecf71f2bf689b935b01eb24bf382cb44bb0c9ffeebdbacd2dadd2bca",
        coordinate_system: "polar",
        grid: "centered two-dimensional",
        safe_status: "implemented",
        safe_path: "slatec::differential_equations::pde::PolarHelmholtz2d::solve",
        provider_feature: "fishpack-cylindrical-polar",
        decision: "owned centered r-theta grid with checked origin/angular combinations",
    },
    FishpackRoutine {
        routine: "HWSSSP",
        source: "hwsssp.f",
        hash: "6106cb40d48d35ac6be9bb714bc097e82ee299106997e0f5d94a11c93f481b1e",
        coordinate_system: "unit-sphere surface",
        grid: "centered two-dimensional",
        safe_status: "implemented",
        safe_path: "slatec::differential_equations::pde::SphereSurfaceHelmholtz2d::solve",
        provider_feature: "fishpack-spherical",
        decision: "owned centered theta-phi grid; full-pole constructors use the reviewed PIMACH value",
    },
    FishpackRoutine {
        routine: "POIS3D",
        source: "pois3d.f",
        hash: "9daf0cd2c9eab106f9b508f426003a98719d21ed29b4fde0f224300d8e88da78",
        coordinate_system: "structured Cartesian",
        grid: "three-dimensional coefficient system",
        safe_status: "implemented",
        safe_path: "slatec::differential_equations::pde::Pois3dProblem::solve",
        provider_feature: "fishpack-pois3d",
        decision: "existing structured-system facade; not a general six-face PDE solver",
    },
    FishpackRoutine {
        routine: "POISTG",
        source: "poistg.f",
        hash: "38123a4c97f593d34cf7a9f0f795516579fdda13f8c3ee2ae98a3d5673d16cb0",
        coordinate_system: "generic",
        grid: "staggered two-dimensional elliptic system",
        safe_status: "subsidiary_raw_only",
        safe_path: "",
        provider_feature: "fishpack-general",
        decision: "generic staggered solver; public geometry-specific drivers remain preferred",
    },
];

/// Returns source-hash-guarded native-state projections for the curvilinear
/// FISHPACK facades added by this milestone.
///
/// Cartesian HWSCRT and structured POIS3D retain their more narrowly scoped
/// projections in `safe_fishpack` and `safe_pois3d`.  These records cover only
/// the new cylindrical, polar, unit-sphere, and axisymmetric-spherical
/// wrappers, so the global ownership and runtime audits cannot mistake the
/// absence of a prior broad archive scan for evidence of reentrancy.
pub(crate) fn native_state_projections() -> Result<Vec<Value>> {
    let metadata_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../slatec-src/metadata");
    let family_manifest: Value =
        serde_json::from_slice(&fs::read(metadata_root.join("family-source-closure.json"))?)?;
    let sources = family_manifest["sources"].as_array().ok_or_else(|| {
        CorpusError::Verification("provider manifest lacks source records".to_owned())
    })?;

    let mut output = Vec::new();
    for (feature, closure_name, expected_closure_size) in [
        (
            "fishpack-cylindrical-polar",
            "fishpack-cylindrical-polar",
            16_usize,
        ),
        ("fishpack-spherical", "fishpack-spherical", 43_usize),
    ] {
        let local_closure: Value = serde_json::from_slice(&fs::read(
            metadata_root.join(format!("{closure_name}-source-closure.json")),
        )?)?;
        let source_ids = local_closure["source_ids"].as_array().ok_or_else(|| {
            CorpusError::Verification(format!("{feature} closure lacks source identities"))
        })?;
        if source_ids.len() != expected_closure_size {
            return Err(CorpusError::Verification(format!(
                "{feature} closure no longer has its reviewed source count"
            )));
        }
        let source_ids = source_ids
            .iter()
            .filter_map(Value::as_str)
            .collect::<std::collections::BTreeSet<_>>();
        if source_ids.len() != expected_closure_size {
            return Err(CorpusError::Verification(format!(
                "{feature} closure contains duplicate or non-string source identities"
            )));
        }
        let closure = sources
            .iter()
            .filter(|source| {
                source["id"]
                    .as_str()
                    .is_some_and(|id| source_ids.contains(id))
            })
            .collect::<Vec<_>>();
        if closure.len() != expected_closure_size {
            return Err(CorpusError::Verification(format!(
                "{feature} closure references an unavailable selected source"
            )));
        }
        let object_closure = closure
            .iter()
            .filter_map(|source| source["id"].as_str())
            .map(|id| format!("{id}.o"))
            .collect::<Vec<_>>();
        let source_closure = closure
            .iter()
            .filter_map(|source| source["path"].as_str())
            .map(str::to_owned)
            .collect::<Vec<_>>();

        for routine in FISHPACK.iter().filter(|routine| {
            routine.safe_status == "implemented" && routine.provider_feature == feature
        }) {
            let entry = closure.iter().find(|source| {
                source["path"].as_str() == Some(routine.source)
                    && source["sha256"].as_str() == Some(routine.hash)
            });
            let entry_id = entry
                .and_then(|source| source["id"].as_str())
                .ok_or_else(|| {
                    CorpusError::Verification(format!(
                        "{feature} closure no longer contains the reviewed {} entry",
                        routine.routine
                    ))
                })?;
            let spherical = feature == "fishpack-spherical";
            output.push(json!({
                "safe_function":routine.safe_path,
                "native_entry_points":[routine.routine],
                "feature":feature,
                "effective_native_families":[feature],
                "entry_object":format!("{entry_id}.o"),
                "object_closure":object_closure,
                "source_closure":source_closure,
                "saved_mutable_locals":Vec::<&str>::new(),
                "common_blocks":[],
                "xerror_state":if spherical { vec!["reviewed spherical closure includes the selected XERROR support path"] } else { Vec::<&str>::new() },
                "fortran_io":[],
                "callback_state":["none"],
                "writable_symbols":["caller-owned right-hand side and private safe-wrapper workspace; every native solve holds the process-global runtime lock"],
                "source_object_unresolved":[],
                "external_undefined_symbols":[],
                "feature_closure_mismatch":false,
                "current_class":"SerializedGlobal",
                "best_possible_class_from_slatec_source":"BackendDependent",
                "native_routine_reentrancy":if spherical { "the reviewed closure reaches XERROR support; the facade remains process serialized" } else { "the exact selected closure is retained as provider evidence; the facade remains process serialized pending provider/runtime qualification" },
                "rust_api_concurrency":"owned grids and boundary data are movable; every native solve uses the process-global runtime lock",
                "provider_runtime_thread_safety":"unqualified external and system provider/runtime contracts",
                "provider_unknowns":["compiler and external provider runtime contract is not qualified for concurrent native entry"],
                "remaining_blockers":if spherical { vec!["selected XERROR support path", "repository-wide process-global native serialization policy", "provider/runtime qualification"] } else { vec!["repository-wide process-global native serialization policy", "provider/runtime qualification"] }
            }));
        }
    }
    output.sort_by(|left, right| {
        left["safe_function"]
            .as_str()
            .cmp(&right["safe_function"].as_str())
    });
    if output.len() != 8 {
        return Err(CorpusError::Verification(
            "curvilinear FISHPACK native-state projection count is not eight".to_owned(),
        ));
    }
    Ok(output)
}

/// Concise deterministic generation result.
#[derive(Debug)]
pub struct ResultSummary {
    /// Stable content hash of every written output.
    pub semantic_hash: String,
    /// FISHPACK routines in the retained-driver inventory.
    pub fishpack_count: usize,
}

/// Generates the bulk family coverage and deferment evidence.
pub fn generate(root: &Path, output_dir: &Path) -> Result<ResultSummary> {
    let manifest: Value = serde_json::from_slice(&fs::read(
        root.join("crates/slatec-src/metadata/family-source-closure.json"),
    )?)?;
    let sources = manifest["sources"]
        .as_array()
        .ok_or_else(|| CorpusError::Verification("provider manifest lacks sources".to_owned()))?;
    for routine in FISHPACK {
        let verified = sources.iter().any(|source| {
            source["path"].as_str() == Some(routine.source)
                && source["sha256"].as_str() == Some(routine.hash)
        });
        if !verified {
            return Err(CorpusError::Verification(format!(
                "FISHPACK evidence source/hash changed for {}",
                routine.routine
            )));
        }
    }
    let records = FISHPACK
        .iter()
        .map(|routine| json!({
            "routine":routine.routine,
            "canonical_raw_path":format!("slatec_sys::pde::fishpack::{}", routine.routine.to_ascii_lowercase()),
            "source":routine.source,
            "source_hash":routine.hash,
            "coordinate_system":routine.coordinate_system,
            "grid":routine.grid,
            "provider_feature":routine.provider_feature,
            "safe_status":routine.safe_status,
            "safe_path":routine.safe_path,
            "link_test_status":if routine.safe_status == "implemented" { "focused_native_linked" } else if routine.provider_feature == "external_only" { "source_closure_missing" } else { "raw_provider_recorded" },
            "runtime_test_status":if routine.safe_status != "implemented" { "not_safe_wrapper" } else if matches!(routine.routine, "HWSCYL" | "HWSPLR" | "HWSSSP" | "HWSCSP") { "native_hardened_boundary_coefficient_and_singular_perturbation_regression" } else { "native_manufactured_or_existing_regression" },
            "decision":routine.decision,
        }))
        .collect::<Vec<_>>();
    let implemented = records
        .iter()
        .filter(|record| record["safe_status"] == "implemented")
        .count();
    let markdown = format!(
        "# Complete FISHPACK coverage\n\n- Retained canonical drivers: {}.\n- Implemented checked safe drivers: {implemented}: HWSCRT, HSTCSP, HSTCYL, HSTPLR, HSTSSP, HWSCSP, HWSCYL, HWSPLR, HWSSSP, and POIS3D.\n- Every remaining driver is explicitly raw-only or subsidiary-only with the reason recorded in `fishpack-complete-coverage.json`; no generated declaration is counted as safe coverage. HSTCRT and HW3CRT now record their exact source grids, face/edge contracts, and workspace formulas as deliberately separate unimplemented safe designs.\n- The cylindrical/polar feature uses an audited 16-source closure for centered and staggered driver roots. The spherical feature uses a separate 43-source closure for unit-sphere and axisymmetric roots, including the exact XERROR dependency closure needed by the reviewed GNU-MinGW machine-constant profile support. Focused native regressions now cover nonconstant boundary data, nonzero coefficients, regularity modes, and singular-Poisson perturbations for the centered cylindrical, polar, sphere-surface, and axisymmetric-spherical wrappers.\n",
        records.len()
    );
    let ode_dae = json!({
        "schema_id":"slatec.safe-api.ode-dae-reconciliation",
        "schema_version":"1.0.0",
        "records":[
            ["SDRIV1/DDRIV1","implemented","Driv1Session<f32/f64>","owned continuation with panic-contained RHS"],
            ["SDRIV2/DDRIV2","implemented","Driv2Session<f32/f64>","owned continuation and indexed real events"],
            ["SDRIV3/DDRIV3","implemented","OdeSession<f32/f64>","owned explicit sessions with functional, internal dense/banded finite-difference, and checked analytic dense/banded JACOBN modes"],
            ["CDRIV1/CDRIV2","implemented","ComplexDriv1Session/ComplexDriv2Session","owned complex sessions with indexed real events"],
            ["CDRIV3","raw_only","","complex expert callback and optional-matrix modes remain unresolved"],
            ["DERKF/DDERKF/DEABM/DDEABM","raw_only","","no reviewed callback-abort protocol"],
            ["DEBDF/DDEBDF","raw_only","","COMMON-backed history prevents an independently safe session"],
            ["SDASSL/DDASSL","implemented_residual_only","DaeSession<f32/f64>","caller supplies a consistent initial pair; internal dense or banded finite-difference mode"],
            ["analytic_DASSL","deferred","","JAC(T,Y,YPRIME,PD,CJ,RPAR,IPAR) has no native termination channel for contained Rust panic, error, or non-finite output"]
        ]
    });
    let coordinate = json!({
        "schema_id":"slatec.safe-api.fishpack-coordinate-system-audit",
        "schema_version":"1.0.0",
        "records":records.iter().map(|record| json!([record["routine"],record["coordinate_system"],record["grid"],record["safe_status"]])).collect::<Vec<_>>()
    });
    let boundaries = json!({
        "schema_id":"slatec.safe-api.fishpack-boundary-contracts",
        "schema_version":"1.0.0",
        "records":[
            ["HWSCRT/HWSCYL/HWSPLR/HWSSSP","centered grids","private edge vectors; checked Dirichlet corners; periodic duplicate RHS samples"],
            ["HWSCYL","r=0","axis codes 5/6 require r.lower()==0 and lambda==0; radial derivatives at the axis are rejected"],
            ["HWSPLR","r=0","axis codes 5/6 require periodic or double-derivative angular boundaries"],
            ["HWSSSP","spherical poles","full-sphere constructors use the reviewed PIMACH value; source pole combinations are checked"],
            ["HWSCSP","axisymmetric spherical centered grid","theta-r nodes; pole modes use exact PIMACH endpoints, origin modes are source-checked, and R1MACH comes from reviewed profile support"],
            ["HSTCSP","axisymmetric spherical staggered grid","open-interval theta-r points; source-specific pole and origin combinations are preflight-checked"],
            ["HSTCYL/HSTPLR","staggered cylindrical and polar grids","interior unknowns are owned independently from source boundary data"],
            ["HSTSSP","staggered spherical surface grid","open-interval theta/phi unknowns with source-defined pole regularity modes"],
            ["HSTCRT","remaining staggered grids","raw only: M-by-N staggered interior and separate M/N edge vectors require a dedicated owned representation and raw-parity audit"],
            ["HW3CRT","remaining Cartesian 3D grids","raw only: full 3D field plus six face arrays with distinct two-dimensional layouts require a dedicated owned representation and raw-parity audit"]
        ]
    });
    let provider = json!({
        "schema_id":"slatec.safe-api.fishpack-provider-closure",
        "schema_version":"1.0.0",
        "records":[
            ["fishpack-cartesian-2d","HWSCRT",11,"existing focused closure"],
            ["fishpack-cylindrical-polar","HWSCYL/HWSPLR/HSTCYL/HSTPLR",16,"centered and staggered roots plus GENBUN/POISTG cyclic-reduction closures"],
            ["fishpack-spherical","HWSSSP/HSTSSP/HWSCSP/HSTCSP",43,"unit-sphere and axisymmetric roots plus GENBUN/POISTG, BLKTRI, and XERROR closures; reviewed GNU-MinGW machine profile support"],
            ["fishpack-pois3d","POIS3D",32,"existing structured-system closure"],
            ["fishpack-general","remaining raw drivers",80,"raw-provider coverage; not selected by a safe feature"]
        ]
    });
    let storage = json!({
        "schema_id":"slatec.safe-api.fishpack-runtime-storage-policy",
        "schema_version":"1.0.0",
        "records":[
            ["HWSCRT/HWSCYL/HWSPLR/HWSSSP/HWSCSP","caller-owned F and W; no safe workspace reuse","SerializedGlobal"],
            ["HSTCYL/HSTPLR/HSTSSP/HSTCSP","caller-owned interior F and W; source boundary arrays kept separate","SerializedGlobal"],
            ["POIS3D","caller-owned F and W; coefficient buffers owned by facade","SerializedGlobal"],
            ["INTL drivers","W may retain native initialization state","raw only pending non-cloneable session review"]
        ]
    });
    let wrapper_index = json!({
        "schema_id":"slatec.safe-api.fishpack-curvilinear.wrapper-index",
        "schema_version":"1.0.0",
        "raw_ffi_profile":"ffi-profile-gnu-mingw-x86_64",
        "columns":["safe_path","raw_routine","raw_symbol","precision","mathematical_model","storage_policy","runtime_policy","feature","documentation","example_file","example_case"],
        "records":[
            ["slatec::differential_equations::pde::CylindricalHelmholtz2d::solve","HWSCYL","hwscyl_","f32","centered cylindrical modified Helmholtz finite-difference solve","owned r-fast FishpackGrid2, edge buffers, and workspace","SerializedGlobal","fishpack-cylindrical-polar","struct.CylindricalHelmholtz2d.html#method.solve","examples/fishpack_cylindrical_polar.rs","manufactured constant curvilinear solution"],
            ["slatec::differential_equations::pde::PolarHelmholtz2d::solve","HWSPLR","hwsplr_","f32","centered polar Helmholtz finite-difference solve","owned r-fast FishpackGrid2, edge buffers, and workspace","SerializedGlobal","fishpack-cylindrical-polar","struct.PolarHelmholtz2d.html#method.solve","examples/fishpack_cylindrical_polar.rs","manufactured constant curvilinear solution"],
            ["slatec::differential_equations::pde::StaggeredCylindricalHelmholtz2d::solve","HSTCYL","hstcyl_","f32","staggered cylindrical modified Helmholtz finite-difference solve","owned interior FishpackGrid2, source boundary arrays, and workspace","SerializedGlobal","fishpack-cylindrical-polar","struct.StaggeredCylindricalHelmholtz2d.html#method.solve","examples/fishpack_cylindrical_polar.rs","manufactured constant staggered curvilinear solution"],
            ["slatec::differential_equations::pde::StaggeredPolarHelmholtz2d::solve","HSTPLR","hstplr_","f32","staggered polar Helmholtz finite-difference solve","owned interior FishpackGrid2, source boundary arrays, and workspace","SerializedGlobal","fishpack-cylindrical-polar","struct.StaggeredPolarHelmholtz2d.html#method.solve","examples/fishpack_cylindrical_polar.rs","manufactured constant staggered curvilinear solution"],
            ["slatec::differential_equations::pde::SphereSurfaceHelmholtz2d::solve","HWSSSP","hwsssp_","f32","centered unit-sphere surface Helmholtz finite-difference solve","owned theta-fast FishpackGrid2, edge buffers, and workspace","SerializedGlobal","fishpack-spherical","struct.SphereSurfaceHelmholtz2d.html#method.solve","examples/fishpack_spherical.rs","manufactured constant unit-sphere surface solution"],
            ["slatec::differential_equations::pde::StaggeredSphereSurfaceHelmholtz2d::solve","HSTSSP","hstssp_","f32","staggered unit-sphere surface Helmholtz finite-difference solve","owned interior theta-fast FishpackGrid2, source boundary arrays, and workspace","SerializedGlobal","fishpack-spherical","struct.StaggeredSphereSurfaceHelmholtz2d.html#method.solve","examples/fishpack_spherical.rs","manufactured constant staggered unit-sphere surface solution"]
            ,["slatec::differential_equations::pde::AxisymmetricSphericalHelmholtz2d::solve","HWSCSP","hwscsp_","f32","centered axisymmetric spherical Helmholtz finite-difference solve","owned theta-fast FishpackGrid2, edge buffers, and workspace","SerializedGlobal","fishpack-spherical","struct.AxisymmetricSphericalHelmholtz2d.html#method.solve","examples/fishpack_spherical.rs","manufactured constant axisymmetric spherical solution"]
            ,["slatec::differential_equations::pde::StaggeredAxisymmetricSphericalHelmholtz2d::solve","HSTCSP","hstcsp_","f32","staggered axisymmetric spherical Helmholtz finite-difference solve","owned interior theta-fast FishpackGrid2, source boundary arrays, and workspace","SerializedGlobal","fishpack-spherical","struct.StaggeredAxisymmetricSphericalHelmholtz2d.html#method.solve","examples/fishpack_spherical.rs","manufactured constant staggered axisymmetric spherical solution"]
        ]
    });
    fs::create_dir_all(output_dir)?;
    let files = [
        (
            "fishpack-complete-coverage.json",
            serde_json::to_vec(
                &json!({"schema_id":"slatec.safe-api.fishpack-complete-coverage","schema_version":"1.0.0","record_count":records.len(),"implemented_safe_count":implemented,"records":records}),
            )?,
        ),
        ("fishpack-complete-coverage.md", markdown.into_bytes()),
        (
            "fishpack-coordinate-system-audit.json",
            serde_json::to_vec(&coordinate)?,
        ),
        (
            "fishpack-boundary-contracts.json",
            serde_json::to_vec(&boundaries)?,
        ),
        (
            "fishpack-provider-closure.json",
            serde_json::to_vec(&provider)?,
        ),
        (
            "fishpack-runtime-storage-policy.json",
            serde_json::to_vec(&storage)?,
        ),
        (
            "fishpack-cylindrical-polar-wrapper-index.json",
            serde_json::to_vec(&wrapper_index)?,
        ),
        ("ode-dae-safe-coverage.json", serde_json::to_vec(&ode_dae)?),
    ];
    let mut bytes = Vec::new();
    for (name, content) in files {
        fs::write(output_dir.join(name), &content)?;
        bytes.extend_from_slice(&content);
    }
    Ok(ResultSummary {
        semantic_hash: hash::bytes(&bytes),
        fishpack_count: FISHPACK.len(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generation_is_deterministic_and_all_drivers_are_accounted_for() {
        let first = tempfile::tempdir().unwrap();
        let second = tempfile::tempdir().unwrap();
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let one = generate(&root, first.path()).unwrap();
        let two = generate(&root, second.path()).unwrap();
        assert_eq!(one.semantic_hash, two.semantic_hash);
        assert_eq!(one.fishpack_count, 17);
        assert_eq!(
            fs::read(first.path().join("fishpack-complete-coverage.json")).unwrap(),
            fs::read(second.path().join("fishpack-complete-coverage.json")).unwrap()
        );
    }
}
