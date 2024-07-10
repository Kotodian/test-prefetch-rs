#![feature(core_intrinsics)]
use std::{intrinsics::prefetch_read_data, time::Instant};
mod buffer;

fn main() {
    let handle = std::thread::spawn(|| {
        let core = core_affinity::CoreId { id: 2 };
        core_affinity::set_for_current(core);
        let data: &[u32] = &[1, 2, 3, 4, 5, 6, 7];
        let mut n_left = data.len();
        let mut data = data.as_ptr();
        let start = Instant::now();
        loop {
            unsafe {
                if n_left >= 4 {
                    prefetch_read_data(data.wrapping_offset(2), 3);
                    prefetch_read_data(data.wrapping_offset(3), 3);
                    println!("4: {}, {}", *data, *data.wrapping_offset(1));
                    data = data.wrapping_add(2);

                    n_left -= 2;
                } else if n_left >= 1 {
                    println!("1: {}", *data);
                    data = data.wrapping_add(1);

                    n_left -= 1;
                } else {
                    break;
                }
            }
        }
        println!("cost nanoseconds: {}", start.elapsed().as_nanos());
    });
    let handle = std::thread::spawn(|| {
        let core = core_affinity::CoreId { id: 2 };
        core_affinity::set_for_current(core);
        let data: &[u32] = &[1, 2, 3, 4, 5, 6, 7];
        let start = Instant::now();
        data.iter().for_each(|v| {
            println!("{}", *v);
        });
        println!("cost nanoseconds: {}", start.elapsed().as_nanos());
    });
    handle.join().unwrap();
}
