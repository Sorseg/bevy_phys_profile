# Bevy physics engine stress test

Specimen are:
- Rapier
- Xpbd (pending rebranding)

This tests a scenario I am interested in:
many static colliders tiling the world.

## Run benches
For regular criterion run, run
```shell
cargo bench
```
To generate the flamegraphs, run

```shell
cargo bench --bench colliders -- --profile-time 60
```

Profile generation will then take (2 engines * 3 sets of colliders) = 6 minutes

## Results
Running on linux on `AMD Ryzen 9 3900X 12-Core`

| Engine | Collider num | Warm up | Mean   |
|--------|--------------|---------|--------|
| Rapier | 10000        | 120 ms  | 4.1 ms |
| Xpbd   | 10000        | 69.9 ms | 2.1 ms |
| Rapier | 250000       | 5.6 s   | 193 ms |
| Xpbd   | 250000       | 2.7 s   | 347 ms |
| Rapier | 1000000      | 25 s    | 854 ms |
| Xpbd   | 1000000      | 13 s    | 2.06 s |

## Analyzing the profile

### Rapier 

Seems like it spends ~48% in the actual step simulation and
~40% of the time in the `writeback_rigid_bodies` system.
There is even a [todo](https://github.com/dimforge/bevy_rapier/blob/6aa960b611b64cdd4d659afcf2fa67429433ee09/src/plugin/systems.rs#L555-L557)
to fix so the system only writes the changed entities,
which I suspect will reduce the time of this system massively in this particular case,
however seems like the fix requires exposing the "activeness" through the rapier API,
which might be the reason no one picked it up yet.

### Xpbd

Spends 26% of time in the `SpatialQueryPipeline::update`
and 60% in an elusive `call_mut` on an unknown function.
