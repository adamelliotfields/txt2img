# txt2img

Text-to-image generation with cloud models. Written in Rust.

## Features

Run image generation models from various cloud providers:

- [Hugging Face](https://huggingface.co): SDXL, SD 3.5 Large, SD 3.5 Large Turbo, FLUX.1 Dev, FLUX.1 Schnell
- [Together AI](https://together.ai): FLUX.1 Schnell, FLUX.1 Dev, FLUX.1 Pro, FLUX.1.1 Pro
- [OpenAI](https://openai.com): DALL-E 2, DALL-E 3

## Usage

> [!TIP]
> Run [upx](https://upx.github.io) on the binary to make it even smaller.

```sh
cargo build --release
./target/release/txt2img --help
```

```
Usage: txt2img [OPTIONS] [PROMPT]

Arguments:
  [PROMPT]  The text to guide the generation (required)

Options:
  -m, --model <MODEL>      Model to use
  -s, --service <SERVICE>  Service to use
  -t, --timeout <TIMEOUT>  Timeout in seconds [default: 60]
  -q, --quiet              Suppress progress
      --debug              Debug logging
      --list-models        Print models
      --list-services      Print services
  -h, --help               Print help
  -V, --version            Print version

Parameters:
  -n, --negative-prompt <NEGATIVE_PROMPT>
          Negative prompt
      --steps <STEPS>
          Inference steps
      --cfg <CFG>
          Classifier-free guidance scale
      --width <WIDTH>
          Width of the image
      --height <HEIGHT>
          Height of the image
      --seed <SEED>
          Seed for reproducibility
      --style <STYLE>
          Image style (OpenAI only) [default: vivid] [possible values: natural, vivid]
  -o, --out <OUT>
          Output file path [default: image.png]

Environment Variables:
  HF_TOKEN                 Required for Hugging Face
  OPENAI_API_KEY           Required for OpenAI
  TOGETHER_API_KEY         Required for Together.ai
```

## MSRV

The minimum supported Rust version is [1.80.0](https://blog.rust-lang.org/2024/07/25/Rust-1.80.0.html) for [LazyLock](https://doc.rust-lang.org/std/sync/struct.LazyLock.html).
