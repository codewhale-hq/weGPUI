from pathlib import Path
import subprocess,time,os
out=Path('<evidence>')
with (out/'text-probe.log').open('w') as log:
 app=subprocess.Popen(['<toolchain-cache>/target/debug/examples/__parity_text_probe'],stdout=log,stderr=log)
 try:
  for _ in range(100):
   found=subprocess.run(['xdotool','search','--onlyvisible','--pid',str(app.pid)],capture_output=True,text=True)
   if found.stdout.strip():break
   time.sleep(.05)
  assert found.stdout.strip(),'no native window'
  time.sleep(1.2)
  subprocess.run(['import','-window',found.stdout.split()[-1],str(out/'text-probe.png')],check=True)
 finally:
  app.terminate()
  try:app.wait(timeout=2)
  except subprocess.TimeoutExpired:app.kill();app.wait()
