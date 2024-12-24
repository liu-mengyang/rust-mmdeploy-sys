# rust-mmdeploy-sys

MMDeploy bindings for Rust.

This repo is a low-level MMDeploy>=1.0.0 abstraction, you can use [rust-mmdeploy](https://github.com/liu-mengyang/rust-mmdeploy) directly if you want to use MMDeploy in Rust. If it cannot cover your requirements, it's easy to build high-level abstraction based on this repo.

## Support matrix from MMDeploy

Supporting all devices and platforms is a big challenge for this personal project. So I just support and test for main devices and platforms.

Supported               : ✔️
MMDeploy not support    : -
Uncertain (without test): ❓

| Device / Platform | Linux                                                           | Windows                                 | macOS    | Android          |
| ----------------- | --------------------------------------------------------------- | --------------------------------------- | -------- | ---------------- |
| x86_64 CPU        | ✔️ONNX Runtime<br>❓pplnn<br>❓ncnn<br>❓OpenVINO<br>❓LibTorch | ❓ONNX Runtime<br>❓OpenVINO            | -        | -                |
| ARM CPU           | ❓ncnn                                                          | -                                       | -        | ❓ncnn           |
| RISC-V            | ❓ncnn                                                          | -                                       | -        | -                |
| NVIDIA GPU        | ✔️ONNX Runtime<br>✔️TensorRT<br>❓pplnn<br>❓LibTorch           | ❓ONNX Runtime<br>❓TensorRT<br>❓pplnn | -        | -                |
| NVIDIA Jetson     | ❓TensorRT                                                      | ❓TensorRT                              | -        | -                |
| Huawei ascend310  | ❓CANN                                                          | -                                       | -        | -                |
| Rockchip          | ❓RKNN                                                          | -                                       | -        | -                |
| Apple M1          | -                                                               | -                                       | ❓CoreML | -                |
| Adreno GPU        | -                                                               | -                                       | -        | ❓ncnn<br>❓SNPE |
| Hexagon DSP       | -                                                               | -                                       | -        | ❓SNPE           |

## Prerequisites

In order to successfully build this repo, you are supposed to install some pre-packages.

**Step 1.** Install Clang required by `Bindgen`.

```bash
apt install llvm-dev libclang-dev clang
```

**Step 2(Pre-built package).** Download and install pre-built mmdeploy package and ONNXRuntime.

```bash
wget https://github.com/open-mmlab/mmdeploy/releases/download/v1.1.0/mmdeploy-1.1.0-linux-x86_64-cuda11.3.tar.gz
tar -zxvf mmdeploy-1.1.0-linux-x86_64-cuda11.3.tar.gz
cd mmdeploy-1.1.0-linux-x86_64-cuda11.3.tar.gz
export MMDEPLOY_DIR=$(pwd)
export ONNXRUNTIME_DIR=$(pwd)/thirdparty/onnxruntime
export LD_LIBRARY_PATH=$ONNXRUNTIME_DIR/lib:$LD_LIBRARY_PATH
export TENSORRT_DIR=$(pwd)/thirdparty/tensorrt
export LD_LIBRARY_PATH=$TENSORRT_DIR/lib:$LD_LIBRARY_PATH
```
Then follow the guide in $MMDEPLOY_DIR/README.md to build the SDK.
**Step 2(Build from source).**Follow the [official guide](https://mmdeploy.readthedocs.io/en/latest/01-how-to-build/build_from_source.html) to build MMDeploy SDK from source. If successfully built, you should have set TENSORRT_DIR, ONNXRUNTIME_DIR and LD_LIBRARY_PATH environment variables. Then
```bash
export MMDEPLOY_DIR=/path/to/mmdeploy/build/install
export LD_LIBRARY_PATH=$MMDEPLOY_DIR/lib:$LD_LIBRARY_PATH
```
## Quick start

Update your *Cargo.toml*

```toml
mmdeploy-sys = "1.1.0"
```

## Build

Move to the root of this repo and then:

```bash
cargo build
```

