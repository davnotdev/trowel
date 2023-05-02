/* Original code[1] by Shane Celis[2] Copyright (c) 2023 Hack Club[3]
   Licensed under the MIT License[4]

   [1]: https://github.com/shanecelis/trowel
   [2]: https://mastodon.gamedev.place/@shanecelis
   [3]: https://hackclub.com
   [4]: https://opensource.org/licenses/MIT
*/
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
use bitflags::bitflags;
use embedded_graphics::{pixelcolor::Rgb565, prelude::DrawTarget};

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub use trowel_macro::add_entry as entry;

#[cfg(any(target_family = "unix", target_family = "windows"))]
pub use trowel_macro::id as entry;

#[cfg(target_family = "wasm")]
pub use trowel_macro::id as entry;

bitflags! {
    pub struct Buttons: u8 {
        const W = 0b00000001;
        const A = 0b00000010;
        const S = 0b00000100;
        const D = 0b00001000;
        const I = 0b00010000;
        const J = 0b00100000;
        const K = 0b01000000;
        const L = 0b10000000;
    }
}

#[derive(Debug)]
pub enum Error {
    DisplayErr,
    AppErr,
    #[cfg(feature = "bmp")]
    BmpErr(tinybmp::ParseError),
    #[cfg(target_family = "wasm")]
    WasmErr(wasm_bindgen::JsValue),
    #[cfg(all(feature = "sdcard", target_arch = "arm"))]
    SdErr(embedded_sdmmc::Error<embedded_sdmmc::SdMmcError>)

}
pub type AppResult = Result<(), Error>;
#[cfg(feature = "sdcard")]
pub use fs::{Mode, FileSys};

pub trait App {
    fn init(&mut self) -> AppResult;
    fn update(&mut self, buttons: Buttons) -> AppResult;
    fn draw<T, E>(&mut self, display: &mut T) -> AppResult
    where
        T: DrawTarget<Color = Rgb565, Error = E>;
}

pub struct JoinApps<A, B>
where
    A: App,
    B: App,
{
    a: A,
    b: B,
}

impl<A, B> App for JoinApps<A, B>
where
    A: App,
    B: App,
{
    fn init(&mut self) -> AppResult {
        self.a.init()?;
        self.b.init()
    }

    fn update(&mut self, buttons: Buttons) -> AppResult {
        self.a.update(buttons)?;
        self.b.update(buttons)
    }

    fn draw<T, E>(&mut self, display: &mut T) -> AppResult
    where
        T: DrawTarget<Color = Rgb565, Error = E>,
    {
        self.a.draw(display)?;
        self.b.draw(display)
    }
}

pub trait AppExt
where
    Self: App + Sized,
{
    fn join<B: App>(self, b: B) -> JoinApps<Self, B> {
        JoinApps { a: self, b }
    }
}

impl<A: App> AppExt for A {}

#[macro_use]
extern crate alloc;
#[cfg(all(target_arch = "arm", target_os = "none"))]
mod sprig;
#[cfg(all(target_arch = "arm", target_os = "none"))]
pub use sprig::{run_with, stdout, file_sys};

mod fps;
mod fs;
pub use fps::FpsApp;

pub mod buffered;

pub mod flipped;


#[cfg(any(target_family = "unix", target_family = "windows"))]
mod pc;
#[cfg(any(target_family = "unix", target_family = "windows"))]
pub use pc::{run_with, stdout, file_sys};

pub fn run(app: impl App + 'static) -> () {
    run_with(move || app);
}

#[cfg(feature = "runty8")]
pub mod runty8;

#[cfg(target_family = "wasm")]
mod wasm;
#[cfg(target_family = "wasm")]
pub use wasm::run_with;
#[cfg(target_family = "wasm")]
pub use wasm::file_sys;
