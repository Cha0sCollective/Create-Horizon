# Private Mixins commit provenance

These exact UTF-8 Git commit objects preserve the genuine private implementation
and formatter-only child identifiers. `git hash-object -t commit --stdin` over
each file reproduces its filename hash. They contain source history metadata,
not binaries or generated gameplay evidence. The private tool repository is
retained locally; its only upstream remote has publishing disabled.

The unformatted source overlay/tree is also retained by private integration
commit66e4cd0; formatted source is in565bfd3. Both reconstruct from public upstream
e8cd0aa38cffef49cb2e088ef716baa107148c02 and the tracked exact replacements.
Materialized source fingerprints and Git tree IDs are in their source.json.
These records make source provenance independent of the ignored local Git cache.
The official launcher still cannot fetch the private compiler; this does not
provide a generic mod archive installation or public distribution route.
