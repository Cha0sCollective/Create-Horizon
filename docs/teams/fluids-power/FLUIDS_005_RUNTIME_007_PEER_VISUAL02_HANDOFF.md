# FLUIDS-005 dedicated peer visual supplement 02

Status: **bounded FLUIDS-005 accepted by Operations on frozen007**. This
is the one same-world high-angle supplement to
`FLUIDS_005_PEER_VISUAL_SUPPLEMENT_PLAN.md`, not a new Pump battery or a change
to the former ambiguous-image HOLD. No game source, control script, frozen pin,
world fluid seed, or item grant changed. Source was frozen007
`f732bd481cfc3d9c4720bc51e4a31e7385e18020`; the driver/runtime source in
the Fluids team stayed at `d53a093fe1a5671a4ccbfa564de0e2e2f9f9b67a`.
The earlier `FLUIDS_005_RUNTIME_007_COLD_HANDOFF.md` remains the numerical
rate, branch, reverse, stop, cold, capacity, and custody evidence. Its original
low-angle peer frames remain visually inconclusive; this supplement supplies
the focused high-angle peer transition only.

## Same-world observation

Reopened only the preserved dedicated `dedicated-d1m1sey4` world as index 02.
Before mutation, the saved Tank/Pump/Pipe/Basin blocks matched; authoritative
Tank held 1000 mB plain water, Basin input/output and item slots were empty,
Pump speed was 0, actor held exactly one `minecraft:bucket`, and peer inventory
was empty. The peer was moved to `(24.3,65,21.3)` after teleport settling and
looked at `(23.5,65.4,20.5)`, actual pitch **47.15860366821289 degrees**. The
first image showed an unobstructed empty Basin interior.

The existing motor at `ScrollValue:16` gave Pump speed 16 after exactly two
frozen server ticks. The bounded 150-tick transfer ended at Tank **0**/Basin
**1000 mB**; actor inventory remained bucket 1. Setting the motor to 0 and
stepping two ticks gave Pump speed 0 with unchanged tanks. After unfreeze and
40 explicit peer waits, Tank 0/Basin 1000/Pump 0 remained and the fixed-pose
peer image visibly showed blue water.

The first ordinary south-face bucket `interact` from the actor's saved
`(25.5,64,23.5)` position returned `ok:true` and `changed:false`, with no
authoritative Basin change. It is a **no-op control attempt**, not a successful
drain or gameplay failure. A single corrected click after moving only the
actor to `(23.5,65,22.5)` used the same held bucket and authoritatively drained
the Basin to 0. Both action ACKs had `changed:false`; the NBT endpoints,
not ACK fields, establish the result. Actor was then moved out of the image.
After another 40 peer waits, Tank 0/Basin 0/Pump 0 and actor bucket 1/peer
inventory empty remained. The final fixed-pose image showed the dark empty
interior. This last 1000 mB endpoint loss is explicitly the **Creative bucket
drain**; no water bucket appeared and no water-item conservation is claimed.

## Images and receipts

All images are 960×600, in
`G:\Create Rusty\.local\native-fluids-005-runtime\dedicated-d1m1sey4`:

| State | File | SHA-256 |
| --- | --- | --- |
| Empty before | `peer-visual-empty-before-02.png` | `04245f306ab420c754160820cd0fe4e0f6cc09b41918c6416794bb8277452dbc` |
| Pump filled | `peer-visual-filled-02.png` | `4a9400763f53ec1a6a7e055b28694a06ac035a604b4dedcb6d14fabb388b044f` |
| Empty after Creative drain | `peer-visual-empty-after-02.png` | `04245f306ab420c754160820cd0fe4e0f6cc09b41918c6416794bb8277452dbc` |

The two empty PNGs are byte identical; the filled PNG visibly contains blue
water in the same Basin interior. Prior low-angle images remain preserved and
ambiguous. Operations directly viewed all three new images and recorded that
bounded visual observation in `operations-peer-visual-review-02.json`, SHA-256
`c21a621307dff3744bd216e1bdefd159ecbf725d8a4908c4187456092cef20f6`.
That receipt covers the images only; numerical, inventory, and exit claims use
the separate raw controls below. Operations' subsequent completion review and
bounded acceptance are recorded below.

Raw control ledger: `calls-02.jsonl`, 113 calls, SHA-256
`24f6315a52ac2e48e15c794472b84be099482ee26187ec770354fb5c510217c1`.
Its first/last raw UTC call timestamps are **2026-11-06 08:12:25.396208+00:00** and
**08:24:03.583645+00:00**, a measured **698.187437-second control interval** including
control overhead and waiting. `session-02.json` SHA-256
`dc2f1a481534397b8929c8a8b772bd24d3a4a663306954dca3ce0efffdd27002`.
Normal save, peer quit, actor quit, and server stop completed; `exit-02.json`
SHA-256 `58ad062a61e5a7ba42c4b1a611259d694c52c1c84bb0f1cf2d0fd1d9f0bda7d5`
records **server 0, actor 0, peer 0**. No game is running; the shared slot was
released before handoff.

Post-close identity check: all **742/742** native source hashes matched the
frozen map `0c97fc70`; binary receipt `2ec57265`, game `fb8ec9b3`, ctl
`170e8660`, prerequisite receipt `43de03a3`, Create JAR `ef87fe57`, all four
base manifests, and all three composed tables matched their frozen digests.
The source sync path alone was not counted as runtime proof; the direct
same-pose empty/blue/empty peer images provide the visual observation.

## Operations completion review and acceptance

On November 6, Operations reported a fresh independent read-only Sol High
`gold_link_review` **CLEAR** and accepted the **bounded FLUIDS-005** result on
frozen `f732bd4`. The review linked the 742-file source map, build receipt and
reviewed driver to the four original numerical streams and their **eight**
normal process exits, then checked this supplement's 113-call ledger, three
peer PNGs and **three** further normal exits. That is **11 qualifying process
exits, all code 0**. Operations also directly viewed and rehashed the new
empty/blue/empty peer images. Its independent review used native-readable
source and receipts; Linux binary and resource byte identity remain supported
by the separate frozen launcher and team post-run hash receipts, not by a
claim that the reviewer independently rehashed Linux files.

This acceptance does not rehabilitate the original low-angle frames or the
earlier held attempts. The first distant bucket click in session 02 remains a
no-op; only the closer second click drained the Basin. The 1000 mB lost in
that Creative drain is a void endpoint with the same empty bucket retained,
not a water-item conservation result. The accepted scope is the water-only
Pump/line behavior and observed peer Basin presentation in the reviewed
finite fixture; it does not qualify broader fluid networks, Survival bucket
pass-through, or a new source/API/schema/host contract.
