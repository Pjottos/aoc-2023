use clap::Parser;
use dotenvy::dotenv;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d20;
mod d21;
mod d22;
mod d23;
mod d24;
mod d25;
mod harness;

#[derive(Parser, Debug)]
#[command(about)]
struct Args {
    /// Whether to run benchmarks
    #[arg(long, default_value = "false")]
    bench: bool,
    /// Days to run
    #[arg(short)]
    days: Vec<u8>,
    /// Run all days, equivalent to passing 1 through 25 to `days`
    #[arg(long, default_value = "false")]
    all_days: bool,
}

fn main() {
    let args = Args::parse();
    let _ = dotenv();

    let days = if args.all_days {
        0x01FFFFFF
    } else {
        let mut mask = 0u32;
        for day in args.days {
            assert!((1..=25).contains(&day), "invalid day");
            mask |= 1 << day;
        }
        mask
    };

    let mut harness = harness::Harness::new(args.bench);

    if days & (1 << 1) != 0 {
        d01::run(&mut harness);
    }
    if days & (1 << 2) != 0 {
        d02::run(&mut harness);
    }
    if days & (1 << 3) != 0 {
        d03::run(&mut harness);
    }
    if days & (1 << 4) != 0 {
        d04::run(&mut harness);
    }
    if days & (1 << 5) != 0 {
        d05::run(&mut harness);
    }
    if days & (1 << 6) != 0 {
        d06::run(&mut harness);
    }
    if days & (1 << 7) != 0 {
        d07::run(&mut harness);
    }
    if days & (1 << 8) != 0 {
        d08::run(&mut harness);
    }
    if days & (1 << 9) != 0 {
        d09::run(&mut harness);
    }
    if days & (1 << 10) != 0 {
        d10::run(&mut harness);
    }
    if days & (1 << 11) != 0 {
        d11::run(&mut harness);
    }
    if days & (1 << 12) != 0 {
        d12::run(&mut harness);
    }
    if days & (1 << 13) != 0 {
        d13::run(&mut harness);
    }
    if days & (1 << 14) != 0 {
        d14::run(&mut harness);
    }
    if days & (1 << 15) != 0 {
        d15::run(&mut harness);
    }
    if days & (1 << 16) != 0 {
        d16::run(&mut harness);
    }
    if days & (1 << 17) != 0 {
        d17::run(&mut harness);
    }
    if days & (1 << 18) != 0 {
        d18::run(&mut harness);
    }
    if days & (1 << 19) != 0 {
        d19::run(&mut harness);
    }
    if days & (1 << 20) != 0 {
        d20::run(&mut harness);
    }
    if days & (1 << 21) != 0 {
        d21::run(&mut harness);
    }
    if days & (1 << 22) != 0 {
        d22::run(&mut harness);
    }
    if days & (1 << 23) != 0 {
        d23::run(&mut harness);
    }
    if days & (1 << 24) != 0 {
        d24::run(&mut harness);
    }
    if days & (1 << 25) != 0 {
        d25::run(&mut harness);
    }
}
