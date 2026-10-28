import json, os, subprocess, time
from pathlib import Path

root = Path("/work/full-parity")
out = root / "tel-input-probe"
snapshot = root / "tel-input-snapshot.json"
env = dict(os.environ, DISPLAY=":216", GPUI_X11_SCALE_FACTOR="1", LIBGL_ALWAYS_SOFTWARE="1", VK_ICD_FILENAMES="/usr/share/vulkan/icd.d/lvp_icd.json", XDG_RUNTIME_DIR="/work/runtime")
log = (out / "native.log").open("w")
xvfb = subprocess.Popen(["Xvfb", ":216", "-screen", "0", "1200x600x24", "-nolisten", "tcp"], env=env, stdout=log, stderr=log)
wm = process = None
steps = []

def key(name):
    subprocess.run(["xdotool", "key", "--clearmodifiers", name], env=env, check=True)
    time.sleep(.4)

def capture(label):
    snapshot.unlink(missing_ok=True)
    key("F12")
    for _ in range(40):
        if snapshot.exists():
            result = json.loads(snapshot.read_text())
            result["step"] = label
            steps.append(result)
            (out / "result.json").write_text(json.dumps({"steps": steps}, indent=2, ensure_ascii=False) + "\n")
            return result
        time.sleep(.05)
    raise RuntimeError("No snapshot for " + label)

def typed(value):
    subprocess.run(["xdotool", "type", "--clearmodifiers", "--delay", "5", "--", value], env=env, check=True)
    time.sleep(.5)

def expect_value(state, value):
    assert state["value"] == value, (state, value)

try:
    for _ in range(100):
        if xvfb.poll() is not None:
            raise RuntimeError("Xvfb exited; display may already be in use")
        if subprocess.run(["xdpyinfo", "-display", ":216"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL).returncode == 0:
            break
        time.sleep(.05)
    else:
        raise RuntimeError("Xvfb not ready")
    wm = subprocess.Popen(["openbox", "--sm-disable"], env=env, stdout=log, stderr=log)
    process = subprocess.Popen(["/target/debug/wegpui-tel-input-probe"], env=env, stdout=log, stderr=log)
    window = None
    for _ in range(300):
        if process.poll() is not None:
            raise RuntimeError("probe exited before completing")
        found = subprocess.run(["xdotool", "search", "--onlyvisible", "--pid", str(process.pid)], env=env, capture_output=True, text=True)
        if found.stdout.strip():
            window = found.stdout.splitlines()[0]
            break
        time.sleep(.05)
    if not window:
        raise RuntimeError("probe window not ready")
    subprocess.run(["xdotool", "windowactivate", "--sync", window], env=env, check=True)
    time.sleep(1)
    capture("initial")
    typed("+1-800-FLOWERS x12")
    typed_state = capture("typed-vanity")
    expect_value(typed_state, "+1-800-FLOWERS x12")
    key("F2"); key("ctrl+v")
    pasted = capture("pasted-unicode")
    expect_value(pasted, "+1-800-FLOWERS x12+٨٦ １２３ ext. 9")
    assert pasted["changes"] == typed_state["changes"] + 1
    key("ctrl+z")
    undone = capture("undo-paste"); expect_value(undone, typed_state["value"])
    assert undone["changes"] == pasted["changes"] + 1
    key("ctrl+y")
    redone = capture("redo-paste"); expect_value(redone, pasted["value"])
    assert redone["changes"] == undone["changes"] + 1
    key("F4"); key("F9")
    limited = capture("max-length-seed"); expect_value(limited, "A😀B")
    key("ctrl+v")
    truncated = capture("max-length-paste"); expect_value(truncated, "A😀B😀")
    assert truncated["len"] == 6 and truncated["changes"] == limited["changes"] + 1
    key("F5")
    readonly = capture("readonly"); assert readonly["readonly"]
    typed("x"); key("ctrl+v")
    guarded = capture("readonly-guard"); expect_value(guarded, readonly["value"])
    assert guarded["changes"] == readonly["changes"]
    key("F6")
    disabled = capture("disabled"); assert disabled["disabled"]
    typed("x"); key("ctrl+v")
    guarded = capture("disabled-guard"); expect_value(guarded, disabled["value"])
    assert guarded["changes"] == disabled["changes"]
    key("F6"); key("F3")
    typed("12abc34")
    before = capture("kind-tel"); expect_value(before, "12abc34")
    key("F7")
    digits = capture("kind-digit"); expect_value(digits, "1234")
    assert digits["kind"] == "Digit" and digits["changes"] == before["changes"] + 1
    key("F8")
    tel = capture("kind-restored"); expect_value(tel, "1234")
    assert tel["kind"] == "Tel" and tel["changes"] == digits["changes"]
    typed("xyz")
    final = capture("typed-after-kind-transition"); expect_value(final, "1234xyz")
    key("F10")
    controls = capture("programmatic-control-sanitization"); expect_value(controls, "AB\tC")
    key("Left"); typed("x")
    tab_edit = capture("tab-caret-edit"); expect_value(tab_edit, "AB\txC")
    key("ctrl+z")
    tab_undo = capture("tab-caret-undo"); expect_value(tab_undo, "AB\tC")
    key("End"); key("F11"); key("ctrl+v")
    multiline = capture("multiline-paste-sanitization"); expect_value(multiline, "AB\tCx y")
    assert multiline["changes"] == tab_undo["changes"] + 1
    key("F1")
    digit_commit = capture("synthetic-digit-ime-commit"); expect_value(digit_commit, "1782")
    assert digit_commit["selection"] == [2, 2] and digit_commit["marked"] is None
    key("F13")
    tel_marked = capture("synthetic-tel-ime-preedit"); expect_value(tel_marked, "A😀中Z")
    assert tel_marked["selection"] == [3, 3] and tel_marked["marked"] == [1, 4]
    key("F14")
    tel_commit = capture("synthetic-tel-ime-commit"); expect_value(tel_commit, "A😀Z")
    assert tel_commit["selection"] == [3, 3] and tel_commit["marked"] is None
    assert tel_commit["changes"] == tel_marked["changes"] + 1
    key("ctrl+z")
    tel_undo = capture("synthetic-tel-ime-undo"); expect_value(tel_undo, "AZ")
    key("F15")
    joined_marked = capture("synthetic-joined-grapheme-preedit"); expect_value(joined_marked, "áb")
    assert joined_marked["selection"] == [1, 1] and joined_marked["marked"] == [1, 2]
    key("F14")
    joined = capture("synthetic-joined-grapheme-commit"); expect_value(joined, "áb")
    assert joined["selection"] == [2, 2] and joined["marked"] is None
    key("BackSpace")
    joined_delete = capture("synthetic-joined-grapheme-backspace"); expect_value(joined_delete, "b")
    subprocess.run(["import", "-window", window, str(out / "native.png")], env=env, check=True)
    (out / "result.json").write_text(json.dumps({"passed": True, "steps": steps}, indent=2, ensure_ascii=False) + "\n")
    print(json.dumps({"passed": True, "steps": len(steps)}))
finally:
    for proc in (process, wm, xvfb):
        if proc is not None and proc.poll() is None:
            proc.terminate()
            proc.wait(timeout=5)
    log.close()
