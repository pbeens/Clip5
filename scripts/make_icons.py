"""Renders assets/icon.svg into the PNG and ICO files the app and bundles use.

Run from the repository root after editing the SVG:

    python scripts/make_icons.py

Needs rsvg-convert (librsvg) on the PATH and the Pillow package (pip install pillow).
"""

import subprocess
from pathlib import Path

from PIL import Image

ASSETS = Path("assets")
SVG = ASSETS / "icon.svg"

# PNG sizes kept in assets/: the window icon (256) and the macOS bundle set.
PNG_SIZES = [32, 64, 128, 256, 512, 1024]
# Sizes packed into the Windows .ico.
ICO_SIZES = [16, 24, 32, 48, 64, 128, 256]


def render(size: int) -> Path:
    out = ASSETS / f"icon-{size}.png"
    subprocess.run(
        ["rsvg-convert", "-w", str(size), "-h", str(size), str(SVG), "-o", str(out)],
        check=True,
    )
    return out


def main() -> None:
    for size in PNG_SIZES:
        print("wrote", render(size))

    # Render each ICO size from the SVG rather than shrinking one big PNG, so small sizes stay sharp.
    frames = []
    for size in ICO_SIZES:
        path = render(size)
        frames.append(Image.open(path).convert("RGBA"))
        if size not in PNG_SIZES:
            path.unlink()
    ico = ASSETS / "icon.ico"
    frames[-1].save(ico, sizes=[(s, s) for s in ICO_SIZES], append_images=frames[:-1])
    print("wrote", ico)


if __name__ == "__main__":
    main()
