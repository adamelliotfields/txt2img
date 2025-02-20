# gen-rs

A rusty image generation CLI. Very WIP! 🚧

## Features

Run text-to-image inference on multiple models across many cloud AI platforms.

**Supported:**
- [Hugging Face](https://huggingface.co): SDXL, SD 3.5 Large, SD 3.5 Large Turbo, FLUX.1 Dev, FLUX.1 Schnell
- [Together AI](https://together.ai): FLUX.1 Schnell, FLUX1.1 Pro

**Coming Soon:**
- [OpenAI](https://openai.com): DALL-E and ChatGPT
- Text generation

## Usage

`cargo run -- --help`

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
  -t, --timeout <TIMEOUT>                  Timeout in seconds [default: 60]
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

## MSRV

The minimum supported Rust version is [1.80.0](https://blog.rust-lang.org/2024/07/25/Rust-1.80.0.html) for [LazyLock](https://doc.rust-lang.org/std/sync/struct.LazyLock.html).
