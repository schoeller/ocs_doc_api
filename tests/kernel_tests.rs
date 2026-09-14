#![cfg(feature = "kernel")]

use acadrust::entities::{Line, MText};
use ocs_doc_api::kernel_ops::KernelOps;

#[test]
fn line_to_planar_curve_returns_some() {
    let line = Line::from_coords(0.0, 0.0, 0.0, 10.0, 0.0, 0.0);
    assert!(line.to_planar_curve().is_some());
}

#[test]
fn mtext_to_planar_curve_returns_none() {
    let mtext = MText::new();
    assert!(mtext.to_planar_curve().is_none());
}
