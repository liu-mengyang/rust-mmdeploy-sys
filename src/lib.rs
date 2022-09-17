use std::os::raw::c_char;
use std::ffi::CString;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl mmdeploy_detector {
    pub fn new() -> mmdeploy_detector {
        mmdeploy_detector {
            _unused: [1; 0],
        }
    }
}

impl mmdeploy_detection_t {
    pub fn new() -> mmdeploy_detection_t {
        let mut new_bbox: mmdeploy_rect_t = mmdeploy_rect_t {
            left: 0.0,
            top: 0.0,
            right: 0.0,
            bottom: 0.0
        };

        let mut new_mask: mmdeploy_instance_mask_t = mmdeploy_instance_mask_t {
            data: &mut ('0' as c_char),
            height: 0,
            width: 0,
        };

        mmdeploy_detection_t {
            label_id: 0,
            score: 0.0,
            bbox: new_bbox,
            mask: &mut new_mask,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
