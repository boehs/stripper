use std::collections::VecDeque;

use rand::{Rng, SeedableRng};
use stripper::{
    primitives::color::{ripple, Mix, Rgba},
    Module, Pixels,
};

#[derive(PartialEq)]
enum Timeframe {
    Never,
    Sometimes,
}

struct Rain {
    drops: VecDeque<(usize, f32)>,
    drop_color: Rgba,
    bg_color: Rgba,
    lightning: Timeframe,
}

trait Weather {
    fn simulate(&mut self, i: u32, pixel_size: usize) -> Pixels;
}

impl Weather for Rain {
    fn simulate(&mut self, i: u32, pixel_size: usize) -> Pixels {
        let mut canvas = vec![self.bg_color; pixel_size];
        let mut rng = rand::rngs::SmallRng::from_entropy();
        // Add drop to queue
        if i % 3 == 0 {
            self.drops.push_back((
                rng.gen_range(0..pixel_size),
                rng.gen_range(70..=100) as f32 * 0.01,
            ));
        }
        // Add drops to scene
        for (i, drop) in self.drops.clone().iter().enumerate() {
            canvas[drop.0] = self.bg_color.mix(self.drop_color, drop.1);
            if self.drops[i].1 == 0.00 {
                self.drops.remove(i);
            }
            self.drops[i].1 = self.drops[i].1 - 0.02
        }
        // Add lightning
        if (i % 70) > 19 && self.lightning == Timeframe::Sometimes {
            canvas = canvas
                .iter()
                .map(|&p| {
                    p.mix(
                        Rgba::new(255.0, 255.0, 255.0, 1.0),
                        0.5 - (0.025 * ((i % 70) - 19) as f32),
                    )
                })
                .collect::<Vec<_>>();
        }
        canvas
    }
}

#[derive(PartialEq)]
enum Sun {
    Sunny,
    Mixed,
    Cloudy,
    Overcast,
}

impl Weather for Sun {
    fn simulate(&mut self, i: u32, pixel_size: usize) -> Pixels {
        // Choose bg color
        let bg: Rgba = if self == &Sun::Overcast {
            Rgba::new(220.0, 220.0, 220.0, 1.0)
        } else {
            Rgba::new(107.0, 212.0, 214.0, 1.0)
        };
        let mut canvas = vec![bg; pixel_size];
        // Add sun
        if self == &Sun::Mixed || self == &Sun::Sunny {
            ripple(
                (i % pixel_size as u32) as u16,
                Rgba::new(245.0, 183.0, 112.0, 1.0),
                7,
                &mut canvas,
            );
        }
        // Add clouds
        if self != &Sun::Sunny {
            ripple(20, Rgba::new(255.0, 255.0, 255.0, 1.0), 6, &mut canvas);
            ripple(70, Rgba::new(255.0, 255.0, 255.0, 1.0), 6, &mut canvas);
            ripple(130, Rgba::new(255.0, 255.0, 255.0, 1.0), 6, &mut canvas);
            ripple(180, Rgba::new(255.0, 255.0, 255.0, 1.0), 6, &mut canvas);
        }
        canvas
    }
}

pub struct WeatherD {
    instance: Box<dyn Weather>,
}

impl Module for WeatherD {
    fn update(input: String) -> Self
    where
        Self: Sized,
    {
        WeatherD {
            instance: match input.as_str() {
                "rainy" => Box::new(Rain {
                    drops: vec![].into(),
                    drop_color: Rgba::new(107.0, 137.0, 148.0, 1.0),
                    bg_color: Rgba::new(62.0, 76.0, 93.0, 1.0),
                    lightning: Timeframe::Never,
                }),
                "lightning" => Box::new(Rain {
                    drops: vec![].into(),
                    drop_color: Rgba::new(107.0, 137.0, 148.0, 1.0),
                    bg_color: Rgba::new(62.0, 76.0, 93.0, 1.0),
                    lightning: Timeframe::Sometimes,
                }),
                "sunny" => Box::new(Sun::Sunny),
                "mixed" => Box::new(Sun::Mixed),
                "cloudy" => Box::new(Sun::Cloudy),
                "overcast" => Box::new(Sun::Overcast),
                _ => Box::new(Sun::Sunny),
            },
        }
    }

    fn render(&mut self, i: u32, pixels: &Pixels) -> stripper::ModR {
        stripper::ModR::Pixels(self.instance.simulate(i, pixels.len()))
    }
}
