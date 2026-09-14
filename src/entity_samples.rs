use acadrust::entities::{
    Arc, AttributeDefinition, AttributeEntity, Block, BlockEnd, Body, Circle, Dimension,
    DimensionLinear, Ellipse, EntityType, ExtendedEntity, ExtendedEntityData, Face3D, Hatch, Helix,
    Insert, Leader, Light, Line, LwPolyline, Mesh, MLine, MText, MultiLeader, Ole2Frame, Point,
    PolyfaceMesh, PolygonMeshEntity, Polyline, Polyline2D, Polyline3D, RasterImage, Ray, Region,
    SectionSymbol, Seqend, Shape, Solid, Solid3D, Spline, Surface, SurfaceKind, Table, Text,
    Tolerance, Underlay, UnderlayType, UnknownEntity, Vertex2D, ViewBorder, Viewport, Wipeout,
    XLine,
};
use acadrust::types::{Handle, Vector2, Vector3};

/// Build a vector with one sample for every first-level `EntityType` variant.
///
/// The samples are used both by the build-time serde-reflection tracer and by
/// the runtime test suite, so the object model and the round-trip tests always
/// cover the same entity surface.
pub fn all() -> Vec<EntityType> {
    let mut polyline2d = Polyline2D::new();
    polyline2d.add_vertex(Vertex2D::from_point(Vector2::new(0.0, 0.0)));
    polyline2d.add_vertex(Vertex2D::from_point(Vector2::new(10.0, 0.0)));
    polyline2d.add_vertex(Vertex2D::from_point(Vector2::new(10.0, 5.0)));

    vec![
        EntityType::Point(Point::new()),
        EntityType::Line(Line::from_coords(0.0, 0.0, 0.0, 10.0, 0.0, 0.0)),
        EntityType::AttributeDefinition(AttributeDefinition::simple("TAG")),
        EntityType::AttributeEntity(AttributeEntity::simple("TAG", "VAL")),
        EntityType::Circle(Circle::from_coords(5.0, 5.0, 0.0, 2.0)),
        EntityType::Arc(Arc::from_coords(
            0.0,
            0.0,
            0.0,
            5.0,
            0.0,
            std::f64::consts::FRAC_PI_2,
        )),
        EntityType::Ellipse(Ellipse::from_center_axes(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(10.0, 0.0, 0.0),
            0.5,
        )),
        EntityType::Polyline(Polyline::from_points(vec![
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(10.0, 0.0, 0.0),
            Vector3::new(10.0, 5.0, 0.0),
        ])),
        EntityType::Polyline2D(polyline2d),
        EntityType::Polyline3D(Polyline3D::from_points(vec![
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(10.0, 0.0, 0.0),
            Vector3::new(10.0, 5.0, 0.0),
        ])),
        EntityType::LwPolyline(LwPolyline::from_points(vec![
            Vector2::new(0.0, 0.0),
            Vector2::new(10.0, 0.0),
            Vector2::new(10.0, 5.0),
        ])),
        EntityType::Text(Text::with_value("A", Vector3::new(0.0, 0.0, 0.0))),
        EntityType::MText(MText::with_value("MT", Vector3::new(0.0, 0.0, 0.0))),
        EntityType::Spline(Spline::from_control_points(
            3,
            vec![
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(1.0, 0.0, 0.0),
                Vector3::new(2.0, 1.0, 0.0),
                Vector3::new(3.0, 0.0, 0.0),
            ],
        )),
        EntityType::Helix(Helix::new()),
        EntityType::Dimension(Dimension::Linear(DimensionLinear::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(10.0, 0.0, 0.0),
        ))),
        EntityType::Hatch(Hatch::new()),
        EntityType::Solid(Solid::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(1.0, 1.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        )),
        EntityType::Face3D(Face3D::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(1.0, 1.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        )),
        EntityType::Insert(Insert::new("*MODEL_SPACE", Vector3::new(0.0, 0.0, 0.0))),
        EntityType::Block(Block::new("B", Vector3::new(0.0, 0.0, 0.0))),
        EntityType::BlockEnd(BlockEnd::new()),
        EntityType::Ray(Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0))),
        EntityType::XLine(XLine::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0))),
        EntityType::Viewport(Viewport::new()),
        EntityType::Leader(Leader::new()),
        EntityType::MultiLeader(MultiLeader::new()),
        EntityType::MLine(MLine::new()),
        EntityType::Mesh(Mesh::new()),
        EntityType::RasterImage(RasterImage::new(
            "img.png",
            Vector3::new(0.0, 0.0, 0.0),
            100.0,
            100.0,
        )),
        EntityType::Solid3D(Solid3D::new()),
        EntityType::Region(Region::new()),
        EntityType::Body(Body::new()),
        EntityType::Surface(Surface::new(SurfaceKind::Generic)),
        EntityType::Table(Table::new(Vector3::new(0.0, 0.0, 0.0), 2, 2)),
        EntityType::Tolerance(Tolerance::new()),
        EntityType::PolyfaceMesh(PolyfaceMesh::new()),
        EntityType::Wipeout(Wipeout::rectangular(Vector3::new(0.0, 0.0, 0.0), 10.0, 5.0)),
        EntityType::Shape(Shape::new()),
        EntityType::Underlay(Underlay::new(UnderlayType::Pdf)),
        EntityType::Seqend(Seqend::new()),
        EntityType::Ole2Frame(Ole2Frame::new()),
        EntityType::PolygonMesh(PolygonMeshEntity::new()),
        EntityType::Light(Light::new()),
        EntityType::SectionSymbol(SectionSymbol::new()),
        EntityType::ViewBorder(ViewBorder::new()),
        EntityType::Extended(ExtendedEntity {
            common: acadrust::entities::EntityCommon::new(),
            data: ExtendedEntityData::Camera {
                view_handle: Handle::new(0),
            },
        }),
        EntityType::Unknown(UnknownEntity::new("UNKNOWN")),
    ]
}
