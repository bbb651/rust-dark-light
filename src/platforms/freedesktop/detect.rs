use crate::Mode;

pub fn detect() -> Mode {
    futures_lite::future::block_on(super::get_color_scheme())
}
