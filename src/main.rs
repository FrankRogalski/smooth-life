use clap::{Parser, value_parser};
use rand::random;
use raylib::ffi::TraceLogLevel::LOG_WARNING;
use std::sync::LazyLock;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[arg(short, long, default_value_t = 60, value_parser = value_parser!(u32).range(1..))]
    fps: u32,

    #[arg(short, long, default_value_t = 1, value_parser = value_parser!(u32).range(1..))]
    cell_size: u32,

    #[arg(short, long, default_value_t = 5, value_parser = value_parser!(u32).range(1..))]
    inner_radius: u32,

    #[arg(short, long, default_value_t = 10, value_parser = value_parser!(u32).range(1..))]
    outer_radius: u32,

    #[arg(long, default_value_t = 1280, value_parser = value_parser!(u32).range(1..))]
    screen_width: u32,

    #[arg(long, default_value_t = 720, value_parser = value_parser!(u32).range(1..))]
    screen_height: u32,
}

static ARGUMENTS: LazyLock<Arguments> = LazyLock::new(|| {
    let args = Arguments::parse();
    assert!(args.screen_width % args.cell_size == 0);
    assert!(args.screen_height % args.cell_size == 0);
    assert!(args.inner_radius < args.outer_radius);
    assert!(args.screen_height.min(args.screen_width) >= args.outer_radius);
    args
});

fn build_kernel(radius: u32) -> Box<[f32]> {
    let diam = radius * 2;
    let mid = radius as i32;
    let r_pow = mid * mid;
    (0..diam.pow(2))
        .map(|i| {
            let x = (i % diam) as i32;
            let y = (i / diam) as i32;
            if (x - mid).pow(2) + (y - mid).pow(2) <= r_pow {
                1.0
            } else {
                0.0
            }
        })
        .collect()
}

fn main() {
    let mut grid: Box<[f32]> = (0..(ARGUMENTS.screen_width * ARGUMENTS.screen_height))
        .map(|_| random())
        .collect();
    let small_kernel = build_kernel(ARGUMENTS.inner_radius);
    let outer_kernel = build_kernel(ARGUMENTS.outer_radius);
    let (mut rl, thread) = raylib::init()
        .title("")
        .size(
            ARGUMENTS.screen_width as i32,
            ARGUMENTS.screen_height as i32,
        )
        .log_level(LOG_WARNING)
        .build();

    rl.set_target_fps(ARGUMENTS.fps);
    while !rl.window_should_close() {}
}
