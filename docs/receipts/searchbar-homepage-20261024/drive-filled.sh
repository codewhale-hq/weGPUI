cd /workspace/weGPUI; O=$1; mkdir -p $O; rm -f $O/*
bash /tmp/run.sh target/debug/examples/gallery --page searchbar_filled >/dev/null 2>&1; export DISPLAY=:96; W=$(cat /tmp/W); c(){ import -window $W $O/$1.png; }
c 00-idle; xdotool key Tab; sleep .4; c 01-tab; xdotool key Return; sleep .5; c 02-focusing; xdotool type weui; sleep .5; c 03-typed
xdotool key Tab; sleep .4; c 04-tab-clear; xdotool key Tab; sleep .4; c 05-tab-cancel; xdotool key shift+Tab shift+Tab; sleep .4; c 06-back-to-input
