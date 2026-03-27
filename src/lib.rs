use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;

use fast_qr::convert::image::ImageBuilder as FqImageBuilder;
use fast_qr::convert::svg::SvgBuilder as FqSvgBuilder;
use fast_qr::convert::{
    Builder, Color as FqColor, ImageBackgroundShape as FqIBShape, Shape as FqShape,
};
use fast_qr::Mask as FqMask;
use fast_qr::Mode as FqMode;
use fast_qr::Version as FqVersion;
use fast_qr::ECL as FqECL;
use fast_qr::{QRBuilder as FqQRBuilder, QRCode as FqQRCode};

// ── ECL ──────────────────────────────────────────────────────────────────────

/// Error Correction Level.
///
/// Controls what fraction of codewords can be restored if the symbol is
/// damaged.  Higher levels trade capacity for resilience.
#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq, Debug)]
pub enum ECL {
    /// 7 % recovery capacity (largest data capacity).
    L,
    /// 15 % recovery capacity.
    M,
    /// 25 % recovery capacity (library default).
    Q,
    /// 30 % recovery capacity (smallest data capacity).
    H,
}

impl From<ECL> for FqECL {
    fn from(v: ECL) -> Self {
        match v {
            ECL::L => FqECL::L,
            ECL::M => FqECL::M,
            ECL::Q => FqECL::Q,
            ECL::H => FqECL::H,
        }
    }
}

#[pymethods]
impl ECL {
    fn __repr__(&self) -> &str {
        match self {
            ECL::L => "ECL.L",
            ECL::M => "ECL.M",
            ECL::Q => "ECL.Q",
            ECL::H => "ECL.H",
        }
    }
}

// ── Mode ─────────────────────────────────────────────────────────────────────

/// Data-encoding mode.
///
/// Choosing the right mode maximises the amount of data that fits in a given
/// QR code version.  The library selects the optimal mode automatically when
/// none is specified.
#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq, Debug)]
pub enum Mode {
    /// Digits 0–9 only (~3.3 bits per character).
    Numeric,
    /// Uppercase letters, digits and ``$%*+-./:`` plus space.
    Alphanumeric,
    /// Any bytes / UTF-8 text (least efficient).
    Byte,
}

impl From<Mode> for FqMode {
    fn from(v: Mode) -> Self {
        match v {
            Mode::Numeric => FqMode::Numeric,
            Mode::Alphanumeric => FqMode::Alphanumeric,
            Mode::Byte => FqMode::Byte,
        }
    }
}

#[pymethods]
impl Mode {
    fn __repr__(&self) -> &str {
        match self {
            Mode::Numeric => "Mode.Numeric",
            Mode::Alphanumeric => "Mode.Alphanumeric",
            Mode::Byte => "Mode.Byte",
        }
    }
}

// ── Version ───────────────────────────────────────────────────────────────────

/// QR code version (1–40).
///
/// Higher versions encode more data at the cost of a larger symbol.
/// If not specified, the library picks the smallest version that fits the
/// data.
#[pyclass]
#[derive(Clone, Debug)]
pub struct Version(FqVersion);

#[pymethods]
impl Version {
    /// Create a version from an integer in the range 1–40.
    #[new]
    pub fn new(v: u8) -> PyResult<Self> {
        let fq = match v {
            1 => FqVersion::V01,
            2 => FqVersion::V02,
            3 => FqVersion::V03,
            4 => FqVersion::V04,
            5 => FqVersion::V05,
            6 => FqVersion::V06,
            7 => FqVersion::V07,
            8 => FqVersion::V08,
            9 => FqVersion::V09,
            10 => FqVersion::V10,
            11 => FqVersion::V11,
            12 => FqVersion::V12,
            13 => FqVersion::V13,
            14 => FqVersion::V14,
            15 => FqVersion::V15,
            16 => FqVersion::V16,
            17 => FqVersion::V17,
            18 => FqVersion::V18,
            19 => FqVersion::V19,
            20 => FqVersion::V20,
            21 => FqVersion::V21,
            22 => FqVersion::V22,
            23 => FqVersion::V23,
            24 => FqVersion::V24,
            25 => FqVersion::V25,
            26 => FqVersion::V26,
            27 => FqVersion::V27,
            28 => FqVersion::V28,
            29 => FqVersion::V29,
            30 => FqVersion::V30,
            31 => FqVersion::V31,
            32 => FqVersion::V32,
            33 => FqVersion::V33,
            34 => FqVersion::V34,
            35 => FqVersion::V35,
            36 => FqVersion::V36,
            37 => FqVersion::V37,
            38 => FqVersion::V38,
            39 => FqVersion::V39,
            40 => FqVersion::V40,
            _ => {
                return Err(PyValueError::new_err(format!(
                    "version must be 1–40, got {v}"
                )))
            }
        };
        Ok(Version(fq))
    }

    fn __repr__(&self) -> String {
        // Extract numeric value from debug output like "V01"
        let s = format!("{:?}", self.0);
        let n: u8 = s.trim_start_matches('V').parse().unwrap_or(0);
        format!("Version({n})")
    }

    fn __eq__(&self, other: &Version) -> bool {
        // Compare via debug repr since FqVersion may not impl PartialEq in a
        // way we can rely on through the wrapper.
        format!("{:?}", self.0) == format!("{:?}", other.0)
    }
}

// ── Mask ─────────────────────────────────────────────────────────────────────

/// Mask pattern applied to the QR code data region.
///
/// Masks are applied to improve readability by breaking up visually confusing
/// patterns.  The library selects the optimal mask automatically.
#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq, Debug)]
pub enum Mask {
    Checkerboard,
    HorizontalLines,
    VerticalLines,
    DiagonalLines,
    LargeCheckerboard,
    Fields,
    Diamonds,
    Meadow,
}

impl From<Mask> for FqMask {
    fn from(v: Mask) -> Self {
        match v {
            Mask::Checkerboard => FqMask::Checkerboard,
            Mask::HorizontalLines => FqMask::HorizontalLines,
            Mask::VerticalLines => FqMask::VerticalLines,
            Mask::DiagonalLines => FqMask::DiagonalLines,
            Mask::LargeCheckerboard => FqMask::LargeCheckerboard,
            Mask::Fields => FqMask::Fields,
            Mask::Diamonds => FqMask::Diamonds,
            Mask::Meadow => FqMask::Meadow,
        }
    }
}

#[pymethods]
impl Mask {
    fn __repr__(&self) -> String {
        format!("Mask.{self:?}")
    }
}

// ── Shape ─────────────────────────────────────────────────────────────────────

/// Module shape used when rendering SVG or raster images.
///
/// Note: ``Command`` (custom SVG path function) is not exposed through Python
/// bindings because it requires a Rust closure.
#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq, Debug)]
pub enum Shape {
    Square,
    Circle,
    RoundedSquare,
    Vertical,
    Horizontal,
    Diamond,
}

impl From<Shape> for FqShape {
    fn from(v: Shape) -> Self {
        match v {
            Shape::Square => FqShape::Square,
            Shape::Circle => FqShape::Circle,
            Shape::RoundedSquare => FqShape::RoundedSquare,
            Shape::Vertical => FqShape::Vertical,
            Shape::Horizontal => FqShape::Horizontal,
            Shape::Diamond => FqShape::Diamond,
        }
    }
}

#[pymethods]
impl Shape {
    fn __repr__(&self) -> String {
        format!("Shape.{self:?}")
    }
}

// ── ImageBackgroundShape ──────────────────────────────────────────────────────

/// Background shape for embedded images in SVG/raster output.
#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq, Debug)]
pub enum ImageBackgroundShape {
    Square,
    Circle,
    RoundedSquare,
}

impl From<ImageBackgroundShape> for FqIBShape {
    fn from(v: ImageBackgroundShape) -> Self {
        match v {
            ImageBackgroundShape::Square => FqIBShape::Square,
            ImageBackgroundShape::Circle => FqIBShape::Circle,
            ImageBackgroundShape::RoundedSquare => FqIBShape::RoundedSquare,
        }
    }
}

#[pymethods]
impl ImageBackgroundShape {
    fn __repr__(&self) -> String {
        format!("ImageBackgroundShape.{self:?}")
    }
}

// ── Color ─────────────────────────────────────────────────────────────────────

/// Color value accepted by SVG and image builders.
///
/// Can be constructed from:
///
/// * a CSS hex string ``"#rrggbb"`` or ``"#rrggbbaa"``
/// * a named CSS color string ``"red"``
/// * a list / tuple of three ints ``[r, g, b]`` (0–255)
/// * a list / tuple of four ints  ``[r, g, b, a]`` (0–255)
#[pyclass]
#[derive(Clone, Debug)]
pub struct Color(String);

#[pymethods]
impl Color {
    #[new]
    pub fn new(color: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(s) = color.extract::<String>() {
            return Ok(Color(s));
        }
        if let Ok(v) = color.extract::<Vec<u8>>() {
            return match v.len() {
                3 => Ok(Color(format!("#{:02x}{:02x}{:02x}", v[0], v[1], v[2]))),
                4 => Ok(Color(format!(
                    "#{:02x}{:02x}{:02x}{:02x}",
                    v[0], v[1], v[2], v[3]
                ))),
                n => Err(PyValueError::new_err(format!(
                    "color list must have 3 or 4 elements, got {n}"
                ))),
            };
        }
        Err(PyValueError::new_err(
            "color must be a string (e.g. '#ff0000') or a list of 3–4 ints",
        ))
    }

    fn __repr__(&self) -> String {
        format!("Color({:?})", self.0)
    }

    fn __eq__(&self, other: &Color) -> bool {
        self.0 == other.0
    }
}

impl From<Color> for FqColor {
    fn from(c: Color) -> Self {
        FqColor::from(c.0)
    }
}

// ── QRCode ────────────────────────────────────────────────────────────────────

/// A generated QR code symbol.
///
/// Instances are produced by :class:`QRBuilder`.  The raw module matrix can
/// be read through :attr:`matrix`, and convenience methods for terminal
/// display are provided.
#[pyclass]
#[derive(Clone, Debug)]
pub struct QRCode(FqQRCode);

#[pymethods]
impl QRCode {
    /// Width/height of the module matrix (number of modules per side).
    #[getter]
    pub fn size(&self) -> usize {
        self.0.size
    }

    /// 2-D list of booleans; ``True`` means a dark module.
    ///
    /// ``matrix[row][col]``
    #[getter]
    pub fn matrix(&self) -> Vec<Vec<bool>> {
        let size = self.0.size;
        // QRCode::Index<usize> returns a row slice (&[Module]).
        // Module::value() returns the dark/light boolean.
        (0..size)
            .map(|row| (0..size).map(|col| self.0[row][col].value()).collect())
            .collect()
    }

    /// Unicode block-character rendering suitable for terminal display.
    pub fn to_str(&self) -> String {
        self.0.to_str()
    }

    fn __repr__(&self) -> String {
        format!("QRCode(size={})", self.0.size)
    }
}

// ── QRBuilder ─────────────────────────────────────────────────────────────────

/// Builder for QR code generation.
///
/// Example::
///
///     qr = QRBuilder("Hello, world!").build()
///
///     qr = (
///         QRBuilder("https://example.com")
///         .ecl(ECL.H)
///         .version(Version(5))
///         .build()
///     )
#[pyclass]
pub struct QRBuilder {
    data: Vec<u8>,
    ecl: Option<FqECL>,
    mode: Option<FqMode>,
    version: Option<FqVersion>,
    mask: Option<FqMask>,
}

#[pymethods]
impl QRBuilder {
    /// Accepts ``str`` or ``bytes`` as input data.
    #[new]
    pub fn new(data: &Bound<'_, PyAny>) -> PyResult<Self> {
        let bytes: Vec<u8> = if let Ok(s) = data.extract::<String>() {
            s.into_bytes()
        } else if let Ok(b) = data.extract::<Vec<u8>>() {
            b
        } else {
            return Err(PyValueError::new_err("data must be str or bytes"));
        };
        Ok(QRBuilder {
            data: bytes,
            ecl: None,
            mode: None,
            version: None,
            mask: None,
        })
    }

    /// Force a specific error correction level.
    pub fn ecl(mut slf: PyRefMut<'_, Self>, ecl: ECL) -> PyRefMut<'_, Self> {
        slf.ecl = Some(ecl.into());
        slf
    }

    /// Force a specific encoding mode.
    pub fn mode(mut slf: PyRefMut<'_, Self>, mode: Mode) -> PyRefMut<'_, Self> {
        slf.mode = Some(mode.into());
        slf
    }

    /// Force a specific QR code version (1–40).
    pub fn version(mut slf: PyRefMut<'_, Self>, version: Version) -> PyRefMut<'_, Self> {
        slf.version = Some(version.0);
        slf
    }

    /// Force a specific mask pattern.
    pub fn mask(mut slf: PyRefMut<'_, Self>, mask: Mask) -> PyRefMut<'_, Self> {
        slf.mask = Some(mask.into());
        slf
    }

    /// Generate and return the :class:`QRCode`.
    ///
    /// Raises ``RuntimeError`` if the data is too large for the chosen
    /// version/ECL combination.
    pub fn build(&self) -> PyResult<QRCode> {
        // FqQRBuilder methods take &mut self and return &mut Self — we call
        // them for their side-effects and ignore the returned reference.
        let mut b = FqQRBuilder::new(self.data.clone());
        if let Some(ecl) = self.ecl {
            b.ecl(ecl);
        }
        if let Some(mode) = self.mode {
            b.mode(mode);
        }
        if let Some(version) = self.version {
            b.version(version);
        }
        if let Some(mask) = self.mask {
            b.mask(mask);
        }
        b.build()
            .map(QRCode)
            .map_err(|e| PyRuntimeError::new_err(format!("QR code generation failed: {e:?}")))
    }
}

// ── SvgBuilder ────────────────────────────────────────────────────────────────

/// Builder for SVG output.
///
/// Example::
///
///     svg_str = (
///         SvgBuilder()
///         .margin(4)
///         .module_color(Color("#1a1a1a"))
///         .shape(Shape.RoundedSquare)
///         .to_str(qr)
///     )
#[pyclass]
pub struct SvgBuilder(FqSvgBuilder);

#[pymethods]
impl SvgBuilder {
    #[new]
    pub fn new() -> Self {
        SvgBuilder(FqSvgBuilder::default())
    }

    /// Quiet-zone margin in modules (default: 4).
    pub fn margin(mut slf: PyRefMut<'_, Self>, margin: usize) -> PyRefMut<'_, Self> {
        slf.0.margin(margin);
        slf
    }

    /// Color for data modules (default: black).
    pub fn module_color(mut slf: PyRefMut<'_, Self>, color: Color) -> PyRefMut<'_, Self> {
        slf.0.module_color(FqColor::from(color));
        slf
    }

    /// Background color (default: white).
    pub fn background_color(mut slf: PyRefMut<'_, Self>, color: Color) -> PyRefMut<'_, Self> {
        slf.0.background_color(FqColor::from(color));
        slf
    }

    /// Shape for all modules.
    pub fn shape(mut slf: PyRefMut<'_, Self>, shape: Shape) -> PyRefMut<'_, Self> {
        slf.0.shape(FqShape::from(shape));
        slf
    }

    /// Apply a specific color to a particular shape (useful for mixed shapes).
    pub fn shape_color(
        mut slf: PyRefMut<'_, Self>,
        shape: Shape,
        color: Color,
    ) -> PyRefMut<'_, Self> {
        slf.0
            .shape_color(FqShape::from(shape), FqColor::from(color));
        slf
    }

    /// Embed an image.  *image* is either a filesystem path or a
    /// base64-encoded data URI.
    pub fn image(mut slf: PyRefMut<'_, Self>, image: String) -> PyRefMut<'_, Self> {
        slf.0.image(image);
        slf
    }

    /// Background color shown behind the embedded image.
    pub fn image_background_color(mut slf: PyRefMut<'_, Self>, color: Color) -> PyRefMut<'_, Self> {
        slf.0.image_background_color(FqColor::from(color));
        slf
    }

    /// Background shape for the embedded image.
    pub fn image_background_shape(
        mut slf: PyRefMut<'_, Self>,
        shape: ImageBackgroundShape,
    ) -> PyRefMut<'_, Self> {
        slf.0.image_background_shape(FqIBShape::from(shape));
        slf
    }

    /// Size of the embedded image, as a fraction of the symbol's total size
    /// (0.0–1.0).
    pub fn image_size(mut slf: PyRefMut<'_, Self>, size: f64) -> PyRefMut<'_, Self> {
        slf.0.image_size(size);
        slf
    }

    /// Gap between the embedded image and surrounding modules.
    pub fn image_gap(mut slf: PyRefMut<'_, Self>, gap: f64) -> PyRefMut<'_, Self> {
        slf.0.image_gap(gap);
        slf
    }

    /// Position of the embedded image centre (in module coordinates).
    pub fn image_position(mut slf: PyRefMut<'_, Self>, x: f64, y: f64) -> PyRefMut<'_, Self> {
        slf.0.image_position(x, y);
        slf
    }

    /// Render *qr* to an SVG string.
    pub fn to_str(&self, qr: &QRCode) -> String {
        self.0.to_str(&qr.0)
    }

    /// Render *qr* and write the SVG to *path*.
    pub fn to_file(&self, qr: &QRCode, path: &str) -> PyResult<()> {
        self.0
            .to_file(&qr.0, path)
            .map_err(|e| PyRuntimeError::new_err(format!("SVG write failed: {e:?}")))
    }
}

// ── ImageBuilder ──────────────────────────────────────────────────────────────

/// Builder for PNG raster output.
///
/// Example::
///
///     png_bytes = (
///         ImageBuilder()
///         .fit_width(512)
///         .shape(Shape.Circle)
///         .to_bytes(qr)
///     )
#[pyclass]
pub struct ImageBuilder(FqImageBuilder);

#[pymethods]
impl ImageBuilder {
    #[new]
    pub fn new() -> Self {
        ImageBuilder(FqImageBuilder::default())
    }

    /// Constrain the rendered image to at most *width* pixels wide.
    pub fn fit_width(mut slf: PyRefMut<'_, Self>, width: u32) -> PyRefMut<'_, Self> {
        slf.0.fit_width(width);
        slf
    }

    /// Constrain the rendered image to at most *height* pixels tall.
    pub fn fit_height(mut slf: PyRefMut<'_, Self>, height: u32) -> PyRefMut<'_, Self> {
        slf.0.fit_height(height);
        slf
    }

    /// Quiet-zone margin in modules (default: 4).
    pub fn margin(mut slf: PyRefMut<'_, Self>, margin: usize) -> PyRefMut<'_, Self> {
        slf.0.margin(margin);
        slf
    }

    /// Color for data modules.
    pub fn module_color(mut slf: PyRefMut<'_, Self>, color: Color) -> PyRefMut<'_, Self> {
        slf.0.module_color(FqColor::from(color));
        slf
    }

    /// Background color.
    pub fn background_color(mut slf: PyRefMut<'_, Self>, color: Color) -> PyRefMut<'_, Self> {
        slf.0.background_color(FqColor::from(color));
        slf
    }

    /// Shape for all modules.
    pub fn shape(mut slf: PyRefMut<'_, Self>, shape: Shape) -> PyRefMut<'_, Self> {
        slf.0.shape(FqShape::from(shape));
        slf
    }

    /// Apply a specific color to a particular shape.
    pub fn shape_color(
        mut slf: PyRefMut<'_, Self>,
        shape: Shape,
        color: Color,
    ) -> PyRefMut<'_, Self> {
        slf.0
            .shape_color(FqShape::from(shape), FqColor::from(color));
        slf
    }

    /// Embed an image (filesystem path or base64 data URI).
    pub fn image(mut slf: PyRefMut<'_, Self>, image: String) -> PyRefMut<'_, Self> {
        slf.0.image(image);
        slf
    }

    /// Background color shown behind the embedded image.
    pub fn image_background_color(mut slf: PyRefMut<'_, Self>, color: Color) -> PyRefMut<'_, Self> {
        slf.0.image_background_color(FqColor::from(color));
        slf
    }

    /// Background shape for the embedded image.
    pub fn image_background_shape(
        mut slf: PyRefMut<'_, Self>,
        shape: ImageBackgroundShape,
    ) -> PyRefMut<'_, Self> {
        slf.0.image_background_shape(FqIBShape::from(shape));
        slf
    }

    /// Size of the embedded image as a fraction of the symbol's total size.
    pub fn image_size(mut slf: PyRefMut<'_, Self>, size: f64) -> PyRefMut<'_, Self> {
        slf.0.image_size(size);
        slf
    }

    /// Gap between the embedded image and surrounding modules.
    pub fn image_gap(mut slf: PyRefMut<'_, Self>, gap: f64) -> PyRefMut<'_, Self> {
        slf.0.image_gap(gap);
        slf
    }

    /// Position of the embedded image centre.
    pub fn image_position(mut slf: PyRefMut<'_, Self>, x: f64, y: f64) -> PyRefMut<'_, Self> {
        slf.0.image_position(x, y);
        slf
    }

    /// Render *qr* to raw PNG bytes.
    pub fn to_bytes(&self, qr: &QRCode) -> PyResult<Vec<u8>> {
        self.0
            .to_bytes(&qr.0)
            .map_err(|e| PyRuntimeError::new_err(format!("image render failed: {e:?}")))
    }

    /// Render *qr* and write PNG to *path*.
    pub fn to_file(&self, qr: &QRCode, path: &str) -> PyResult<()> {
        self.0
            .to_file(&qr.0, path)
            .map_err(|e| PyRuntimeError::new_err(format!("image write failed: {e:?}")))
    }
}

// ── Module ────────────────────────────────────────────────────────────────────

#[pymodule]
fn _fast_qr(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ECL>()?;
    m.add_class::<Mode>()?;
    m.add_class::<Version>()?;
    m.add_class::<Mask>()?;
    m.add_class::<Shape>()?;
    m.add_class::<ImageBackgroundShape>()?;
    m.add_class::<Color>()?;
    m.add_class::<QRCode>()?;
    m.add_class::<QRBuilder>()?;
    m.add_class::<SvgBuilder>()?;
    m.add_class::<ImageBuilder>()?;
    Ok(())
}
