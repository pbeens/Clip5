# website

Quarto source for the Clip5 website at <https://peter.beens.ca/Clip5/>.

- Render with `quarto render website` from the repository root (or `quarto preview website` while editing).
- Output goes to [`../docs/`](../docs/), which GitHub Pages serves (`main` branch, `/docs` folder). Never edit `docs/` by hand.
- [`write-docs-readme.ts`](write-docs-readme.ts) runs after each render. It adds `docs/README.md` and `.nojekyll`.
- [`styles.css`](styles.css) holds the site's few custom styles.
- Website-only images go in this folder. App images go in [`../assets/`](../assets/README.md).
