/// Tests for v4 cross-group conflict resolution.
/// Shorthand classes should override longhand classes in the same category.
use tailwind_fuse::merge::tw_merge;

// === Padding shorthand overrides logical padding ===

#[test]
fn padding_overrides_logical_padding() {
    // p-4 is shorthand, should override ps/pe/pbs/pbe
    assert_eq!(tw_merge("ps-4 p-4"), "p-4");
    assert_eq!(tw_merge("pe-4 p-4"), "p-4");
    assert_eq!(tw_merge("pbs-4 p-4"), "p-4");
    assert_eq!(tw_merge("pbe-4 p-4"), "p-4");
    // But refinement is allowed
    assert_eq!(tw_merge("p-4 ps-2"), "p-4 ps-2");
}

// === Margin shorthand overrides logical margin ===

#[test]
fn margin_overrides_logical_margin() {
    assert_eq!(tw_merge("mbs-4 m-4"), "m-4");
    assert_eq!(tw_merge("mbe-4 m-4"), "m-4");
    assert_eq!(tw_merge("m-4 mbs-2"), "m-4 mbs-2");
}

// === Inset shorthand overrides logical inset ===

#[test]
fn inset_overrides_logical_inset() {
    assert_eq!(tw_merge("inset-s-4 inset-4"), "inset-4");
    assert_eq!(tw_merge("inset-e-4 inset-4"), "inset-4");
    assert_eq!(tw_merge("inset-bs-4 inset-4"), "inset-4");
    assert_eq!(tw_merge("inset-be-4 inset-4"), "inset-4");
    assert_eq!(tw_merge("inset-4 inset-s-2"), "inset-4 inset-s-2");
}

// === Border width shorthand overrides logical border width ===

#[test]
fn border_w_overrides_logical_border_w() {
    assert_eq!(tw_merge("border-bs-2 border-2"), "border-2");
    assert_eq!(tw_merge("border-be-2 border-2"), "border-2");
    assert_eq!(tw_merge("border-2 border-bs-4"), "border-2 border-bs-4");
}

// === Border color shorthand overrides logical border color ===

#[test]
fn border_color_overrides_logical_border_color() {
    assert_eq!(
        tw_merge("border-bs-red-500 border-red-500"),
        "border-red-500"
    );
    assert_eq!(
        tw_merge("border-be-red-500 border-red-500"),
        "border-red-500"
    );
}

// === Scale overrides scale-z ===

#[test]
fn scale_overrides_scale_z() {
    assert_eq!(tw_merge("scale-z-50 scale-100"), "scale-100");
    assert_eq!(tw_merge("scale-100 scale-z-50"), "scale-100 scale-z-50");
}
