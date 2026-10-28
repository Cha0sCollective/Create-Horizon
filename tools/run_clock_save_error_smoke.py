"""Actual Game clock-save error propagation in fresh integrated/in-frame worlds.

Blocks only an owned empty world_clocks.dat_new directory. Clock-byte preservation
is not whole-world atomicity, coherent recovery or complete metadata qualification.
"""
import argparse
import importlib.util
import json
import os
from pathlib import Path
import re
import subprocess
import time
import traceback

import run_paired_game_smoke as base


class ClockSession(base.Session):
    def __init__(self, mode, game, resources, expected_revision):
        super().__init__(mode, game, resources)
        self.expected_revision = expected_revision
        self.blocker = None
        self.blocker_identity = None
        self.cleanup_errors = []
        try:
            self.env.pop('HORIZON_SESSION_FILE', None)
            for key, name in [('HOME', 'child-home'), ('USERPROFILE', 'child-home'),
                              ('XDG_CACHE_HOME', 'child-cache')]:
                path = self.scratch / name
                path.mkdir(exist_ok=True)
                self.env[key] = str(path)
            own = Path(__file__).resolve()
            helper = Path(base.__file__).resolve()
            nbt_source = base.ENGINE / 'tools/anvil_dump.py'
            for name, path in [('driver.py', own), ('base-driver.py', helper), ('anvil_dump.py', nbt_source)]:
                (self.out / name).write_bytes(path.read_bytes())
            self.meta.update(driver=str(own), driver_sha256=base.sha(own),
                             base_driver_sha256=base.sha(helper), nbt_driver_sha256=base.sha(nbt_source),
                             expected_revision=expected_revision, session_file_environment_cleared=True,
                             display_runtime_directory=self.env.get('XDG_RUNTIME_DIR'),
                             scope=__doc__, isolated_environment={k: self.env[k] for k in
                                 ['HOME', 'USERPROFILE', 'HORIZON_HOME', 'HORIZON_LAUNCHER_HOME',
                                  'XDG_DATA_HOME', 'XDG_CONFIG_HOME', 'XDG_CACHE_HOME']})
            spec = importlib.util.spec_from_file_location('clock_smoke_nbt', nbt_source)
            self.nbt = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(self.nbt)
            self.preflight()
        except BaseException as error:
            base.dump(self.out / 'environment.json', self.meta)
            base.dump(self.out / 'result.json', {'passed': False, 'phase': 'setup', 'error': str(error),
                      'traceback': traceback.format_exc(), 'checks': self.checks})
            raise

    def preflight(self):
        self.check('expected source revision is a full commit identity',
                   re.fullmatch('[0-9a-f]{40}', self.expected_revision) is not None)
        self.check('frozen executable receipt matches requested source',
                   self.meta['engine_revision'] == self.expected_revision
                   and base.sha(self.game) == self.meta['game_sha256'])
        evidence = {}
        for label, args in [('revision', ['rev-parse', 'HEAD']),
                            ('tree', ['rev-parse', 'HEAD^{tree}']), ('dirty', ['status', '--porcelain'])]:
            command = ['git', '-c', 'core.safecrlf=false', '-c', 'safe.directory=' + str(base.ENGINE),
                       '--no-optional-locks', '-C', str(base.ENGINE), *args]
            result = subprocess.run(command, env=self.env, capture_output=True, text=True, check=False)
            evidence[label] = {'args': command, 'exit_code': result.returncode,
                               'stdout': result.stdout, 'stderr': result.stderr}
            self.check('source identity query succeeded: ' + label, result.returncode == 0)
        base.dump(self.out / 'source-identity.json', evidence)
        self.check('engine source and tree are the exact clean frozen build',
                   evidence['revision']['stdout'].strip() == self.expected_revision
                   and evidence['tree']['stdout'].strip() == self.meta['engine_tree']
                   and not evidence['dirty']['stdout'].strip())
        manifest = self.resources / 'horizon-resources.json'
        prepared = json.loads(manifest.read_text())
        self.check('prepared resource manifest is the pinned 26.3 schema',
                   prepared['minecraft'] == '26.3' and prepared['schema'] == 7)
        self.check('all child homes are inside this fresh scratch run', all(
            Path(value).resolve().is_relative_to(self.scratch.resolve())
            for value in self.meta['isolated_environment'].values()))
        self.check('no inherited account session-file environment reaches any child',
                   'HORIZON_SESSION_FILE' not in self.env)
        self.meta['control_sha256'] = base.sha(base.CTL)
        for name, source in [('build.json', self.game.with_name('build.json')),
                             ('resources-manifest.json', manifest), ('options.txt', self.home / 'options.txt')]:
            (self.out / name).write_bytes(source.read_bytes())
        base.dump(self.out / 'environment.json', self.meta)

    def start(self, phase, expect_refusal=False):
        if expect_refusal:
            raise AssertionError('clock smoke has no package-admission refusal mode')
        self.phase = phase
        self.report = self.out / (phase + '-smoke-report.json')
        self.socket.unlink(missing_ok=True)
        args = [str(self.game), '--resources', str(self.resources), '--world', 'PairedSmoke', '--seed', '0',
                '--creative', '--test-terrain', '--render-distance', '2', '--offscreen',
                '--width', '960', '--height', '540', '--name', 'SmokeTester',
                '--control-socket', str(self.socket), '--smoke-report', str(self.report)]
        if self.mode == 'in-frame':
            args.append('--in-frame')
        base.dump(self.out / (phase + '-command.json'), args)
        self.log = (self.out / (phase + '-console.log')).open('w')
        self.process = subprocess.Popen(args, env=self.env, cwd=self.home,
                                        stdout=self.log, stderr=subprocess.STDOUT)
        deadline = time.monotonic() + 180
        while not self.socket.exists():
            if self.process.poll() is not None or time.monotonic() >= deadline:
                raise RuntimeError('game exited or never opened the isolated control socket')
            time.sleep(0.2)
        self.control('wait-ready', 180, timeout=190)
        self.check(phase + ': requested isolated world opened',
                   self.control('state').get('world') == 'PairedSmoke')

    def clock(self, label):
        path = self.clock_path()
        raw = path.read_bytes()
        (self.out / (label + '-world_clocks.dat')).write_bytes(raw)
        value = self.nbt.plain(self.nbt.read_dat(path), full=True)
        base.dump(self.out / (label + '-clock.json'), value)
        return raw, value['data']['minecraft:overworld']

    def clock_path(self):
        # Validate literal ownership as well as containment: no symlink ancestor
        # may redirect the injection or empty-directory cleanup elsewhere.
        home = self.home.resolve(strict=True)
        scratch = self.scratch.resolve(strict=True)
        if self.home.is_symlink() or not home.is_relative_to(scratch):
            raise AssertionError('game home is not an owned canonical scratch directory')
        current = self.home
        for segment in ['saves', 'PairedSmoke', 'data', 'minecraft']:
            current /= segment
            if current.is_symlink() or not current.is_dir():
                raise AssertionError('clock parent is missing or redirected')
        if current.resolve(strict=True) != home / 'saves/PairedSmoke/data/minecraft':
            raise AssertionError('clock parent escaped its literal owned path')
        path = current / 'world_clocks.dat'
        if path.is_symlink() or not path.is_file():
            raise AssertionError('clock source is not the owned regular baseline file')
        return path

    def install_blocker(self):
        path = self.clock_path().with_name('world_clocks.dat_new')
        try:
            path.lstat()
        except FileNotFoundError:
            pass
        else:
            raise AssertionError('refusing any pre-existing clock _new entry, including a symlink')
        path.mkdir()
        identity = path.lstat()
        self.blocker, self.blocker_identity = path, (identity.st_dev, identity.st_ino)
        base.dump(self.out / 'blocker.json', {'path': str(path), 'device': identity.st_dev,
                                             'inode': identity.st_ino, 'kind': 'owned empty directory'})

    def remove_blocker(self):
        if self.blocker is None:
            return
        expected = self.clock_path().with_name('world_clocks.dat_new')
        stat = self.blocker.lstat()
        if (self.blocker != expected or self.blocker.is_symlink() or not self.blocker.is_dir()
                or (stat.st_dev, stat.st_ino) != self.blocker_identity):
            raise AssertionError('blocker identity changed; refusing cleanup')
        self.blocker.rmdir()  # Only the exact empty directory this harness made.
        self.blocker = None

    def command(self, text):
        value = self.control('command', text)
        output = json.dumps(value.get('chat', []))
        self.check('actual clock command accepted: ' + text,
                   not re.search(r'Unknown|Expected|Incorrect|Invalid|Could not|Unable|Failed|error', output, re.I))

    @staticmethod
    def clock_error(value):
        return isinstance(value, str) and 'world_clocks.dat' in value

    def expected_inframe_exit(self, reply):
        process = self.process
        process.wait(timeout=45)  # A forced exit is never expected-error evidence.
        self.log.close()
        report = json.loads(self.report.read_text()) if self.report.exists() else {}
        console = (self.out / (self.phase + '-console.log')).read_text()
        base.dump(self.out / (self.phase + '-expected-error-exit.json'),
                  {'pid': process.pid, 'exit_code': process.returncode, 'forced': False, 'report': report})
        self.process, self.log = None, None
        self.check('in-frame save failure exits naturally with code one', process.returncode == 1)
        self.check('in-frame failure has clock-specific Result evidence, beyond transport closure',
                   (self.clock_error(reply.get('error')) or self.clock_error(report.get('error')))
                   and 'world_clocks.dat' in console)

    def run(self):
        failure = None
        try:
            self.start('initial')
            self.command('time of minecraft:overworld rate 0.5')
            self.command('time of minecraft:overworld pause')
            self.control('save')
            before, controls = self.clock('baseline')
            self.check('baseline clock persists actual rate and pause controls',
                       controls.get('rate') == 0.5 and controls.get('paused') == 1)
            # A different live value proves same-process repair did not reset it.
            self.command('time of minecraft:overworld rate 0.25')
            self.control('server-hold', 'on')
            self.install_blocker()
            reply = self.control('save', allow_error=True, timeout=90)
            self.check('blocked save never acknowledges success', reply.get('ok') is not True)
            base.dump(self.out / 'expected-save-error.json', reply)
            if self.mode == 'integrated':
                self.check('integrated save returns a clock-specific synchronous error',
                           self.clock_error(reply.get('error')) and self.process.poll() is None)
            else:
                self.expected_inframe_exit(reply)
            after, _ = self.clock('failed-save')
            self.check('failed clock write preserves existing clock bytes exactly', after == before)
            self.remove_blocker()
            if self.mode == 'integrated':
                self.control('save')
                _, repaired = self.clock('repaired')
                self.check('same-process repair writes retained live clock controls',
                           repaired.get('rate') == 0.25 and repaired.get('paused') == 1)
                self.control('server-hold', 'off')
                self.stop()
            else:
                self.start('repair-restart')
                self.control('save')
                _, repaired = self.clock('repaired')
                self.check('repair/reopen reads prior saved controls after expected process death',
                           repaired.get('rate') == 0.5 and repaired.get('paused') == 1)
                self.stop()
        except BaseException as error:
            failure = {'error': str(error), 'traceback': traceback.format_exc()}
        finally:
            # Every process is owned by this run. Failure cleanup cannot turn a
            # missing natural exit or missing Result receipt into a passing run.
            if self.process is not None:
                try:
                    self.stop(success=True)
                except Exception as error:
                    self.cleanup_errors.append(str(error))
            elif self.log is not None:
                self.log.close()
            try:
                self.remove_blocker()
            except Exception as error:
                self.cleanup_errors.append('owned blocker cleanup: ' + str(error))
            base.dump(self.out / 'result.json', {'passed': failure is None and not self.cleanup_errors,
                      'mode': self.mode, 'failure': failure, 'cleanup_errors': self.cleanup_errors,
                      'checks': self.checks, 'driver_sha256': self.meta['driver_sha256'],
                      'base_driver_sha256': self.meta['base_driver_sha256'],
                      'nbt_driver_sha256': self.meta['nbt_driver_sha256'],
                      'game_sha256': self.meta['game_sha256'], 'scope': __doc__,
                      'limitations': ['fault normally reaches the earlier Level clock writer',
                                      'no whole-world byte preservation or coherent checkpoint claim',
                                      'no same-process control retention claim after in-frame process death']})
        if failure or self.cleanup_errors:
            raise RuntimeError('clock smoke failed; inspect ' + str(self.out))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--mode', choices=['integrated', 'in-frame', 'both'], default='both')
    parser.add_argument('--game', type=Path, required=True)
    parser.add_argument('--resources', type=Path, default=base.ENGINE / '.local/horizon-client/26.3')
    parser.add_argument('--expected-revision', required=True)
    args = parser.parse_args()
    for mode in ['integrated', 'in-frame'] if args.mode == 'both' else [args.mode]:
        ClockSession(mode, args.game, args.resources, args.expected_revision).run()


if __name__ == '__main__':
    main()
