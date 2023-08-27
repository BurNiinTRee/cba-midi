#!/usr/bin/env python3
import shutil
import subprocess
import sys



cargo_output = sys.argv[1]
meson_output = sys.argv[2]

cargo_options = sys.argv[3:]


subprocess.run(["cargo", "build", *cargo_options]).check_returncode()

shutil.copy(cargo_output, meson_output)