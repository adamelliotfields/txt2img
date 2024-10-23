# gen-rs

A rusty Stable Diffusion CLI.

## Features

Very WIP.

- Generate images from multiple cloud services and models.
- Run inference locally with `candle` (TBD).

Currently only text-to-image is supported, but image-to-image and LoRA support is coming soon. Planned features include a local server (warp or axum) and a simple Tauri GUI. Inpainting and training are not on the roadmap.

## Usage

```
Image generation CLI.

Usage: gen [OPTIONS] <PROMPT>

Arguments:
  <PROMPT>  The text prompt to generate the image (required)

Options:
  -n, --negative-prompt <NEGATIVE_PROMPT>
          Negative prompt
      --seed <SEED>
          Seed for deterministic generation
      --height <HEIGHT>
          Height of the image [default: 1024]
      --width <WIDTH>
          Width of the image [default: 1024]
  -g, --guidance-scale <GUIDANCE_SCALE>
          Guidance scale [default: 10.0]
  -s, --num-inference-steps <NUM_INFERENCE_STEPS>
          Inference steps [default: 50]
  -m, --model <MODEL>
          Model to use [default: stabilityai/stable-diffusion-xl-base-1.0]
  -o, --out <OUT>
          Output file path [default: output.png]
  -h, --help
          Print help
  -V, --version
          Print version
```

## Development

```
.
├── Cargo.toml
└── src
    ├── cli
    │   └── mod.rs
    ├── client
    │   └── mod.rs
    ├── config
    │   └── mod.rs
    ├── error
    │   └── mod.rs
    ├── log
    │   └── mod.rs
    ├── spinner
    │   └── mod.rs
    ├── util
    │   └── mod.rs
    ├── lib.rs
    └── main.rs
```

## TODO

- [ ] Tests (unit and integration with `mockito`)
- [ ] Local GPU inference with `candle`
- [ ] Configuration with `config` (YAML/TOML and env)
- [ ] Logging with `log`
- [ ] Loading spinner
- [ ] Clippy
- [ ] Docs

## Inspiration

- [`diffusers-rs`](https://github.com/LaurentMazare/diffusers-rs): Diffusers API implemented in Rust via [tch-rs](https://github.com/LaurentMazare/tch-rs) (libtorch Rust bindings).
- [`stable-diffusion.cpp`](https://github.com/leejet/stable-diffusion.cpp): Stable Diffusion inference in pure C++.
- [`aichat`](https://github.com/sigoden/aichat): Rust CLI for cloud LLM inference.
