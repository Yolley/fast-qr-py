"""fast_qr — Python bindings for the fast_qr Rust QR code library.

Quick start::

    from fast_qr import QRBuilder, ECL, Shape, SvgBuilder, ImageBuilder, Color

    # Minimal usage
    qr = QRBuilder("https://example.com").build()
    print(qr.to_str())

    # Custom error-correction level
    qr = QRBuilder("Hello").ecl(ECL.H).build()

    # SVG output
    svg = SvgBuilder().shape(Shape.RoundedSquare).to_str(qr)

    # PNG bytes
    png = ImageBuilder().fit_width(400).to_bytes(qr)
"""

from fast_qr._fast_qr import (
    ECL,
    Color,
    ImageBackgroundShape,
    ImageBuilder,
    Mask,
    Mode,
    QRBuilder,
    QRCode,
    Shape,
    SvgBuilder,
    Version,
)

__all__ = [
    "ECL",
    "Color",
    "ImageBackgroundShape",
    "ImageBuilder",
    "Mask",
    "Mode",
    "QRBuilder",
    "QRCode",
    "Shape",
    "SvgBuilder",
    "Version",
]
