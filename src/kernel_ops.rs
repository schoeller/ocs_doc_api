use crate::convert::{lwpolyline_to_polyline, polyline_to_lwpolyline};
use acadrust::entities::{
    AttributeDefinition, AttributeEntity, Block, EntityType, Face3D, Hatch, Helix, Insert, LwPolyline,
    Mesh, MText, Point, PolyfaceMesh, PolygonMeshEntity, Polyline, Polyline2D, Polyline3D, Ray,
    Shape, Solid, Spline, Text, Tolerance, Wipeout,
};
use acadrust::entities::{Arc, Circle, Ellipse, Line, Solid3D, Surface, XLine};
use acadrust::types::{Vector2, Vector3};
use cadkernel::geom2d::curve::{Curve, EllipseArc, Line as LineCurve};
use cadkernel::geom2d::{NurbsCurve, Polyline as PolylineCurve, PolylineVertex};
use cadkernel::space::{PlanarCurve, Plane};

fn is_default_normal(normal: Vector3) -> bool {
    // Only the world +Z axis is accepted as a default extrusion normal.
    // A -Z normal would mirror the 2-D parameter space and is therefore
    // rejected rather than silently re-oriented.
    normal.x.abs() <= 1e-9 && normal.y.abs() <= 1e-9 && (normal.z - 1.0).abs() <= 1e-9
}

fn xy_plane_at(z: f64) -> Plane {
    Plane::from_axes([0.0, 0.0, z], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0])
}

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

    /// Explode a compound entity into its constituent entities.
    ///
    /// The returned entities are standalone copies that do not live in any
    /// document; their handles are null and must be assigned before adding
    /// them to a document.
    fn explode(&self) -> Vec<EntityType> {
        Vec::new()
    }

    /// Convert a 3-D or mesh-backed entity to a tessellated mesh.
    ///
    /// Returns `None` when tessellation is not supported or the source data is
    /// not available.
    fn to_mesh(&self) -> Option<acadrust::entities::Mesh> {
        None
    }
}

impl KernelOps for Line {
    fn to_planar_curve(&self) -> Option<PlanarCurve> {
        if !is_default_normal(self.normal) || (self.end.z - self.start.z).abs() > 1e-9 {
            return None;
        }
        let z = self.start.z;
        Some(PlanarCurve::new(
            xy_plane_at(z),
            Curve::Line(LineCurve {
                start: [self.start.x, self.start.y],
                end: [self.end.x, self.end.y],
            }),
        ))
    }
}

impl KernelOps for Circle {
    fn to_planar_curve(&self) -> Option<PlanarCurve> {
        if !is_default_normal(self.normal) {
            return None;
        }
        Some(PlanarCurve::new(
            xy_plane_at(self.center.z),
            Curve::Circle(cadkernel::geom2d::curve::Circle {
                centre: [self.center.x, self.center.y],
                radius: self.radius,
            }),
        ))
    }
}

impl KernelOps for Arc {
    fn to_planar_curve(&self) -> Option<PlanarCurve> {
        if !is_default_normal(self.normal) {
            return None;
        }
        Some(PlanarCurve::new(
            xy_plane_at(self.center.z),
            Curve::Arc(cadkernel::geom2d::curve::Arc {
                centre: [self.center.x, self.center.y],
                radius: self.radius,
                start_angle: self.start_angle,
                end_angle: self.end_angle,
            }),
        ))
    }
}

impl KernelOps for Ellipse {
    fn to_planar_curve(&self) -> Option<PlanarCurve> {
        if !is_default_normal(self.normal) || self.major_axis.z.abs() > 1e-9 {
            return None;
        }
        let major_dx = self.major_axis.x;
        let major_dy = self.major_axis.y;
        let major_radius = (major_dx * major_dx + major_dy * major_dy).sqrt();
        if major_radius <= 1e-12 {
            return None;
        }
        Some(PlanarCurve::new(
            xy_plane_at(self.center.z),
            Curve::Ellipse(EllipseArc {
                ellipse: cadkernel::geom2d::Ellipse {
                    centre: [self.center.x, self.center.y],
                    major_radius,
                    minor_radius: major_radius * self.minor_axis_ratio,
                    major_axis: [major_dx / major_radius, major_dy / major_radius],
                },
                start_parameter: self.start_parameter,
                end_parameter: self.end_parameter,
            }),
        ))
    }
}

impl KernelOps for Spline {
    fn to_planar_curve(&self) -> Option<PlanarCurve> {
        if self.control_points.is_empty() {
            return None;
        }
        let source = if self.fit_points.is_empty() {
            &self.control_points
        } else {
            &self.fit_points
        };
        let z = source.first()?.z;
        if !is_default_normal(self.normal) || source.iter().any(|p| (p.z - z).abs() > 1e-9) {
            return None;
        }
        let points: Vec<[f64; 2]> = source.iter().map(|p| [p.x, p.y]).collect();
        let weights = if self.weights.len() == source.len() {
            Some(self.weights.clone())
        } else {
            None
        };
        let max_degree = points.len().saturating_sub(1).max(1) as i32;
        let degree = self.degree.max(1).min(max_degree) as usize;
        let nurbs = NurbsCurve::new(degree, points, self.knots.clone(), weights)?;
        Some(PlanarCurve::new(xy_plane_at(z), Curve::Nurbs(nurbs)))
    }
}

impl KernelOps for Polyline {
    fn to_planar_curve(&self) -> Option<PlanarCurve> {
        if self.vertices.len() < 2 {
            return None;
        }
        let z = self.vertices.first()?.location.z;
        if self.vertices.iter().any(|v| (v.location.z - z).abs() > 1e-9) {
            return None;
        }
        Some(PlanarCurve::new(
            xy_plane_at(z),
            Curve::Polyline(PolylineCurve {
                vertices: self
                    .vertices
                    .iter()
                    .map(|v| PolylineVertex {
                        position: [v.location.x, v.location.y],
                        bulge: 0.0,
                    })
                    .collect(),
                closed: self.flags.is_closed(),
            }),
        ))
    }
}

impl KernelOps for Polyline2D {
    fn to_planar_curve(&self) -> Option<PlanarCurve> {
        if self.vertices.len() < 2 {
            return None;
        }
        if !is_default_normal(self.normal) {
            return None;
        }
        Some(PlanarCurve::new(
            xy_plane_at(self.elevation),
            Curve::Polyline(PolylineCurve {
                vertices: self
                    .vertices
                    .iter()
                    .map(|v| PolylineVertex {
                        position: [v.location.x, v.location.y],
                        bulge: v.bulge,
                    })
                    .collect(),
                closed: self.is_closed(),
            }),
        ))
    }

    fn offset(&self, distance: f64, side: [f64; 2]) -> Vec<LwPolyline> {
        // Polyline2D is represented as a sequence of 2-D vertices. Convert it
        // to an LwPolyline, offset that, and return the resulting polylines.
        let tmp = lwpolyline_from_polyline2d(self);
        tmp.offset(distance, side)
    }
}

impl KernelOps for Polyline3D {
    fn to_planar_curve(&self) -> Option<PlanarCurve> {
        if self.vertices.len() < 2 {
            return None;
        }
        let z = self.vertices.first()?.position.z;
        if self.vertices.iter().any(|v| (v.position.z - z).abs() > 1e-9) {
            return None;
        }
        Some(PlanarCurve::new(
            xy_plane_at(z),
            Curve::Polyline(PolylineCurve {
                vertices: self
                    .vertices
                    .iter()
                    .map(|v| PolylineVertex {
                        position: [v.position.x, v.position.y],
                        bulge: 0.0,
                    })
                    .collect(),
                closed: self.flags.closed,
            }),
        ))
    }
}

impl KernelOps for Ray {
    fn to_planar_curve(&self) -> Option<PlanarCurve> {
        if self.direction.z.abs() > 1e-9 {
            return None;
        }
        Some(PlanarCurve::new(
            xy_plane_at(self.base_point.z),
            Curve::Ray(cadkernel::geom2d::curve::Ray {
                origin: [self.base_point.x, self.base_point.y],
                direction: [self.direction.x, self.direction.y],
            }),
        ))
    }
}

impl KernelOps for XLine {
    fn to_planar_curve(&self) -> Option<PlanarCurve> {
        if self.direction.z.abs() > 1e-9 {
            return None;
        }
        Some(PlanarCurve::new(
            xy_plane_at(self.base_point.z),
            Curve::XLine(cadkernel::geom2d::curve::XLine {
                base: [self.base_point.x, self.base_point.y],
                direction: [self.direction.x, self.direction.y],
            }),
        ))
    }
}

impl KernelOps for Helix {
    fn to_planar_curve(&self) -> Option<PlanarCurve> {
        // A helix is inherently 3D; it has no faithful planar projection.
        None
    }
}

impl KernelOps for LwPolyline {
    fn to_planar_curve(&self) -> Option<PlanarCurve> {
        if self.vertices.len() < 2 {
            return None;
        }
        if !is_default_normal(self.normal) {
            return None;
        }
        Some(PlanarCurve::new(
            xy_plane_at(self.elevation),
            Curve::Polyline(lwpolyline_to_polyline(self)),
        ))
    }

    fn offset(&self, distance: f64, side: [f64; 2]) -> Vec<LwPolyline> {
        let source = lwpolyline_to_polyline(self);
        cadkernel::geom2d::offset::offset_polyline(&source, distance, side)
            .into_iter()
            .map(|result| polyline_to_lwpolyline(self, result))
            .collect()
    }
}

impl KernelOps for Point {}
impl KernelOps for Text {}
impl KernelOps for MText {}
impl KernelOps for Tolerance {}
impl KernelOps for AttributeDefinition {}
impl KernelOps for AttributeEntity {}
impl KernelOps for Shape {}
impl KernelOps for Hatch {}
impl KernelOps for Wipeout {}

impl KernelOps for Solid {
    fn to_mesh(&self) -> Option<acadrust::entities::Mesh> {
        Some(Mesh::new())
    }
}

impl KernelOps for Insert {
    fn explode(&self) -> Vec<EntityType> {
        // Without access to a document / block definition we cannot produce
        // the block's entities. Returning an empty vector matches the
        // fallback semantics of the trait.
        Vec::new()
    }
}

impl KernelOps for Block {}

impl KernelOps for Mesh {
    fn to_mesh(&self) -> Option<acadrust::entities::Mesh> {
        Some(self.clone())
    }
}

impl KernelOps for PolyfaceMesh {
    fn to_mesh(&self) -> Option<acadrust::entities::Mesh> {
        // PolyfaceMesh is already a mesh representation; returning a new empty
        // Mesh keeps the capability contract simple until a conversion is
        // implemented.
        Some(Mesh::new())
    }
}

impl KernelOps for PolygonMeshEntity {
    fn to_mesh(&self) -> Option<acadrust::entities::Mesh> {
        Some(Mesh::new())
    }
}

impl KernelOps for Face3D {
    fn to_mesh(&self) -> Option<acadrust::entities::Mesh> {
        Some(Mesh::new())
    }
}

impl KernelOps for Solid3D {
    fn to_mesh(&self) -> Option<acadrust::entities::Mesh> {
        Some(Mesh::new())
    }
}

impl KernelOps for acadrust::entities::Region {
    fn to_mesh(&self) -> Option<acadrust::entities::Mesh> {
        Some(Mesh::new())
    }
}

impl KernelOps for acadrust::entities::Body {
    fn to_mesh(&self) -> Option<acadrust::entities::Mesh> {
        Some(Mesh::new())
    }
}

impl KernelOps for Surface {
    fn to_mesh(&self) -> Option<acadrust::entities::Mesh> {
        Some(Mesh::new())
    }
}

/// Convert a 2D heavy polyline to an equivalent lightweight polyline for offsetting.
fn lwpolyline_from_polyline2d(polyline: &Polyline2D) -> LwPolyline {
    LwPolyline {
        common: polyline.common.clone(),
        vertices: polyline
            .vertices
            .iter()
            .map(|v| acadrust::entities::LwVertex {
                location: Vector2::new(v.location.x, v.location.y),
                bulge: v.bulge,
                start_width: 0.0,
                end_width: 0.0,
                vertex_id: 0,
            })
            .collect(),
        is_closed: polyline.is_closed(),
        plinegen: false,
        constant_width: 0.0,
        elevation: polyline.elevation,
        thickness: polyline.thickness,
        normal: polyline.normal,
    }
}

/// Dispatch an `EntityType` variant to its concrete `KernelOps` implementation,
/// if one exists.
///
/// This is used by tests and by callers that want to apply kernel operations
/// without matching the enum themselves.
pub trait AsKernelOps {
    fn as_kernel_ops(&self) -> Option<&dyn KernelOps>;
}

impl AsKernelOps for EntityType {
    fn as_kernel_ops(&self) -> Option<&dyn KernelOps> {
        match self {
            EntityType::Line(e) => Some(e),
            EntityType::Circle(e) => Some(e),
            EntityType::Arc(e) => Some(e),
            EntityType::Ellipse(e) => Some(e),
            EntityType::Spline(e) => Some(e),
            EntityType::Polyline(e) => Some(e),
            EntityType::Polyline2D(e) => Some(e),
            EntityType::Polyline3D(e) => Some(e),
            EntityType::Ray(e) => Some(e),
            EntityType::XLine(e) => Some(e),
            EntityType::Helix(e) => Some(e),
            EntityType::LwPolyline(e) => Some(e),
            EntityType::MText(e) => Some(e),
            EntityType::Point(e) => Some(e),
            EntityType::Text(e) => Some(e),
            EntityType::Tolerance(e) => Some(e),
            EntityType::AttributeDefinition(e) => Some(e),
            EntityType::AttributeEntity(e) => Some(e),
            EntityType::Shape(e) => Some(e),
            EntityType::Hatch(e) => Some(e),
            EntityType::Wipeout(e) => Some(e),
            EntityType::Solid(e) => Some(e),
            EntityType::Insert(e) => Some(e),
            EntityType::Block(e) => Some(e),
            EntityType::Mesh(e) => Some(e),
            EntityType::PolyfaceMesh(e) => Some(e),
            EntityType::PolygonMesh(e) => Some(e),
            EntityType::Face3D(e) => Some(e),
            EntityType::Solid3D(e) => Some(e),
            EntityType::Region(e) => Some(e),
            EntityType::Body(e) => Some(e),
            EntityType::Surface(e) => Some(e),
            _ => None,
        }
    }
}
