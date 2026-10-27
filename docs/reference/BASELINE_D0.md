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
