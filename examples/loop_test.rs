use palette::rgb::Rgba;
use stripper::{effects::pulse::{self, Pulse, Beat}, Module, primitives::tween};

fn pulse() {
    let pulser = pulse::Pulse::new(Pulse {
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
    let pulser = pulse::Beat::new(Beat {
        first: 0,
        duration: 50,
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

fn main() {
    pulse();
    println!("-----");
    breath()
}
