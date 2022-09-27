# rust-mmdeploy-sys

MMDeploy bindings for Rust.

:construction: **THIS REPO IS STILL UNDER CONSTRUCTION!**

## Prerequisites

**NOTICE:** Linux only. Onnxruntime only.

In order to successfully build this repo, you are supposed to install some pre-packages.

**Step 1.** Install Clang required by `Bindgen`.

```bash
apt install llvm-dev libclang-dev clang
```

**Step 2.** Download and install pre-built mmdeploy package.

```bash
wget https://github.com/open-mmlab/mmdeploy/releases/download/v0.8.0/mmdeploy-0.8.0-linux-x86_64-onnxruntime1.8.1.tar.gz
tar -zxvf mmdeploy-0.8.0-linux-x86_64-onnxruntime1.8.1.tar.gz
cd mmdeploy-0.8.0-linux-x86_64-onnxruntime1.8.1.tar.gz
export MMDEPLOY_DIR=$(pwd)/sdk
```

## Build

Move to the root of this repo and then:

```bash
cargo build
```

