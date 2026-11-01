import hashlib,json,os,re,shutil,subprocess,sys,tempfile,time
from pathlib import Path
if sys.flags.optimize != 0 or not sys.flags.isolated:raise RuntimeError('use python3 -I with active assertions')
assert sys.platform=='linux' and os.getuid()==1000, 'writer fixtures require Linux arbiter UID1000'
ROOT=Path('/mnt/g/Create Rusty');REPO=ROOT/'.local/worktrees/s1-retained-acquisition'
PREFIX=ROOT/'.local/s1ra1';BASE='e369cac6da8f6e0b604a928fcb2e5233be3cf17c'
RESOURCES=Path('/home/arbiter/.cache/create-horizon/resources/26.3-cf62fe-smoke')
PAIR=ROOT/'.local/h09cs1-counter-pair-fixture'
FEATURES={'mods','paired-packages','native-writer-test-control'}
PATHS=set(["crates/horizon-client/src/save.rs","crates/horizon-client/src/game/world_loading/host_identity/session_authority.rs","crates/horizon-world/src/worldgen/generator.rs"])
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
SCRIPT_SHA=sha(Path(__file__))
ENV=os.environ.copy()
for key in list(ENV):
    if key.startswith(('GIT_','HORIZON_','CARGO_PROFILE_','CARGO_BUILD_','CARGO_TARGET_')) or key in {
     'PYTHONHOME','PYTHONPATH','PYTHONOPTIMIZE','RUST_MIN_STACK','RUSTFLAGS','CARGO_ENCODED_RUSTFLAGS',
     'RUSTC','RUSTDOC','RUSTDOCFLAGS','CARGO_ENCODED_RUSTDOCFLAGS','RUSTC_BOOTSTRAP',
     'RUSTC_WRAPPER','RUSTC_WORKSPACE_WRAPPER','LD_PRELOAD','LD_LIBRARY_PATH','TZ'}:
        ENV.pop(key,None)
ENV.update(CARGO_TARGET_DIR='/home/arbiter/.cache/create-horizon/target-s1ra1',CARGO_BUILD_JOBS='2',
 CARGO_NET_OFFLINE='true',RUST_TEST_THREADS='1',XDG_RUNTIME_DIR='/run/user/1000')
GIT=['git','--git-dir='+str(ROOT/'horizon/.git/worktrees/s1-retained-acquisition'),
 '--work-tree='+str(REPO),'-c','core.safecrlf=false']
def git(*args):return subprocess.check_output(GIT+list(args),env=ENV,text=True).rstrip('\r\n')
def guard():
    manifest=Path(str(PREFIX)+'-source-sha256.json')
    review=json.loads(Path(str(PREFIX)+'-source-review.json').read_text())
    source=json.loads(manifest.read_text())
    assert review['status']=='SOURCE CLEAR' and review['base']==BASE
    assert sha(manifest)==review['manifest_sha256'] and set(source)==PATHS
    assert git('rev-parse','HEAD')==BASE and git('branch','--show-current')=='codex/s1-retained-acquisition'
    assert git('rev-list','--parents','-n','1',BASE).split()==[BASE,'f6a3c6a06f500234ed1b3140c8d7cdd37bb89e6f']
    assert set(git('diff','--cached','--name-only').splitlines())==PATHS and not git('diff','--name-only')
    status=git('status','--porcelain','--untracked-files=all').splitlines()
    assert len(status)==3 and all(line.startswith('M  ') for line in status) and {line[3:] for line in status}==PATHS
    for name,digest in source.items():
        path=REPO/name
        assert path.is_file() and not path.is_symlink() and path.resolve()==path
        assert all(not p.is_symlink() for p in path.parents) and sha(path)==digest
        assert path.read_bytes().replace(b'\r\n',b'\n')==subprocess.check_output(GIT+['show',':'+name],env=ENV)
    final_path=ROOT/'.local/s1wa1-final-review.json'
    assert sha(final_path)=='ef655921d38090dcb834bc43adb1289a460c7ebfe92fbda65da6dec872a39063'
    parent=json.loads(final_path.read_text())
    assert parent['status']=='FINAL ACCEPT' and parent['candidate']==BASE and parent['tree']=='459d436b07bc35d32264c4e1def9bca072868ccd'
    gate=json.loads(Path(str(PREFIX)+'-parent-gate.json').read_text())
    assert gate['status']=='SOURCE ASSIGNMENT RELEASED after private checkpoint'
    assert gate['parent']==BASE and gate['private_checkpoint']=='dd54742b3d1a0136417d0b31d37feb714eb9088a'
    assert gate['proposal_sha256']=='2df6d8fe087ba3f945b6a5c267455002340190c6c3c39d4a2bc418e7057e4a46'
    assert sha(RESOURCES/'horizon-resources.json')=='f35140182f95862e5330fabe4c6a449805f2d83be94dce2f01e770a2ede68ce2'
    for name in ['worldgen/world_presets.json','worldgen/block_flags.json','worldgen/block_tags.json']:
        assert (RESOURCES/name).is_file()
    receipt=ROOT/'.local/h09cs1-counter-pair-fixture.json'
    assert sha(receipt)=='32c83f6b1fbffe44696f42dd3617b7a6e6836d18a46303f0d21189dcacb940b4'
    files={}
    for path in PAIR.rglob('*'):
        assert not path.is_symlink()
        if path.is_file():files[path.relative_to(PAIR).as_posix()]=sha(path)
        else:assert path.is_dir()
    assert files==json.loads(receipt.read_text())['files'] and sha(Path(__file__))==SCRIPT_SHA
    return review['manifest_sha256']

def isolated(label):
    scratch=Path(tempfile.mkdtemp(prefix='s1ra1-'+label+'-',dir='/tmp'));env=ENV.copy()
    for key,child in {'HOME':'home','USERPROFILE':'home','XDG_DATA_HOME':'data','XDG_CONFIG_HOME':'config',
     'XDG_CACHE_HOME':'cache','TMPDIR':'temp','TMP':'temp','TEMP':'temp','HORIZON_HOME':'game',
     'HORIZON_LAUNCHER_HOME':'launcher','HORIZON_LAUNCHER_TRASH':'trash','HORIZON_LAUNCHER_CLIPBOARD':'clipboard',
     'HORIZON_LAUNCHER_DESKTOP':'desktop'}.items():
        path=scratch/child;path.mkdir(exist_ok=True);env[key]=str(path)
    env.update(HORIZON_TEST_RESOURCES=str(RESOURCES),HORIZON_RESOURCES=str(RESOURCES),HORIZON_COUNTER_PAIR_FIXTURE=str(PAIR),
     HORIZON_NO_AUDIO='1',HORIZON_LAUNCHER_NO_SELF_UPDATE='1')
    cwd=scratch/'crates/horizon-client';cwd.mkdir(parents=True)
    return env,cwd
def command(command,cwd,env,log,timeout=None):
    guard();started=time.monotonic();assert not log.exists()
    with log.open('x') as output:
        result=subprocess.run(command,cwd=cwd,env=env,stdout=output,stderr=subprocess.STDOUT,timeout=timeout)
    guard()
    row={'command':command,'exit_code':result.returncode,'seconds':round(time.monotonic()-started,2),
     'log':str(log),'log_sha256':sha(log)}
    if result.returncode:raise RuntimeError(json.dumps(row))
    return row
assert len(sys.argv)==2 and sys.argv[1] in {'build','tests','exceptions','checks'}
stage=sys.argv[1];source_sha=guard();env,cwd=isolated(stage)
compile_env=env.copy()
compile_env.update(GIT_DIR=str(ROOT/'horizon/.git/worktrees/s1-retained-acquisition'),GIT_WORK_TREE=str(REPO))
record={'stage':stage,'source_manifest_sha256':source_sha,'script_sha256':SCRIPT_SHA,'verified':False,'rows':[]}
out=Path(str(PREFIX)+'-'+stage+'-result.json');assert not out.exists()
try:
    if stage=='build':
        # Cargo is supervised to actual completion: no timeout that kills only
        # its parent while compiler children continue in a changed checkout.
        artifacts=Path(str(PREFIX)+'-artifacts.jsonl');errors=Path(str(PREFIX)+'-build.log')
        assert not artifacts.exists() and not errors.exists()
        cmd=[str(ROOT/'create-horizon/tools/dev.sh'),'cargo','test','--locked','--offline','-p','horizon-client',
         '--no-default-features','--features',','.join(sorted(FEATURES)),'--lib','--bin','horizon-client','--no-run','--message-format=json']
        with artifacts.open('x') as output,errors.open('x') as error:
            result=subprocess.run(cmd,cwd=REPO,env=compile_env,stdout=output,stderr=error)
        guard();assert result.returncode==0,errors.read_text()[-10000:]
        rows=[json.loads(line) for line in artifacts.read_text().splitlines() if line.startswith('{')]
        folder=Path(tempfile.mkdtemp(prefix='s1ra1-tests-',dir='/home/arbiter/.cache/create-horizon/bin'))
        record.update(command=cmd,build_log_sha256=sha(errors),artifacts_sha256=sha(artifacts),executed=False)
        for kind,name,source in [('lib','horizon_client','lib.rs'),('bin','horizon-client','main.rs')]:
            matches=[r for r in rows if r.get('reason')=='compiler-artifact' and r.get('target',{}).get('name')==name and r.get('profile',{}).get('test') and r.get('executable')]
            assert len(matches)==1;row=matches[0]
            assert set(row['features'])==FEATURES and row['profile']['debug_assertions'] is True and row['profile']['overflow_checks'] is True and str(row['profile']['opt_level'])=='0'
            assert Path(row['target']['src_path'])==REPO/'crates/horizon-client/src'/source
            binary=folder/(kind+'-tests');shutil.copy2(row['executable'],binary);assert sha(binary)==sha(Path(row['executable']))
            record[kind]={'binary':str(binary),'sha256':sha(binary),'profile':row['profile'],'features':row['features']}
        artifacts=Path(str(PREFIX)+'-world-artifacts.jsonl');errors=Path(str(PREFIX)+'-world-build.log')
        assert not artifacts.exists() and not errors.exists()
        cmd=[str(ROOT/'create-horizon/tools/dev.sh'),'cargo','test','--locked','--offline','-p','horizon-world',
         '--no-default-features','--lib','--no-run','--message-format=json']
        with artifacts.open('x') as output,errors.open('x') as error:
            result=subprocess.run(cmd,cwd=REPO,env=compile_env,stdout=output,stderr=error)
        guard();assert result.returncode==0,errors.read_text()[-10000:]
        rows=[json.loads(line) for line in artifacts.read_text().splitlines() if line.startswith('{')]
        matches=[r for r in rows if r.get('reason')=='compiler-artifact' and r.get('target',{}).get('name')=='horizon_world' and r.get('profile',{}).get('test') and r.get('executable')]
        assert len(matches)==1;row=matches[0]
        assert row['features']==[] and row['profile']['debug_assertions'] is True and row['profile']['overflow_checks'] is True and str(row['profile']['opt_level'])=='0'
        assert Path(row['target']['src_path'])==REPO/'crates/horizon-world/src/lib.rs'
        binary=folder/'world-tests';shutil.copy2(row['executable'],binary);assert sha(binary)==sha(Path(row['executable']))
        record['world']={'binary':str(binary),'sha256':sha(binary),'profile':row['profile'],'features':row['features'],
         'command':cmd,'build_log_sha256':sha(errors),'artifacts_sha256':sha(artifacts)}
    elif stage=='tests':
        build_path=Path(str(PREFIX)+'-build-result.json');build=json.loads(build_path.read_text())
        assert build['verified'] and build['source_manifest_sha256']==source_sha and build['script_sha256']==SCRIPT_SHA
        record['build_receipt_sha256']=sha(build_path)
        expected_path=Path(str(PREFIX)+'-test-expected.json');expected=json.loads(expected_path.read_text())
        expected_hash=sha(expected_path);assert expected['source_manifest_sha256']==source_sha
        record['test_expected_sha256']=expected_hash
        groups=expected['groups'];assert len(groups)==8
        for group in groups:
            label,kind,selection=group['label'],group['kind'],group['selection']
            binary=Path(build[kind]['binary']);assert sha(binary)==build[kind]['sha256']
            env,cwd=isolated(label)
            cmd=[str(binary),selection,'--test-threads=1','--nocapture','--include-ignored']
            if group.get('exact'):cmd+=['--exact']
            for skipped in group.get('skip',[]):cmd+=['--skip',skipped]
            log=Path(str(PREFIX)+'-tests-'+label+'.log');row=command(cmd,cwd,env,log,900)
            text=log.read_text();names=re.findall(r'^test (\S+) \.\.\.',text,re.M)
            assert len(names)==len(set(names))==group['count'] and all(name.startswith(selection) for name in names)
            if group.get('exact'):assert names==[selection]
            assert set(group['required']).issubset(names)
            assert re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;',text)==[(str(group['count']),'0','0')]
            assert not re.search(r'(?i)skipping:|skipping because|skipped due',text),'resource-dependent branch silently skipped'
            assert sha(expected_path)==expected_hash
            row.update(label=label,test_names=names,binary_sha256=sha(binary));record['rows'].append(row)
            print(label,len(names),'PASS',flush=True)
        names=[name for row in record['rows'] for name in row['test_names']]
        assert len(names)==len(set(names))==110
    elif stage=='exceptions':
        import resource
        resource.setrlimit(resource.RLIMIT_CORE,(0,0))
        build_path=Path(str(PREFIX)+'-build-result.json');build=json.loads(build_path.read_text())
        assert build['verified'] and build['source_manifest_sha256']==source_sha and build['script_sha256']==SCRIPT_SHA
        record['build_receipt_sha256']=sha(build_path)
        expected_path=Path(str(PREFIX)+'-test-expected.json');expected=json.loads(expected_path.read_text())
        expected_hash=sha(expected_path);assert expected['source_manifest_sha256']==source_sha
        record['test_expected_sha256']=expected_hash
        fatal=expected['fatal']
        assert expected['platform']=='linux-arbiter-uid1000' and len(fatal)==25
        for name,definition in fatal.items():
            guard();binary=Path(build[definition['kind']]['binary']);assert sha(binary)==build[definition['kind']]['sha256']
            env,cwd=isolated(name)
            cmd=[str(binary),name,'--exact','--test-threads=1','--nocapture','--include-ignored']
            log=Path(str(PREFIX)+'-exceptions-'+name.rsplit('::',1)[-1]+'.log');assert not log.exists()
            started=time.monotonic()
            with log.open('x') as output:
                result=subprocess.run(cmd,cwd=cwd,env=env,stdout=output,stderr=subprocess.STDOUT,timeout=180)
            guard();text=log.read_text()
            row={'command':cmd,'exit_code':result.returncode,'seconds':round(time.monotonic()-started,2),
             'log':str(log),'log_sha256':sha(log),'marker':definition['marker'],
             'binary_sha256':sha(binary),'label':name,'separate_exception':True}
            record['rows'].append(row)
            assert result.returncode==-6 and text.count(row['marker'])==1
            assert not re.search(r'^test result: (ok|FAILED)',text,re.M)
            assert sha(expected_path)==expected_hash and sha(binary)==build[definition['kind']]['sha256']
            print(name,'ASSERTED/SIGABRT/REAPED',flush=True)
    else:
        checks=[('fmt',['fmt','--all','--check'])]
        for label,features in [('on','mods,paired-packages,native-writer-test-control'),('no-runtime','paired-packages'),('off','')]:
            args=['clippy','--locked','--offline','-p','horizon-client','--no-default-features','--lib','--bin','horizon-client','--tests']
            if features:args+=['--features',features]
            checks.append((label,args+['--','-D','warnings']))
        for label,args in checks:
            row=command([str(ROOT/'create-horizon/tools/dev.sh'),'cargo']+args,REPO,compile_env,Path(str(PREFIX)+'-checks-'+label+'.log'))
            record['rows'].append(row);print(label,'PASS',flush=True)
        row=command([str(ROOT/'create-horizon/tools/dev.sh'),'cargo','clippy','--locked','--offline',
         '-p','horizon-world','--no-default-features','--lib','--tests','--','-D','warnings'],
         REPO,compile_env,Path(str(PREFIX)+'-checks-world.log'))
        record['rows'].append(row)
        row=command(['python3','-I','tools/check_public_tree.py'],REPO,compile_env,Path(str(PREFIX)+'-checks-public.log'))
        record['rows'].append(row)
    guard();record['verified']=True
finally:
    out.write_text(json.dumps(record,indent=2)+'\n')
