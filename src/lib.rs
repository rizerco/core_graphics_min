/// The basic type for floating-point scalar values in Core Graphics and related frameworks.
pub type CGFloat = f64;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
/// A structure that contains a point in a two-dimensional coordinate system.
pub struct CGPoint {
    /// The x-coordinate of the point.
    pub x: CGFloat,
    /// The y-coordinate of the point.
    pub y: CGFloat,
}

impl CGPoint {
    /// The point with location (0,0).
    pub const ZERO: CGPoint = CGPoint { x: 0.0, y: 0.0 };
}

#[repr(C)]
#[derive(Debug, PartialEq, Copy, Clone)]
/// A structure that contains width and height values.
pub struct CGSize {
    /// A width value.
    pub width: CGFloat,
    /// A height value.
    pub height: CGFloat,
}

impl CGSize {
    /// The size whose width and height are both zero.
    pub const ZERO: CGSize = CGSize {
        width: 0.0,
        height: 0.0,
    };
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
/// A structure that contains the location and dimensions of a rectangle.
pub struct CGRect {
    /// A point that specifies the coordinates of the rectangle’s origin.
    pub origin: CGPoint,
    /// A size that specifies the height and width of the rectangle.
    pub size: CGSize,
}
impl CGRect {
    /// Creates a rectangle with coordinates and dimensions specified as CGFloat values.
    pub fn new(x: CGFloat, y: CGFloat, width: CGFloat, height: CGFloat) -> CGRect {
        CGRect {
            origin: CGPoint { x, y },
            size: CGSize { width, height },
        }
    }

    /// The rectangle whose origin and size are both zero.
    pub const ZERO: CGRect = CGRect {
        origin: CGPoint { x: 0.0, y: 0.0 },
        size: CGSize {
            width: 0.0,
            height: 0.0,
        },
    };

    /// Returns the height of a rectangle.
    pub fn height(&self) -> CGFloat {
        self.size.height
    }

    /// Returns the width of a rectangle.
    pub fn width(&self) -> CGFloat {
        self.size.width
    }
}
