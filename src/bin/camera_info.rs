use std::ffi::CStr;

use asi_camera2::asi_camera2_sdk;
use asi_camera2::asi_camera2_sdk::ASICamera;

// Simple tool to enumerate the attached ASI camera(s) and print information
// about each.

fn main() {
    let num_cameras = ASICamera::num_connected_asi_cameras();
    println!("num_cameras: {}", num_cameras);
    for cam_index in 0..num_cameras {
        let mut camera = ASICamera::new(cam_index);
        println!("Camera info for index {}", cam_index);
        let camera_info = camera.get_property().unwrap();
        print_camera_info(&camera_info);

        camera.open().unwrap();
        camera.init().unwrap();

        let num_controls = camera.get_num_controls().unwrap();
        for control_index in 0..num_controls {
            println!("Control caps for control {}", control_index);
            let control_caps = camera.get_control_caps(control_index).unwrap();
            print_control_caps(&control_caps);
        }
        // camera.close() is called when it is dropped at end of scope.
    }
}

fn print_camera_info(camera_info: &asi_camera2_sdk::ASI_CAMERA_INFO) {
    println!("  Name: {:?}", CStr::from_bytes_until_nul(&camera_info.Name).unwrap());
    println!("  CameraID: {}", camera_info.CameraID);
    println!("  MaxHeight,MaxWidth: {},{}",
             camera_info.MaxHeight, camera_info.MaxWidth);
    println!("  IsColorCam: {}", camera_info.IsColorCam != 0);
    println!("  BayerPattern: {}", camera_info.BayerPattern);
    print!(  "  SupportedBins: ");
    for j in 0..16 {
        let bin = camera_info.SupportedBins[j];
        if bin == 0 {
            break;
        }
        if j > 0 {
            print!(",");
        }
        print!("{}", bin);
    }
    println!();
    print!(  "  SupportedVideoFormat: ");
    for j in 0..8 {
        let vf = camera_info.SupportedVideoFormat[j];
        if vf == -1 {
            break;
        }
        if j > 0 {
            print!(",");
        }
        print!("{}", vf);
    }
    println!();
    println!("  PixelSize: {}", camera_info.PixelSize);
    println!("  MechanicalShutter: {}", camera_info.MechanicalShutter != 0);
    println!("  ST4Port: {}", camera_info.ST4Port != 0);
    println!("  IsCoolerCam: {}", camera_info.IsCoolerCam != 0);
    println!("  IsUSB3Host: {}", camera_info.IsUSB3Host != 0);
    println!("  IsUSB3Camera: {}", camera_info.IsUSB3Camera != 0);
    println!("  ElecPerADU: {}", camera_info.ElecPerADU);
    println!("  BitDepth: {}", camera_info.BitDepth);
    println!("  IsTriggerCam: {}", camera_info.IsTriggerCam != 0);
}

fn print_control_caps(control_caps: &asi_camera2_sdk::ASI_CONTROL_CAPS) {
    println!("  Name: {:?}", CStr::from_bytes_until_nul(&control_caps.Name).unwrap());
    println!("  Description: {:?}",
             CStr::from_bytes_until_nul(&control_caps.Description).unwrap());
    println!("  MaxValue/MinValue/DefaultValue: {}/{}/{}",
             control_caps.MaxValue,
             control_caps.MinValue,
             control_caps.DefaultValue);
    println!("  IsAutoSupported: {}", control_caps.IsAutoSupported != 0);
    println!("  IsWritable: {}", control_caps.IsWritable != 0);
    println!("  ControlType: {}", control_caps.ControlType);
}
