/// Tests that verify tailwind-fuse can handle all real Tailwind CSS v4 classes.
///
/// Uses the generated fixture files from `scripts/generate-class-list.mjs`:
/// - `fixtures/tw4-classes.txt`: all 22,685 TW v4 class names
/// - `fixtures/tw4-collision-groups.json`: classes grouped by CSS property signature
use std::collections::HashMap;
use tailwind_fuse::merge::tw_merge;

const CLASS_LIST: &str = include_str!("../../fixtures/tw4-classes.txt");
const COLLISION_GROUPS: &str = include_str!("../../fixtures/tw4-collision-groups.json");

fn all_classes() -> Vec<&'static str> {
    CLASS_LIST.lines().filter(|l| !l.is_empty()).collect()
}

fn collision_groups() -> HashMap<String, Vec<String>> {
    serde_json::from_str(COLLISION_GROUPS).unwrap()
}

/// Every single TW v4 class can be passed through tw_merge without panicking.
#[test]
fn every_class_parses_without_panic() {
    let classes = all_classes();
    assert!(classes.len() > 22000, "fixture should have 22k+ classes");
    for class in &classes {
        let _ = tw_merge(class);
    }
}

/// Merging a class with itself should produce exactly that class.
/// Classes without collision ID mappings won't dedup (they pass through as unknown).
/// This test guards against regressions — the success rate should only go up.
#[test]
fn self_dedup_for_all_classes() {
    let classes = all_classes();
    let mut failures = Vec::new();
    for class in &classes {
        let input = format!("{class} {class}");
        let result = tw_merge(&input);
        if result != *class {
            failures.push(format!(
                "  tw_merge(\"{input}\") = \"{result}\", expected \"{class}\""
            ));
        }
    }
    let total = classes.len();
    let success = total - failures.len();
    let success_rate = success as f64 / total as f64 * 100.0;
    eprintln!(
        "Self-dedup coverage: {success}/{total} ({success_rate:.1}%) — {} unmapped",
        failures.len()
    );
    // All TW v4 classes should be mapped.
    assert!(
        failures.is_empty(),
        "Self-dedup failed for {} / {total} classes ({success_rate:.1}%). Failures:\n{}",
        failures.len(),
        failures[..failures.len().min(50)].join("\n")
    );
}

/// Classes within the same collision group should merge (later wins).
/// Picks deterministic random pairs from each group.
#[test]
fn same_group_pairs_merge() {
    let groups = collision_groups();
    let mut rng = fastrand::Rng::with_seed(42);
    let mut failures = Vec::new();

    // The fixture groups classes by CSS property signature, which is sometimes
    // coarser than tailwind-merge's semantic grouping. These groups intentionally
    // contain classes from different utility families that share a CSS variable
    // but should NOT conflict (e.g., border-style vs divide-style both set
    // --tw-border-style but are separate collision groups in tailwind-merge).
    let skip_groups: &[&str] = &[
        "--tw-border-style",     // border-style and divide-style are separate groups
        "--tw-divide-x-reverse", // divide-x-reverse is complementary to divide-x-*, not a conflict
        "--tw-divide-y-reverse", // divide-y-reverse is complementary to divide-y-*, not a conflict
        "--tw-space-x-reverse",  // space-x-reverse is complementary to space-x-*, not a conflict
        "--tw-space-y-reverse",  // space-y-reverse is complementary to space-y-*, not a conflict
    ];

    for (group_key, members) in &groups {
        if members.len() < 2 || skip_groups.contains(&group_key.as_str()) {
            continue;
        }
        // Test up to 5 random pairs per group
        for _ in 0..5.min(members.len()) {
            let i = rng.usize(..members.len());
            let j = rng.usize(..members.len());
            if i == j {
                continue;
            }
            let a = &members[i];
            let b = &members[j];
            let input = format!("{a} {b}");
            let result = tw_merge(&input);

            // The result should contain class b (the later one)
            // and should NOT contain class a (the earlier one, overridden)
            // Use word-boundary matching to avoid substring false positives
            // (e.g., "shadow-lg" contains "shadow" as substring but shadow was correctly overridden)
            let result_classes: Vec<&str> = result.split_whitespace().collect();
            if result_classes.contains(&a.as_str()) && a != b {
                failures.push(format!(
                    "  group={group_key}: tw_merge(\"{input}\") = \"{result}\" — expected \"{a}\" to be overridden by \"{b}\""
                ));
            }
        }
    }

    if !failures.is_empty() {
        // Print all failures but don't fail the test yet — this is aspirational
        // coverage that documents gaps in collision ID mapping.
        eprintln!(
            "Same-group merge gaps ({} failures across {} groups):\n{}",
            failures.len(),
            groups.len(),
            failures.join("\n")
        );
    }
    let total_tested: usize = groups
        .values()
        .filter(|m| m.len() >= 2)
        .map(|m| 5.min(m.len()))
        .sum();
    let success = total_tested - failures.len();
    let success_rate = success as f64 / total_tested as f64 * 100.0;
    eprintln!("Same-group merge coverage: {success}/{total_tested} ({success_rate:.1}%)");
    assert!(
        failures.is_empty(),
        "Same-group merge failed for {} / {total_tested} pairs ({success_rate:.1}%). Failures:\n{}",
        failures.len(),
        failures[..failures.len().min(30)].join("\n")
    );
}

/// Classes from different collision groups should both be preserved.
/// Picks deterministic random pairs across groups.
#[test]
fn cross_group_pairs_coexist() {
    let groups = collision_groups();
    let group_keys: Vec<&String> = groups.keys().collect();
    let mut rng = fastrand::Rng::with_seed(123);
    let mut failures = Vec::new();

    // Test 200 random cross-group pairs
    for _ in 0..200 {
        let gi = rng.usize(..group_keys.len());
        let gj = rng.usize(..group_keys.len());
        if gi == gj {
            continue;
        }
        let group_a = &groups[group_keys[gi]];
        let group_b = &groups[group_keys[gj]];
        let a = &group_a[rng.usize(..group_a.len())];
        let b = &group_b[rng.usize(..group_b.len())];

        let input = format!("{a} {b}");
        let result = tw_merge(&input);

        // Both classes should be preserved (no conflict across groups)
        // unless there's a collision relationship (shorthand overrides longhand)
        let result_classes: Vec<&str> = result.split_whitespace().collect();
        let has_both = result_classes.contains(&a.as_str()) && result_classes.contains(&b.as_str());
        if !has_both {
            // If b (later class) is kept and a was dropped, this is a legitimate
            // shorthand→longhand override (e.g., skew overrides skew-y).
            if result_classes.contains(&b.as_str()) {
                continue;
            }
            // b (later class) was dropped — check reverse to distinguish real
            // bidirectional conflicts from bugs.
            let reverse_input = format!("{b} {a}");
            let reverse_result = tw_merge(&reverse_input);
            let reverse_classes: Vec<&str> = reverse_result.split_whitespace().collect();
            let reverse_has_both =
                reverse_classes.contains(&a.as_str()) && reverse_classes.contains(&b.as_str());
            if reverse_has_both {
                // Only the forward direction dropped the later class — likely a bug
                failures.push(format!(
                    "  {}/{}: tw_merge(\"{input}\") = \"{result}\"",
                    group_keys[gi], group_keys[gj]
                ));
            }
        }
    }

    if !failures.is_empty() {
        eprintln!(
            "Cross-group coexistence issues ({} / 200):\n{}",
            failures.len(),
            failures[..failures.len().min(20)].join("\n")
        );
    }
    // Cross-group pairs should coexist.
    assert!(
        failures.is_empty(),
        "Cross-group coexistence failed for {} / 200 pairs. Failures:\n{}",
        failures.len(),
        failures[..failures.len().min(20)].join("\n")
    );
}

/// Random combinations of N classes (3-10) can be merged without panicking.
/// Uses deterministic seed for reproducibility.
#[test]
fn random_combinations_no_panic() {
    let classes = all_classes();
    let mut rng = fastrand::Rng::with_seed(999);

    for _ in 0..500 {
        let n = rng.usize(3..=10);
        let combo: Vec<&str> = (0..n)
            .map(|_| classes[rng.usize(..classes.len())])
            .collect();
        let input = combo.join(" ");
        let _ = tw_merge(&input);
    }
}

/// Random combinations with variants applied should not panic.
#[test]
fn random_combinations_with_variants_no_panic() {
    let classes = all_classes();
    let mut rng = fastrand::Rng::with_seed(777);
    let variants = [
        "hover:",
        "focus:",
        "dark:",
        "sm:",
        "md:",
        "lg:",
        "xl:",
        "first:",
        "last:",
        "disabled:",
        "active:",
        "group-hover:",
        "*:",
        "**:",
    ];

    for _ in 0..500 {
        let n = rng.usize(2..=6);
        let combo: Vec<String> = (0..n)
            .map(|_| {
                let class = classes[rng.usize(..classes.len())];
                if rng.bool() {
                    let variant = variants[rng.usize(..variants.len())];
                    format!("{variant}{class}")
                } else {
                    class.to_string()
                }
            })
            .collect();
        let input = combo.join(" ");
        let _ = tw_merge(&input);
    }
}

/// Merging all classes in each collision group at once should produce
/// exactly the last class (since all conflict with each other).
#[test]
fn full_group_merge_keeps_last() {
    let groups = collision_groups();
    let mut failures = Vec::new();

    for (group_key, members) in &groups {
        if members.len() < 2 {
            continue;
        }
        // Take first 20 members to keep test time reasonable
        let subset: Vec<&str> = members.iter().take(20).map(|s| s.as_str()).collect();
        let input = subset.join(" ");
        let result = tw_merge(&input);
        let last = subset.last().unwrap();

        // The result should at minimum contain the last class
        let result_classes: Vec<&str> = result.split_whitespace().collect();
        if !result_classes.contains(last) {
            failures.push(format!(
                "  group={group_key}: last class \"{last}\" missing from result \"{result}\""
            ));
        }
    }

    if !failures.is_empty() {
        eprintln!(
            "Full-group merge issues ({} / {} groups):\n{}",
            failures.len(),
            groups.len(),
            failures[..failures.len().min(20)].join("\n")
        );
    }
    let total_groups = groups.values().filter(|m| m.len() >= 2).count();
    assert!(
        failures.is_empty(),
        "Full-group merge failed for {} / {} groups. Failures:\n{}",
        failures.len(),
        total_groups,
        failures[..failures.len().min(20)].join("\n")
    );
}
