// Quarto post-render script: docs/ is rebuilt on every render, so recreate its README here.
const outDir = Deno.env.get("QUARTO_PROJECT_OUTPUT_DIR") ?? "../docs";

Deno.writeTextFileSync(
  `${outDir}/README.md`,
  `# docs

Generated website output for Clip5, served by GitHub Pages.

Do not edit these files by hand. They are rebuilt from \`website/\` with \`quarto render website\`.
`,
);

// Tell GitHub Pages to serve the files as-is, without running Jekyll.
Deno.writeTextFileSync(`${outDir}/.nojekyll`, "");
