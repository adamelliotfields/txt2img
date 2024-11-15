# gen-rs

A rusty image generation CLI. Very WIP! 🚧

## Features

Run text-to-image inference on multiple models across many cloud AI platforms.

**Supported:**
- [Hugging Face](https://huggingface.co): SDXL, SD 3.5 Large, SD 3.5 Large Turbo, FLUX.1 Dev, FLUX.1 Schnell
- [Together AI](https://together.ai): FLUX.1 Schnell, FLUX1.1 Pro

**Coming Soon:**
- [OpenAI](https://openai.com): DALL-E
- [Recraft](https://recraft.ai): v3 (aka "red panda")
- [Ideogram](https://ideogram.ai): v2
- [BFL](https://blackforestlabs.ai): FLUX1.1 Pro
- [Stability](https://stability.ai): Stable Image Core, Stable Image Ultra
- [Replicate](https://replicate.com): Many
- [Fal](https://fal.ai): Many

## Usage

```
Rusty image generation CLI

Usage: gen [OPTIONS] [PROMPT]

Arguments:
  [PROMPT]  The text to guide the generation (required)

Options:
  -n, --negative-prompt <NEGATIVE_PROMPT>  Negative prompt
  -s, --service <SERVICE>                  Service to use
  -m, --model <MODEL>                      Model to use
      --height <HEIGHT>                    Height of the image
      --width <WIDTH>                      Width of the image
      --cfg <CFG>                          Guidance scale
      --steps <STEPS>                      Inference steps
      --seed <SEED>                        Seed for reproducibility
  -o, --out <OUT>                          Output file path [default: image.jpg]
      --list-models                        Print models
      --list-services                      Print services
  -h, --help                               Print help
  -V, --version                            Print version
```

## Inspiration

- [`diffusers-rs`](https://github.com/LaurentMazare/diffusers-rs): Diffusers API implemented in Rust via [tch-rs](https://github.com/LaurentMazare/tch-rs) (libtorch Rust bindings).
- [`stable-diffusion.cpp`](https://github.com/leejet/stable-diffusion.cpp): Stable Diffusion inference in pure C++.
- [`aichat`](https://github.com/sigoden/aichat): Rust CLI for cloud LLM inference.
