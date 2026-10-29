"""Shared inventory for browser routes with native-only gallery pages excluded."""

import re


NATIVE_ONLY_ROUTES = frozenset({'languages', 'desktop-form'})


def browser_route_map(source):
    """Return the native registry entries that require browser examples and recorded captures."""
    routes = dict(re.findall(r'\("([\w-]+)",\s*(\w+)::build\)', source))
    missing = NATIVE_ONLY_ROUTES - routes.keys()
    if missing:
        raise ValueError(f'Native-only route exceptions are not registered: {sorted(missing)}')
    return {name: module for name, module in routes.items() if name not in NATIVE_ONLY_ROUTES}
