# lightgbm-rs
LightGBM Rust binding

Fork from https://github.com/vaaaaanquish/lightgbm-rs

LightGBM version: V4.3.0


# Require

You need an environment that can build LightGBM.

```
# linux
apt install -y cmake libclang-dev libc++-dev gcc-multilib

# OS X
brew install cmake libomp
```

On Windows

1. Install CMake and VS Build Tools.
1. Install LLVM and set an environment variable `LIBCLANG_PATH` to PATH_TO_LLVM_BINARY (example: `C:\Program Files\LLVM\bin`)

Please see below for details.

- [LightGBM Installation-Guide](https://lightgbm.readthedocs.io/en/latest/Installation-Guide.html)

# Usage

Add dependency:
```
lightgbm = { git = "https://github.com/magicalne/lightgbm-rs.git", branch = "main", features = ["dataframe"] }
```

Example LightGBM train.
```
extern crate serde_json;
use lightgbm::{Dataset, Booster};
use serde_json::json;

let data = vec![vec![1.0, 0.1, 0.2, 0.1],
               vec![0.7, 0.4, 0.5, 0.1],
               vec![0.9, 0.8, 0.5, 0.1],
               vec![0.2, 0.2, 0.8, 0.7],
               vec![0.1, 0.7, 1.0, 0.9]];
let label = vec![0.0, 0.0, 0.0, 1.0, 1.0];
let dataset = Dataset::from_mat(data, label).unwrap();
let params = json!{
   {
        "num_iterations": 3,
        "objective": "binary",
        "metric": "auc"
    }
};
let bst = Booster::train(dataset, &params).unwrap();
```

Please see the `./examples` for details.

|example|link|
|---|---|
|binary classification|[link](https://github.com/magicalne/lightgbm-rs/blob/main/examples/binary_classification/src/main.rs)|
|multiclass classification|[link](https://github.com/magicalne/lightgbm-rs/blob/main/examples/multiclass_classification/src/main.rs)|
|regression|[link](https://github.com/magicalne/lightgbm-rs/blob/main/examples/regression/src/main.rs)|



# Develop

```
git clone --recursive https://github.com/magicalne/lightgbm-rs
```

```
docker build -t lgbmrs .
docker run -it -v $PWD:/app lgbmrs bash

# cargo build
```


# Thanks

Much reference was made to implementation and documentation. Thanks.

- [microsoft/LightGBM](https://github.com/microsoft/LightGBM)
- [davechallis/rust-xgboost](https://github.com/davechallis/rust-xgboost)
