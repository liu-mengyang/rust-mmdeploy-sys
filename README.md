# rust-mmdeploy-sys

MMDeploy bindings for Rust.

This repo is a low-level MMDeploy abstraction, you can use [rust-mmdeploy](https://github.com/liu-mengyang/rust-mmdeploy) directly if you want to use MMDeploy in Rust. If it cannot cover your requirements, it's easy to build high-level abstraction based on this repo.

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

**Step 2.1. (For ONNXRuntime)** Download and install pre-built mmdeploy package and ONNXRuntime.

```bash
wget https://github.com/open-mmlab/mmdeploy/releases/download/v0.9.0/mmdeploy-0.9.0-linux-x86_64-onnxruntime1.8.1.tar.gz
tar -zxvf mmdeploy-0.9.0-linux-x86_64-onnxruntime1.8.1.tar.gz
cd mmdeploy-0.9.0-linux-x86_64-onnxruntime1.8.1
export MMDEPLOY_DIR=$(pwd)/sdk

wget https://github.com/microsoft/onnxruntime/releases/download/v1.8.1/onnxruntime-linux-x64-1.8.1.tgz
tar -zxvf onnxruntime-linux-x64-1.8.1.tgz
cd onnxruntime-linux-x64-1.8.1
export ONNXRUNTIME_DIR=$(pwd)
export LD_LIBRARY_PATH=$ONNXRUNTIME_DIR/lib:$LD_LIBRARY_PATH
```

**Step 2.2. (For TensorRT)** Download and install pre-built mmdeploy package and TensorRT.

```bash
wget https://github.com/open-mmlab/mmdeploy/releases/download/v0.9.0/mmdeploy-0.9.0-linux-x86_64-cuda11.1-tensorrt8.2.3.0.tar.gz
tar -zxvf mmdeploy-0.9.0-linux-x86_64-cuda11.1-tensorrt8.2.3.0.tar.gz
cd mmdeploy-0.9.0-linux-x86_64-cuda11.1-tensorrt8.2.3.0
export MMDEPLOY_DIR=$(pwd)/sdk
```

## Quick start

Update your *Cargo.toml*

```toml
mmdeploy-sys = "1.0.0"
```

## Build

Move to the root of this repo and then:

```bash
cargo build
```

