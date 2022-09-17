
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl mmdeploy_detector {
    pub fn new() -> mmdeploy_detector {
        mmdeploy_detector {
            _unused: [1; 0],
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
