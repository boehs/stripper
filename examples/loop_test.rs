use palette::rgb::Rgba;
use stripper::{effects::{pulse::{self, Pulse, Beat}, looper::{self, WithResetI}}, Module, primitives::tween};

fn pulse() {
    let mut pulser = pulse::Pulse::new(Pulse {
        first: 0,
        duration: 100,
        intensity: 0.5,
        function: tween::Expo
    });
    let mut pixels: Vec<Rgba> = Vec::new();
    pixels.push(Rgba::new(10.0, 20.0, 30.0, 1.0));
    for i in (0..100) {
        let render = pulser.render(i, &pixels);
        println!("{:?}", render)
    }
}

fn breath() {
    let mut pulser = pulse::Beat::new(Beat {
        first: 0,
        duration: 25,
        intensity: 0.5,
        function: tween::Expo
    });
    let mut pixels: Vec<Rgba> = Vec::new();
    pixels.push(Rgba::new(10.0, 20.0, 30.0, 1.0));
    for i in (0..50) {
        let render = pulser.render(i, &pixels);
        println!("{:?}", render)
    }
}

fn with_looper() {
    let mut pulser = pulse::Pulse::new(Pulse {
        first: 0,
        duration: 5,
        intensity: 0.5,
        function: tween::Expo
    });
    let mut pixels: Vec<Rgba> = Vec::new();
    pixels.push(Rgba::new(10.0, 20.0, 30.0, 1.0));
    let mut pl = looper::WithReset::new(WithResetI {
        fun: |x,pixels| pulser.render(x, &pixels)
    });
    for i in (0..10) {
        let render = pl.render(i,&pixels);
        println!("{:?}", render)
    }
}

fn main() {
    pulse();
    println!("-----");
    breath();
    println!("-----");
    with_looper()
}
