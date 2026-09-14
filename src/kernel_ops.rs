use crate::convert::{lwpolyline_to_polyline, polyline_to_lwpolyline};
use acadrust::entities::{Circle, Line, LwPolyline, MText};
use cadkernel::geom2d::curve::Curve;
use cadkernel::space::PlanarCurve;

/// Geometry-kernel operations that may be available for an entity type.
///
/// Not every entity implements every method; the default implementations
/// return `None` or an empty vector so callers can fall back gracefully.
pub trait KernelOps {
    /// Convert the entity to a planar curve, if it has a direct curve
    /// representation.
    fn to_planar_curve(&self) -> Option<PlanarCurve> {
        None
    }

    /// Offset the entity by `distance`, using `side` to decide the direction.
    ///
    /// The returned entities are standalone copies that do not live in any
    /// document; their handles are null and must be assigned before adding
    /// them to a document.
    fn offset(&self, _distance: f64, _side: [f64; 2]) -> Vec<LwPolyline> {
        Vec::new()
    }
}

impl KernelOps for Line {
    fn to_planar_curve(&self) -> Option<PlanarCurve> {
        Some(PlanarCurve::flat(Curve::Line(cadkernel::geom2d::curve::Line {
            start: [self.start.x, self.start.y],
            end: [self.end.x, self.end.y],
        })))
    }
}

impl KernelOps for Circle {
    fn to_planar_curve(&self) -> Option<PlanarCurve> {
        Some(PlanarCurve::flat(Curve::Circle(cadkernel::geom2d::curve::Circle {
            centre: [self.center.x, self.center.y],
            radius: self.radius,
        })))
    }
}

impl KernelOps for LwPolyline {
    fn offset(&self, distance: f64, side: [f64; 2]) -> Vec<LwPolyline> {
        let source = lwpolyline_to_polyline(self);
        cadkernel::geom2d::offset::offset_polyline(&source, distance, side)
            .into_iter()
            .map(|result| polyline_to_lwpolyline(self, result))
            .collect()
    }
}

impl KernelOps for MText {}
