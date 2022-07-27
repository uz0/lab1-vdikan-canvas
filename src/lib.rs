use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
// use near_sdk::collections::{Vector};
// use near_sdk::json_types::Base64VecU8;
// use near_sdk::serde::{Deserialize, Serialize};
// use near_sdk::{
//     env, near_bindgen, BlockHeight, BorshStorageKey, PanicOnDefault,
// };
// use chrono::{DateTime, Duration, Utc};
// use std::time::{Duration, Instant};

// near_sdk::setup_alloc!();

// RULES:
// empty block -> has 3 neighboring blocks-> live
// live block -> has 2 neighboring blocks -> live
// empty|live -> empty

// ....
// .X..
// .XX.
// ....

// u8 = ........

const WIDTH: usize = 16;
const HEIGHT: usize = 16;

// const FIELD_LEN: usize = (WIDTH / 8) * HEIGHT;

#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub struct CanvasPixel {
    pub color: [u8; 3],
    pub release_timestamp: u64,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Canvas {
    pub field: Vec<CanvasPixel>, // look upon near's Vector here
}

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
}

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
    fn test_new() {
        let context = get_context(false);
        testing_env!(context);
        let _canvas = Canvas::new();

        println!("{}", _canvas.field[0].release_timestamp);
        println!("{}", _canvas.field[0].color[0]);
        println!("{}", _canvas.field[0].color[1]);
        println!("{}", _canvas.field[0].color[2]);
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
