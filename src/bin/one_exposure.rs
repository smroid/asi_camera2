use std::vec;
use std::time::{Duration, Instant};
use std::thread::sleep;

use image::GrayImage;

use asi_camera2::asi_camera2_sdk;
use asi_camera2::asi_camera2_sdk::ASICamera;

// Simple tool to use single-exposure mode to capture a greyscale image from the
// attached ASI camera.

fn main() {
    let num_cameras = ASICamera::num_connected_asi_cameras();
    if num_cameras == 0 {
        panic!("No camera??");
    }
    if num_cameras > 1 {
        println!("num_cameras: {}; using first camera", num_cameras);
    }

    let camera_info = ASICamera::get_property(0).unwrap();
    let width = camera_info.MaxWidth;
    let height = camera_info.MaxHeight;

    let mut camera = ASICamera::new(camera_info.CameraID);
    camera.open().unwrap();
    camera.init().unwrap();

    // Allocate buffer to receive camera data.
    println!("width/height: {}/{}", width, height);
    let mut pixels = vec![0u8; (width*height).try_into().unwrap()];

    // Set ROI: whole sensor, no binning, greyscale.
    camera.set_roi_format(
        width as i32, height as i32,
        /*bin=*/1, asi_camera2_sdk::ASI_IMG_TYPE_ASI_IMG_RAW8).unwrap();

    let exposure_time_millisec = 10;
    camera.set_control_value(asi_camera2_sdk::ASI_CONTROL_TYPE_ASI_EXPOSURE,
                             exposure_time_millisec * 1000,
                             /*auto=*/false).unwrap();
    // Invert.
    camera.set_control_value(asi_camera2_sdk::ASI_CONTROL_TYPE_ASI_FLIP,
                             asi_camera2_sdk::ASI_FLIP_STATUS_ASI_FLIP_BOTH as i64,
                             /*auto=*/false).unwrap();

    let exp_start = Instant::now();
    camera.start_exposure(/*is_dark=*/false).unwrap();
    sleep(Duration::from_millis(exposure_time_millisec as u64));
    // Note: in single-exposure mode, the ASI camera seems to take ~300ms to reach
    // ASI_EXP_SUCCESS state, even if the camera exposure time is much shorter.
    loop {
        let exp_status = camera.get_exp_status().unwrap();
        if exp_status == asi_camera2_sdk::ASI_EXPOSURE_STATUS_ASI_EXP_WORKING {
            sleep(Duration::from_millis(10));
            continue;
        }
        if exp_status != asi_camera2_sdk::ASI_EXPOSURE_STATUS_ASI_EXP_SUCCESS {
            panic!("Exposure failed with status: {}", exp_status);
        }
        break;
    }
    println!("Elapsed from exposure start: {:?}", exp_start.elapsed());

    let readout_start = Instant::now();
    camera.get_data_after_exp(pixels.as_mut_ptr(), width*height).unwrap();
    println!("Elapsed from readout start: {:?}", readout_start.elapsed());

    // Move 'pixels' into a GrayImage.
    let image = GrayImage::from_raw(width as u32, height as u32, pixels).unwrap();
    image.save("image.jpg").unwrap();
}
