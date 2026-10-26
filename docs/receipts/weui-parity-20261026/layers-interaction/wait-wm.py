import subprocess,time
for attempt in range(100):
 result=subprocess.run(['xprop','-root','_NET_SUPPORTING_WM_CHECK'],capture_output=True,text=True)
 if 'window id' in result.stdout:break
 time.sleep(.05)
else:raise RuntimeError('window manager not ready')
