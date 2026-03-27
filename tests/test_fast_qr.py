"""Tests for the fast_qr Python bindings."""

from __future__ import annotations

import pytest

from fast_qr import (
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

# ── helpers ───────────────────────────────────────────────────────────────────


def build(text: str = "https://example.com", **kwargs) -> QRCode:
    b = QRBuilder(text)
    for k, v in kwargs.items():
        getattr(b, k)(v)
    return b.build()


# ── QRBuilder / QRCode basics ─────────────────────────────────────────────────


class TestQRBuilder:
    def test_basic_str(self):
        qr = QRBuilder("Hello, world!").build()
        assert isinstance(qr, QRCode)

    def test_basic_bytes(self):
        qr = QRBuilder(b"binary\x00data").build()
        assert isinstance(qr, QRCode)

    def test_invalid_input_raises(self):
        with pytest.raises((TypeError, ValueError)):
            QRBuilder(42).build()  # ty: ignore[invalid-argument-type]

    def test_size_is_positive(self):
        qr = build()
        assert qr.size > 0
        assert qr.size % 4 == 1  # QR sizes follow 4k+1 pattern (21, 25, ...)

    def test_matrix_dimensions(self):
        qr = build()
        mat = qr.matrix
        assert len(mat) == qr.size
        assert all(len(row) == qr.size for row in mat)

    def test_matrix_contains_booleans(self):
        qr = build()
        assert all(isinstance(v, bool) for row in qr.matrix for v in row)

    def test_to_str_returns_string(self):
        qr = build()
        s = qr.to_str()
        assert isinstance(s, str)
        assert len(s) > 0

    def test_repr(self):
        qr = build()
        assert "QRCode" in repr(qr)

    def test_ecl_l(self):
        qr = QRBuilder("hello").ecl(ECL.L).build()
        assert qr.size > 0

    def test_ecl_m(self):
        qr = QRBuilder("hello").ecl(ECL.M).build()
        assert qr.size > 0

    def test_ecl_q(self):
        qr = QRBuilder("hello").ecl(ECL.Q).build()
        assert qr.size > 0

    def test_ecl_h(self):
        qr = QRBuilder("hello").ecl(ECL.H).build()
        assert qr.size > 0

    def test_mode_numeric(self):
        qr = QRBuilder("12345").mode(Mode.Numeric).build()
        assert qr.size > 0

    def test_mode_alphanumeric(self):
        qr = QRBuilder("HELLO WORLD").mode(Mode.Alphanumeric).build()
        assert qr.size > 0

    def test_mode_byte(self):
        qr = QRBuilder("Hello").mode(Mode.Byte).build()
        assert qr.size > 0

    def test_version(self):
        qr = QRBuilder("Hi").version(Version(5)).build()
        # Version 5 → 37×37 modules
        assert qr.size == 37

    def test_version_bounds(self):
        with pytest.raises(ValueError):
            Version(0)
        with pytest.raises(ValueError):
            Version(41)

    def test_all_versions_valid(self):
        for n in range(1, 41):
            v = Version(n)
            assert repr(v) == f"Version({n})"

    def test_mask_patterns(self):
        masks = [
            Mask.Checkerboard,
            Mask.HorizontalLines,
            Mask.VerticalLines,
            Mask.DiagonalLines,
            Mask.LargeCheckerboard,
            Mask.Fields,
            Mask.Diamonds,
            Mask.Meadow,
        ]
        for mask in masks:
            qr = QRBuilder("test").mask(mask).build()
            assert qr.size > 0

    def test_chained_options(self):
        qr = QRBuilder("https://example.com").ecl(ECL.H).mode(Mode.Byte).version(Version(10)).build()
        # Version 10 → 57×57
        assert qr.size == 57

    def test_data_too_large_raises(self):
        """Forcing a tiny version with too much data should raise."""
        with pytest.raises(RuntimeError):
            QRBuilder("A" * 4296).version(Version(1)).ecl(ECL.H).build()


# ── ECL ───────────────────────────────────────────────────────────────────────


class TestECL:
    def test_equality(self):
        assert ECL.L == ECL.L
        assert ECL.H != ECL.L

    def test_repr(self):
        assert repr(ECL.Q) == "ECL.Q"

    def test_all_variants_exist(self):
        for name in ("L", "M", "Q", "H"):
            assert hasattr(ECL, name)


# ── Mode ──────────────────────────────────────────────────────────────────────


class TestMode:
    def test_equality(self):
        assert Mode.Numeric == Mode.Numeric
        assert Mode.Byte != Mode.Numeric

    def test_repr(self):
        assert repr(Mode.Alphanumeric) == "Mode.Alphanumeric"

    def test_all_variants_exist(self):
        for name in ("Numeric", "Alphanumeric", "Byte"):
            assert hasattr(Mode, name)


# ── Version ───────────────────────────────────────────────────────────────────


class TestVersion:
    def test_repr(self):
        assert repr(Version(1)) == "Version(1)"
        assert repr(Version(40)) == "Version(40)"

    def test_equality(self):
        assert Version(1) == Version(1)
        assert Version(1) != Version(2)


# ── Mask ──────────────────────────────────────────────────────────────────────


class TestMask:
    def test_equality(self):
        assert Mask.Checkerboard == Mask.Checkerboard
        assert Mask.Fields != Mask.Diamonds

    def test_repr_contains_name(self):
        assert "Checkerboard" in repr(Mask.Checkerboard)


# ── Shape ─────────────────────────────────────────────────────────────────────


class TestShape:
    def test_all_variants_exist(self):
        names = (
            "Square",
            "Circle",
            "RoundedSquare",
            "Vertical",
            "Horizontal",
            "Diamond",
        )
        for name in names:
            assert hasattr(Shape, name)

    def test_equality(self):
        assert Shape.Square == Shape.Square
        assert Shape.Circle != Shape.Diamond


# ── Color ─────────────────────────────────────────────────────────────────────


class TestColor:
    def test_from_hex_string(self):
        c = Color("#ff0000")
        assert isinstance(c, Color)

    def test_from_named_color(self):
        c = Color("red")
        assert isinstance(c, Color)

    def test_from_rgb_list(self):
        c = Color([255, 0, 0])
        assert isinstance(c, Color)

    def test_from_rgba_list(self):
        c = Color([255, 0, 0, 128])
        assert isinstance(c, Color)

    def test_invalid_list_length_raises(self):
        with pytest.raises(ValueError):
            Color([255, 0])

    def test_invalid_type_raises(self):
        with pytest.raises((TypeError, ValueError)):
            Color(123)  # ty: ignore[invalid-argument-type]

    def test_repr(self):
        assert "Color" in repr(Color("#000000"))

    def test_equality(self):
        assert Color("#ffffff") == Color("#ffffff")
        assert Color("#ffffff") != Color("#000000")


# ── SvgBuilder ────────────────────────────────────────────────────────────────


class TestSvgBuilder:
    def test_default_to_str(self):
        qr = build()
        svg = SvgBuilder().to_str(qr)
        assert svg.startswith("<svg")

    def test_margin(self):
        qr = build()
        svg = SvgBuilder().margin(8).to_str(qr)
        assert "<svg" in svg

    def test_module_color(self):
        qr = build()
        svg = SvgBuilder().module_color(Color("#1a1a1a")).to_str(qr)
        assert "1a1a1a" in svg

    def test_background_color(self):
        qr = build()
        svg = SvgBuilder().background_color(Color("#fafafa")).to_str(qr)
        assert "fafafa" in svg

    def test_all_shapes(self):
        qr = build()
        shapes = [
            Shape.Square,
            Shape.Circle,
            Shape.RoundedSquare,
            Shape.Vertical,
            Shape.Horizontal,
            Shape.Diamond,
        ]
        for shape in shapes:
            svg = SvgBuilder().shape(shape).to_str(qr)
            assert "<svg" in svg

    def test_shape_color(self):
        qr = build()
        svg = SvgBuilder().shape_color(Shape.Circle, Color("#ff0000")).to_str(qr)
        assert "<svg" in svg

    def test_image_background_shape(self):
        qr = build()
        for bg_shape in (
            ImageBackgroundShape.Square,
            ImageBackgroundShape.Circle,
            ImageBackgroundShape.RoundedSquare,
        ):
            svg = SvgBuilder().image_background_shape(bg_shape).to_str(qr)
            assert "<svg" in svg

    def test_image_size_and_gap(self):
        qr = build()
        svg = SvgBuilder().image_size(0.3).image_gap(1.0).to_str(qr)
        assert "<svg" in svg

    def test_image_position(self):
        qr = build()
        svg = SvgBuilder().image_position(0.5, 0.5).to_str(qr)
        assert "<svg" in svg

    def test_to_file(self, tmp_path):
        qr = build()
        path = str(tmp_path / "test.svg")
        SvgBuilder().to_file(qr, path)
        with open(path) as f:
            content = f.read()
        assert content.startswith("<svg")

    def test_chained(self):
        qr = build()
        svg = (
            SvgBuilder()
            .margin(4)
            .module_color(Color("#222222"))
            .background_color(Color("#ffffff"))
            .shape(Shape.RoundedSquare)
            .to_str(qr)
        )
        assert "<svg" in svg


# ── ImageBuilder ──────────────────────────────────────────────────────────────


class TestImageBuilder:
    def test_to_bytes_returns_png(self):
        qr = build()
        data = ImageBuilder().to_bytes(qr)
        # PNG magic bytes
        assert data[:8] == b"\x89PNG\r\n\x1a\n"

    def test_fit_width(self):
        qr = build()
        data = ImageBuilder().fit_width(200).to_bytes(qr)
        assert data[:4] == b"\x89PNG"

    def test_fit_height(self):
        qr = build()
        data = ImageBuilder().fit_height(200).to_bytes(qr)
        assert data[:4] == b"\x89PNG"

    def test_shapes(self):
        qr = build()
        for shape in (Shape.Square, Shape.Circle, Shape.RoundedSquare):
            data = ImageBuilder().shape(shape).to_bytes(qr)
            assert data[:4] == b"\x89PNG"

    def test_module_color(self):
        qr = build()
        data = ImageBuilder().module_color(Color("#ff0000")).to_bytes(qr)
        assert data[:4] == b"\x89PNG"

    def test_background_color(self):
        qr = build()
        data = ImageBuilder().background_color(Color("#ffffff")).to_bytes(qr)
        assert data[:4] == b"\x89PNG"

    def test_margin(self):
        qr = build()
        data = ImageBuilder().margin(8).to_bytes(qr)
        assert data[:4] == b"\x89PNG"

    def test_to_file(self, tmp_path):
        qr = build()
        path = str(tmp_path / "test.png")
        ImageBuilder().fit_width(100).to_file(qr, path)
        with open(path, "rb") as f:
            magic = f.read(8)
        assert magic == b"\x89PNG\r\n\x1a\n"

    def test_chained(self):
        qr = build()
        data = (
            ImageBuilder()
            .fit_width(256)
            .module_color(Color("#333333"))
            .background_color(Color("#eeeeee"))
            .shape(Shape.Circle)
            .to_bytes(qr)
        )
        assert data[:4] == b"\x89PNG"
