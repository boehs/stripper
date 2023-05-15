use core::todo;
use embassy_rp::{
    peripherals::PIO0,
    pio::{
        Config, FifoJoin, Pio, ShiftConfig, ShiftDirection, StateMachine,
    },
    relocate::RelocatedProgram,
};
use fixed_macro::fixed;

// Some docs https://learn.adafruit.com/intro-to-rp2040-pio-with-circuitpython?view=all

use super::Runtime;
pub struct PicoInit {
    leng: usize,
}

pub struct Pico<'d> {
    leng: usize,
    sm: StateMachine<'d, PIO0, 0>,
}

impl<'d> Runtime<PicoInit> for Pico<'d> {
    fn new(input: PicoInit) -> Self
    where
        Self: Sized,
    {
        let p = embassy_rp::init(Default::default());
        let Pio {
            mut common,
            mut sm0,
            ..
        } = Pio::new(p.PIO0);

        let side_set = pio::SideSet::new(false, 1, false);
        let mut a: pio::Assembler<32> = pio::Assembler::new_with_side_set(side_set);

        const T1: u8 = 2; // start bit
        const T2: u8 = 5; // data bit
        const T3: u8 = 3; // stop bit
        const CYCLES_PER_BIT: u32 = (T1 + T2 + T3) as u32;

        let mut wrap_target = a.label();
        let mut wrap_source = a.label();
        let mut do_zero = a.label();
        a.set_with_side_set(pio::SetDestination::PINDIRS, 1, 0);
        a.bind(&mut wrap_target);
        // Do stop bit
        a.out_with_delay_and_side_set(pio::OutDestination::X, 1, T3 - 1, 0);
        // Do start bit
        a.jmp_with_delay_and_side_set(pio::JmpCondition::XIsZero, &mut do_zero, T1 - 1, 1);
        // Do data bit = 1
        a.jmp_with_delay_and_side_set(pio::JmpCondition::Always, &mut wrap_target, T2 - 1, 1);
        a.bind(&mut do_zero);
        // Do data bit = 0
        a.nop_with_delay_and_side_set(T2 - 1, 0);
        a.bind(&mut wrap_source);

        let prg = a.assemble_with_wrap(wrap_source, wrap_target);
        let mut cfg = Config::<PIO0>::default();

        let out_pin = common.make_pio_pin(p.PIN_8);

        let relocated = RelocatedProgram::new(&prg);
        cfg.use_program(&common.load_program(&relocated), &[&out_pin]);

        // Clock config, measured in kHz to avoid overflows
        // TODO CLOCK_FREQ should come from embassy_rp
        let clock_freq = fixed!(125_000: U24F8);
        let ws2812_freq = fixed!(800: U24F8);
        let bit_freq = ws2812_freq * CYCLES_PER_BIT;
        cfg.clock_divider = clock_freq / bit_freq;

        // FIFO config
        cfg.fifo_join = FifoJoin::TxOnly;
        cfg.shift_out = ShiftConfig {
            auto_fill: true,
            threshold: 24,
            direction: ShiftDirection::Left,
        };

        sm0.set_config(&cfg);
        sm0.set_enable(true);

        Self {
            sm: sm0,
            leng: input.leng,
        }
    }

    fn display(
        self: &mut Pico<'d>,
        pixels: &crate::Pixels,
    ) -> Result<(), alloc::boxed::Box<dyn core::error::Error>> {
        for pixel in pixels {
            // should be wait push
            self.sm.tx().push(
                (((pixel.green * pixel.alpha * 256.0) as u32) << 24)
                    | (((pixel.red * pixel.alpha * 256.0) as u32) << 16)
                    | (((pixel.red * pixel.alpha * 256.0) as u32) << 8),
            );
        }
        Ok(())
    }

    fn get_number_of_pixels(&self) -> u16 {
        todo!()
    }
}
