// Unlicense — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
//! f148=wowasticker_cli. CLI entry point: demo, govdocs, --sbom.
//! The binary IS the compliance artifact.

#![allow(non_camel_case_types, non_snake_case)]

use std::sync::Arc;
use wowasticker::db::{t119, t123};
use wowasticker::report;

// Bake govdocs into the binary
const SBOM: &str = include_str!("../../govdocs/SBOM.md");
const SECURITY: &str = include_str!("../../govdocs/SECURITY.md");
const PRIVACY: &str = include_str!("../../govdocs/PRIVACY.md");
const SSDF: &str = include_str!("../../govdocs/SSDF.md");
const SUPPLY_CHAIN: &str = include_str!("../../govdocs/SUPPLY_CHAIN.md");
const ACCESSIBILITY: &str = include_str!("../../govdocs/ACCESSIBILITY.md");
const FIPS: &str = include_str!("../../govdocs/FIPS.md");
const FEDRAMP: &str = include_str!("../../govdocs/FedRAMP_NOTES.md");
const CMMC: &str = include_str!("../../govdocs/CMMC.md");
const ITAR_EAR: &str = include_str!("../../govdocs/ITAR_EAR.md");
const FEDERAL_USE: &str = include_str!("../../govdocs/FEDERAL_USE_CASES.md");
const CARGO_TOML: &str = include_str!("../../Cargo.toml");

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "--help" | "-h" => print_help(),
        "--sbom" => print_spdx_sbom(),
        "--version" | "-V" => println!("wowasticker-cli {}", env!("CARGO_PKG_VERSION")),
        "demo" => run_demo(),
        "govdocs" => cmd_govdocs(&args[2..]),
        other => {
            eprintln!("Unknown command: {}", other);
            print_help();
            std::process::exit(1);
        }
    }
}

fn print_help() {
    println!(
        "wowasticker-cli {} — offline behavioral sticker chart\n\
         \n\
         USAGE:\n\
         \x20 wowasticker-cli demo              Run full sticker workflow demo\n\
         \x20 wowasticker-cli govdocs [DOC]      Print compliance docs (sbom, security, privacy, ...)\n\
         \x20 wowasticker-cli --sbom             Machine-readable SPDX SBOM\n\
         \x20 wowasticker-cli --version          Print version\n\
         \n\
         GOVDOCS:\n\
         \x20 sbom, security, privacy, ssdf, supply-chain, accessibility,\n\
         \x20 fips, fedramp, cmmc, itar-ear, federal-use\n\
         \x20 (no argument lists all)",
        env!("CARGO_PKG_VERSION")
    );
}

fn cmd_govdocs(args: &[String]) {
    if args.is_empty() {
        // List all
        for (name, _) in govdoc_list() {
            println!("  {}", name);
        }
        println!("\nUse: wowasticker-cli govdocs <name>");
        return;
    }
    let name = args[0].as_str();
    for (key, content) in govdoc_list() {
        if key == name {
            println!("{}", content);
            return;
        }
    }
    eprintln!("Unknown govdoc: {}. Run 'wowasticker-cli govdocs' to list.", name);
    std::process::exit(1);
}

fn govdoc_list() -> Vec<(&'static str, &'static str)> {
    vec![
        ("sbom", SBOM),
        ("security", SECURITY),
        ("privacy", PRIVACY),
        ("ssdf", SSDF),
        ("supply-chain", SUPPLY_CHAIN),
        ("accessibility", ACCESSIBILITY),
        ("fips", FIPS),
        ("fedramp", FEDRAMP),
        ("cmmc", CMMC),
        ("itar-ear", ITAR_EAR),
        ("federal-use", FEDERAL_USE),
    ]
}

/// Machine-readable SPDX SBOM parsed live from baked Cargo.toml
fn print_spdx_sbom() {
    println!("SPDXVersion: SPDX-2.3");
    println!("DataLicense: CC0-1.0");
    println!("SPDXID: SPDXRef-DOCUMENT");
    println!("DocumentName: wowasticker-sbom");
    println!(
        "DocumentNamespace: https://github.com/cochranblock/wowasticker/sbom/{}",
        env!("CARGO_PKG_VERSION")
    );
    println!("Creator: Tool: wowasticker-cli-{}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("PackageName: wowasticker");
    println!("SPDXID: SPDXRef-Package");
    println!("PackageVersion: {}", env!("CARGO_PKG_VERSION"));
    println!("PackageDownloadLocation: https://github.com/cochranblock/wowasticker");
    println!("PackageLicenseDeclared: Unlicense");
    println!();

    // Parse deps from baked Cargo.toml
    let mut in_deps = false;
    let mut idx = 0;
    for line in CARGO_TOML.lines() {
        if line.starts_with("[dependencies]") {
            in_deps = true;
            continue;
        }
        if line.starts_with('[') && in_deps {
            in_deps = false;
            continue;
        }
        if !in_deps || line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        // Parse: name = { version = "X.Y", ... } or name = "X.Y"
        if let Some(name) = line.split('=').next().map(|s| s.trim()) {
            let version = extract_version(line);
            let optional = line.contains("optional = true");
            idx += 1;
            println!("PackageName: {}", name);
            println!("SPDXID: SPDXRef-Dep-{}", idx);
            println!("PackageVersion: {}", version);
            if optional {
                println!("PackageComment: optional");
            }
            println!("Relationship: SPDXRef-Package DEPENDS_ON SPDXRef-Dep-{}", idx);
            println!();
        }
    }
}

fn extract_version(line: &str) -> String {
    // version = "X.Y" or { version = "X.Y", ... }
    if let Some(start) = line.find("version") {
        let rest = &line[start..];
        if let Some(q1) = rest.find('"') {
            let after = &rest[q1 + 1..];
            if let Some(q2) = after.find('"') {
                return after[..q2].to_string();
            }
        }
    }
    // Simple: name = "version"
    if let Some(q1) = line.find('"') {
        let after = &line[q1 + 1..];
        if let Some(q2) = after.find('"') {
            return after[..q2].to_string();
        }
    }
    "unknown".to_string()
}

/// Full demo: create student → create blocks → set stickers → generate report
fn run_demo() {
    println!("=== WowaSticker Demo ===\n");

    // 1. Open DB in temp dir
    let dir = std::env::temp_dir().join("wowasticker-demo");
    let _ = std::fs::create_dir_all(&dir);
    let db_path = dir.join("demo.db");
    println!("DB: {}", db_path.display());

    let db = Arc::new(t123::f121(&db_path).expect("open db"));

    // 2. Create student and schedule
    db.f140().expect("create student");
    db.f123().expect("create schedule");

    let student = db.f141().expect("get student").expect("student exists");
    let blocks = db.f124().expect("list blocks");

    println!("Student: {} (goal: {} stickers)", student.s7, student.s8);
    println!("Schedule: {}\n", blocks.iter().map(|b| b.s1.as_str()).collect::<Vec<_>>().join(", "));

    // 3. Score stickers — simulate a teacher tapping scores
    let scores = [
        (0, t119::Two, "Great focus during art"),
        (1, t119::One, "Needed one reminder"),
        (2, t119::Two, "Excellent math work"),
        (3, t119::One, "Ok at recess"),
        (4, t119::Zero, "Elopement during lunch"),
    ];

    println!("--- Scoring ---");
    for (idx, value, note) in &scores {
        let block = &blocks[*idx];
        db.f135(block.s0, *value, Some(note)).expect("set sticker");
        let label = match value {
            t119::Zero => "0 (concern)",
            t119::One => "1 (good)",
            t119::Two => "2 (great)",
        };
        println!("  {}: {} — {}", block.s1, label, note);
    }

    // 4. Check progress
    let earned = db.f142().expect("count");
    println!("\nProgress: {} / {} stickers", earned, student.s8);
    if earned >= student.s8 {
        println!("Goal met!");
    }

    // 5. Generate daily report
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let records = db.f144(&today).expect("day records");
    let text = report::f147(&student, &today, &records, earned);

    println!("\n--- Daily Report (shareable) ---\n");
    println!("{}", text);

    // 6. Undo last entry
    let last_block = &blocks[4];
    db.f146(last_block.s0, &today).expect("undo");
    let earned_after = db.f142().expect("count after undo");
    println!("--- Undo: removed {} ---", last_block.s1);
    println!("Progress after undo: {} / {} stickers", earned_after, student.s8);

    // 7. Browse history
    let yesterday = (chrono::Local::now() - chrono::Duration::days(1))
        .format("%Y-%m-%d")
        .to_string();
    let yesterday_earned = db.f145(&yesterday).expect("yesterday count");
    println!("\nYesterday ({}): {} stickers", yesterday, yesterday_earned);

    // Cleanup
    let _ = std::fs::remove_dir_all(&dir);
    println!("\n=== Demo complete. Temp DB cleaned up. ===");
}
