# Private Mixins source overlay

This MIT tool repair is private source-workspace tooling, based on upstream
`e8cd0aa38cffef49cb2e088ef716baa107148c02`. Its genuine private source commit is
`ac5d9e766380d575838c8250f2a7982281c1f44e`, retained in the isolated local tool
repository `.local/private-tools/horizon-mixins-004`. No upstream contribution or
public remote contains this commit. It permits ordinary local imports only for
MODIFY_VARIABLE, preserves imports/coordinates, validates import attributes and
conservatively rejects ambiguous standard-macro identity, including aliases,
globs, self imports and raw identifier bindings. Other hook behavior is unchanged.

`source.json` pins the exact upstream base and complete materialized source by
SHA-256. The digest is sorted UTF-8 relative POSIX paths, each followed by NUL
and the fixed 32-byte SHA-256 of that file. Only the root `.revision` marker is
omitted. This differs explicitly from Transport's separate all236-file evidence
map, which includes that marker and uses path/TAB/hex-digest/LF records.
`setup_mixins.py` copies the immutable base, checks original file hashes, applies
these three exact replacements and verifies the full result before compiling.
Compatibility pins the manifest digest as well as the private commit. The original
upstream snapshot is never patched. These files retain the upstream MIT license.

The official launcher fetches Mixins revisions only from the official repository;
it cannot fetch this private commit. Generic native archive packaging therefore
refuses this source profile. Local pinned source staging/build and private game
qualification are supported by the workspace builder. No distribution or release
support is claimed. Tool source/tests were independently reviewed; the focused
variables group passed8 tests (compile17.534s, tests14.695s), with unchanged source.
The tests used its semantic parent `f7a9815256d82e77f8cac3b8642a651f7c9c19fe`;
the current child differs only by pinned-rustfmt layout in two files. Exact Create
hook staging/build and gameplay remain separate qualification gates.
