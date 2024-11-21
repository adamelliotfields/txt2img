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
  -m, --model <MODEL>                      Model to use
  -s, --service <SERVICE>                  Service to use
      --seed <SEED>                        Seed for reproducibility
      --steps <STEPS>                      Inference steps
      --cfg <CFG>                          Guidance scale
      --width <WIDTH>                      Width of the image
      --height <HEIGHT>                    Height of the image
  -t, --timeout <TIMEOUT>                  Timeout in seconds
  -o, --out <OUT>                          Output file path [default: image.jpg]
  -q, --quiet                              Suppress progress bar
      --debug                              Use debug logging
      --list-models                        Print models
      --list-services                      Print services
  -h, --help                               Print help
  -V, --version                            Print version

Environment Variables:
  HF_TOKEN                                 Required for Hugging Face
  TOGETHER_API_KEY                         Required for Together.ai
```

## Inspiration

- [`diffusers-rs`](https://github.com/LaurentMazare/diffusers-rs): Diffusers API implemented in Rust via [tch-rs](https://github.com/LaurentMazare/tch-rs) (libtorch Rust bindings).
- [`stable-diffusion.cpp`](https://github.com/leejet/stable-diffusion.cpp): Stable Diffusion inference in pure C++.
- [`aichat`](https://github.com/sigoden/aichat): Rust CLI for cloud LLM inference.
