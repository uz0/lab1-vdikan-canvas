use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen};
// use near_sdk::collections::{Vector};
// use near_sdk::json_types::Base64&VecU8;
// use near_sdk::serde::{Deserialize, Serialize};
// use near_sdk::{
//     env, near_bindgen, BlockHeight, BorshStorageKey, PanicOnDefault,
// };

// near_sdk::setup_alloc!();

pub const CONST_BOOKING_TIME: u64 = 30_000_000_000; // half a minute

/// Flatten (x, y) coordinates into an index of a 1-dimensional vector.
/// Also checks for the limits ant appplies -1 offset.
pub fn coords_to_index(x: usize, y: usize, xlim: usize, ylim: usize) -> usize {
    assert!(
        (1..=xlim).contains(&x),
        "Requested off-limits X = {}. Canvas shape: ({},{}).",
        x,
        xlim,
        ylim
    );
    assert!(
        (1..=xlim).contains(&y),
        "Requested off-limits Y = {}. Canvas shape: ({},{}).",
        y,
        xlim,
        ylim
    );

    (x - 1) * ylim + (y - 1)
}

/// Structure of a single pixel that form a `Canvas`.
#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub struct CanvasPixel {
    pub color: [u8; 3],
    pub release_timestamp: u64,
}

/// `Canvas` contract state definition.
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Canvas {
    pub field: Vec<CanvasPixel>, // look upon near's Vector here
    pub width: usize,
    pub height: usize,
}

/// `Canvas` contract function implementations.
#[near_bindgen]
impl Canvas {
    #[init]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            field: vec![
                CanvasPixel {
                    color: [255u8; 3],
                    release_timestamp: env::block_timestamp(),
                };
                width * height
            ],
            width,
            height,
        }
    }

    /// Return the status of the pixel at (`x`,`y`):
    /// current `color` and if it is booked or free based on `release_timestamp`.  
    pub fn get_pixel(&self, x: usize, y: usize) -> String {
        let index = coords_to_index(x, y, self.width, self.height);
        let pixel = self.field.get(index).unwrap();

        let release_time = pixel.release_timestamp as i64;
        let block_time = env::block_timestamp() as i64;
        let time_diff = release_time - block_time;

        let pixel_status = match time_diff {
            td if td <= 0 => String::from("is free, one can paint it now"),
            td if td > 0 => {
                format!("is held, released in {} seconds", time_diff / 1_000_000_000)
            }
            _ => unreachable!(),
        };

        format!(
            "Pixel({},{}): color({:?}); status: {} (rel_ts={}).",
            x, y, pixel.color, pixel_status, pixel.release_timestamp,
        )
    }

    /// Repaint pixel at (`x`,`y`) to a new `color`,
    /// if it is not held at the time of transaction.
    /// At the moment, it gets locked for the same `CONST_BOOKING_TIME`.
    pub fn set_pixel(&mut self, x: usize, y: usize, color: [u8; 3]) {
        let index = coords_to_index(x, y, self.width, self.height);
        let pixel = self.field.get(index).unwrap();

        let release_time = pixel.release_timestamp as i64;
        let block_time = env::block_timestamp() as i64;
        let time_diff = release_time - block_time;

        match time_diff {
            td if td <= 0 => {
                let new_release_time = env::block_timestamp() + CONST_BOOKING_TIME;

                self.field[index] = CanvasPixel {
                    color,
                    release_timestamp: new_release_time,
                };
                log!(
                    "Pixel is booked: at ({},{}), color:{:?}, released after {} seconds (rel_ts={})",
                    x,
                    y,
                    color,
                    CONST_BOOKING_TIME / 1_000_000_000,
                    new_release_time
                );
            }
            td if td > 0 => {
                panic!(
                    "Pixel({},{}) is held, released in {} seconds (rel_ts={}).",
                    x,
                    y,
                    (time_diff / 1_000_000_000), // Tell in seconds to the end user
                    pixel.release_timestamp,
                )
            }
            _ => unreachable!(),
        };
    }
}

/// Tests for `Canvas` contract.
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new().is_view(is_view).build()
    }

    #[test]
    fn test_new() {
        let context = get_context(false);
        testing_env!(context);
        let canvas = Canvas::new(32, 32);

        println!("{}", canvas.field[0].release_timestamp);
        println!("{}", canvas.field[0].color[0]);
        println!("{}", canvas.field[0].color[1]);
        println!("{}", canvas.field[0].color[2]);
    }

    #[test]
    fn test_get_pixel() {
        let context = get_context(false);
        testing_env!(context);
        let canvas = Canvas::new(16, 16);

        let x: usize = 2;
        let y: usize = 3;
        println!("{:?}", canvas.get_pixel(x, y));
    }

    #[test]
    fn test_set_pixel() {
        let context = get_context(false);
        testing_env!(context);
        let mut canvas = Canvas::new(20, 20);

        let x: usize = 3;
        let y: usize = 4;
        let color = [10u8, 20u8, 30u8];

        canvas.set_pixel(x, y, color);
        println!("{:?}", canvas.get_pixel(x, y));
    }
}
