"""Tile screenshot pairs into labelled contact sheets for review.

Usage:
  python3 -I scripts/contact_sheets.py PAIR_DIR OUT_DIR [--glob '*-pair.png'] [--prefix sheet]
      [--per 6] [--scale 0.5]

Reads the pair images written by scripts/capture.py when PAIR_DIR is set (native on the left,
upstream on the right), and writes OUT_DIR/PREFIX01.png and so on, three pairs per row, each
labelled with its route, state and theme. Needs Pillow.
"""

import argparse
import glob
import math
import os

from PIL import Image, ImageDraw


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('pairs')
    parser.add_argument('out')
    parser.add_argument('--glob', default='*-pair.png')
    parser.add_argument('--prefix', default='sheet')
    parser.add_argument('--per', type=int, default=6)
    parser.add_argument('--scale', type=float, default=0.5)
    args = parser.parse_args()
    if args.per < 1:
        parser.error('--per must be positive')
    if not math.isfinite(args.scale) or args.scale <= 0:
        parser.error('--scale must be finite and positive')
    pairs = sorted(glob.glob(os.path.join(args.pairs, args.glob)))
    if not pairs:
        parser.error('no pair images matched')
    os.makedirs(args.out, exist_ok=True)
    columns = 3
    for start in range(0, len(pairs), args.per):
        chunk = pairs[start:start + args.per]
        images = [Image.open(path).convert('RGB') for path in chunk]
        width = int(max(image.width for image in images) * args.scale)
        height = int(max(image.height for image in images) * args.scale)
        if min(width, height) < 1:
            parser.error('--scale produces an image smaller than one pixel')
        rows = (len(chunk) + columns - 1) // columns
        sheet = Image.new('RGB', (width * columns, (height + 18) * rows), (255, 255, 255))
        draw = ImageDraw.Draw(sheet)
        for n, (path, image) in enumerate(zip(chunk, images)):
            image = image.resize((int(image.width * args.scale), int(image.height * args.scale)))
            x = (n % columns) * width
            y = (n // columns) * (height + 18)
            sheet.paste(image, (x, y + 18))
            label = os.path.basename(path).replace('-pair.png', '')
            draw.text((x + 4, y + 2), label, fill=(200, 0, 0))
        sheet.save(os.path.join(args.out, f'{args.prefix}{start // args.per + 1:02d}.png'))
    print(f'{len(pairs)} pairs')


if __name__ == '__main__':
    main()
