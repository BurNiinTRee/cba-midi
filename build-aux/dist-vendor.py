#!/usr/bin/env python3
import shutil
import subprocess
import sys
import os

os.chdir(os.environ['MESON_DIST_ROOT'])
try:
    os.mkdir(os.environ['MESON_DIST_ROOT'] + '/.cargo')
except FileExistsError:
    pass
    
vendorProcess = subprocess.run(['cargo', 'vendor'], check=True, stdout=subprocess.PIPE, text=True)
vendorConfig = vendorProcess.stdout
print(vendorConfig)
with open(os.environ['MESON_DIST_ROOT']+'/.cargo/config', 'a') as f:
    f.write(vendorConfig)
