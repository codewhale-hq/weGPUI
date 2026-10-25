#!/usr/bin/env python3
"""Validate archived observations; this deliberately reports the reproduced safety failures."""
import json
from pathlib import Path
import re

base = Path(__file__).resolve().parent
summary = []
for kind, delays in [("click", [0, 1, 20, 100, 500]), ("focus", [0, 1])]:
    for delay in delays:
        run = base / f"{'mask' if kind == 'click' else 'focus'}-fast-{delay}ms"
        events = json.loads((run / "observations/driver-events.json").read_text())
        by_kind = {event["kind"]: event for event in events}
        text = (run / "app.log").read_text()
        requests = re.findall(r"(\d+) request=(\d+) open=(true|false) presence_open=(true|false)", text)
        false_requests = [list(row) for row in requests if row[3] == "false"]
        assert len(re.findall(r"^\d+ open$", text, re.M)) == 2
        assert len(re.findall(r"^\d+ hide$", text, re.M)) == 1
        assert [row[2] for row in requests[:2]] == ["true", "false"]
        assert all(row[3] == "true" for row in requests[:2])
        assert requests[-1][2:] == ("true", "true")
        assert by_kind["baseline-open"]["result"] is True
        assert by_kind["baseline-close"]["result"] is True
        assert by_kind["reopened-click"]["result"] is True
        if kind == "focus":
            assert by_kind["baseline-component-focus"]["result"] is True
        saved = by_kind.get(f"saved-{kind}") or by_kind[f"saved-{kind}-error"]
        later = by_kind.get(f"saved-{kind}-after-closed") or by_kind[f"saved-{kind}-after-closed-error"]
        assert later["elapsed_since_close_ms"] > 2000
        after = json.loads((run / "observations/after-close-tree.json").read_text())
        focused = [row["name"] for row in after if any("STATE_FOCUSED" in state for state in row["states"])]
        assert not any(row["name"] == "Saved custom action row" for row in after)
        expected_failure = delay in (0, 1)
        assert ("Open slow modal" not in focused) == expected_failure
        assert len(false_requests) == (1 if kind == "click" and expected_failure else 0)
        if expected_failure:
            assert saved["result"] is True
        else:
            assert "Unknown object" in saved["error"]
        assert "error" in later
        summary.append({
            "route": kind,
            "requested_delay_ms": delay,
            "after_command_return_ms": (saved["action_invoked_ns"] - saved["close_command_returned_ns"]) / 1e6,
            "after_command_start_ms": (saved["action_invoked_ns"] - saved["close_command_started_ns"]) / 1e6,
            "saved_result": saved.get("result"),
            "saved_error": saved.get("error"),
            "callbacks_while_presence_false": false_requests,
            "focused_after_close": focused,
            "late_action_ms": later["elapsed_since_close_ms"],
            "late_error": later.get("error"),
            "reopen_click": by_kind["reopened-click"]["result"],
            "closing_safety": "FAIL: model callback/focus loss" if false_requests else
                              "FAIL: focus loss" if expected_failure else "no violation observed",
        })
(base / "observations-summary.json").write_text(json.dumps(summary, indent=2))
print("Archived observations are internally consistent across all seven fresh sessions.")
print("Closing safety FAILED: custom Click at0/1ms; builtin Focus at0/1ms.")
print("Click at20/100/500ms: proxy rejected, no closed-model callback, opener restored.")
print("All seven reopen controls passed. This checker does not report the bug fixed.")
