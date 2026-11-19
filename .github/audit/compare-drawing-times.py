import json
import platform
import statistics
import subprocess
import sys
import zipfile
from pathlib import Path

work = Path.cwd()
case = work / '.github/audit/time-drawing.py'
roots = {}
for name in ('previous', 'candidate'):
    wheel, = (work / name).glob('*.whl')
    root = work / (name + '-runtime')
    with zipfile.ZipFile(wheel) as archive:
        archive.extractall(root)
    roots[name] = root

measurements = {name: [] for name in roots}
for name in ('previous','candidate','candidate','previous','candidate','previous'):
    stdout = subprocess.check_output([sys.executable,'-I',str(case),str(roots[name])], text=True)
    measurements[name].append(json.loads(stdout.splitlines()[-1]))
rows = []
for i, datum in enumerate(measurements['candidate'][0]):
    row = {field:datum[field] for field in ('width','height','left','visible','route')}
    for name, runs in measurements.items():
        for run in runs:
            assert all(row[k]==run[i][k] for k in ('width','height','left','visible','route'))
        row[name+'_us'] = statistics.median([n/1000 for run in runs for n in run[i]['ns']])
    rows.append(row)
print(json.dumps({'platform':platform.platform(),'processor':platform.machine(),'median_draw_times':rows,'raw':measurements}),flush=True)
