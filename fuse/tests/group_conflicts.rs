use tailwind_fuse::merge::tw_merge;
use tailwind_fuse::tw_merge;

#[test]
fn test_tw_merge_border_width_color() {
    let all_sides = tw_merge!("border-2", "border-blue-500");
    assert_eq!(all_sides, "border-2 border-blue-500");

    let arbitrary = tw_merge!("border-[100px]", "border-blue-500");
    assert_eq!(arbitrary, "border-[100px] border-blue-500");
}

#[test]
fn test_tw_merge_mixed_blend() {
    let classes = tw_merge!("mix-blend-normal", "mix-blend-multiply");
    assert_eq!(classes, "mix-blend-multiply");
}

#[test]
fn test_tw_merge_height() {
    let classes = tw_merge!("h-10", "h-min");
    assert_eq!(classes, "h-min");
}

#[test]
fn test_tw_merge_stroke() {
    let classes = tw_merge!("stroke-black", "stroke-1");
    assert_eq!(classes, "stroke-black stroke-1");

    let classes = tw_merge!("stroke-2", "stroke-[3px]");
    assert_eq!(classes, "stroke-[3px]");

    let classes = tw_merge!("stroke-black", "stroke-red-500", "stroke-blue-100");
    assert_eq!(classes, "stroke-blue-100");
}

#[test]
fn test_tw_merge_outline() {
    let classes = tw_merge!("outline-black", "outline-1");
    assert_eq!(classes, "outline-black outline-1");
}

#[test]
fn test_tw_merge_grayscale() {
    let classes = tw_merge!("grayscale-0", "grayscale-[50%]");
    assert_eq!(classes, "grayscale-[50%]");
}

#[test]
fn test_padding_narrowing() {
    let classes = tw_merge!("p-10", "px-5");
    assert_eq!(classes, "p-10 px-5");
    let classes = tw_merge!("px-5", "py-5", "p-10",);
    assert_eq!(classes, "p-10");
}

#[test]
fn test_gap_narrowing() {
    let classes = tw_merge!("gap-10", "gap-x-5");
    assert_eq!(classes, "gap-10 gap-x-5");

    let classes = tw_merge!("gap-x-5", "gap-y-5", "gap-10");
    assert_eq!(classes, "gap-10");
}

#[test]
fn merges_classes_from_same_group_correctly() {
    let class = "overflow-x-auto overflow-x-hidden";
    let result = tw_merge(class);
    assert_eq!(result, "overflow-x-hidden");

    let class = "basis-full basis-auto";
    let result = tw_merge(class);
    assert_eq!(result, "basis-auto");

    let class = "w-full w-fit";
    let result = tw_merge(class);
    assert_eq!(result, "w-fit");

    let class = "overflow-x-auto overflow-x-hidden overflow-x-scroll";
    let result = tw_merge(class);
    assert_eq!(result, "overflow-x-scroll");

    let class = "overflow-x-auto hover:overflow-x-hidden overflow-x-scroll";
    let result = tw_merge(class);
    assert_eq!(result, "hover:overflow-x-hidden overflow-x-scroll");

    let class = "overflow-x-auto hover:overflow-x-hidden hover:overflow-x-auto overflow-x-scroll";
    let result = tw_merge(class);
    assert_eq!(result, "hover:overflow-x-auto overflow-x-scroll");

    let class = "col-span-1 col-span-full";
    let result = tw_merge(class);
    assert_eq!(result, "col-span-full");
}

#[test]
fn merges_classes_from_font_variant_numeric_section_correctly() {
    let class = "lining-nums tabular-nums diagonal-fractions";
    let result = tw_merge(class);
    assert_eq!(result, "lining-nums tabular-nums diagonal-fractions");

    let class = "normal-nums tabular-nums diagonal-fractions";
    let result = tw_merge(class);
    assert_eq!(result, "tabular-nums diagonal-fractions");

    let class = "tabular-nums diagonal-fractions normal-nums";
    let result = tw_merge(class);
    assert_eq!(result, "normal-nums");

    let class = "tabular-nums proportional-nums";
    let result = tw_merge(class);
    assert_eq!(result, "proportional-nums");
}

#[test]
fn handles_color_conflicts_properly() {
    let class = "bg-grey-5 bg-hotpink";
    let result = tw_merge(class);
    assert_eq!(result, "bg-hotpink");

    let class = "hover:bg-grey-5 hover:bg-hotpink";
    let result = tw_merge(class);
    assert_eq!(result, "hover:bg-hotpink");

    let class = "stroke-[hsl(350_80%_0%)] stroke-[10px]";
    let result = tw_merge(class);
    assert_eq!(result, "stroke-[hsl(350_80%_0%)] stroke-[10px]");
}

#[test]
fn handles_conflicts_across_class_groups_correctly() {
    assert_eq!(tw_merge("inset-1 inset-x-1"), "inset-1 inset-x-1");
    assert_eq!(tw_merge("inset-x-1 inset-1"), "inset-1");
    assert_eq!(tw_merge("inset-x-1 left-1 inset-1"), "inset-1");
    assert_eq!(tw_merge("inset-x-1 inset-1 left-1"), "inset-1 left-1");
    assert_eq!(tw_merge("inset-x-1 right-1 inset-1"), "inset-1");
    assert_eq!(tw_merge("inset-x-1 right-1 inset-x-1"), "inset-x-1");
    assert_eq!(
        tw_merge("inset-x-1 right-1 inset-y-1"),
        "inset-x-1 right-1 inset-y-1"
    );
    assert_eq!(
        tw_merge("right-1 inset-x-1 inset-y-1"),
        "inset-x-1 inset-y-1"
    );
    assert_eq!(
        tw_merge("inset-x-1 hover:left-1 inset-1"),
        "hover:left-1 inset-1"
    );
}

#[test]
fn ring_and_shadow_classes_do_not_create_conflict() {
    assert_eq!(tw_merge("ring shadow"), "ring shadow");
    assert_eq!(tw_merge("ring-2 shadow-md"), "ring-2 shadow-md");
    assert_eq!(tw_merge("shadow ring"), "shadow ring");
    assert_eq!(tw_merge("shadow-md ring-2"), "shadow-md ring-2");
}

#[test]
fn ring_width_classes_merge_correctly() {
    // ring width conflicts with ring width
    assert_eq!(tw_merge("ring-1 ring-2"), "ring-2");
    assert_eq!(tw_merge("ring ring-2"), "ring-2");
    assert_eq!(tw_merge("ring-2 ring"), "ring");
}

#[test]
fn ring_color_classes_merge_correctly() {
    assert_eq!(tw_merge("ring-red-500 ring-blue-500"), "ring-blue-500");
}

#[test]
fn ring_width_and_color_do_not_conflict() {
    // ring width and ring color are different groups
    assert_eq!(tw_merge("ring ring-red-500"), "ring ring-red-500");
    assert_eq!(tw_merge("ring-2 ring-blue-500"), "ring-2 ring-blue-500");
}

#[test]
fn ring_inset_does_not_conflict_with_ring_width() {
    // Issue #28: ring-inset should NOT conflict with ring width
    assert_eq!(tw_merge("ring-1 ring-inset"), "ring-1 ring-inset");
    assert_eq!(tw_merge("ring-inset ring-2"), "ring-inset ring-2");
}

#[test]
fn ring_arbitrary_values() {
    assert_eq!(
        tw_merge("ring-[3px] ring-primary"),
        "ring-[3px] ring-primary"
    );
}

#[test]
fn touch_classes_do_create_conflicts_correctly() {
    assert_eq!(tw_merge("touch-pan-x touch-pan-right"), "touch-pan-right");
    assert_eq!(tw_merge("touch-none touch-pan-x"), "touch-pan-x");
    assert_eq!(tw_merge("touch-pan-x touch-none"), "touch-none");
    assert_eq!(
        tw_merge("touch-pan-x touch-pan-y touch-pinch-zoom"),
        "touch-pan-x touch-pan-y touch-pinch-zoom"
    );
    assert_eq!(
        tw_merge("touch-manipulation touch-pan-x touch-pan-y touch-pinch-zoom"),
        "touch-pan-x touch-pan-y touch-pinch-zoom"
    );
    assert_eq!(
        tw_merge("touch-pan-x touch-pan-y touch-pinch-zoom touch-auto"),
        "touch-auto"
    );
}

#[test]
fn line_clamp_classes_do_create_conflicts_correctly() {
    assert_eq!(
        tw_merge("overflow-auto inline line-clamp-1"),
        "line-clamp-1"
    );
    assert_eq!(
        tw_merge("line-clamp-1 overflow-auto inline"),
        "line-clamp-1 overflow-auto inline"
    );
}

#[test]
fn test_line_height_font_size() {
    assert_eq!(tw_merge("leading-9 text-lg"), "text-lg");
}

#[test]
fn text_color_font_size() {
    assert_eq!(tw_merge("text-red-500 text-lg"), "text-red-500 text-lg");

    // https://tailwindcss.com/docs/font-size#setting-the-line-height
    assert_eq!(
        tw_merge("text-red-500/10 text-lg/9"),
        "text-red-500/10 text-lg/9"
    );
}

#[test]
fn stroke_width() {
    assert_eq!(tw_merge("stroke-2 stroke-[3]"), "stroke-[3]");
}

#[test]
fn handles_negative_value_conflicts_correctly() {
    assert_eq!(tw_merge("-top-12 -top-2000"), "-top-2000");
}

#[test]
fn tailwind_3_4() {
    assert_eq!(tw_merge("text-red text-lg/8"), "text-red text-lg/8");
}

#[test]
fn test_group_data_important_modifiers() {
    let classes = tw_merge!(
        "group-data-[collapsible=icon]:!p-2",
        "group-data-[collapsible=icon]:!p-0"
    );
    assert_eq!(classes, "group-data-[collapsible=icon]:!p-0");

    let classes = tw_merge!(
        "group-data-[collapsible=icon]:!size-8",
        "group-data-[collapsible=icon]:!p-2",
        "group-data-[collapsible=icon]:!p-0"
    );
    assert_eq!(
        classes,
        "group-data-[collapsible=icon]:!size-8 group-data-[collapsible=icon]:!p-0"
    );

    let classes = tw_merge!(
        "group-data-[collapsible=icon]:!p-2",
        "group-data-[collapsible=icon]:!size-8",
        "group-data-[collapsible=icon]:!p-0"
    );
    assert_eq!(
        classes,
        "group-data-[collapsible=icon]:!size-8 group-data-[collapsible=icon]:!p-0"
    );
}

#[test]
fn text_size_with_slash_modifier() {
    // text-base/7 should be recognized as font-size and override leading-*
    assert_eq!(tw_merge("leading-9 text-base/7"), "text-base/7");
    assert_eq!(tw_merge("leading-9 text-base/none"), "text-base/none");
    // Should not conflict with text-color
    assert_eq!(
        tw_merge("text-red-500 text-base/7"),
        "text-red-500 text-base/7"
    );
}

// Issue #24: Custom font families should not conflict with font-weight
#[test]
fn font_family_does_not_conflict_with_font_weight() {
    assert_eq!(tw_merge("font-english font-bold"), "font-english font-bold");
    assert_eq!(tw_merge("font-bold font-english"), "font-bold font-english");
    // Known font families still merge with each other
    assert_eq!(tw_merge("font-sans font-mono"), "font-mono");
}

#[test]
fn font_weight_merges_correctly() {
    assert_eq!(tw_merge("font-bold font-thin"), "font-thin");
    assert_eq!(tw_merge("font-light font-extrabold"), "font-extrabold");
}

#[test]
fn border_r_b_single_word_color_not_misclassified_as_width() {
    // border-r-black is a color, not a width — should not conflict with border-r-2
    assert_eq!(
        tw_merge("border-r-black border-r-2"),
        "border-r-black border-r-2"
    );
    assert_eq!(
        tw_merge("border-b-white border-b-4"),
        "border-b-white border-b-4"
    );
    // But same-type should still merge
    assert_eq!(tw_merge("border-r-black border-r-blue"), "border-r-blue");
    assert_eq!(tw_merge("border-b-2 border-b-4"), "border-b-4");
}

#[test]
fn border_width_shorthand_overrides_x_y() {
    // border-w overrides border-w-x and border-w-y
    assert_eq!(tw_merge("border-x-2 border-4"), "border-4");
    assert_eq!(tw_merge("border-y-2 border-4"), "border-4");
    // But x/y don't override shorthand
    assert_eq!(tw_merge("border-4 border-x-2"), "border-4 border-x-2");
}

#[test]
fn border_color_shorthand_overrides_x_y() {
    // border-color overrides border-color-x and border-color-y
    assert_eq!(
        tw_merge("border-x-red-500 border-blue-500"),
        "border-blue-500"
    );
    assert_eq!(
        tw_merge("border-y-red-500 border-blue-500"),
        "border-blue-500"
    );
    // But x/y don't override shorthand
    assert_eq!(
        tw_merge("border-blue-500 border-x-red-500"),
        "border-blue-500 border-x-red-500"
    );
}

#[test]
fn translate_shorthand_overrides_z() {
    // translate shorthand overrides translate-z
    assert_eq!(tw_merge("translate-z-4 translate-4"), "translate-4");
    // translate-z doesn't override shorthand
    assert_eq!(
        tw_merge("translate-4 translate-z-4"),
        "translate-4 translate-z-4"
    );
}

#[test]
fn font_stretch_merges_correctly() {
    assert_eq!(
        tw_merge("font-stretch-condensed font-stretch-expanded"),
        "font-stretch-expanded"
    );
    // font-stretch should not conflict with font-weight
    assert_eq!(
        tw_merge("font-bold font-stretch-condensed"),
        "font-bold font-stretch-condensed"
    );
}
