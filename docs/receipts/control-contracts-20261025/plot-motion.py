"""Plot the preserved 60fps X11 knob-center samples; no animation simulation."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
from matplotlib import pyplot as plt
from PIL import Image, ImageDraw

root = Path(__file__).resolve().parent
fig, axis = plt.subplots(figsize=(8, 3.4), constrained_layout=True)
axis.axhline(0, color='#666666', linewidth=.7)
axis.axhline(20, color='#999999', linewidth=.7, linestyle=':')
frames = []
for label, folder, color in [('Baseline 9c4515c', 'baseline-switch-motion', '#cc5c4d'),
                              ('Current source', 'final/switch-motion', '#007d72')]:
    data = json.loads((root / folder / 'motion-summary.json').read_text())
    centers = data['centers']
    initial = next(value for value in centers if value is not None)
    first = next(index for index, value in enumerate(centers) if value is not None and abs(value - initial) > .5)
    axis.plot([(index - first) / 60 for index in range(len(centers))],
              [value - initial if value is not None else float('nan') for value in centers],
              color=color, label=f"{label} (largest sampled step {data['max_adjacent_step']:g}px)", linewidth=1.8)
    delta_index = max(range(1, len(centers)), key=lambda i: abs(centers[i]-centers[i-1]) if centers[i] is not None and centers[i-1] is not None else 0)
    for index in range(max(0, delta_index-2), min(len(centers), delta_index+3)):
        frames.append((label, index, centers[index], root / folder / f'frame-{index+1:03}.png'))
axis.set(xlim=(-.1,.65), ylim=(-2,22), xlabel='Time from first visible motion (seconds)', ylabel='Knob travel from off (logical px)', title='Switch reversal · X11 capture at 60fps')
axis.legend(frameon=False, fontsize=8)
axis.spines[['top','right']].set_visible(False)
fig.savefig(root / 'switch-motion-comparison.png', dpi=180)
fig.savefig(root / 'switch-motion-comparison.svg')
plt.close(fig)
sheet = Image.new('RGB', (5*288, 2*232), 'white')
draw = ImageDraw.Draw(sheet)
for n, (label,index,center,path) in enumerate(frames):
    x=(n%5)*288;y=(n//5)*232
    sheet.paste(Image.open(path).convert('RGB').resize((288,208)),(x,y+24))
    draw.text((x+4,y+4),f'{label}: frame {index+1}, center {center:g}',fill='black')
sheet.save(root / 'switch-motion-frames.png')
