"""Validate modal accessibility exposure and focus from actual runtime trees."""
from pathlib import Path
import sys

folder = Path(sys.argv[1])
for label in ('inner', 'inner-cancel', 'inner-ok', 'inner-wrap', 'inner-reverse'):
    tree = (folder / f'{label}-atspi.txt').read_text()
    assert "dialog 'Inner dialog'" in tree, (label, tree)
    for hidden in ('Outer sheet', 'Open inner dialog', 'Close sheet', 'Open sheet', 'Background button'):
        assert repr(hidden) not in tree, (label, hidden, tree)
for label in ('outer', 'outer-tab', 'outer-restored'):
    tree = (folder / f'{label}-atspi.txt').read_text()
    assert "dialog 'Outer sheet'" in tree and "'Open inner dialog'" in tree, (label, tree)
    assert "dialog 'Inner dialog'" not in tree and "'Open sheet'" not in tree, (label, tree)
for label in ('initial', 'page-restored'):
    tree = (folder / f'{label}-atspi.txt').read_text()
    assert "'Open sheet'" in tree and "'Background button'" in tree, (label, tree)
    assert "dialog '" not in tree, (label, tree)
for label, name in (('inner-cancel', 'Cancel'), ('inner-ok', 'OK'), ('inner-wrap', 'Cancel'),
                    ('inner-reverse', 'OK'), ('outer-restored', 'Open inner dialog'),
                    ('page-restored', 'Open sheet')):
    lines = (folder / f'{label}-atspi.txt').read_text().splitlines()
    assert any(repr(name) in line and "'focused'" in line for line in lines), (label, name, lines)
for label in ('toast-active', 'toast-tab'):
    tree = (folder / f'{label}-atspi.txt').read_text()
    assert "'成功提示'" in tree and "'已完成'" in tree, (label, tree)
    assert "dialog '" not in tree, (label, tree)
tree = (folder / 'toast-tab-atspi.txt').read_text()
assert any("'成功提示'" in line and "'focused'" in line for line in tree.splitlines()), tree
print('PASS: only top modal exposed; Tab/Shift+Tab trap, both Escape focus restorations, and passive toast page exposure verified.')
