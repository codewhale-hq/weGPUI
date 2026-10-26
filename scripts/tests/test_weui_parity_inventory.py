import importlib.util
from pathlib import Path
import unittest


SPEC = importlib.util.spec_from_file_location(
    "weui_parity_inventory", Path(__file__).resolve().parents[1] / "weui_parity_inventory.py"
)
inventory = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(inventory)


class ParityInventoryTests(unittest.TestCase):
    def test_every_scene_has_all_eight_appearance_combinations(self):
        matrix = inventory.capture_matrix({"dialog", "footer"}, {"dialog:ios1": "replay"})
        self.assertEqual(len(matrix), 24)
        for route, state in [("dialog", None), ("footer", None), ("dialog", "ios1")]:
            rows = [row for row in matrix if (row["route"], row["state"]) == (route, state)]
            self.assertEqual(len({(r["theme"], r["care"], r["brand"]) for r in rows}), 8)

    def test_native_only_or_unknown_replays_are_rejected(self):
        for state in ["languages:typing", "dialog", "dialog:"]:
            with self.subTest(state=state), self.assertRaises(ValueError):
                inventory.capture_matrix({"dialog"}, {state: "replay"})

    def test_unknown_widget_files_are_not_silently_counted_as_covered(self):
        for path in ["new-widget/new.less", "weui-tips/unknown.less", "weui-cell/new-control.less"]:
            with self.subTest(path=path), self.assertRaisesRegex(ValueError, "Unclassified"):
                inventory.modules_for_style(Path(path))

    def test_shared_style_files_map_to_all_native_consumers(self):
        self.assertEqual(inventory.modules_for_style(Path("weui-cell/weui-check/weui-checkbox.less")),
                         ["checkbox", "agree"])
        self.assertEqual(inventory.modules_for_style(Path("weui-tips/weui-mask.less")), ["modal"])


if __name__ == "__main__":
    unittest.main()
