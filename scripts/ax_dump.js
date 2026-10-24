function run(argv) {
  const pid = parseInt(argv[0], 10);
  const se = Application('System Events');
  const proc = se.processes.whose({ unixId: pid })[0];
  const out = [];
  function attr(e, name) {
    try { const v = e[name](); return v === null || v === undefined ? '' : String(v); } catch (err) { return ''; }
  }
  function walk(e, depth) {
    if (depth > 40) return;
    const parts = [attr(e, 'role'), attr(e, 'description'), attr(e, 'name'), attr(e, 'value')];
    let extra = '';
    try { if (e.attributes.byName('AXEnabled').value() === false) extra += ' [disabled]'; } catch (err) {}
    try { const s = e.attributes.byName('AXSelected').value(); if (s) extra += ' [selected]'; } catch (err) {}
    try { const x = e.attributes.byName('AXExpanded').value(); if (x !== null && x !== undefined) extra += ' [expanded=' + x + ']'; } catch (err) {}
    try { if (e.attributes.byName('AXFocused').value() === true) extra += ' [focused]'; } catch (err) {}
    out.push('  '.repeat(depth) + parts.filter(p => p !== '').join(' | ') + extra);
    let n = 0;
    try { n = e.uiElements.length; } catch (err) {}
    for (let i = 0; i < n; i++) walk(e.uiElements[i], depth + 1);
  }
  walk(proc.windows[0], 0);
  return out.join('\n');
}
