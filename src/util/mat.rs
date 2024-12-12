extern crate opencv;

use opencv::core::*;
use opencv::imgproc;
use opencv::imgproc::*;
use opencv::prelude::*;
use opencv::imgcodecs::*;

pub fn calc_integral_image(src: Mat) -> Mat {
    let mut empty_mat:Mat = Mat::default();
    imgproc::integral(&src, empty_mat, 10);
}
