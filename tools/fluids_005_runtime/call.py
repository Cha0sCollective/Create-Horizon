"""Record one existing control command against one frozen FLUIDS-005 session."""
from pathlib import Path
import argparse
import datetime
import json
import subprocess
import sys
import time

p = argparse.ArgumentParser()
p.add_argument('session', type=Path)
p.add_argument('role', choices=['actor', 'peer'])
p.add_argument('words', nargs=argparse.REMAINDER)
a = p.parse_args()
session = json.loads(a.session.read_text())
ctl = next(path for path in session['files'] if path.endswith('/horizon-ctl'))
assert session['files'][ctl] == session['binary_sha256']['horizon-ctl']
command = [ctl, '--socket', session['processes'][a.role]['socket'], *a.words]
started = time.monotonic()
result = subprocess.run(command, capture_output=True, text=True, timeout=180)
record = {'at': datetime.datetime.now(datetime.timezone.utc).isoformat(),
          'role': a.role, 'words': a.words, 'seconds': round(time.monotonic() - started, 3),
          'code': result.returncode, 'stdout': result.stdout, 'stderr': result.stderr}
with (a.session.parent / f"calls-{session['index']:02d}.jsonl").open('a') as out:
    out.write(json.dumps(record) + '\n')
sys.stdout.write(result.stdout)
sys.stderr.write(result.stderr)
sys.exit(result.returncode)
