/// Tests for Tailwind CSS v4 utility merging.
/// Covers new v4 utilities, renamed utilities, and syntax changes.
use tailwind_fuse::merge::tw_merge;

// === Inset Shadow (new in v4) ===

#[test]
fn inset_shadow_size() {
    assert_eq!(
        tw_merge("inset-shadow-sm inset-shadow-md"),
        "inset-shadow-md"
    );
    assert_eq!(
        tw_merge("inset-shadow-none inset-shadow-lg"),
        "inset-shadow-lg"
    );
}

#[test]
fn inset_shadow_color() {
    assert_eq!(
        tw_merge("inset-shadow-red-500 inset-shadow-blue-500"),
        "inset-shadow-blue-500"
    );
}

#[test]
fn inset_shadow_size_and_color_no_conflict() {
    assert_eq!(
        tw_merge("inset-shadow-sm inset-shadow-red-500"),
        "inset-shadow-sm inset-shadow-red-500"
    );
}

// === Inset Ring (new in v4) ===

#[test]
fn inset_ring_width() {
    assert_eq!(tw_merge("inset-ring inset-ring-2"), "inset-ring-2");
    assert_eq!(tw_merge("inset-ring-4 inset-ring"), "inset-ring");
}

#[test]
fn inset_ring_color() {
    assert_eq!(
        tw_merge("inset-ring-red-500 inset-ring-blue-500"),
        "inset-ring-blue-500"
    );
}

#[test]
fn inset_ring_width_and_color_no_conflict() {
    assert_eq!(
        tw_merge("inset-ring-2 inset-ring-red-500"),
        "inset-ring-2 inset-ring-red-500"
    );
}

// === Text Shadow (new in v4) ===

#[test]
fn text_shadow_size() {
    assert_eq!(tw_merge("text-shadow-sm text-shadow-lg"), "text-shadow-lg");
    assert_eq!(tw_merge("text-shadow text-shadow-none"), "text-shadow-none");
}

#[test]
fn text_shadow_color() {
    assert_eq!(
        tw_merge("text-shadow-red-500 text-shadow-blue-500"),
        "text-shadow-blue-500"
    );
}

#[test]
fn text_shadow_size_and_color_no_conflict() {
    assert_eq!(
        tw_merge("text-shadow-sm text-shadow-red-500"),
        "text-shadow-sm text-shadow-red-500"
    );
}

// === v4 Gradient Syntax ===

#[test]
fn bg_linear_gradient() {
    assert_eq!(tw_merge("bg-linear-to-r bg-linear-to-l"), "bg-linear-to-l");
    assert_eq!(tw_merge("bg-linear-to-t bg-linear-65"), "bg-linear-65");
}

#[test]
fn bg_radial_gradient() {
    assert_eq!(tw_merge("bg-radial bg-radial-at-t"), "bg-radial-at-t");
}

#[test]
fn bg_conic_gradient() {
    assert_eq!(tw_merge("bg-conic bg-conic-45"), "bg-conic-45");
}

#[test]
fn gradient_from_via_to_positions() {
    assert_eq!(tw_merge("from-0% from-10%"), "from-10%");
    assert_eq!(tw_merge("via-0% via-50%"), "via-50%");
    assert_eq!(tw_merge("to-50% to-100%"), "to-100%");
}

// === 3D Transforms (new in v4) ===

#[test]
fn rotate_3d() {
    assert_eq!(tw_merge("rotate-x-12 rotate-x-45"), "rotate-x-45");
    assert_eq!(tw_merge("rotate-y-12 rotate-y-45"), "rotate-y-45");
    assert_eq!(tw_merge("rotate-z-12 rotate-z-45"), "rotate-z-45");
}

#[test]
fn rotate_3d_axes_no_conflict() {
    assert_eq!(
        tw_merge("rotate-x-12 rotate-y-45"),
        "rotate-x-12 rotate-y-45"
    );
}

#[test]
fn scale_3d() {
    assert_eq!(tw_merge("scale-z-50 scale-z-100"), "scale-z-100");
}

#[test]
fn translate_z() {
    assert_eq!(tw_merge("translate-z-4 translate-z-8"), "translate-z-8");
}

#[test]
fn perspective() {
    assert_eq!(
        tw_merge("perspective-near perspective-far"),
        "perspective-far"
    );
    assert_eq!(
        tw_merge("perspective-100 perspective-200"),
        "perspective-200"
    );
}

#[test]
fn transform_style() {
    assert_eq!(tw_merge("transform-3d transform-flat"), "transform-flat");
}

#[test]
fn backface_visibility() {
    assert_eq!(
        tw_merge("backface-visible backface-hidden"),
        "backface-hidden"
    );
}

// === Logical Properties (new in v4) ===

#[test]
fn logical_padding() {
    assert_eq!(tw_merge("ps-4 ps-8"), "ps-8");
    assert_eq!(tw_merge("pe-4 pe-8"), "pe-8");
    assert_eq!(tw_merge("pbs-4 pbs-8"), "pbs-8");
    assert_eq!(tw_merge("pbe-4 pbe-8"), "pbe-8");
}

#[test]
fn logical_margin() {
    assert_eq!(tw_merge("mbs-4 mbs-8"), "mbs-8");
    assert_eq!(tw_merge("mbe-4 mbe-8"), "mbe-8");
}

#[test]
fn logical_inset() {
    assert_eq!(tw_merge("inset-s-4 inset-s-8"), "inset-s-8");
    assert_eq!(tw_merge("inset-e-4 inset-e-8"), "inset-e-8");
    assert_eq!(tw_merge("inset-bs-4 inset-bs-8"), "inset-bs-8");
    assert_eq!(tw_merge("inset-be-4 inset-be-8"), "inset-be-8");
}

#[test]
fn logical_sizing() {
    assert_eq!(tw_merge("inline-auto inline-full"), "inline-full");
    assert_eq!(tw_merge("block-auto block-full"), "block-full");
    assert_eq!(tw_merge("min-inline-0 min-inline-full"), "min-inline-full");
    assert_eq!(tw_merge("max-inline-sm max-inline-md"), "max-inline-md");
    assert_eq!(tw_merge("min-block-0 min-block-full"), "min-block-full");
    assert_eq!(tw_merge("max-block-sm max-block-md"), "max-block-md");
}

#[test]
fn logical_border_width() {
    assert_eq!(tw_merge("border-bs border-bs-2"), "border-bs-2");
    assert_eq!(tw_merge("border-be border-be-4"), "border-be-4");
}

#[test]
fn logical_border_color() {
    assert_eq!(
        tw_merge("border-bs-red-500 border-bs-blue-500"),
        "border-bs-blue-500"
    );
    assert_eq!(
        tw_merge("border-be-red-500 border-be-blue-500"),
        "border-be-blue-500"
    );
}

// === New Standalone Utilities (v4) ===

#[test]
fn field_sizing() {
    assert_eq!(
        tw_merge("field-sizing-content field-sizing-fixed"),
        "field-sizing-fixed"
    );
}

#[test]
fn color_scheme() {
    assert_eq!(tw_merge("scheme-light scheme-dark"), "scheme-dark");
    assert_eq!(
        tw_merge("scheme-light-dark scheme-only-dark"),
        "scheme-only-dark"
    );
}

#[test]
fn wrap_text() {
    assert_eq!(tw_merge("wrap-normal wrap-anywhere"), "wrap-anywhere");
    assert_eq!(tw_merge("wrap-break-word wrap-normal"), "wrap-normal");
}

// === Shadow Scale (v4 renames) ===

#[test]
fn shadow_v4_scale() {
    // v4 adds shadow-2xs, which should merge with other shadow sizes
    assert_eq!(tw_merge("shadow-sm shadow-2xs"), "shadow-2xs");
    assert_eq!(tw_merge("shadow-xs shadow-sm"), "shadow-sm");
}

// === Suffix Important ===

#[test]
fn suffix_important_merges() {
    assert_eq!(tw_merge("p-4! p-8!"), "p-8!");
    assert_eq!(tw_merge("!p-4 p-8!"), "p-8!");
    assert_eq!(tw_merge("p-4! !p-8"), "!p-8");
}

// === Parenthesized Arbitrary Values ===

#[test]
fn paren_arbitrary_values() {
    assert_eq!(
        tw_merge("w-(--my-width) w-(--other-width)"),
        "w-(--other-width)"
    );
    assert_eq!(tw_merge("p-4 p-(--my-padding)"), "p-(--my-padding)");
}

// === Container Query Variants ===

#[test]
fn container_query_variants() {
    assert_eq!(tw_merge("@sm:flex @sm:block"), "@sm:block");
    assert_eq!(tw_merge("@sm:flex @md:flex"), "@sm:flex @md:flex");
    assert_eq!(tw_merge("@[500px]:p-4 @[500px]:p-8"), "@[500px]:p-8");
}

// === Star Variants ===

#[test]
fn star_variants() {
    assert_eq!(tw_merge("*:p-4 *:p-8"), "*:p-8");
    assert_eq!(
        tw_merge("**:text-red-500 **:text-blue-500"),
        "**:text-blue-500"
    );
    assert_eq!(tw_merge("*:p-4 **:p-4"), "*:p-4 **:p-4");
}

// === Drop Shadow Color (separate from drop-shadow size in v4) ===

#[test]
fn drop_shadow_size_and_color() {
    assert_eq!(tw_merge("drop-shadow-sm drop-shadow-lg"), "drop-shadow-lg");
    assert_eq!(
        tw_merge("drop-shadow-red-500 drop-shadow-blue-500"),
        "drop-shadow-blue-500"
    );
    assert_eq!(
        tw_merge("drop-shadow-sm drop-shadow-red-500"),
        "drop-shadow-sm drop-shadow-red-500"
    );
}

// === Placeholder Color ===

#[test]
fn placeholder_color() {
    assert_eq!(
        tw_merge("placeholder-red-500 placeholder-blue-500"),
        "placeholder-blue-500"
    );
}

// === via-none ===

#[test]
fn via_none() {
    assert_eq!(tw_merge("via-red-500 via-none"), "via-none");
}
