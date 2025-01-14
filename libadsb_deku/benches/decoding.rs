use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use adsb_deku::cpr::get_position;
use adsb_deku::deku::prelude::*;
use adsb_deku::{Altitude, CPRFormat, Frame};
use criterion::{criterion_group, criterion_main, Criterion};

fn b_get_position() {
    let odd = Altitude {
        odd_flag: CPRFormat::Odd,
        lat_cpr: 74158,
        lon_cpr: 50194,
        ..Altitude::default()
    };
    let even = Altitude {
        odd_flag: CPRFormat::Even,
        lat_cpr: 93000,
        lon_cpr: 51372,
        ..Altitude::default()
    };

    let position = get_position((&odd, &even)).unwrap();
    assert!((position.latitude - 52.257_202_148_437_5).abs() < f64::EPSILON);
    assert!((position.longitude - 3.919_372_558_593_75).abs() < f64::EPSILON);

    let even = Altitude {
        odd_flag: CPRFormat::Even,
        lat_cpr: 108_011,
        lon_cpr: 110_088,
        ..Altitude::default()
    };
    let odd = Altitude {
        odd_flag: CPRFormat::Odd,
        lat_cpr: 75_050,
        lon_cpr: 36_777,
        ..Altitude::default()
    };
    let position = get_position((&even, &odd)).unwrap();
    assert!((position.latitude - 88.917_474_261_784_96).abs() < f64::EPSILON);
    assert!((position.longitude - 101.011_047_363_281_25).abs() < f64::EPSILON);
}

fn lax_message() {
    // Read from test file and assert display implemented and non panic decode
    let file = File::open("tests/lax-messages.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let len = line.chars().count();
        let hex = &mut line.to_string()[1..len - 1].to_string();
        let bytes = hex::decode(&hex).unwrap();
        // test non panic decode
        let _frame = Frame::from_bytes((&bytes, 0)).unwrap().1;
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("lax_messsages", |b| b.iter(lax_message));
    c.bench_function("get_position", |b| b.iter(b_get_position));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
