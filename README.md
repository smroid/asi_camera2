This is a thin Rust wrapper of the ASI Camera2 SDK.

# Preparation

1. Download the ASI Camera2 SDK from ZWO's
   [downloads page](https://www.zwoastro.com/downloads/developers).
2. Unzip the download and make note of where the "lib" directory
   lands.
3. Edit build.rs to update the "cargo:rustc-link-search" setting.

# Usage

Please consult the binaries in src/bin for examples of how to use the
wrapped ASI Camera2 SDK. These programs are:

## camera_info

This program uses the SDK to enumerate all attached ASI camera(s) and print
out properties for each.

## one_exposure

This program opens the first ASI camera, and:

1. Configures the ROI for greyscale image, whole sensor, no binning.
2. Sets an exposure time. You can adjust the exposure time by editing
   the example code.
3. Starts an exposure and sleeps for the expected exposure duration.
4. Waits for the exposure to finish, sleeping in additional increments
   if necessary.
5. Reads out the image and saves it to a JPG file.

## video_frame

This is similar to the "one_exposure" program except it operates the
camera in video capture mode.
