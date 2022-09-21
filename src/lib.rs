use std::os::raw::c_char;
use std::ffi::CString;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


// classifier
impl mmdeploy_classifier {
    pub fn new() -> mmdeploy_classifier {
        mmdeploy_classifier {
            _unused: [1; 0],
        }
    }
}

impl mmdeploy_classification_t {
    pub fn new() -> mmdeploy_classification_t {
        mmdeploy_classification_t {
            label_id: 0,
            score: 0.0,
        }
    }
}

// detector
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

// segmentor
impl mmdeploy_segmentor {
    pub fn new() -> mmdeploy_segmentor {
        mmdeploy_segmentor {
            _unused: [1; 0],
        }
    }
}

impl mmdeploy_segmentation_t {
    pub fn new() -> mmdeploy_segmentation_t {
        mmdeploy_segmentation_t {
            height: 0,
            width: 0,
            classes: 0,
            mask: Box::into_raw(Box::new(0)),
        }
    }
}

// pose detector
impl mmdeploy_pose_detector {
    pub fn new() -> mmdeploy_pose_detector {
        mmdeploy_pose_detector {
            _unused: [1; 0],
        }
    }
}

impl mmdeploy_pose_detection_t {
    pub fn new() -> mmdeploy_pose_detection_t {
        let point = mmdeploy_point_t {x: 0.0, y: 0.0};

        mmdeploy_pose_detection_t {
            point: Box::into_raw(Box::new(point)),
            score: Box::into_raw(Box::new(0.0)),
            length: 0,
        }
    }
}

// rotated detector
impl mmdeploy_rotated_detector {
    pub fn new() -> mmdeploy_rotated_detector {
        mmdeploy_rotated_detector {
            _unused: [1; 0],
        }
    }
}

impl mmdeploy_rotated_detection_t {
    pub fn new() -> mmdeploy_rotated_detection_t {

        mmdeploy_rotated_detection_t {
            label_id: 0,
            score: 0.0,
            rbbox: [0.0, 0.0, 0.0, 0.0, 0.0],
        }
    }
}

// text detector
impl mmdeploy_text_detector {
    pub fn new() -> mmdeploy_text_detector {
        mmdeploy_text_detector {
            _unused: [1; 0],
        }
    }
}

impl mmdeploy_text_detection_t {
    pub fn new() -> mmdeploy_text_detection_t {
        let point1 = mmdeploy_point_t {x: 0.0, y: 0.0};
        let point2 = mmdeploy_point_t {x: 0.0, y: 0.0};
        let point3 = mmdeploy_point_t {x: 0.0, y: 0.0};
        let point4 = mmdeploy_point_t {x: 0.0, y: 0.0};

        mmdeploy_text_detection_t {
            bbox: [point1, point2, point3, point4],
            score: 0.0,
        }
    }
}

// text recoginizer
impl mmdeploy_text_recognizer {
    pub fn new() -> mmdeploy_text_recognizer {
        mmdeploy_text_recognizer {
            _unused: [1; 0],
        }
    }
}

impl mmdeploy_text_recognition_t {
    pub fn new() -> mmdeploy_text_recognition_t {
        mmdeploy_text_recognition_t {
            text: &mut ('0' as c_char),
            score: Box::into_raw(Box::new(0.0)),
            length: 0,
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
