mod audio;

use std::cmp::Ordering;

use macroquad::prelude::*;

pub trait Sorter {
    fn next(&mut self);
    fn data(&self) -> &[f32];
}

pub struct BogoSort<'a> {
    data: &'a mut [f32],
}

impl<'a> Sorter for BogoSort<'a> {
    fn next(&mut self) {
        if self.data.windows(2).all(|w| {
            w[0].partial_cmp(&w[1]).map(|o| o != Ordering::Greater).unwrap_or(false)
        }) {
            return
        }

        fastrand::shuffle(self.data);
    }

    fn data(&self) -> &[f32] {
        &self.data
    }
}

pub struct BubbleSort<'a> {
    data: &'a mut [f32],
    current: usize,
    max_iter: usize,
}

impl<'a> Sorter for BubbleSort<'a> {
    fn next(&mut self) {
        if self.max_iter == 0 {
            return;
        }
        if self.current >= self.max_iter {
            self.current = 0;
            self.max_iter -= 1;
        }

        if self.data[self.current] > self.data[self.current + 1] {
            self.data.swap(self.current, self.current + 1);
        }

        self.current += 1;
    }

    fn data(&self) -> &[f32] {
        &self.data
    }
}


#[macroquad::main("Sorter")]
async fn main() {
    let mut data = (1..8).into_iter().map(|val| val as f32).collect::<Vec<f32>>();
    fastrand::shuffle(&mut data);
    let data_len = data.len();

    /*let mut sort = BubbleSort {
        max_iter: data.len()-1,
        data: &mut data,
        current: 0,
    };*/

    let mut sort = BogoSort { data: &mut data };


    let mut time = get_time();
    
    loop {

        clear_background(WHITE);

        let rec_width = screen_width() / data_len as f32;

        for (move_right, data_point) in sort.data().iter().enumerate() {
            let rec_height = data_point * 1. / data_len as f32 * screen_height();

            draw_rectangle(rec_width * move_right as f32, screen_height() - rec_height, rec_width, rec_height, BLACK)
        }
        
        if get_time() - time >= 0.01 {
            time = get_time();
        }
        sort.next();

        next_frame().await;
    }
}