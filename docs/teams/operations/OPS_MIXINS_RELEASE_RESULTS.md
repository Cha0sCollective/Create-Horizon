# Checked release Mixins CLI — accepted tooling optimization

Source candidate8180771c2a7bd36be541ffdf3e3b7cc079a198a8 passed six focused
Python checks and fresh independent read-only Sol High source review. Operations
adopted it as candidate007 commit7fb999ef42d88f70fb0dfc3da5bc04e4caf3eb7f
only after the real exclusive benchmark below passed. No host/Mixins revision,
gameplay source, permission, packaging support or accepted behavior changed.

The benchmark used frozen005/9651dba, all711 native input files, the same
pristine host/mod inputs and private Mixinsac5d9e7. Its preserved generated-source
archive contains4710 files and100561190 bytes; SHA-256
559be6be3230e84485e375fe6cfea0ffca271aeb384bee579bfdac83336c610c.
Fresh release output had no added, missing or changed files, the same directory
set, and every file was streamed against the archived bytes. Debug binary/marker,
source and input checks remained unchanged. The benchmark exited0.

| Measured command | Wall seconds | Result |
| --- | ---: | --- |
| One-time pinned CLI release build |57.399488|0|
| Release plan |5.645223|0|
| Release apply |97.166758|0|

The CLI reported release transformation97.082654s versus the prior005 debug
transformation165.057513s, about41% less time in this single comparison.
This is a bounded observed result, not a general performance guarantee. Archive
freezing and repeated independent integrity checks are benchmark overhead;
they are not the normal staging command or a gameplay test.

Release CLI SHA-256:
03efc337b522b3ceebd2a998fbdc9a436b4508594fea21bc003c03dcade34717.
Raw logs, timing receipts, complete manifests/comparison and PASS result remain
outside Git at `.local/native-mixins-release-001/`, with baseline and release
output in distinct Linux children. No raw generated source, binary or resource
enters Git. Exact generated-byte equality permits reuse of existing005 gameplay
evidence; no historical battery or additional game replay is required for this
tooling adoption. Candidate007 still needs its own changed-source build and
finite new-family checks.
