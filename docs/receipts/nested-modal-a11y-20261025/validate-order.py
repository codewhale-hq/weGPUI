"""Check modal exposure against the actual three-layer draw order."""
from pathlib import Path
import sys

folder = Path(sys.argv[1])
for label, title in (('all-open', 'Inner B'), ('b-cancel', 'Inner B'), ('b-ok', 'Inner B'),
                     ('c-visible', 'Sibling C'), ('c-focused', 'Sibling C'), ('a-visible', 'Outer A')):
    tree = (folder / f'{label}-atspi.txt').read_text()
    assert f"dialog {title!r}" in tree, (label, tree)
    for other in {'Outer A', 'Inner B', 'Sibling C'} - {title}:
        assert f"dialog {other!r}" not in tree, (label, tree)
for label, name in (('b-cancel', 'Cancel B'), ('b-ok', 'OK B'), ('c-focused', 'Close C')):
    tree = (folder / f'{label}-atspi.txt').read_text()
    assert any(repr(name) in line and "'focused'" in line for line in tree.splitlines()), (label, tree)
tree = (folder / 'page-atspi.txt').read_text()
assert "dialog '" not in tree, tree
print('PASS: nested B stays exposed above later root sibling C; Escape exposes C, then A, then page.')
