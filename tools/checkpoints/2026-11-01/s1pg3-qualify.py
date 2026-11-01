import hashlib,json,os,re,shutil,subprocess,sys,tempfile,time
from pathlib import Path
if sys.flags.optimize != 0 or not sys.flags.isolated:raise RuntimeError('use python3 -I with active assertions')
assert sys.platform=='linux' and os.getuid()==1000, 'writer fixtures require Linux arbiter UID1000'
ROOT=Path('/mnt/g/Create Rusty');REPO=ROOT/'.local/worktrees/s1-persistent-guest-counter-adopted'
PREFIX=ROOT/'.local/s1pg3';BASE='e369cac6da8f6e0b604a928fcb2e5233be3cf17c'
RESOURCES=Path('/home/arbiter/.cache/create-horizon/resources/26.3-cf62fe-smoke')
PAIR=ROOT/'.local/h09cs1-counter-pair-fixture'
FEATURES={'mods','paired-packages','native-writer-test-control'}
PATHS=set(["crates/horizon-client/src/counter_trial.rs","crates/horizon-client/src/game/counter_control.rs","crates/horizon-client/src/game/lan.rs","crates/horizon-client/src/game/levels.rs","crates/horizon-client/src/game/levels/session_checkpoint.rs","crates/horizon-client/src/game/levels/session_checkpoint_items.rs","crates/horizon-client/src/game/levels/session_checkpoint_items_tests.rs","crates/horizon-client/src/game/levels/session_checkpoint_registered_tests.rs","crates/horizon-client/src/game/levels/session_checkpoint_tests.rs","crates/horizon-client/src/game/mod_packages.rs","crates/horizon-client/src/game/nether_portal.rs","crates/horizon-client/src/game/peer_contract_snapshot_tests.rs","crates/horizon-client/src/game/persist.rs","crates/horizon-client/src/game/persistent_guest_counter_tests.rs","crates/horizon-client/src/lan/conn.rs","crates/horizon-client/src/lan/login_negotiation.rs","crates/horizon-client/src/lan/login_negotiation_tests.rs","crates/horizon-client/src/lan/mod.rs","crates/horizon-client/src/net.rs","crates/horizon-client/src/save.rs","crates/horizon-client/src/streaming.rs","crates/horizon-client/src/world.rs","crates/horizon-client/src/world/counter_restore.rs","crates/horizon-client/src/world/slot_restore.rs","crates/horizon-client/src/world/slot_restore_tests.rs","crates/horizon-client/src/world_slot_trial.rs","crates/horizon-protocol/src/connection.rs"])
GUEST_PATHS=set(["crates/horizon-client/src/counter_trial.rs","crates/horizon-client/src/world.rs","crates/horizon-client/src/world/counter_restore.rs","crates/horizon-client/src/world/slot_restore.rs","crates/horizon-client/src/world/slot_restore_tests.rs","crates/horizon-client/src/game/counter_control.rs","crates/horizon-client/src/game/levels/session_checkpoint.rs","crates/horizon-client/src/game/levels/session_checkpoint_registered_tests.rs","crates/horizon-client/src/game/persistent_guest_counter_tests.rs","crates/horizon-client/src/world_slot_trial.rs"])
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
SCRIPT_SHA=sha(Path(__file__))
ENV=os.environ.copy()
for key in list(ENV):
    if key.startswith(('GIT_','HORIZON_','CARGO_PROFILE_','CARGO_BUILD_','CARGO_TARGET_')) or key in {
     'PYTHONHOME','PYTHONPATH','PYTHONOPTIMIZE','RUST_MIN_STACK','RUSTFLAGS','CARGO_ENCODED_RUSTFLAGS',
     'RUSTC','RUSTDOC','RUSTDOCFLAGS','CARGO_ENCODED_RUSTDOCFLAGS','RUSTC_BOOTSTRAP',
     'RUSTC_WRAPPER','RUSTC_WORKSPACE_WRAPPER','LD_PRELOAD','LD_LIBRARY_PATH','TZ'}:
        ENV.pop(key,None)
ENV.update(CARGO_TARGET_DIR='/home/arbiter/.cache/create-horizon/target-s1pg3',CARGO_BUILD_JOBS='2',
 CARGO_NET_OFFLINE='true',RUST_TEST_THREADS='1',XDG_RUNTIME_DIR='/run/user/1000')
GIT=['git','--git-dir='+str(ROOT/'horizon/.git/worktrees/s1-persistent-guest-counter-adopted'),
 '--work-tree='+str(REPO),'-c','core.safecrlf=false']
def git(*args):return subprocess.check_output(GIT+list(args),env=ENV,text=True).rstrip('\r\n')
def guard():
    review=json.loads(Path(str(PREFIX)+'-source-review.json').read_text())
    manifest=Path(str(PREFIX)+'-source-sha256.json');source=json.loads(manifest.read_text())
    assert review['status']=='SOURCE CLEAR' and review['base']==BASE
    assert sha(manifest)==review['manifest_sha256'] and set(source)==PATHS and len(PATHS)==27
    assert git('rev-parse','HEAD')==BASE and git('branch','--show-current')=='codex/s1-persistent-guest-counter-adopted'
    assert git('rev-list','--parents','-n','1',BASE).split()==[BASE,'f6a3c6a06f500234ed1b3140c8d7cdd37bb89e6f']
    assert set(git('diff','--cached','--name-only').splitlines())==GUEST_PATHS and not git('diff','--name-only')
    status=git('status','--porcelain','--untracked-files=all').splitlines()
    assert len(status)==10 and all(line[:2] in ('M ','A ') for line in status)
    assert {line[3:] for line in status}==GUEST_PATHS
    adopted=ROOT/'.local/s1wa1-final-review.json';parent=json.loads(adopted.read_text())
    gate=json.loads(Path(str(PREFIX)+'-parent-gate.json').read_text())
    assert parent['status']=='FINAL ACCEPT' and parent['candidate']==BASE
    assert parent['tree']=='459d436b07bc35d32264c4e1def9bca072868ccd'
    assert sha(adopted)==gate['parent_final_receipt_sha256'] and gate['status']=='READY after private checkpoint'
    assert gate['parent']==BASE and re.fullmatch('[0-9a-f]{40}',gate['private_checkpoint'])
    assert gate['source_manifest_sha256']==sha(manifest)
    draft_path=ROOT/'.local/s1pg3-current-guest-sha256.json'
    assert sha(draft_path)=='2ba623d52a3d6376008bc16ccd24b85ea5612c066291b79946195f1ac19c0197'
    draft=json.loads(draft_path.read_text());assert set(draft)==GUEST_PATHS
    adopted_manifest=ROOT/'.local/s1wa1-source-sha256.json'
    assert sha(adopted_manifest)=='702c8448047da0eb0a45a15a3d993b215f64382394fdd26538089c91a93419b5'
    baseline=json.loads(adopted_manifest.read_text())
    for name,digest in source.items():
        path=REPO/name
        assert path.is_file() and not path.is_symlink() and path.resolve()==path
        assert all(not p.is_symlink() for p in path.parents) and sha(path)==digest
        canonical=subprocess.check_output(GIT+['show',':'+name],env=ENV)
        assert path.read_bytes().replace(b'\r\n',b'\n')==canonical
        if name in GUEST_PATHS:assert digest==draft[name]
        else:assert canonical==subprocess.check_output(GIT+['show',BASE+':'+name],env=ENV)
    assert sha(RESOURCES/'horizon-resources.json')=='f35140182f95862e5330fabe4c6a449805f2d83be94dce2f01e770a2ede68ce2'
    receipt=ROOT/'.local/h09cs1-counter-pair-fixture.json'
    assert sha(receipt)=='32c83f6b1fbffe44696f42dd3617b7a6e6836d18a46303f0d21189dcacb940b4'
    fixture_path=ROOT/'.local/s1pg2-pair-fixtures.json'
    assert sha(fixture_path)=='c25aa49c4adf44a02d1ee1bc1149e4c30dd0fa77edfa5c35f68042d5bdbddaad'
    fixtures=json.loads(fixture_path.read_text())
    assert set(fixtures)=={'normal','trap','reject','invalid','fuel'}
    for mode,row in fixtures.items():
        folder=ROOT/row['path'];files={}
        assert folder.resolve()==folder and not folder.is_symlink()
        for path in folder.rglob('*'):
            assert not path.is_symlink()
            if path.is_file():files[path.relative_to(folder).as_posix()]=sha(path)
            else:assert path.is_dir()
        assert len(files)==5 and files==row['files']
    assert fixtures['normal']['path']==str(PAIR.relative_to(ROOT))
    fuel=json.loads((ROOT/'.local/s1gf1-qualify-result.json').read_text())
    assert sha(ROOT/'.local/s1gf1-qualify-result.json')=='410f56d7a5077bfa878f107781e47deed9850546a9eb8c2a04d34bb9b085582e'
    assert fuel['verified'] and fixtures['fuel']['files']==fuel['paired_files']
    preflight=json.loads((ROOT/'.local/s1pg2-fixture-preflight-result.json').read_text())
    assert preflight['verified'] and preflight['fixture_manifest_sha256']==sha(fixture_path)
    assert preflight['script_sha256']=='726fe4b22aae5c2a288bbce4ab53743c742f506fc4149566f777c86421726fd7'
    assert len(preflight['rows'])==3 and {r['variant'] for r in preflight['rows']}=={'trap','reject','invalid'}
    for row in preflight['rows']:
        assert row['exit_code']==0 and row['pair_files']==fixtures[row['variant']]['files']
        assert sha(Path(row['log']))==row['log_sha256']
    assert sha(Path(__file__))==SCRIPT_SHA
    return review['manifest_sha256']

def isolated(label):
    scratch=Path(tempfile.mkdtemp(prefix='s1pg3-'+label+'-',dir='/tmp'));env=ENV.copy()
    for key,child in {'HOME':'home','USERPROFILE':'home','XDG_DATA_HOME':'data','XDG_CONFIG_HOME':'config',
     'XDG_CACHE_HOME':'cache','TMPDIR':'temp','TMP':'temp','TEMP':'temp','HORIZON_HOME':'game',
     'HORIZON_LAUNCHER_HOME':'launcher','HORIZON_LAUNCHER_TRASH':'trash','HORIZON_LAUNCHER_CLIPBOARD':'clipboard',
     'HORIZON_LAUNCHER_DESKTOP':'desktop'}.items():
        path=scratch/child;path.mkdir(exist_ok=True);env[key]=str(path)
    env.update(HORIZON_TEST_RESOURCES=str(RESOURCES),HORIZON_COUNTER_PAIR_FIXTURE=str(PAIR),
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
assert len(sys.argv)==2 and sys.argv[1] in {'build','tests','exceptions','checks','boundaries'}
stage=sys.argv[1];source_sha=guard();env,cwd=isolated(stage)
compile_env=env.copy()
compile_env.update(GIT_DIR=str(ROOT/'horizon/.git/worktrees/s1-persistent-guest-counter-adopted'),GIT_WORK_TREE=str(REPO))
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
        folder=Path(tempfile.mkdtemp(prefix='s1pg3-tests-',dir='/home/arbiter/.cache/create-horizon/bin'))
        record.update(command=cmd,build_log_sha256=sha(errors),artifacts_sha256=sha(artifacts),executed=False)
        for kind,name,source in [('lib','horizon_client','lib.rs'),('bin','horizon-client','main.rs')]:
            matches=[r for r in rows if r.get('reason')=='compiler-artifact' and r.get('target',{}).get('name')==name and r.get('profile',{}).get('test') and r.get('executable')]
            assert len(matches)==1;row=matches[0]
            assert set(row['features'])==FEATURES and row['profile']['debug_assertions'] is True and row['profile']['overflow_checks'] is True and str(row['profile']['opt_level'])=='0'
            assert Path(row['target']['src_path'])==REPO/'crates/horizon-client/src'/source
            binary=folder/(kind+'-tests');shutil.copy2(row['executable'],binary);assert sha(binary)==sha(Path(row['executable']))
            record[kind]={'binary':str(binary),'sha256':sha(binary),'profile':row['profile'],'features':row['features']}
        artifacts=Path(str(PREFIX)+'-protocol-artifacts.jsonl');errors=Path(str(PREFIX)+'-protocol-build.log')
        assert not artifacts.exists() and not errors.exists()
        cmd=[str(ROOT/'create-horizon/tools/dev.sh'),'cargo','test','--locked','--offline','-p','horizon-protocol',
         '--no-default-features','--lib','--no-run','--message-format=json']
        with artifacts.open('x') as output,errors.open('x') as error:
            result=subprocess.run(cmd,cwd=REPO,env=compile_env,stdout=output,stderr=error)
        guard();assert result.returncode==0,errors.read_text()[-10000:]
        rows=[json.loads(line) for line in artifacts.read_text().splitlines() if line.startswith('{')]
        matches=[r for r in rows if r.get('reason')=='compiler-artifact' and r.get('target',{}).get('name')=='horizon_protocol' and r.get('profile',{}).get('test') and r.get('executable')]
        assert len(matches)==1;row=matches[0]
        assert row['features']==[] and row['profile']['debug_assertions'] is True and row['profile']['overflow_checks'] is True and str(row['profile']['opt_level'])=='0'
        assert Path(row['target']['src_path'])==REPO/'crates/horizon-protocol/src/lib.rs'
        binary=folder/'protocol-tests';shutil.copy2(row['executable'],binary);assert sha(binary)==sha(Path(row['executable']))
        record['protocol']={'binary':str(binary),'sha256':sha(binary),'profile':row['profile'],'features':row['features'],
         'command':cmd,'build_log_sha256':sha(errors),'artifacts_sha256':sha(artifacts)}
    elif stage=='tests':
        build_path=Path(str(PREFIX)+'-build-result.json');build=json.loads(build_path.read_text())
        assert build['verified'] and build['source_manifest_sha256']==source_sha and build['script_sha256']==SCRIPT_SHA
        record['build_receipt_sha256']=sha(build_path)
        expected_path=Path(str(PREFIX)+'-test-expected.json');expected=json.loads(expected_path.read_text())
        assert expected['source_manifest_sha256']==source_sha
        assert [len(expected[key]) for key in ['actual','pure','stream']]==[13,8,2]
        expected_hash=sha(expected_path);record['test_expected_sha256']=expected_hash
        groups=[('items-pure','bin','game::levels::session_checkpoint::items::tests::session_items_',True,set(expected['pure'])),
         ('stream-control','lib','streaming::capture_pristine_tests::native_column_load_control_',True,set(expected['stream'])),
         ('slots','lib','world::slot_restore::tests::',False,set(expected['registered_slot'])),
         ('trial','lib','world_slot_trial::tests::',False,set()),('counter','lib','world::counter_restore::tests::',False,set(expected['registered_counter'])),
         ('transfer','lib','slot_transfer::tests::',False,set()),
         ('receivers','bin','game::integrated::transition_ownership_tests::slot_retirement_tests::',True,set()),
         ('rich','bin','game::levels::session_checkpoint::tests::session_actual_persistent_retirement_rich_delivery_and_sink_refusals',True,set()),
         ('empty','bin','game::levels::session_checkpoint::tests::session_actual_persistent_retirement_empty_reconcile_checkpoint_roundtrip',True,set()),
         ('old-epochs','bin','game::levels::session_checkpoint::tests::session_actual_native_game_two_epochs_variable_shapes_fresh_keys_and_body_conservation',True,set())]
        groups[6:6]=[('actual-'+str(index).zfill(2),'bin','game::levels::session_checkpoint::tests::'+name,True,{name})
         for index,name in enumerate(expected['actual'])]
        required={'exact_codec_boundaries_all_truncations_versions_phases_lengths_and_trailing',
         'transcript_binds_each_nonce_profile_transaction_and_both_contracts',
         'actual_independent_capture_matching_receipts_live_then_refused_and_invalidated',
         'actual_tcp_all_nine_remote_mismatch_classes_are_refused',
         'actual_disconnect_before_and_after_every_transition_closes_and_reaps_both_endpoints',
         'required_server_preserves_duplicate_and_never_publishes_guest_or_registry',
         'required_session_silent_unsupported_peer_has_total_first_query_deadline_and_no_join',
         'required_session_and_server_agree_then_refuse_with_no_joined_or_duplicate_event'}
        groups+=[('negotiation','lib','lan::login_negotiation::tests::',True,required),
         ('net','lib','net::tests::',True,{'default_login_keeps_query_none_compression_known_packs_join_and_reconfiguration'}),
         ('captures','bin','game::mod_packages::peer_contract_snapshot_tests::',True,{'actual_captured_pair_materialized_tables_and_frozen_snapshot_controls'}),
         ('protocol','protocol','',True,{'total_read_deadline_stalled_partial_prefix_body_and_trickle_restore_timeout',
          'total_write_deadline_on_observed_full_tcp_send_queue_restores_timeout','expired_deadline_refuses_already_buffered_frame_and_send'})]
        assert len(expected['registered_actual'])==9 and len(expected['registered_counter'])==6 and len(expected['registered_slot'])==5
        groups[:0]=[('registered-'+str(index).zfill(2),'bin',
          'game::levels::session_checkpoint::tests::registered::'+name,True,{name})
          for index,name in enumerate(expected['registered_actual'])]
        # Guest defining cases run first; each actual Game gets its pinned artifact.
        guest_cases=expected['guest']
        assert len(guest_cases['actual'])==14
        pure_frames=guest_cases['normal']['world::counter_restore::persistent_guest_frame_tests::']
        assert len(pure_frames)==3
        groups[:0]=[('guest-frames','lib','world::counter_restore::persistent_guest_frame_tests::',False,set(pure_frames))]
        groups[:0]=[('guest-actual-'+str(index).zfill(2),'bin',row['name'],True,{row['name'].rsplit('::',1)[-1]})
          for index,row in enumerate(guest_cases['actual'])]
        for label,kind,selection,ignored,required in groups:
            binary=Path(build[kind]['binary']);assert sha(binary)==build[kind]['sha256']
            env,cwd=isolated(label)
            if label.startswith('guest-actual-'):
                case=guest_cases['actual'][int(label.rsplit('-',1)[-1])]
                fixtures=json.loads((ROOT/'.local/s1pg2-pair-fixtures.json').read_text())
                env['HORIZON_COUNTER_PAIR_FIXTURE']=str(ROOT/fixtures[case['pair']]['path'])
                if 'other_pair' in case:
                    assert fixtures[case['pair']]['files']['server.wasm']!=fixtures[case['other_pair']]['files']['server.wasm']
                    env['HORIZON_COUNTER_OTHER_PAIR_FIXTURE']=str(ROOT/fixtures[case['other_pair']]['path'])
            cmd=[str(binary),selection,'--test-threads=1','--nocapture']+(['--include-ignored'] if ignored else [])
            log=Path(str(PREFIX)+'-tests-'+label+'.log');row=command(cmd,cwd,env,log,900)
            text=log.read_text();names=re.findall(r'^test (\S+) \.\.\.',text,re.M)
            summaries=re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;',text)
            assert len(summaries)==1 and int(summaries[0][0])==len(names)>0 and summaries[0][1:]==('0','0')
            assert all(name.startswith(selection) for name in names) and len(set(names))==len(names)
            assert required.issubset({name.rsplit('::',1)[-1] for name in names})
            if label.startswith('guest-actual-'):assert names==[selection]
            if label=='guest-frames':assert len(names)==3
            if label=='negotiation':assert len(names)==17
            if label in {'net','captures'}:assert len(names)==5
            if label=='protocol':assert len(names)==107
            if label.startswith('registered-'):assert names==[selection]
            if label=='slots':assert len(names)==18
            if label=='counter':assert len(names)==22
            if label=='items-pure':assert len(names)==8
            if label=='stream-control':assert len(names)==2
            if label.startswith('actual-'):assert names==[selection]
            assert sha(expected_path)==expected_hash
            if label=='receivers':assert len(names)==6
            if label in {'rich','empty','old-epochs'}:assert names==[selection]
            row.update(label=label,test_names=names,binary_sha256=sha(binary));record['rows'].append(row)
            print(label,len(names),'PASS',flush=True)
        prefix='save::retained_writer::retained_writer_tests::'
        writer=expected['writer'];fatal=writer['fatal']
        assert writer['platform']=='linux-arbiter-uid1000'
        assert len(fatal)==21 and len(writer['normal'])==16 and len(writer['resource'])==3
        binary=Path(build['lib']['binary'])
        for label,selection,required in [
         ('retained',prefix,set(writer['normal']+writer['resource'])),
         ('legacy-save','save::tests::',set(writer['legacy_resource']))]:
            guard();assert sha(binary)==build['lib']['sha256']
            env,cwd=isolated(label)
            cmd=[str(binary),selection,'--test-threads=1','--nocapture','--include-ignored']
            if label=='retained':
                for name in fatal:cmd+=['--skip',prefix+name]
            log=Path(str(PREFIX)+'-tests-'+label+'.log');row=command(cmd,cwd,env,log,900)
            text=log.read_text();names=re.findall(r'^test (\S+) \.\.\.',text,re.M)
            assert re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;',text)==[(str(len(names)),'0','0')]
            assert len(set(names))==len(names) and all(name.startswith(selection) for name in names)
            if label=='retained':assert set(names)=={prefix+name for name in required} and len(names)==19
            else:assert len(names)==58 and len(required)==8 and required.issubset(names)
            assert sha(expected_path)==expected_hash and sha(binary)==build['lib']['sha256']
            row.update(label=label,test_names=names,binary_sha256=sha(binary));record['rows'].append(row)
            print(label,len(names),'PASS',flush=True)
        all_names=[name for row in record['rows'] for name in row['test_names']]
        assert len(record['rows'])==53 and len(all_names)==len(set(all_names))==325
    elif stage=='boundaries':
        # Separate required unchanged evaluator-boundary queue: NOT part of325.
        assert git('rev-parse',BASE+':crates/horizon-mods')=='1e2d42f88c86dc1b2fb025ba46fceae5cde7afa1'
        artifacts=Path(str(PREFIX)+'-boundary-artifacts.jsonl');errors=Path(str(PREFIX)+'-boundary-build.log')
        assert not artifacts.exists() and not errors.exists()
        cmd=[str(ROOT/'create-horizon/tools/dev.sh'),'cargo','test','--locked','--offline',
         '-p','horizon-mods','--no-default-features','--features','runtime,paired-packages','--lib','--no-run','--message-format=json']
        with artifacts.open('x') as output,errors.open('x') as error:
            result=subprocess.run(cmd,cwd=REPO,env=compile_env,stdout=output,stderr=error)
        guard();assert result.returncode==0,errors.read_text()[-10000:]
        rows=[json.loads(line) for line in artifacts.read_text().splitlines() if line.startswith('{')]
        matches=[r for r in rows if r.get('reason')=='compiler-artifact' and r.get('target',{}).get('name')=='horizon_mods' and r.get('profile',{}).get('test') and r.get('executable')]
        assert len(matches)==1;row=matches[0]
        assert set(row['features'])=={'runtime','paired-packages'}
        assert row['profile']['debug_assertions'] is True and row['profile']['overflow_checks'] is True and str(row['profile']['opt_level'])=='0'
        assert Path(row['target']['src_path'])==REPO/'crates/horizon-mods/src/lib.rs'
        folder=Path(tempfile.mkdtemp(prefix='s1pg3-boundaries-',dir='/home/arbiter/.cache/create-horizon/bin'))
        binary=folder/'mods-boundary-tests';shutil.copy2(row['executable'],binary)
        assert sha(binary)==sha(Path(row['executable']))
        record['artifact']={'binary':str(binary),'sha256':sha(binary),'profile':row['profile'],'features':row['features'],
         'build_log_sha256':sha(errors),'artifacts_sha256':sha(artifacts),'command':cmd}
        required={
         'actual_request_projection_plus_one_and_fresh_store_discard_guest_globals',
         'imports_missing_and_wrong_exports_refuse_before_trapping_start',
         'async_abi_is_refused_by_the_current_engine_before_start',
         'actual_start_body_realloc_post_return_and_lifting_failures_yield_no_proposal',
         'instantiation_and_step_share_one_unreplenished_fuel_budget',
         'actual_memory_table_and_instance_limits_refuse_excess_and_allow_boundaries',
         'guest_rejection_and_every_invalid_scalar_are_refused_without_mutating_request',
         'request_byte_caps_refuse_before_start_and_bound_returned_diagnostics',
         'captured_rust_pair_selector_requires_matching_live_server_and_cache_identity'}
        selection='runtime::counter_trial::tests::'
        env,cwd=isolated('evaluator-boundaries')
        cmd=[str(binary),selection,'--test-threads=1','--nocapture','--include-ignored']
        log=Path(str(PREFIX)+'-boundary-tests.log');result=command(cmd,cwd,env,log,900)
        text=log.read_text();names=re.findall(r'^test (\S+) \.\.\.',text,re.M)
        assert set(names)=={selection+name for name in required} and len(names)==9
        assert re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;',text)==[('9','0','0')]
        assert sha(binary)==record['artifact']['sha256']
        result.update(test_names=names,binary_sha256=sha(binary));record['rows'].append(result)
        print('evaluator-boundaries9 PASS (separate from325)',flush=True)
    elif stage=='exceptions':
        import resource
        resource.setrlimit(resource.RLIMIT_CORE,(0,0))
        build_path=Path(str(PREFIX)+'-build-result.json');build=json.loads(build_path.read_text())
        assert build['verified'] and build['source_manifest_sha256']==source_sha and build['script_sha256']==SCRIPT_SHA
        record['build_receipt_sha256']=sha(build_path)
        expected_path=Path(str(PREFIX)+'-test-expected.json');expected=json.loads(expected_path.read_text())
        expected_hash=sha(expected_path);assert expected['source_manifest_sha256']==source_sha
        record['test_expected_sha256']=expected_hash
        prefix='save::retained_writer::retained_writer_tests::';fatal=expected['writer']['fatal']
        assert expected['writer']['platform']=='linux-arbiter-uid1000'
        assert len(fatal)==21
        binary=Path(build['lib']['binary'])
        for name,marker in fatal.items():
            guard();assert sha(binary)==build['lib']['sha256']
            env,cwd=isolated(name)
            cmd=[str(binary),prefix+name,'--exact','--test-threads=1','--nocapture','--include-ignored']
            log=Path(str(PREFIX)+'-exceptions-'+name+'.log');assert not log.exists()
            started=time.monotonic()
            with log.open('x') as output:
                result=subprocess.run(cmd,cwd=cwd,env=env,stdout=output,stderr=subprocess.STDOUT,timeout=180)
            guard();text=log.read_text()
            row={'command':cmd,'exit_code':result.returncode,'seconds':round(time.monotonic()-started,2),
             'log':str(log),'log_sha256':sha(log),'marker':'HORIZON_RETAINED_WRITER_ASSERTED:'+marker,
             'binary_sha256':sha(binary),'label':name,'separate_exception':True}
            record['rows'].append(row)
            assert result.returncode==-6 and text.count(row['marker'])==1
            assert not re.search(r'^test result: (ok|FAILED)',text,re.M)
            assert sha(expected_path)==expected_hash and sha(binary)==build['lib']['sha256']
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
         '-p','horizon-protocol','--no-default-features','--lib','--tests','--','-D','warnings'],
         REPO,compile_env,Path(str(PREFIX)+'-checks-protocol.log'))
        record['rows'].append(row)
        row=command(['python3','-I','tools/check_public_tree.py'],REPO,compile_env,Path(str(PREFIX)+'-checks-public.log'))
        record['rows'].append(row)
    guard();record['verified']=True
finally:
    out.write_text(json.dumps(record,indent=2)+'\n')
