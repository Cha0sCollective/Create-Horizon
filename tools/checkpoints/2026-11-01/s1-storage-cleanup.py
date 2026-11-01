import hashlib,json,os,shutil,subprocess,sys,time
from pathlib import Path
if sys.flags.optimize or not sys.flags.isolated:raise RuntimeError('use python3 -I with active assertions')
assert sys.platform=='linux' and os.getuid()==1000
ROOT=Path('/mnt/g/Create Rusty')
CACHE=Path('/home/arbiter/.cache/create-horizon')
NAMES=["target","target-s1bc1","target-s1bc1-ci","target-s1di1","target-s1di1-ci","target-s1gf1-consumer","target-s1gf1-server","target-s1ln1","target-s1ln1-ci","target-s1rb1","target-s1rb1-ci","target-s1wa1","target-s1wa1-ci","target-s1wr1","target-s1wr1-ci","launcher-target","kinetic-probe-target"]
KEEP={'bin','resources','fixtures','runs','checkouts','target-s1pg3','session-tests','vanilla-readback','parity-runs','ci-homes'}
OUT=ROOT/'.local/s1-storage-cleanup-result.json'
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
def guard():
 assert CACHE.resolve()==CACHE and CACHE.is_dir() and not CACHE.is_symlink()
 assert all(not p.is_symlink() for p in CACHE.parents)
 assert len(NAMES)==len(set(NAMES))==17 and not KEEP.intersection(NAMES)
 sync_path=ROOT/'.local/s1-storage-sync-complete.json'
 sync=json.loads(sync_path.read_text())
 assert sync['status']=='VERIFIED remote checkpoints before cleanup'
 assert sync['private_visibility']=='private' and sync['public_reviewed_only'] is True
 assert sync['private_repository']=='https://github.com/Cha0sCollective/Create-Horizon.git'
 assert sync['public_repository']=='https://github.com/Cha0sCollective/Horizon.git'
 assert sync['source_and_evidence_preserved'] is True and sync['active_run_reaped'] is True
 assert sync['cleanup_script_sha256']==sha(Path(__file__))
 assert sync['cleanup_targets']==NAMES
 assert isinstance(sync['preserved_files'],list) and sync['preserved_files'],'nonempty preserved source/evidence manifest required'
 for receipt in sync['preserved_files']:
  path=ROOT/receipt['path']
  assert path.resolve()==path and path.is_file() and sha(path)==receipt['sha256']
 for p in Path('/proc').iterdir():
  if not p.name.isdecimal() or int(p.name)==os.getpid():continue
  try:
   name=(p/'comm').read_text().strip()
   args=(p/'cmdline').read_bytes()
  except (FileNotFoundError,PermissionError,ProcessLookupError):continue
  assert not (name in {'cargo','rustc','rustdoc','clippy-driver','rustfmt'} or
   name.startswith(('horizon','s1gf1-consumer')) or b's1pg3-qualify.py' in args or b's1ra1-qualify.py' in args),('active project process',p.name,name)
 return sync
sync=guard()
assert not OUT.exists() and shutil.rmtree.avoids_symlink_attacks
allowed={'.rustc_info.json','.future-incompat-report.json','CACHEDIR.TAG','debug','release','fast','tmp','paired-guest','wasm32-unknown-unknown'}
inventory=[]
for name in NAMES:
 path=CACHE/name
 assert path.parent==CACHE and path.resolve()==path and path.is_dir() and not path.is_symlink()
 assert (path/'.rustc_info.json').is_file(),('not a verified Cargo target',name)
 entries={p.name for p in path.iterdir()}
 assert entries.issubset(allowed),('unexpected target contents',name,sorted(entries-allowed))
 inventory.append({'name':name,'resolved_path':str(path),'top_entries':sorted(entries)})
record={'verified':False,'script_sha256':sha(Path(__file__)),'sync_receipt_sha256':sha(ROOT/'.local/s1-storage-sync-complete.json'),
 'preserved_directories':sorted(KEEP),'before_free_bytes':shutil.disk_usage(CACHE).free,'targets':inventory,'deleted':[]}
try:
 for row in inventory:
  guard()
  path=Path(row['resolved_path'])
  assert path.parent==CACHE and path.resolve()==path and not path.is_symlink()
  start=time.monotonic()
  shutil.rmtree(path)
  assert not path.exists()
  record['deleted'].append({'name':row['name'],'seconds':round(time.monotonic()-start,2)})
  print('REMOVED_CARGO_CACHE',row['name'],flush=True)
 for name in ['bin','resources','fixtures','runs','checkouts','target-s1pg3']:
  assert (CACHE/name).is_dir()
 record['after_free_bytes']=shutil.disk_usage(CACHE).free
 record['verified']=True
finally:OUT.write_text(json.dumps(record,indent=2)+'\n')
