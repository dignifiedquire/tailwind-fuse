use std::collections::HashSet;

use crate::ast::AstStyle;

use super::{CollisionIdFn, GetCollisionsFn, MergeOptions};
use crate::core::merge::get_collisions::get_collisions;

/// Modifiers where position relative to other modifiers changes the generated CSS.
/// e.g., `*:hover:block` (hover on direct children) ≠ `hover:*:block` (children of hovered element).
/// These must NOT be reordered during variant normalization.
const ORDER_SENSITIVE_MODIFIERS: &[&str] = &[
    "*",
    "**",
    "after",
    "backdrop",
    "before",
    "details-content",
    "file",
    "first-letter",
    "first-line",
    "marker",
    "placeholder",
    "selection",
];

/// Sort variants for collision detection, preserving the relative position of
/// order-sensitive modifiers while sorting the rest alphabetically.
fn sort_variants_for_collision<'a>(variants: &[&'a str]) -> Vec<&'a str> {
    let mut result = variants.to_vec();

    // Collect indices and values of non-order-sensitive variants
    let mut sortable_indices: Vec<usize> = Vec::new();
    let mut sortable_values: Vec<&str> = Vec::new();

    for (i, v) in result.iter().enumerate() {
        if !ORDER_SENSITIVE_MODIFIERS.contains(v) {
            sortable_indices.push(i);
            sortable_values.push(v);
        }
    }

    sortable_values.sort_unstable();

    for (idx, val) in sortable_indices.iter().zip(sortable_values.iter()) {
        result[*idx] = val;
    }

    result
}

/// Merges all the Tailwind classes, resolving conflicts.
/// Can supply custom options, collision_id_fn and collisions_fn.
pub fn tw_merge_override(
    class: &[&str],
    options: MergeOptions,
    collision_id_fn: impl CollisionIdFn,
    collisions_fn: impl GetCollisionsFn,
) -> String {
    let styles: Vec<Result<AstStyle, &str>> = crate::ast::parse_tailwind(class, options.into());

    let mut valid_styles: Vec<Result<AstStyle, &str>> = vec![];
    let mut collision_styles: HashSet<Collision> = HashSet::new();

    for style in styles.into_iter().rev() {
        let style = match style {
            Ok(style) => style,
            Err(s) => {
                valid_styles.push(Err(s));
                continue;
            }
        };

        let elements = style.elements.as_slice();
        let result = collision_id_fn
            .apply(elements, style.arbitrary)
            .map(Ok)
            .unwrap_or_else(|| {
                let arbitrary = style.arbitrary.unwrap_or_default();
                super::get_collision_id::get_collision_id(elements, arbitrary)
            });

        match result {
            Err(error) => match Collision::check_arbitrary(style.clone()) {
                Some(collision) => {
                    if collision_styles.contains(&collision) {
                        continue;
                    }
                    collision_styles.insert(collision);
                }
                None => {
                    #[cfg(feature = "debug")]
                    println!("No Instance found: {style:?} {error:?}");
                    let _ = error;
                }
            },
            Ok(collision_id) => {
                // Sort non-order-sensitive variants so that ordering doesn't matter
                // for conflict detection. e.g., hover:focus:block ≡ focus:hover:block.
                // Order-sensitive modifiers (*, **, after, before, etc.) keep their
                // positions since reordering them changes the generated CSS.
                let all_variants = sort_variants_for_collision(&style.variants);

                let collision = Collision {
                    important: style.important,
                    variants: all_variants.clone(),
                    collision_id,
                };

                if collision_styles.contains(&collision) {
                    continue;
                }

                // Add the current collision_id.
                collision_styles.insert(collision);

                let collisions = collisions_fn
                    .apply(collision_id)
                    .or_else(|| get_collisions(collision_id));

                if let Some(collisions) = collisions {
                    collisions.into_iter().for_each(|collision_id| {
                        let collision = Collision {
                            important: style.important,
                            variants: all_variants.clone(),
                            collision_id,
                        };

                        collision_styles.insert(collision);
                    });
                }
            }
        }
        valid_styles.push(Ok(style));
    }

    valid_styles.reverse();

    valid_styles
        .into_iter()
        .map(|s| match s {
            Ok(style) => style.source,
            Err(s) => s,
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Collision<'a> {
    important: bool,
    variants: Vec<&'a str>,
    collision_id: &'a str,
}

// For [color:blue] => label = "color"
impl<'a> Collision<'a> {
    fn check_arbitrary(style: AstStyle<'a>) -> Option<Self> {
        let arbitrary = style.arbitrary?;
        let index = arbitrary.find(':')?;
        let (collision_id, _) = arbitrary.split_at(index);
        let variants = sort_variants_for_collision(&style.variants);
        Some(Self {
            collision_id,
            important: style.important,
            variants,
        })
    }
}

#[test]
fn check_arbitrary() {
    let style = crate::ast::parse_tailwind(&["[color:blue]"], Default::default())
        .into_iter()
        .next()
        .unwrap()
        .unwrap();

    assert_eq!(
        Collision::check_arbitrary(style),
        Some(Collision {
            important: false,
            variants: vec![],
            collision_id: "color"
        })
    );
}
