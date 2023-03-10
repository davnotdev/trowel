/* Original code[1] by Shane Celis[2] Copyright (c) 2023 Hack Club[3]
   Licensed under the MIT License[4]

   [1]: https://github.com/shanecelis/trowel
   [2]: https://mastodon.gamedev.place/@shanecelis
   [3]: https://hackclub.com
   [4]: https://opensource.org/licenses/MIT
*/

use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{BinaryColorTheme, SimulatorDisplay, Window, OutputSettingsBuilder, SimulatorEvent, sdl2::Keycode};

use crate::{App, Buttons};

/// The `run` function configures the RP2040 peripherals, then runs the app.
pub fn run(
    app: &mut impl App<SimulatorDisplay<Rgb565>, core::convert::Infallible>,
) -> ! {
    let mut display : SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(160, 128));

    // display.set_orientation(&Orientation::Landscape).unwrap();
    display.clear(Rgb565::BLACK).expect("error clearing display");
    // display.set_allow_overdraw(true);
    // display.set_allow_out_of_bounds_drawing(true);

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::Default)
        .build();
    let mut window = Window::new("Sprig Simulator", &output_settings);
    // window.show_static(&display);

    app.init().expect("error initializing");

        // window.update(&display);
    let mut buttons;
    // 'running: loop {
    loop {
        window.update(&display);
        buttons = Buttons::empty();
        for event in window.events() {
            match event {
                SimulatorEvent::KeyDown { keycode, .. } => {
                    match keycode {
                        Keycode::W => buttons |= Buttons::W,
                        Keycode::A => buttons |= Buttons::A,
                        Keycode::S => buttons |= Buttons::S,
                        Keycode::D => buttons |= Buttons::D,
                        Keycode::I => buttons |= Buttons::I,
                        Keycode::J => buttons |= Buttons::J,
                        Keycode::K => buttons |= Buttons::K,
                        Keycode::L => buttons |= Buttons::L,
                        _ => { }
                    }
                },
                SimulatorEvent::Quit => panic!("quit"), //break 'running,
                _ => { }
            }
        }

        app.update(buttons).expect("error updating");
        app.draw(&mut display).expect("error drawing");
        window.update(&display);
    }
}
