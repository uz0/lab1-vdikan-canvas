use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
use std::time::SystemTime;
// use near_sdk::collections::{Vector};
// use near_sdk::json_types::Base64&VecU8;
// use near_sdk::serde::{Deserialize, Serialize};
// use near_sdk::{
//     env, near_bindgen, BlockHeight, BorshStorageKey, PanicOnDefault,
// };
// use chrono::{DateTime, Duration, Utc};
// use std::time::{Duration, Instant};

// near_sdk::setup_alloc!();

const WIDTH: usize = 16;
const HEIGHT: usize = 16;

/// Flatten (x, y) coordinates into an index of a 1-dimensional vector.
// /// Also checks for the limits ant appplies -1 offset.
pub fn coords_to_index(x: usize, y: usize) -> usize {
    (x - 1) * HEIGHT + (y - 1)
}

pub fn get_stime_as_nanos() -> i64 {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    let now_ns = match now {
        Ok(n) => n.as_nanos() as i64,
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };
    now_ns
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
}

/// `Canvas` contract function implementations.
#[near_bindgen]
impl Canvas {
    #[init]
    pub fn new() -> Self {
        Self {
            field: vec![
                CanvasPixel {
                    color: [255u8; 3],
                    release_timestamp: env::block_timestamp(),
                };
                HEIGHT * WIDTH
            ],
        }
    }

    fn get_pixel(&self, x: usize, y: usize) -> &CanvasPixel {
        let pixel = match self.field.get(coords_to_index(x, y)) {
            Some(p) => p,
            None => {
                panic!(
                    "Requested off-limits coordinates {} {}. Canvas limits: ({},{}).",
                    x, y, WIDTH, HEIGHT
                )
            }
        };
        pixel
    }

    pub fn get_pixel_info(&self, x: usize, y: usize) -> String {
        // let pixel = self.get_pixel(x, y);
        let pixel = match self.field.get(coords_to_index(x, y)) {
            Some(p) => p,
            None => {
                panic!(
                    "Requested off-limits coordinates {} {}. Canvas limits: ({},{}).",
                    x, y, WIDTH, HEIGHT
                )
            }
        };

        let release_time = pixel.release_timestamp as i64;
        let system_time = get_stime_as_nanos();
        let time_diff = release_time - system_time;

        let timereport = match time_diff {
            td if td <= 0 => String::from("is free, one can paint it now"),
            td if td > 0 => {
                format!("is held, released in {} seconds", time_diff / 1_000_000_000)
            }
            _ => unreachable!(),
        };

        format!(
            "Pixel({},{}): color({:?}); status: {}.",
            x, y, pixel.color, timereport,
        )
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: [u8; 3]) {
        let index = coords_to_index(x, y);
        let pixel = match self.field.get(index) {
            Some(p) => p,
            None => {
                panic!(
                    "Requested off-limits coordinates {} {}. Canvas limits: ({},{}).",
                    x, y, WIDTH, HEIGHT
                )
            }
        };

        let release_time = pixel.release_timestamp as i64;
        let system_time = get_stime_as_nanos();
        let time_diff = release_time - system_time;

        match time_diff {
            td if td <= 0 => {
                self.field[index] = CanvasPixel {
                    color,
                    release_timestamp: env::block_timestamp() + 5_000_000_000,
                }
            }
            td if td > 0 => {
                panic!(
                    "Pixel({},{}) is held, released in {} seconds",
                    x,
                    y,
                    (time_diff / 1_000_000_000)
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
    use std::time::SystemTime;

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new().is_view(is_view).build()
    }

    #[test]
    fn index_for_coords_within_limits() {
        assert_eq!(coords_to_index(2, 3), 18);
        assert_eq!(coords_to_index(5, 5), 68);
        assert_eq!(coords_to_index(1, 1), 0);
        assert_eq!(coords_to_index(16, 16), 255);
    }

    #[test]
    #[should_panic]
    fn coords_off_limits_should_panic() {
        let _i = coords_to_index(0, 1);
    }

    #[test]
    fn test_new() {
        let context = get_context(false);
        testing_env!(context);
        let canvas = Canvas::new();

        println!("{}", canvas.field[0].release_timestamp);
        println!("{}", canvas.field[0].color[0]);
        println!("{}", canvas.field[0].color[1]);
        println!("{}", canvas.field[0].color[2]);
    }

    #[test]
    fn test_get_pixel_info() {
        let context = get_context(false);
        testing_env!(context);
        let canvas = Canvas::new();

        let x: usize = 2;
        let y: usize = 3;
        println!("{:?}", canvas.get_pixel_info(x, y));
    }

    #[test]
    fn test_set_pixel() {
        let context = get_context(false);
        testing_env!(context);
        let mut canvas = Canvas::new();

        let x: usize = 3;
        let y: usize = 4;
        let color = [10u8, 20u8, 30u8];

        canvas.set_pixel(x, y, color);
        println!("{:?}", canvas.get_pixel_info(x, y));
        let p = canvas.get_pixel(x, y);
        println!("{}", p.release_timestamp);
    }

    #[test]
    fn test_canvas_init_with_system_time() {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
        let now_ns = match now {
            Ok(n) => n.as_nanos() as u64,
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };

        let field = vec![
            CanvasPixel {
                color: [0u8; 3],
                release_timestamp: now_ns,
            };
            HEIGHT * WIDTH
        ];
        let init_canvas = Canvas { field };
        println!("{}", init_canvas.field[0].release_timestamp);
    }

    // use chrono::{DateTime, Duration, NaiveDateTime, Utc};
    // use palette::{Pixel, Srgb};

    // #[test]
    // fn test_board() {
    //     let now = Utc::now();
    //     let ts_millis = now.timestamp_millis();
    //     let field = vec![
    //         CanvasPixel {
    //             color: [0u8; 3],
    //             release_ms: ts_millis
    //         };
    //         HEIGHT * WIDTH
    //     ];
    //     let mut initial_canvas = Canvas { field };
    //     println!("{}", initial_canvas.field[1].release_ms);

    //     let five_minutes_from_now = now.checked_add_signed(Duration::minutes(5));
    //     match five_minutes_from_now {
    //         Some(ts) => {
    //             let ts_millis = ts.timestamp_millis();
    //             initial_canvas.field[1].release_ms = ts_millis;
    //             println!("{}", initial_canvas.field[1].release_ms);
    //         }
    //         None => eprintln!("Release time now overflows!"),
    //     }
    // }

    // #[test]
    // fn test_palette() {
    //     let buffer = [255, 0, 255];
    //     let raw = Srgb::from_raw(&buffer);
    //     assert_eq!(raw, &Srgb::<u8>::new(255u8, 0, 255));
    //     println!("{}", buffer[2]);
    // }

    // #[test]
    // fn test_chrono_checked_dt() {
    //     let now: DateTime<Utc> = Utc::now();
    //     println!("{}", now);

    //     let five_minutes_from_now = now.checked_add_signed(Duration::minutes(5));
    //     match five_minutes_from_now {
    //         Some(x) => println!("{}", x),
    //         None => eprintln!("Release time now overflows!"),
    //     }
    // }

    // #[test]
    // fn test_time_ser_deser_chrono() {
    //     let time = chrono::Utc::now();
    //     let ts_millis = time.timestamp_millis();
    //     println!("timestamp milli {} -> {}", time, ts_millis);

    //     let ts_secs = ts_millis / 1000;
    //     let ts_ns = (ts_millis % 1000) * 1_000_000;
    //     let dt =
    //         DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(ts_secs, ts_ns as u32), Utc);

    //     println!("timestamp milli {} -> {}", dt, dt.timestamp_millis());
    //     let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(ts_secs, 0), Utc);

    //     println!("timestamp milli {} -> {}", dt, dt.timestamp_millis());
    // }

    // #[test]
    // fn test_time() {
    //     let now: DateTime<Utc> = Utc::now();

    //     println!("UTC now is: {}", now);
    //     println!("UTC now in RFC 2822 is: {}", now.to_rfc2822());
    //     println!("UTC now in RFC 3339 is: {}", now.to_rfc3339());
    //     println!(
    //         "UTC now in a custom format is: {}",
    //         now.format("%a %b %e %T %Y")
    //     );
    // }
}
