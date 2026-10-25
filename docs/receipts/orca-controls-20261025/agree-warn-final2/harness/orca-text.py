"""Inspect input Text interfaces separately from Orca's speech evidence.

Run inside the same AT-SPI session as the application. This read-only diagnostic
does not establish screen-reader acceptance. Password text is never printed.
"""

import pyatspi


def walk(node):
    role = node.getRole()
    if role in (pyatspi.ROLE_ENTRY, pyatspi.ROLE_PASSWORD_TEXT):
        print(f'{node.getRoleName()} {node.name!r}')
        try:
            text = node.queryText()
            value = text.getText(0, -1)
            if role == pyatspi.ROLE_PASSWORD_TEXT:
                print(f'  password character count: {text.characterCount}')
                print(f'  password contains only mask characters: {all(c in "•●*" for c in value)}')
            else:
                print(f'  text: {value!r}')
                print(f'  character count: {text.characterCount}')
            print(f'  caret: {text.caretOffset}')
            print(f'  selections: {[text.getSelection(i) for i in range(text.getNSelections())]}')
        except (NotImplementedError, AttributeError) as error:
            print(f'  Text interface unavailable: {type(error).__name__}: {error}')
    for index in range(node.childCount):
        child = node.getChildAtIndex(index)
        if child is not None:
            walk(child)


desktop = pyatspi.Registry.getDesktop(0)
for index in range(desktop.childCount):
    application = desktop.getChildAtIndex(index)
    if application is not None:
        walk(application)
