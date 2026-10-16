#!/usr/bin/env python3
# Resize a PNG icon, add transparent padding, and round its corners.

import argparse
from pathlib import Path

from PIL import Image, ImageChops, ImageDraw


CANVAS_SIZE = 512
PADDING = 10
CORNER_RADIUS = 20


def process_image(source: Path, destination: Path) -> None:
    with Image.open(source) as opened:
        image = opened.convert("RGBA")

    max_content_size = CANVAS_SIZE - PADDING * 2
    image.thumbnail((max_content_size, max_content_size), Image.Resampling.LANCZOS)

    canvas = Image.new("RGBA", (CANVAS_SIZE, CANVAS_SIZE), (0, 0, 0, 0))
    left = (CANVAS_SIZE - image.width) // 2
    top = (CANVAS_SIZE - image.height) // 2
    canvas.alpha_composite(image, (left, top))

    rounded_mask = Image.new("L", (CANVAS_SIZE, CANVAS_SIZE), 0)
    ImageDraw.Draw(rounded_mask).rounded_rectangle(
        (left, top, left + image.width - 1, top + image.height - 1),
        radius=CORNER_RADIUS,
        fill=255,
    )
    canvas.putalpha(ImageChops.multiply(canvas.getchannel("A"), rounded_mask))

    destination.parent.mkdir(parents=True, exist_ok=True)
    canvas.save(destination, format="PNG")


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Create a 512x512 PNG icon with 10px margins and 20px rounded corners."
    )
    parser.add_argument("input", type=Path, help="Source PNG image")
    parser.add_argument("output", type=Path, help="Destination PNG image")
    args = parser.parse_args()

    if args.input.resolve() == args.output.resolve():
        parser.error("input and output paths must be different")
    if not args.input.is_file():
        parser.error(f"input file does not exist: {args.input}")
    if args.input.suffix.lower() != ".png" or args.output.suffix.lower() != ".png":
        parser.error("input and output files must use the .png extension")

    try:
        process_image(args.input, args.output)
    except (OSError, ValueError) as error:
        parser.error(f"could not process image: {error}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
