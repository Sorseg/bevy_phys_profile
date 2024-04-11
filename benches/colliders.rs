mod rapier;
mod xpbd;

use std::time::{Duration, Instant};

use bevy::{
    app::{RunMode, ScheduleRunnerPlugin},
    log::LogPlugin,
    prelude::{App, Time},
    time::TimePlugin,
};
use criterion::{criterion_group, criterion_main, Criterion};
use pprof::{
    criterion::{Output, PProfProfiler},
    flamegraph::{Direction, Options},
};

use crate::{rapier::setup_rapier, xpbd::setup_xpbd};

fn create_bench_world() -> App {
    let mut app = App::new();
    app.add_plugins((
        TimePlugin,
        LogPlugin::default(),
        ScheduleRunnerPlugin {
            run_mode: RunMode::Once,
        },
    ));
    app
}

fn advance_world(app: &mut App) {
    app.world
        .resource_mut::<Time>()
        .advance_by(Duration::from_millis(10));
    app.update();
}

fn colliders(c: &mut Criterion) {
    let specimen: &[(&str, fn(&mut App, usize))] =
        &[("rapier", setup_rapier), ("xpbd", setup_xpbd)];

    for (name, setup_plugin) in specimen {
        for n in [100, 500, 1000] {
            let mut world = create_bench_world();
            setup_plugin(&mut world, n);
            world.finish();
            world.cleanup();

            let mut initial_times = vec![];
            let warmup_length = 5;
            for _ in 0..warmup_length {
                let start = Instant::now();
                world.update();
                initial_times.push(start.elapsed());
            }
            println!("Warm-up times {initial_times:#?}");

            c.bench_function(&format!("{name} with {} colliders", n * n), |b| {
                b.iter(|| advance_world(&mut world));
            });
        }
    }
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(Some({
        let mut o = Options::default();
        o.direction = Direction::Inverted;
        o
    }))));
    targets = colliders
);
criterion_main!(benches);
