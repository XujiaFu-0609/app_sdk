# app_sdk

单份Rust代码，支持以下几种构建与使用方式：

- Web 前端：WASM（wasm-bindgen），前端直接 `import` 使用
- 原生 C FFI：动态库与头文件，可被 C/C++ 等直接导入
- 鸿蒙（OpenHarmony/ohos）：基于 napi-ohos 的 N-API 模块（ArkTS）
- Android：UniFFI 生成 Kotlin 绑定，原生直接导入

## 核心能力

- `Message` 结构体：`id`, `sender`, `content`, `timestamp`
- `create_message(sender, content) -> Message`
- `send_message(msg) -> ()`
- 为跨平台简化，额外提供 `send_message_json(json)` 与 WASM/FFI 封装

## 构建

### 1) Web/WASM（前端）

前置：

- 安装目标：`rustup target add wasm32-unknown-unknown`
- 安装 `wasm-bindgen-cli`：`cargo install wasm-bindgen-cli`

构建与产物生成：

```bash
cargo build --release --target wasm32-unknown-unknown --features wasm
wasm-bindgen --target bundler \
  --out-dir pkg \
  target/wasm32-unknown-unknown/release/app_sdk.wasm
```

前端使用示例（bundler环境）：

```ts
import { create_message, send_message_json } from './pkg/app_sdk.js';

const msgJson = create_message('alice', 'hello wasm');
await send_message_json(msgJson);
```

### 2) 原生 C FFI（通用）

以 C FFI 的动态库形式提供，可被 C/C++ 等直接调用：

```bash
cargo build --release
```

生成 C 头文件（可选）：

```bash
cbindgen --crate app_sdk --output target/app_sdk.h
```

导出函数：

- `char* app_create_message(const char* sender, const char* content);`
- `int32_t app_send_message_json(const char* json);`
- `int32_t app_send_message(const char* sender, const char* content);`
- `void app_string_free(char* s);`

示例（C/C++）：

```c
#include "app_sdk.h"

char* json = app_create_message("alice", "hello native");
int rc = app_send_message_json(json);
app_string_free(json);
```

### 3) 鸿蒙（ohos）N-API（napi-ohos / ArkTS）

鸿蒙侧推荐使用 napi-ohos 提供的 N-API 模块，在 ArkTS 中直接调用。
本仓库已内置相关依赖与构建脚本：

- Cargo.toml 中的特性：
  - `ohos_napi = ["dep:napi-ohos", "dep:napi-derive-ohos"]`
- build.rs 中在启用 `ohos_napi` 特性时调用：
  - `napi_build_ohos::setup();`

在上层鸿蒙/ohos-rs 工程中：

1. 将本 crate 作为依赖引入，并启用 `ohos_napi` 特性；
2. 使用 `ohrs build` 或 DevEco 集成的 Rust 构建流程构建 N-API 模块；
3. 生成的 `.so` 与类型定义可在 ArkTS 侧直接 `import` 并调用。

N-API 导出函数（见 `src/ohos_napi.rs`）：

- `createMessage(sender: string, content: string): string`：返回 `Message` 的 JSON 字符串
- `sendMessageJson(json: string): void`：解析 JSON 并调用底层 `send_message`

### 4) Android（UniFFI / Kotlin）

启用 `uniffi` 特性生成脚手架：

```bash
cargo build --release --features uniffi
```

生成 Kotlin 绑定代码（需要安装 `uniffi-bindgen`）：

```bash
cargo run --features "uniffi,uniffi/cli" --bin uniffi-bindgen -- \
  generate uniffi/app_sdk.udl --language kotlin --out-dir gen/kotlin
```

将生成的 Kotlin 源整合到 Android 工程（或打包为 AAR）。

Kotlin 使用示例：

```kotlin
import app_sdk.Message
import app_sdk.App_sdk // 具体对象名按生成结果而定

val msg = Message(1L, "alice", "hello android", 1700000000000L)
App_sdk.sendMessage(msg)
```

> 说明：Kotlin 生成命名空间与对象名取决于 UDL `namespace` 与生成工具版本，可根据生成文件调整导入路径与调用方式。

## 设计说明

- 业务逻辑仅实现一份：`src/lib.rs` 中的 `Message` 与 `send_message`。
- 平台层只做“封装/适配”：
-  - WASM：`src/wasm.rs` 使用 `wasm-bindgen` 导出 JSON 接口；
-  - 原生 C：`src/ffi.rs` 提供 `extern "C"` 接口与字符串释放；
-  - 鸿蒙：`src/ohos_napi.rs` 使用 `napi-ohos` 导出 N-API 接口；
-  - UniFFI：通过 UDL 与 `build.rs` 生成跨语言脚手架，直接暴露结构体与函数。
- 这样可以保证“代码只写一份，然后编译不同包”，避免重复实现。

## 注意事项

- 真实场景中 `send_message` 可接入网络/IPC/队列等，当前为 Demo 行为（打印）。
- WASM 产物如何打包（`--target bundler` / `--target web`）可按前端环境调整。
- 鸿蒙推荐通过 napi-ohos / N-API 的方式集成，在 ArkTS 中直接使用。
- Android 产物通常打包为 AAR，更易于依赖管理；也可直接集成生成的 Kotlin 源。
