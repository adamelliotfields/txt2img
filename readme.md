# gen-rs

A rusty generative AI CLI.

## Features

Run text and image generation models from various services.

**Supported:**
- [Hugging Face](https://huggingface.co): SDXL, SD 3.5 Large, SD 3.5 Large Turbo, FLUX.1 Dev, FLUX.1 Schnell
- [OpenAI](https://openai.com): DALL-E 2, DALL-E 3, GPT-4o
- [Together AI](https://together.ai): FLUX.1 Schnell, FLUX.1 Dev, FLUX.1 Pro, FLUX.1.1 Pro

## Usage

`cargo run -- --help`

```
Arguments:
  [PROMPT]  The text to guide the generation (required)

Options:
  -m, --model <MODEL>      Model to use
  -s, --service <SERVICE>  Service to use
      --seed <SEED>        Seed for reproducibility
  -t, --timeout <TIMEOUT>  Timeout in seconds [default: 60]
  -q, --quiet              Suppress progress bar
      --debug              Use debug logging
      --list-models        Print models
      --list-services      Print services
  -h, --help               Print help
  -V, --version            Print version

Options (Image Generation):
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
      --style <STYLE>
          Image style (OpenAI only) [default: vivid] [possible values: natural, vivid]
  -o, --out <OUT>
          Output file path [default: image.jpg]

Options (Text Generation):
      --system-prompt <SYSTEM_PROMPT>  Instructions that the model should follow
      --frequency <FREQUENCY>          Frequency penalty
      --presence <PRESENCE>            Presence penalty
      --temperature <TEMPERATURE>      Temperature

Environment Variables:
  HF_TOKEN                                 Required for Hugging Face
  OPENAI_API_KEY                           Required for OpenAI
  TOGETHER_API_KEY                         Required for Together.ai
```

## MSRV

The minimum supported Rust version is [1.80.0](https://blog.rust-lang.org/2024/07/25/Rust-1.80.0.html) for [LazyLock](https://doc.rust-lang.org/std/sync/struct.LazyLock.html).
