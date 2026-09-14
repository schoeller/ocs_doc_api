use acadrust::entities::{LwPolyline, LwVertex};
use acadrust::types::{Vector2, Vector3};

/// Convert an `acadrust` 3-D vector to a `cadkernel` 3-D vector.
pub fn vec3(v: Vector3) -> cadkernel::space::Vec3 {
    cadkernel::space::Vec3::new(v.x, v.y, v.z)
}

/// Convert a `cadkernel` 3-D vector to an `acadrust` 3-D vector.
pub fn to_acad_vec3(v: cadkernel::space::Vec3) -> Vector3 {
    Vector3::new(v.x, v.y, v.z)
}

/// Convert a `cadkernel` 2-D array to an `acadrust` 2-D vector.
pub fn to_acad_vec2(v: [f64; 2]) -> Vector2 {
    Vector2::new(v[0], v[1])
}

/// Convert an `acadrust` lightweight polyline to a `cadkernel` 2-D polyline.
pub fn lwpolyline_to_polyline(polyline: &LwPolyline) -> cadkernel::geom2d::Polyline {
    cadkernel::geom2d::Polyline {
        closed: polyline.is_closed,
        vertices: polyline
            .vertices
            .iter()
            .map(|v| cadkernel::geom2d::PolylineVertex {
                position: [v.location.x, v.location.y],
                bulge: v.bulge,
            })
            .collect(),
    }
}

/// Convert a `cadkernel` 2-D polyline back to an `acadrust` lightweight polyline,
/// preserving the source elevation, normal, and common data.
pub fn polyline_to_lwpolyline(
    source: &LwPolyline,
    polyline: cadkernel::geom2d::Polyline,
) -> LwPolyline {
    LwPolyline {
        common: source.common.clone(),
        vertices: polyline
            .vertices
            .iter()
            .map(|v| LwVertex {
                location: Vector2::new(v.position[0], v.position[1]),
                bulge: v.bulge,
                start_width: 0.0,
                end_width: 0.0,
                vertex_id: 0,
            })
            .collect(),
        is_closed: polyline.closed,
        plinegen: source.plinegen,
        constant_width: source.constant_width,
        elevation: source.elevation,
        thickness: source.thickness,
        normal: source.normal,
    }
}
