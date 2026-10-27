# D0 unmodified-engine development baseline

Five approximately 300-second windows completed on October 27, 2026, after a
five-minute warm period. The retained binary matches the S0 build manifest:
Horizon `cf62fe270b243780f9aa86c49b456d9dd9078b42`, SHA-256
`40de598c9b0a495d960a8665258d070ca9432d24820859cdf85607ca6ccf75b9`.
The game exited cleanly with status 0.

Profile: Ryzen 9 7900X / 24 logical CPUs, Ubuntu 24.04 WSL2, Mesa 25.2.8 llvmpipe
software Vulkan, 1920×1080 offscreen, immediate presentation, render distance 2,
simulation distance requested 5, test terrain/seed 0, fixed camera, default
integrated server, no installed/active mod work, audio/telemetry/Discord disabled.
The asset index remains the recorded Prism index 34. All homes/worlds were fresh.
This is B0 context, not the B1 workflow or B2 machinery workload.

| Window | Frames | Average FPS | p95 frame ms | Mean sampled smoothed server tick ms | Max sampled smoothed tick ms |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1 | 15,063 | 50.21 | 45.00 | 0.621 | 2.017 |
| 2 | 13,064 | 43.54 | 56.15 | 0.936 | 1.606 |
| 3 | 12,914 | 43.05 | 56.58 | 0.870 | 2.446 |
| 4 | 14,970 | 49.89 | 40.30 | 0.861 | 2.243 |
| 5 | 14,965 | 49.88 | 50.95 | 0.859 | 1.316 |

Peak process RSS was approximately 1.37 GiB (1,439,380 KiB). Window 1 overlapped
brief private-kernel compilation/checks; it is not a quiet performance sample.
No Horizon compilation ran during these windows. Ordinary desktop/background
activity was not controlled. Keep the individual windows rather than presenting
a precision aggregate or confidence interval.

The tick values are samples of the engine's **smoothed** server statistic, not
raw per-tick percentiles or a demonstrated B2 TPS result. GPU timing comes from a
software renderer; it does not qualify hardware-GPU cost. These results neither
ratify N03/N07 nor establish any added-mod budget. G1/H14/Q04 still need the actual
machine workload, appropriate hardware and quiet repeat/baseline measurements.

Raw evidence is outside Git at parent-workspace
`.local/s0/baseline-runs/a5c799f5-f1d2-42b5-a937-10643766fd71/`: command/environment,
per-frame CPU/GPU arrays, per-window summaries, initial/final state, process usage,
console, quit receipt and exit code. `summary.json` verifies the binary against
the retained S0 build manifest. The earlier aborted run is explicitly separate.

## Fresh five-window repeat after H02b

Run `414b81cf-110c-490d-b50c-78d4536b2d26` repeats the same profile and verified
retained binary after all H02b builds/tests finished. No agent build ran during
warm-up or measurement. Source inspection and documentation continued; ordinary
desktop/background activity was not controlled. A brief chat interruption did not
stop the game or collector. The game exited cleanly with status 0, and no Linux
Cargo, rustc or horizon-client process remained afterward.

| Window | Frames | Average FPS | p95 frame ms | Mean sampled smoothed server tick ms | Max sampled smoothed tick ms |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1 | 15,000 | 50.00 | 43.61 | 0.576 | 1.301 |
| 2 | 12,938 | 43.12 | 56.40 | 0.923 | 2.939 |
| 3 | 12,800 | 42.66 | 57.00 | 0.843 | 2.252 |
| 4 | 15,095 | 50.31 | 40.15 | 0.782 | 1.767 |
| 5 | 15,108 | 50.36 | 51.29 | 0.778 | 1.349 |

Peak process RSS was 1,426,828 KiB, approximately 1.36 GiB. The one-off receipt
check verifies the five-minute warm-up and five measurement durations, frame/CPU/
GPU array counts and finite values, reported FPS arithmetic, retained binary hash,
software renderer/display/server identity, completion marker and clean quit/exit.
`verified-summary.json` retains the extracted values and hashes of raw receipts
in the run directory under parent-workspace `.local/s0/baseline-runs/`.

The earlier run remains separate: its first window also has one more GPU timing
sample than frame/CPU samples, so it does not pass the new strict equal-array-count
receipt check. Its original reported scalar measurements are preserved; it is not
silently relabeled as a fully verified repeat. Use the fresh five-window run for
current B0 context. Neither run qualifies B1/B2, hardware GPU cost, mod overhead,
raw tick percentiles or a release budget.
