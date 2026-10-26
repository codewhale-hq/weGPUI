set -eu
export DISPLAY=:99 GPUI_X11_SCALE_FACTOR=2 LIBGL_ALWAYS_SOFTWARE=1 VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json XDG_RUNTIME_DIR=/work/runtime
mkdir -p /work/runtime /work/captures
chmod 700 /work/runtime
for lang in en ar; do
 for theme in light dark; do
  python3 -I scripts/capture.py languages --settle 5 --only gpui --lang "$lang" --theme "$theme" --scroll 2310 --out /work/captures --name "languages-$lang-paragraphs"
  python3 -I scripts/capture.py languages --settle 5 --only gpui --lang "$lang" --theme "$theme" --state type:7:all --out /work/captures --name "textarea-$lang-all"
 done
done
