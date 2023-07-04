#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod asi_camera2_sdk {
    // This module provides a thin wrapper of the ASI Camera2 SDK.
    // TODO: pointer to usage examples.

    use std::mem::MaybeUninit;

    include!(concat!(env!("OUT_DIR"), "/asi_sdk_bindings.rs"));

    #[derive(Debug)]
    pub struct ASICamera {
        camera_id: i32,
        opened: bool,
    }

    impl ASICamera {
        pub fn open(&mut self) -> Result<(), i32> {
            let error_code = unsafe{ ASIOpenCamera(self.camera_id) };
            if error_code != 0 {
                Err(error_code)
            } else {
                self.opened = true;
                Ok(())
            }
        }

        pub fn init(&self) -> Result<(), i32> {
            let error_code = unsafe{ ASIInitCamera(self.camera_id) };
            if error_code != 0 {
                Err(error_code)
            } else {
                Ok(())
            }
        }

        pub fn close(&mut self) -> Result<(), i32> {
            if self.opened {
                let error_code = unsafe{ ASICloseCamera(self.camera_id) };
                if error_code != 0 {
                    return Err(error_code)
                }
            }
            self.opened = false;
            Ok(())
        }

        pub fn get_property(&self) -> Result<ASI_CAMERA_INFO, i32> {
            let mut uninit_camera_info: MaybeUninit<ASI_CAMERA_INFO> =
                MaybeUninit::zeroed();
            let error_code = unsafe { ASIGetCameraProperty(
                &mut *uninit_camera_info.as_mut_ptr(), self.camera_id)
            };
            if error_code != 0 {
                Err(error_code)
            } else {
                Ok(unsafe{ uninit_camera_info.assume_init() })
            }
        }

        pub fn get_num_controls(&self) -> Result<i32, i32> {
            let mut num_controls = 0;
            let error_code = unsafe {
                ASIGetNumOfControls(self.camera_id, &mut num_controls)
            };
            if error_code != 0 {
                Err(error_code)
            } else {
                Ok(num_controls)
            }
        }

        pub fn get_control_caps(&self, control_index: i32)
                                -> Result<ASI_CONTROL_CAPS, i32> {
            let mut uninit_control_caps: MaybeUninit<ASI_CONTROL_CAPS> =
                MaybeUninit::zeroed();
            let error_code = unsafe { ASIGetControlCaps(
                self.camera_id, control_index, &mut *uninit_control_caps.as_mut_ptr())
            };
            if error_code != 0 {
                Err(error_code)
            } else {
                Ok(unsafe{ uninit_control_caps.assume_init() })
            }
        }

        // TODO: additional methods.
    }  // impl ASICamera

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

    // TODO: function to map ASI error code to string message.

}  // mod asi_camera2_sdk
