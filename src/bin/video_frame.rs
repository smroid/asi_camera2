use std::vec;
use std::time::Instant;

use image::GrayImage;

use asi_camera2::asi_camera2_sdk;
use asi_camera2::asi_camera2_sdk::ASICamera;

// Simple tool to use video mode to capture a single greyscale image from the
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

    let exposure_time_millisec = 50;
    camera.set_control_value(asi_camera2_sdk::ASI_CONTROL_TYPE_ASI_EXPOSURE,
                             exposure_time_millisec * 1000,
                             /*auto=*/false).unwrap();
    camera.set_control_value(asi_camera2_sdk::ASI_CONTROL_TYPE_ASI_GAIN,
                             50, /*auto=*/false).unwrap();
    camera.set_control_value(asi_camera2_sdk::ASI_CONTROL_TYPE_ASI_OFFSET,
                             10, /*auto=*/false).unwrap();

    let video_capture_start = Instant::now();
    camera.start_video_capture().unwrap();
    println!("start_video_capture took: {:?}", video_capture_start.elapsed());

    // Capture and discard several frames to get a sense for timings. The first
    // frame takes ~300ms to capture; subsequent frames capture at the exposure
    // interval, down to 30ms or so.
    for i in 0..5 {
        // Play with changing parameters while video is running. Findings:
        // * Changing ROI incurs a ~250ms penalty.
        // * Changing start position incurs no time penalty.
        // * Changing exposure duration incurs no time penalty.
        if i == 3 {
        //     let update_start = Instant::now();
        //     camera.set_control_value(asi_camera2_sdk::ASI_CONTROL_TYPE_ASI_EXPOSURE,
        //                              10 * 1000,
        //                              /*auto=*/false).unwrap();
        //     println!("update took: {:?}", update_start.elapsed());
        }
        // Get the video data. This will block until the currently exposing frame
        // is complete.
        let get_data_start = Instant::now();
        camera.get_video_data(
            pixels.as_mut_ptr(), width*height, /*wait_ms=*/-1).unwrap();
        println!("get_video_data took: {:?}", get_data_start.elapsed());
    }
    let video_capture_stop = Instant::now();
    camera.stop_video_capture().unwrap();
    println!("stop_video_capture took: {:?}", video_capture_stop.elapsed());

    println!("dropped frames: {}", camera.get_dropped_frames().unwrap());

    // Move 'pixels' into a GrayImage.
    let image = GrayImage::from_raw(width as u32, height as u32, pixels).unwrap();
    let save_start = Instant::now();
    image.save("vid_image.jpg").unwrap();
    println!("save took: {:?}", save_start.elapsed());
}
