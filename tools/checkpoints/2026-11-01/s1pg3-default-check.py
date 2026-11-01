import hashlib,json,os,subprocess,sys,tempfile,time
from pathlib import Path
if sys.platform!='linux' or os.getuid()!=1000:raise RuntimeError('requires Linux arbiter UID1000')
if not sys.flags.isolated or sys.flags.optimize:raise RuntimeError('use python3 -I with active assertions')
ROOT=Path('/mnt/g/Create Rusty');REPO=ROOT/'.local/worktrees/s1-persistent-guest-counter-adopted'
P=ROOT/'.local/s1pg3-default-check';REV='74cafefcdebf4e7adc013c76de7b49886ff06064'
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
SCRIPT=sha(Path(__file__))
ENV=os.environ.copy()
for key in list(ENV):
 if key.startswith(('GIT_','HORIZON_','CARGO_')) or key in {'PYTHONPATH','PYTHONHOME','PYTHONOPTIMIZE',
  'RUST_MIN_STACK','RUSTFLAGS','RUSTC','RUSTDOC','RUSTDOCFLAGS','RUSTC_BOOTSTRAP','RUSTC_WRAPPER','RUSTC_WORKSPACE_WRAPPER',
  'LD_PRELOAD','LD_LIBRARY_PATH','TZ'}:ENV.pop(key,None)
GIT=['git','--git-dir='+str(ROOT/'horizon/.git/worktrees/s1-persistent-guest-counter-adopted'),'--work-tree='+str(REPO),'-c','core.safecrlf=false']
def git(*args):return subprocess.check_output(GIT+list(args),env=ENV,text=True).strip()
def verify():
 assert sha(Path(__file__))==SCRIPT
 assert git('rev-parse','HEAD')==REV and git('rev-parse','HEAD^{tree}')=='d48e644ea999c7241b057626ecc4e2719f513e8a'
 assert git('rev-list','--parents','-n','1',REV).split()==[REV,'e369cac6da8f6e0b604a928fcb2e5233be3cf17c']
 assert not git('status','--porcelain','--untracked-files=all')
 manifest=ROOT/'.local/s1pg3-source-sha256.json'
 assert sha(manifest)=='2035b79ea5c7898770a06abf3e6f13cbf530fc58dc917e54bd4c50021ba7a5ca'
 source=json.loads(manifest.read_text());assert len(source)==27
 for name,value in source.items():assert sha(REPO/name)==value
 assert sha(ROOT/'.local/s1pg3-commit-source-comparison.json')=='bea46a6b897195e17719acb357d252bbf14e4a16b340c3f8751f6cee4d06338b'
 assert sha(ROOT/'.local/s1pg3-compatibility-result.json')=='1f17209faf1ba8c0b929a3c6c5409ca4b59cbac34e8d67256dd16d7600f79cd2'
 assert json.loads((ROOT/'.local/s1pg3-compatibility-result.json').read_text())['verified']
 cleanup=json.loads((ROOT/'.local/s1-storage-cleanup-result.json').read_text())
 assert cleanup['verified'] and len(cleanup['deleted'])==17
 return source
source=verify()
scratch=Path(tempfile.mkdtemp(prefix='s1pg3-default-',dir='/tmp'))
for key,child in {'HOME':'home','USERPROFILE':'home','XDG_DATA_HOME':'data','XDG_CONFIG_HOME':'config','XDG_CACHE_HOME':'cache',
 'TMPDIR':'temp','TMP':'temp','TEMP':'temp','HORIZON_HOME':'game','HORIZON_LAUNCHER_HOME':'launcher','HORIZON_LAUNCHER_TRASH':'trash',
 'HORIZON_LAUNCHER_CLIPBOARD':'clipboard','HORIZON_LAUNCHER_DESKTOP':'desktop'}.items():
 path=scratch/child;path.mkdir(exist_ok=True);ENV[key]=str(path)
ENV.update(GIT_DIR=str(ROOT/'horizon/.git/worktrees/s1-persistent-guest-counter-adopted'),GIT_WORK_TREE=str(REPO),
 CARGO_TARGET_DIR='/home/arbiter/.cache/create-horizon/target-s1pg3',CARGO_BUILD_JOBS='2',CARGO_NET_OFFLINE='true',
 XDG_RUNTIME_DIR='/run/user/1000',HORIZON_NO_AUDIO='1',HORIZON_LAUNCHER_NO_SELF_UPDATE='1')
out=Path(str(P)+'-result.json');log=Path(str(P)+'.log');artifacts=Path(str(P)+'-artifacts.jsonl')
assert not out.exists() and not log.exists() and not artifacts.exists()
cmd=[str(ROOT/'create-horizon/tools/dev.sh'),'cargo','clippy','--locked','--offline','-p','horizon-client',
 '--features','mods,paired-packages','--all-targets','--message-format=json','--','-D','warnings']
record={'verified':False,'candidate':REV,'script_sha256':SCRIPT,'source_manifest_sha256':'2035b79ea5c7898770a06abf3e6f13cbf530fc58dc917e54bd4c50021ba7a5ca',
 'scope':'ONE defaultaudio/controllers alltargetscompile withmods+paired,NOtestcontrol; no tests/game/parity/performance execution','command':cmd}
try:
 start=time.monotonic()
 with artifacts.open('x') as output,log.open('x') as error:
  result=subprocess.run(cmd,cwd=REPO,env=ENV,stdout=output,stderr=error)
 record.update(exit_code=result.returncode,seconds=round(time.monotonic()-start,2),log=str(log),log_sha256=sha(log),artifacts_sha256=sha(artifacts))
 verify();assert result.returncode==0,log.read_text()[-10000:]
 rows=[json.loads(line) for line in artifacts.read_text().splitlines() if line.startswith('{')]
 own=[r for r in rows if r.get('reason')=='compiler-artifact' and '/crates/horizon-client/' in r.get('target',{}).get('src_path','')]
 assert own and {r['target']['name'] for r in own}.issuperset({'horizon_client','horizon-client'})
 for row in own:
  assert set(row['features'])=={'default','audio','controllers','mods','paired-packages'}
  assert row['profile']['debug_assertions'] is True and row['profile']['overflow_checks'] is True and str(row['profile']['opt_level'])=='0'
 record['client_artifacts']=[{'name':r['target']['name'],'kind':r['target']['kind'],'profile':r['profile'],'features':r['features']} for r in own]
 record['verified']=True
finally:out.write_text(json.dumps(record,indent=2)+'\n')
