# fast-qr-py

> [!WARNING]  
> The bindings were 100% vibe-coded. The code looks legit, doesn’t contain any backdoors, and works as expected - but use it at your own discretion.

Python bindings for the [fast_qr](https://crates.io/crates/fast_qr) Rust QR code library.

## Features

- Generate QR codes from strings or bytes
- Control error correction level (L / M / Q / H)
- Force encoding mode (Numeric / Alphanumeric / Byte)
- Force QR code version (1–40)
- Force mask pattern
- Export to **SVG** with full style customisation
- Export to **PNG** (raster) with configurable width/height
- Embed images inside QR codes
- Multiple module shapes: Square, Circle, RoundedSquare, Vertical, Horizontal, Diamond

## Quick start

```python
from fast_qr import QRBuilder, ECL, Shape, SvgBuilder, ImageBuilder, Color

# Minimal — auto-selects version, ECL=Q, mode=Byte
qr = QRBuilder("https://example.com").build()
print(qr.to_str())

# Custom ECL
qr = QRBuilder("Hello").ecl(ECL.H).build()

# SVG string
svg = SvgBuilder().shape(Shape.RoundedSquare).to_str(qr)

# PNG bytes
png = ImageBuilder().fit_width(400).to_bytes(qr)
```

## Installation

```bash
pip install fast-qr-py   # once published to PyPI
```

Or from source (requires Rust ≥ 1.70 and maturin):

```bash
uv venv && uv pip install maturin
maturin develop --uv
```

## License

MIT
