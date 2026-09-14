#![cfg(feature = "kernel")]

use acadrust::entities::{
    Arc, Circle, Ellipse, Helix, Line, MText, Polyline, Polyline2D, Polyline3D, Ray, Spline, XLine,
};
use acadrust::types::{Vector2, Vector3};
use ocs_doc_api::kernel_ops::{AsKernelOps, KernelOps};

#[derive(serde::Deserialize)]
struct CapabilitiesFile {
    #[serde(default)]
    capability: Vec<CapabilityEntry>,
}

#[derive(serde::Deserialize)]
struct CapabilityEntry {
    entity: String,
    name: String,
    #[serde(default)]
    requires: String,
}

fn entity_kind(entity: &acadrust::entities::EntityType) -> String {
    let value = serde_json::to_value(entity).unwrap();
    let object = value.as_object().unwrap();
    object.keys().next().unwrap().to_lowercase()
}

#[test]
fn kernel_capabilities_toml_entries_are_implemented() {
    let manifest_dir = std::env!("CARGO_MANIFEST_DIR");
    let path = std::path::Path::new(manifest_dir).join("kernel_capabilities.toml");
    let text = std::fs::read_to_string(path).unwrap();
    let file: CapabilitiesFile = toml::from_str(&text).unwrap();

    let samples = ocs_doc_api::entity_samples::all();

    for cap in file.capability {
        let sample = samples
            .iter()
            .find(|e| entity_kind(e) == cap.entity.to_lowercase())
            .unwrap_or_else(|| panic!("no sample entity for {}", cap.entity));
        let ops = sample
            .as_kernel_ops()
            .unwrap_or_else(|| panic!("no KernelOps implementation for {}", cap.entity));

        match cap.name.as_str() {
            "to_planar_curve" => {
                let result = ops.to_planar_curve();
                if cap.requires.contains("not supported") || cap.requires.contains("not a curve") {
                    assert!(
                        result.is_none(),
                        "{} to_planar_curve should return None (unsupported)",
                        cap.entity
                    );
                } else {
                    assert!(
                        result.is_some(),
                        "{} to_planar_curve returned None but capability expects Some",
                        cap.entity
                    );
                }
            }
            "offset" => {
                let results = ops.offset(0.5, [5.0, 2.5]);
                assert!(
                    !results.is_empty(),
                    "{} offset returned no results but capability is declared",
                    cap.entity
                );
            }
            "explode" => {
                let results = ops.explode();
                if cap.requires.contains("not supported") {
                    assert!(
                        results.is_empty(),
                        "{} explode should return empty (unsupported)",
                        cap.entity
                    );
                } else {
                    assert!(
                        !results.is_empty(),
                        "{} explode returned no results but capability is declared",
                        cap.entity
                    );
                }
            }
            "to_mesh" => {
                assert!(
                    ops.to_mesh().is_some(),
                    "{} to_mesh returned None but capability is declared",
                    cap.entity
                );
            }
            "bounding_box" => {
                // `bounding_box` is available through the `Entity` trait, so
                // presence of a `KernelOps` implementation is enough to satisfy
                // the capability declaration.
            }
            other => panic!("unknown capability '{}' for {}", other, cap.entity),
        }
    }
}

#[test]
fn line_to_planar_curve_returns_some() {
    let line = Line::from_coords(0.0, 0.0, 0.0, 10.0, 0.0, 0.0);
    assert!(line.to_planar_curve().is_some());
}

#[test]
fn circle_to_planar_curve_returns_some() {
    let circle = Circle::from_coords(0.0, 0.0, 0.0, 5.0);
    assert!(circle.to_planar_curve().is_some());
}

#[test]
fn arc_to_planar_curve_returns_some() {
    let arc = Arc::from_coords(0.0, 0.0, 0.0, 5.0, 0.0, 1.570796);
    assert!(arc.to_planar_curve().is_some());
}

#[test]
fn ellipse_to_planar_curve_returns_some() {
    let ellipse = Ellipse::from_center_axes(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(10.0, 0.0, 0.0),
        0.5,
    );
    assert!(ellipse.to_planar_curve().is_some());
}

#[test]
fn spline_to_planar_curve_returns_some_for_planar_points() {
    let spline = Spline::from_control_points(
        3,
        vec![
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(2.0, 1.0, 0.0),
            Vector3::new(3.0, 0.0, 0.0),
        ],
    );
    assert!(spline.to_planar_curve().is_some());
}

#[test]
fn polyline2d_to_planar_curve_returns_some() {
    let mut polyline = Polyline2D::new();
    polyline.add_vertex(acadrust::entities::Vertex2D::from_point(Vector2::new(0.0, 0.0)));
    polyline.add_vertex(acadrust::entities::Vertex2D::from_point(Vector2::new(10.0, 0.0)));
    assert!(polyline.to_planar_curve().is_some());
}

#[test]
fn polyline3d_to_planar_curve_returns_some_when_level() {
    let polyline = Polyline3D::from_points(vec![
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(10.0, 0.0, 0.0),
        Vector3::new(10.0, 5.0, 0.0),
    ]);
    assert!(polyline.to_planar_curve().is_some());
}

#[test]
fn ray_to_planar_curve_returns_some() {
    let ray = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));
    assert!(ray.to_planar_curve().is_some());
}

#[test]
fn xline_to_planar_curve_returns_some() {
    let xline = XLine::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));
    assert!(xline.to_planar_curve().is_some());
}

#[test]
fn helix_to_planar_curve_returns_none() {
    let helix = Helix::new();
    assert!(helix.to_planar_curve().is_none());
}

#[test]
fn mtext_to_planar_curve_returns_none() {
    let mtext = MText::new();
    assert!(mtext.to_planar_curve().is_none());
}

#[test]
fn empty_polyline_to_planar_curve_returns_none() {
    let polyline = Polyline::new();
    assert!(polyline.to_planar_curve().is_none());
}
