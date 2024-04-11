# Bevy physics engine stress test

Specimen are:
- Rapier
- Xpbd (pending rebranding)

This tests a scenario I am interested in:
many static colliders tiling the world, emulating terrain.

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

## Specs
|            |                           |
|------------|---------------------------|
| CPU        | AMD Ryzen 9 3900X 12-Core |
| RAM total  | 16 Gb                     |
| RAM memcpy | 9968 MB/s                 |
| RAM memset | 8592 MB/s                 |

## Results

| Engine | Collider num | Warm up | Mean iteration time |
|--------|--------------|---------|---------------------|
| Rapier | 10_000       | 120 ms  | 4.1 ms              |
| Xpbd   | 10_000       | 69.9 ms | 2.1 ms              |
| Rapier | 250_000      | 5.6 s   | 193 ms              |
| Xpbd   | 250_000      | 2.7 s   | 347 ms              |
| Rapier | 1_000_000    | 25 s    | 854 ms              |
| Xpbd   | 1_000_000    | 13 s    | 2.06 s              |

Warm up times are fine, as they happen once during level loading, but 
iteration time over 16 ms is unacceptable for a real-time physics simulation targeting 60
fps.

The times are around several milliseconds,
when all the colliders are attached to a single entity,
so if the terrain is immutable, this might be a good workaround.

## Analyzing the profile

### Rapier 

Seems like it spends ~48% in the actual step simulation and
~40% of the time in the `writeback_rigid_bodies` system.
There is even a [todo](https://github.com/dimforge/bevy_rapier/blob/6aa960b611b64cdd4d659afcf2fa67429433ee09/src/plugin/systems.rs#L555-L557)
to fix so the system only writes back the changed entities,
which I suspect will reduce the time of this system massively in this particular case,
however seems like the fix requires exposing the "activeness" through the rapier API,
which might be the reason no one picked it up yet.

### Xpbd

Spends 26% of time in the `SpatialQueryPipeline::update`
and 60% in an elusive `call_mut` on an unknown function.
`Samply` shows, that `update_previous_global_transforms` takes 22% of the iteration time and
`transform_to_position` takes up another 15%.
