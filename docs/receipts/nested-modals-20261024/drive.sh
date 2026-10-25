export DISPLAY=:96; O=$1; mkdir -p $O; rm -f $O/*
pkill -9 -f examples/nested_modals; sleep .5
(GPUI_X11_SCALE_FACTOR=1 target/debug/examples/nested_modals >/tmp/nm.log 2>&1 &); sleep 6
W=$(xdotool search --name "nested modals"|tail -1); xdotool windowmove $W 0 0; xdotool windowactivate $W; sleep .3
xdotool mousemove 186 90 click 1; sleep .3
s(){ sleep .6; import -window $W $O/$1.png; }
s 02-sheet-open-by-click
xdotool key Tab; s 03-tab1; xdotool key Return; s 04-inner-open
for i in 1 2 3 4 5; do xdotool key Tab; s 05-inner-tab$i; done
xdotool key shift+Tab; s 06-inner-shifttab
xdotool key Escape; s 07-esc1; xdotool key Tab; s 08-after-esc1-tab
xdotool key Escape; s 09-esc2; xdotool key Return; s 10-enter-after-esc2
