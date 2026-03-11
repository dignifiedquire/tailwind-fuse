/**
 * Generates a comprehensive class list from Tailwind CSS v4's design system.
 *
 * Uses the `__unstable__loadDesignSystem` API to enumerate all utility classes
 * and their CSS properties. Outputs JSON fixtures used by Rust tests to validate
 * collision ID coverage.
 *
 * Usage:
 *   cd scripts && npm install && npm run generate
 */

import { __unstable__loadDesignSystem } from "tailwindcss";
import { readFileSync, writeFileSync, mkdirSync } from "fs";
import { createRequire } from "module";
import { dirname, resolve } from "path";
import { fileURLToPath } from "url";

const require = createRequire(import.meta.url);
const __dirname = dirname(fileURLToPath(import.meta.url));

// Read the full tailwindcss CSS (includes @theme default + utilities)
const cssPath = require.resolve("tailwindcss/index.css");
const css = readFileSync(cssPath, "utf8");

console.log("Loading Tailwind CSS v4 design system...");
const designSystem = await __unstable__loadDesignSystem(css);

// Get all classes
const classList = designSystem.getClassList();
console.log(`Found ${classList.length} classes`);

// Get all variants
const variants = designSystem.getVariants();
console.log(`Found ${variants.length} variants`);

// Extract CSS properties for each class using candidatesToCss
// Process in batches to avoid memory issues
const BATCH_SIZE = 500;
const classNames = classList.map(([name]) => name);

console.log("Generating CSS for all classes...");

const classMap = {};
const errors = [];

for (let i = 0; i < classNames.length; i += BATCH_SIZE) {
  const batch = classNames.slice(i, i + BATCH_SIZE);
  const cssResults = designSystem.candidatesToCss(batch);

  for (let j = 0; j < batch.length; j++) {
    const className = batch[j];
    const cssOutput = cssResults[j];

    if (!cssOutput) {
      errors.push(className);
      continue;
    }

    // Parse CSS properties from the output
    const properties = extractCssProperties(cssOutput);
    const modifiers = classList[i + j][1]?.modifiers || [];

    classMap[className] = {
      properties,
      ...(modifiers.length > 0 ? { modifiers } : {}),
    };
  }
}

if (errors.length > 0) {
  console.warn(`Warning: ${errors.length} classes produced no CSS output`);
}

// Group classes by their CSS property signature for collision analysis
const collisionGroups = {};
for (const [className, data] of Object.entries(classMap)) {
  const key = data.properties.sort().join("+");
  if (!key) continue;
  if (!collisionGroups[key]) {
    collisionGroups[key] = [];
  }
  collisionGroups[key].push(className);
}

// Build variant list
const variantList = variants.map((v) => ({
  name: v.name,
  isArbitrary: v.isArbitrary,
  hasDash: v.hasDash,
  selectors: v.selectors?.() || [],
}));

// Output directory
const outputDir = resolve(__dirname, "..", "fixtures");
mkdirSync(outputDir, { recursive: true });

// Write class map (class → properties)
writeFileSync(
  resolve(outputDir, "tw4-class-map.json"),
  JSON.stringify(classMap, null, 2),
);
console.log(
  `Wrote fixtures/tw4-class-map.json (${Object.keys(classMap).length} classes)`,
);

// Write collision groups (property signature → classes)
writeFileSync(
  resolve(outputDir, "tw4-collision-groups.json"),
  JSON.stringify(collisionGroups, null, 2),
);
console.log(
  `Wrote fixtures/tw4-collision-groups.json (${Object.keys(collisionGroups).length} groups)`,
);

// Write variant list
writeFileSync(
  resolve(outputDir, "tw4-variants.json"),
  JSON.stringify(variantList, null, 2),
);
console.log(`Wrote fixtures/tw4-variants.json (${variantList.length} variants)`);

// Write summary statistics
const summary = {
  tailwindVersion: "4.x",
  generatedAt: new Date().toISOString(),
  totalClasses: classList.length,
  classesWithCss: Object.keys(classMap).length,
  classesWithoutCss: errors.length,
  collisionGroups: Object.keys(collisionGroups).length,
  variants: variants.length,
  topPropertyGroups: Object.entries(collisionGroups)
    .sort((a, b) => b[1].length - a[1].length)
    .slice(0, 20)
    .map(([key, classes]) => ({
      properties: key,
      count: classes.length,
      sample: classes.slice(0, 5),
    })),
};

writeFileSync(
  resolve(outputDir, "tw4-summary.json"),
  JSON.stringify(summary, null, 2),
);
console.log(`Wrote fixtures/tw4-summary.json`);

// Write a flat list of all class names (useful for quick validation)
writeFileSync(
  resolve(outputDir, "tw4-classes.txt"),
  classNames.sort().join("\n") + "\n",
);
console.log(`Wrote fixtures/tw4-classes.txt`);

console.log("\nDone!");

/**
 * Extract CSS property names from the main rule block only.
 * Skips @property declarations and other at-rules that TW v4 emits
 * alongside the main utility rule.
 */
function extractCssProperties(cssString) {
  const properties = new Set();

  // Only extract from the main rule block (first .class-name { ... })
  // Skip @property, @supports, and other at-rules
  let depth = 0;
  let inMainRule = false;
  let inAtRule = false;
  let atRuleDepth = 0;

  const lines = cssString.split("\n");
  for (const line of lines) {
    const trimmed = line.trim();
    if (!trimmed) continue;

    // Track @property and other at-rules to skip them
    if (trimmed.startsWith("@property")) {
      inAtRule = true;
      atRuleDepth = 0;
    }

    if (inAtRule) {
      atRuleDepth += (trimmed.match(/\{/g) || []).length;
      atRuleDepth -= (trimmed.match(/\}/g) || []).length;
      if (atRuleDepth <= 0 && trimmed.includes("}")) {
        inAtRule = false;
      }
      continue;
    }

    // Track the main rule block
    if (trimmed.startsWith(".") && trimmed.endsWith("{")) {
      inMainRule = true;
      depth = 1;
      continue;
    }

    // Handle @supports inside the main rule (nested at-rules are ok)
    if (trimmed.startsWith("@supports") && trimmed.endsWith("{")) {
      depth++;
      continue;
    }

    if (inMainRule) {
      const opens = (trimmed.match(/\{/g) || []).length;
      const closes = (trimmed.match(/\}/g) || []).length;
      depth += opens - closes;

      if (depth <= 0) {
        inMainRule = false;
        continue;
      }

      if (trimmed === "}" || trimmed.endsWith("{")) continue;

      // Match "property: value;" or "--custom-prop: value;"
      const match = trimmed.match(/^(--?[\w-]+)\s*:/);
      if (match) {
        properties.add(match[1]);
      }
    }
  }

  return [...properties];
}
