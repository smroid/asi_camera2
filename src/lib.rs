#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod asi_camera2_sdk {
    // This module provides a thin wrapper of the ASI Camera2 SDK.
    // TODO: pointer to usage examples.

    use std::fmt;
    use std::error::Error;
    use std::mem::MaybeUninit;

    include!(concat!(env!("OUT_DIR"), "/asi_sdk_bindings.rs"));

    #[derive(Debug)]
    pub struct ASICamera {
        camera_id: i32,
        opened: bool,
    }

    impl ASICamera {
        pub fn open(&mut self) -> Result<(), ASIError> {
            let error_code = unsafe{ ASIOpenCamera(self.camera_id) };
            if error_code != 0 {
                Err(ASIError{error_code})
            } else {
                self.opened = true;
                Ok(())
            }
        }

        pub fn init(&self) -> Result<(), ASIError> {
            let error_code = unsafe{ ASIInitCamera(self.camera_id) };
            if error_code != 0 {
                Err(ASIError{error_code})
            } else {
                Ok(())
            }
        }

        pub fn close(&mut self) -> Result<(), ASIError> {
            if self.opened {
                let error_code = unsafe{ ASICloseCamera(self.camera_id) };
                if error_code != 0 {
                    return Err(ASIError{error_code})
                }
            }
            self.opened = false;
            Ok(())
        }

        pub fn get_property(&self) -> Result<ASI_CAMERA_INFO, ASIError> {
            let mut uninit_camera_info: MaybeUninit<ASI_CAMERA_INFO> =
                MaybeUninit::zeroed();
            let error_code = unsafe { ASIGetCameraProperty(
                &mut *uninit_camera_info.as_mut_ptr(), self.camera_id)
            };
            if error_code != 0 {
                Err(ASIError{error_code})
            } else {
                Ok(unsafe{ uninit_camera_info.assume_init() })
            }
        }

        pub fn get_num_controls(&self) -> Result<i32, ASIError> {
            let mut num_controls = 0;
            let error_code = unsafe {
                ASIGetNumOfControls(self.camera_id, &mut num_controls)
            };
            if error_code != 0 {
                Err(ASIError{error_code})
            } else {
                Ok(num_controls)
            }
        }

        pub fn get_control_caps(&self, control_index: i32)
                                -> Result<ASI_CONTROL_CAPS, ASIError> {
            let mut uninit_control_caps: MaybeUninit<ASI_CONTROL_CAPS> =
                MaybeUninit::zeroed();
            let error_code = unsafe { ASIGetControlCaps(
                self.camera_id, control_index, &mut *uninit_control_caps.as_mut_ptr())
            };
            if error_code != 0 {
                Err(ASIError{error_code})
            } else {
                Ok(unsafe{ uninit_control_caps.assume_init() })
            }
        }

        // The return value is the control's value and whether it is automatic.
        pub fn get_control_value(&self, control_type: ASI_CONTROL_TYPE)
                                 -> Result<(i64, bool), ASIError> {
            let mut value: i64 = 0;
            let mut auto: i32 = 0;
            let error_code = unsafe { ASIGetControlValue(
                self.camera_id, control_type.try_into().unwrap(), &mut value, &mut auto)
            };
            if error_code != 0 {
                Err(ASIError{error_code})
            } else {
                Ok((value, auto != 0))
            }
        }

        pub fn set_control_value(&mut self, control_type: ASI_CONTROL_TYPE,
                                 value: i64, auto: bool) -> Result<(), ASIError> {
            let error_code = unsafe { ASISetControlValue(
                self.camera_id, control_type.try_into().unwrap(), value, auto as i32)
            };
            if error_code != 0 {
                Err(ASIError{error_code})
            } else {
                Ok(())
            }
        }

        // The return value is (width, height, bin, img_type).
        pub fn get_roi_format(&self)
                              -> Result<(i32, i32, i32, ASI_IMG_TYPE), ASIError> {
            let mut width = 0;
            let mut height = 0;
            let mut bin = 0;
            let mut img_type:ASI_IMG_TYPE = ASI_IMG_TYPE_ASI_IMG_RAW8;
            let error_code = unsafe { ASIGetROIFormat(
                self.camera_id, &mut width, &mut height, &mut bin, &mut img_type)
            };
            if error_code != 0 {
                Err(ASIError{error_code})
            } else {
                Ok((width, height, bin, img_type))
            }
        }

        pub fn set_roi_format(&mut self, width: i32, height: i32,
                              bin: i32, img_type:ASI_IMG_TYPE)
                              -> Result<(), ASIError> {
            let error_code = unsafe { ASISetROIFormat(
                self.camera_id, width, height, bin, img_type)
            };
            if error_code != 0 {
                Err(ASIError{error_code})
            } else {
                Ok(())
            }
        }

        // The return value is (x, y).
        pub fn get_start_pos(&self) -> Result<(i32, i32), ASIError> {
            let mut start_x = 0;
            let mut start_y = 0;
            let error_code = unsafe { ASIGetStartPos(
                self.camera_id, &mut start_x, &mut start_y)
            };
            if error_code != 0 {
                Err(ASIError{error_code})
            } else {
                Ok((start_x, start_y))
            }
        }

        pub fn set_start_pos(&mut self, start_x: i32, start_y: i32)
                             -> Result<(), ASIError> {
            let error_code = unsafe { ASISetStartPos(
                self.camera_id, start_x, start_y)
            };
            if error_code != 0 {
                Err(ASIError{error_code})
            } else {
                Ok(())
            }
        }

        pub fn get_dropped_frames(&self) -> Result<i32, ASIError> {
            let mut dropped_frames: i32 = 0;
            let error_code = unsafe { ASIGetDroppedFrames(
                self.camera_id, &mut dropped_frames)
            };
            if error_code != 0 {
                Err(ASIError{error_code})
            } else {
                Ok(dropped_frames)
            }
        }

        // TODO: enable_dark_subtract()
        // TODO: disable_dark_subtract()

        pub fn start_video_capture(&mut self) -> Result<(), ASIError> {
            let error_code = unsafe { ASIStartVideoCapture(self.camera_id) };
            if error_code != 0 {
                Err(ASIError{error_code})
            } else {
                Ok(())
            }
        }

        pub fn stop_video_capture(&mut self) -> Result<(), ASIError> {
            let error_code = unsafe { ASIStopVideoCapture(self.camera_id) };
            if error_code != 0 {
                Err(ASIError{error_code})
            } else {
                Ok(())
            }
        }

        pub fn get_video_data(&self, buffer: *mut u8, buff_size: i64, wait_ms: i32)
                                  -> Result<(), ASIError> {
            let error_code = unsafe { ASIGetVideoData(
                self.camera_id, buffer, buff_size, wait_ms) };
            if error_code != 0 {
                Err(ASIError{error_code})
            } else {
                Ok(())
            }
        }

        // TODO: pulse_guide_on()
        // TODO: pulse_guide_off()

        // is_dark is relevant only if the camera has a mechanical shutter.
        pub fn start_exposure(&mut self, is_dark: bool) -> Result<(), ASIError> {
            let error_code = unsafe { ASIStartExposure(self.camera_id, is_dark as i32) };
            if error_code != 0 {
                Err(ASIError{error_code})
            } else {
                Ok(())
            }
        }

        pub fn stop_exposure(&mut self) -> Result<(), ASIError> {
            let error_code = unsafe { ASIStopExposure(self.camera_id) };
            if error_code != 0 {
                Err(ASIError{error_code})
            } else {
                Ok(())
            }
        }

        pub fn get_exp_status(&self) -> Result<ASI_EXPOSURE_STATUS, ASIError> {
            let mut exp_status: ASI_EXPOSURE_STATUS = ASI_EXPOSURE_STATUS_ASI_EXP_IDLE;
            let error_code = unsafe { ASIGetExpStatus(self.camera_id, &mut exp_status) };
            if error_code != 0 {
                Err(ASIError{error_code})
            } else {
                Ok(exp_status)
            }
        }

        pub fn get_data_after_exp(&self, buffer: *mut u8, buff_size: i64)
                                  -> Result<(), ASIError> {
            let error_code = unsafe { ASIGetDataAfterExp(
                self.camera_id, buffer, buff_size) };
            if error_code != 0 {
                Err(ASIError{error_code})
            } else {
                Ok(())
            }
        }

        // TODO: get_id()
        // TODO: set_id()
        // TODO: camera_check()
        // TODO: get_sdk_version()
        // TODO: get_camera_support_mode()
        // TODO: get_camera_mode()
        // TODO: set_camera_mode()
        // TODO: send_soft_trigger()
    }  // impl ASICamera

    // Arrange to call close() when ASICamera object goes out of scope.
    impl Drop for ASICamera {
        fn drop(&mut self) {
            self.close().unwrap_or_else(|err| {
                panic!("Error closing camera id {}: {}", self.camera_id, err);
            });
        }
    }

    pub fn num_connected_asi_cameras() -> i32 {
        unsafe { ASIGetNumOfConnectedCameras() }
    }

    pub fn create_asi_camera(camera_id: i32) -> ASICamera {
        let num_cameras = num_connected_asi_cameras();
        if camera_id >= num_cameras {
            panic!("Cannot create camera {} with {} cameras detected",
                   camera_id, num_cameras);
        }
        ASICamera{camera_id, opened: false}
    }

    pub struct ASIError {
      error_code: i32
    }

    impl fmt::Display for ASIError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let msg = match self.error_code as u32 {
                ASI_ERROR_CODE_ASI_SUCCESS => "OK",
                ASI_ERROR_CODE_ASI_ERROR_INVALID_INDEX =>
                    "No camera connected or index value out of boundary",
                ASI_ERROR_CODE_ASI_ERROR_INVALID_ID => "Invalid ID",
                ASI_ERROR_CODE_ASI_ERROR_INVALID_CONTROL_TYPE => "Invalid control type",
                ASI_ERROR_CODE_ASI_ERROR_CAMERA_CLOSED => "Camera didn't open",
                ASI_ERROR_CODE_ASI_ERROR_CAMERA_REMOVED =>
                    "Failed to find the camera, maybe the camera has been removed",
                ASI_ERROR_CODE_ASI_ERROR_INVALID_PATH =>
                    "Cannot find the path of the file",
                ASI_ERROR_CODE_ASI_ERROR_INVALID_FILEFORMAT => "Invalid file format",
                ASI_ERROR_CODE_ASI_ERROR_INVALID_SIZE => "Wrong video format size",
                ASI_ERROR_CODE_ASI_ERROR_INVALID_IMGTYPE => "Unsupported image format",
                ASI_ERROR_CODE_ASI_ERROR_OUTOF_BOUNDARY =>
                    "The startpos is outside the image boundary",
                ASI_ERROR_CODE_ASI_ERROR_TIMEOUT => "Timeout",
                ASI_ERROR_CODE_ASI_ERROR_INVALID_SEQUENCE => "Stop capture first",
                ASI_ERROR_CODE_ASI_ERROR_BUFFER_TOO_SMALL =>
                    "Buffer size is not big enough",
                ASI_ERROR_CODE_ASI_ERROR_VIDEO_MODE_ACTIVE => "Video mode active",
                ASI_ERROR_CODE_ASI_ERROR_EXPOSURE_IN_PROGRESS => "Exposure in progress",
                ASI_ERROR_CODE_ASI_ERROR_GENERAL_ERROR =>
                    "General error, e.g. value is out of valid range",
                _ => "Unknown error",
            };
            write!(f, "{} (code={})", msg, self.error_code)
        }
    }

    impl fmt::Debug for ASIError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self)  // Just re-use Display.
        }
    }

    impl Error for ASIError {}

}  // mod asi_camera2_sdk
