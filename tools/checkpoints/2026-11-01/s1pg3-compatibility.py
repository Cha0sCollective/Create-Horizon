import hashlib,json,os,re,subprocess,sys,tempfile,time
from pathlib import Path
if sys.platform!='linux' or os.getuid()!=1000:raise RuntimeError('requires Linux arbiter UID1000')
if not sys.flags.isolated or sys.flags.optimize:raise RuntimeError('use python3 -I with active assertions')
ROOT=Path('/mnt/g/Create Rusty');PREFIX=ROOT/'.local/s1pg3-compatibility'
REPO=ROOT/'.local/worktrees/s1-persistent-guest-counter-adopted'
RES=Path('/home/arbiter/.cache/create-horizon/resources/26.3-cf62fe-smoke')
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
SCRIPT=sha(Path(__file__))
SOURCE='2035b79ea5c7898770a06abf3e6f13cbf530fc58dc917e54bd4c50021ba7a5ca'
build_path=ROOT/'.local/s1pg3-build-result.json'
assert sha(build_path)=='6657a01523735a9e5dc2ff64f23cdb2eb0cc1786fa7fc8e9a244cebd3ac19e7a'
build=json.loads(build_path.read_text());assert build['verified'] and build['source_manifest_sha256']==SOURCE
manifest=ROOT/'.local/s1pg3-source-sha256.json';assert sha(manifest)==SOURCE
source=json.loads(manifest.read_text());assert len(source)==27
groups=[
 ('transient-guest','lib','counter_trial::tests::guest_',[
 'guest_binding_uses_captured_owner_type_block_and_identity',
 'guest_phase_commits_once_without_native_fallback_and_preserves_rewind',
 'guest_prepare_and_completion_respect_fresh_authority_and_storage',
 'guest_invalid_replies_and_failures_latch_without_losing_committed_fields',
 'guest_stale_results_cannot_commit_or_fault_another_record_or_configuration',
 'guest_rechecks_actual_block_during_prepare_and_completion',
 'guest_and_native_precedence_preserve_duplicate_mismatch_and_overflow']),
 ('projection-receipt','bin','game::counter_control::tests::',[
 'guest_receipt_bounds_and_scalar_proposal_conversion_preserve_values',
 'captured_counter_token_projection_and_native_snapshot_shape'])]
def verify():
 assert sha(Path(__file__))==SCRIPT and sha(manifest)==SOURCE and sha(build_path)=='6657a01523735a9e5dc2ff64f23cdb2eb0cc1786fa7fc8e9a244cebd3ac19e7a'
 for name,value in source.items():assert sha(REPO/name)==value
 assert sha(RES/'horizon-resources.json')=='f35140182f95862e5330fabe4c6a449805f2d83be94dce2f01e770a2ede68ce2'
 for stage,count in [('tests',53),('exceptions',21),('checks',6),('boundaries',1)]:
  receipt=json.loads((ROOT/('.local/s1pg3-'+stage+'-result.json')).read_text())
  assert receipt['verified'] and receipt['source_manifest_sha256']==SOURCE and len(receipt['rows'])==count
 for kind in ['lib','bin']:assert sha(Path(build[kind]['binary']))==build[kind]['sha256']
verify()
out=Path(str(PREFIX)+'-result.json');assert not out.exists()
record={'verified':False,'source_manifest_sha256':SOURCE,'script_sha256':SCRIPT,'build_receipt_sha256':sha(build_path),
 'scope':'7transientGuest plus2binprojection/receipt; currentpinnedartifacts; separatefrom325and9evaluator','rows':[]}
try:
 for label,kind,selection,required in groups:
  env=os.environ.copy()
  for key in list(env):
   if key.startswith(('GIT_','HORIZON_','CARGO_')) or key in {
    'PYTHONPATH','PYTHONHOME','PYTHONOPTIMIZE','RUST_MIN_STACK','RUSTFLAGS','RUSTC','RUSTDOC',
    'RUSTDOCFLAGS','RUSTC_BOOTSTRAP','RUSTC_WRAPPER','RUSTC_WORKSPACE_WRAPPER','LD_PRELOAD','LD_LIBRARY_PATH','TZ'}:env.pop(key,None)
  scratch=Path(tempfile.mkdtemp(prefix='s1pg3-compat-'+label+'-',dir='/tmp'))
  for key,child in {'HOME':'home','USERPROFILE':'home','XDG_DATA_HOME':'data','XDG_CONFIG_HOME':'config','XDG_CACHE_HOME':'cache',
   'TMPDIR':'temp','TMP':'temp','TEMP':'temp','HORIZON_HOME':'game','HORIZON_LAUNCHER_HOME':'launcher','HORIZON_LAUNCHER_TRASH':'trash',
   'HORIZON_LAUNCHER_CLIPBOARD':'clipboard','HORIZON_LAUNCHER_DESKTOP':'desktop'}.items():
   path=scratch/child;path.mkdir(exist_ok=True);env[key]=str(path)
  env.update(XDG_RUNTIME_DIR='/run/user/1000',HORIZON_TEST_RESOURCES=str(RES),HORIZON_RESOURCES=str(RES),
   HORIZON_COUNTER_PAIR_FIXTURE=str(ROOT/'.local/h09cs1-counter-pair-fixture'),HORIZON_NO_AUDIO='1',HORIZON_LAUNCHER_NO_SELF_UPDATE='1')
  log=Path(str(PREFIX)+'-'+label+'.log');assert not log.exists()
  args=[build[kind]['binary'],selection,'--test-threads=1','--nocapture','--include-ignored'];start=time.monotonic()
  with log.open('x') as output:result=subprocess.run(args,cwd=scratch,env=env,stdout=output,stderr=subprocess.STDOUT,timeout=300)
  text=log.read_text();names=re.findall(r'^test (\S+) \.\.\.',text,re.M)
  row={'label':label,'command':args,'binary_sha256':build[kind]['sha256'],'log':str(log),'log_sha256':sha(log),
   'exit_code':result.returncode,'seconds':round(time.monotonic()-start,2),'test_names':names}
  record['rows'].append(row)
  assert result.returncode==0 and len(names)==len(set(names))==len(required)
  expected={('counter_trial::tests::' if kind=='lib' else selection)+name for name in required}
  assert set(names)==expected and re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;',text)==[(str(len(required)),'0','0')]
  print(label,len(names),'PASS',flush=True)
 verify();record['verified']=True
finally:out.write_text(json.dumps(record,indent=2)+'\n')
