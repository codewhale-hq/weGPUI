"""Create labeled overview sheets; raw browser captures remain unchanged."""
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont

root = Path(__file__).resolve().parent
captures = root / 'final-candidate1-rerun/project-prefix'
groups = {
    'controls': ['dialog-open', 'half-screen-dialog-open', 'actionsheet-open',
                 'button-guards', 'html-input-guards', 'html-unicode-multiline',
                 'agreement-vcode', 'picker-confirmed', 'slider-pointer',
                 'swipe-pointer-open', 'local-image-preview', 'information-bar-restored'],
    'themes': sorted(path.stem for path in captures.glob('theme-*.png')),
    'shell': ['shell-375-live-dialog', 'shell-768-live-dialog', 'shell-1280-live-dialog',
              'shell-phone-wide-preview-scroll', 'shell-prefs-keyboard-native-history',
              'appearance-preserves-draft', 'article-narrow-scrolled', 'route-audit-final'],
}
font = ImageFont.truetype('/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf', 12)
for group, names in groups.items():
    width, height, columns = 260, 360, 4
    rows = (len(names) + columns - 1) // columns
    sheet = Image.new('RGB', (width * columns, height * rows), '#dce3da')
    draw = ImageDraw.Draw(sheet)
    for index, name in enumerate(names):
        raw = Image.open(captures / (name + '.png')).convert('RGB')
        raw.thumbnail((width - 12, height - 38))
        x, y = (index % columns) * width, (index // columns) * height
        draw.text((x + 6, y + 7), name, fill='#18372b', font=font)
        sheet.paste(raw, (x + (width - raw.width) // 2, y + 30))
    sheet.save(root / f'review-{group}.jpg', quality=88)
