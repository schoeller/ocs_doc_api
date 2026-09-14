# Object Model

## `Arc`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `center` | `Object` | no |
| `common` | `Object` | no |
| `end_angle` | `f64` | no |
| `normal` | `Object` | no |
| `radius` | `f64` | no |
| `start_angle` | `f64` | no |
| `thickness` | `f64` | no |

Capabilities:
- `to_planar_curve` → `Option<cadkernel::space::PlanarCurve>`

## `AttributeDefinition`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `alignment_point` | `Object` | no |
| `common` | `Object` | no |
| `default_value` | `String` | no |
| `embedded_mtext` | `Option<unknown>` | no |
| `field_length` | `f64` | no |
| `flags` | `Object` | no |
| `height` | `f64` | no |
| `horizontal_alignment` | `String` | no |
| `insertion_point` | `Object` | no |
| `is_multiline` | `bool` | no |
| `line_count` | `f64` | no |
| `lock_position` | `bool` | no |
| `mtext_flag` | `String` | no |
| `normal` | `Object` | no |
| `oblique_angle` | `f64` | no |
| `prompt` | `String` | no |
| `rotation` | `f64` | no |
| `tag` | `String` | no |
| `text_generation_flags` | `f64` | no |
| `text_style` | `String` | no |
| `vertical_alignment` | `String` | no |
| `width_factor` | `f64` | no |

Capabilities:
- `to_planar_curve` → `Option<cadkernel::space::PlanarCurve>` (requires: not a curve; text glyph extraction not supported)

## `AttributeEntity`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `alignment_point` | `Object` | no |
| `attdef_handle` | `f64` | no |
| `common` | `Object` | no |
| `embedded_mtext` | `Option<unknown>` | no |
| `field_length` | `f64` | no |
| `flags` | `Object` | no |
| `height` | `f64` | no |
| `horizontal_alignment` | `String` | no |
| `insertion_point` | `Object` | no |
| `is_multiline` | `bool` | no |
| `line_count` | `f64` | no |
| `lock_position` | `bool` | no |
| `mtext_flag` | `String` | no |
| `normal` | `Object` | no |
| `oblique_angle` | `f64` | no |
| `rotation` | `f64` | no |
| `tag` | `String` | no |
| `text_generation_flags` | `f64` | no |
| `text_style` | `String` | no |
| `value` | `String` | no |
| `vertical_alignment` | `String` | no |
| `width_factor` | `f64` | no |

Capabilities:
- `to_planar_curve` → `Option<cadkernel::space::PlanarCurve>` (requires: not a curve; text glyph extraction not supported)

## `Block`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `base_point` | `Object` | no |
| `common` | `Object` | no |
| `description` | `String` | no |
| `name` | `String` | no |
| `xref_path` | `String` | no |

Capabilities:
- `bounding_box` → `acadrust::types::BoundingBox3D` (requires: block definition must be available (not supported))

## `BlockEnd`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `common` | `Object` | no |

## `Body`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `acis_data` | `Object` | no |
| `common` | `Object` | no |
| `history_handle` | `Option<unknown>` | no |
| `point_of_reference` | `Object` | no |
| `silhouettes` | `Array` | no |
| `uid` | `String` | no |
| `wires` | `Array` | no |

Capabilities:
- `to_mesh` → `Option<acadrust::entities::Mesh>` (requires: ACIS data available; tessellation not supported)

## `Circle`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `common` | `EntityCommon` | yes |
| `center` | `Vector3` | yes |
| `radius` | `f64` | yes |
| `thickness` | `f64` | yes |
| `normal` | `Vector3` | yes |

Capabilities:
- `to_planar_curve` → `Option<cadkernel::space::PlanarCurve>`

## `Color`

Kind: `Enum`

| Variant | Fields |
|---|---|
| `ByLayer` | - |
| `None` | - |
| `ByBlock` | - |
| `Index` | value: u8 |
| `Rgb` | r: u8, g: u8, b: u8 |

## `Dimension`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `Linear` | `Object` | no |

## `DxfVersion`

Kind: `Enum`

| Variant | Fields |
|---|---|
| `Unknown` | - |
| `AC1012` | - |
| `AC1014` | - |
| `AC1015` | - |
| `AC1018` | - |
| `AC1021` | - |
| `AC1024` | - |
| `AC1027` | - |
| `AC1032` | - |

## `Ellipse`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `center` | `Object` | no |
| `common` | `Object` | no |
| `end_parameter` | `f64` | no |
| `major_axis` | `Object` | no |
| `minor_axis_ratio` | `f64` | no |
| `normal` | `Object` | no |
| `start_parameter` | `f64` | no |

Capabilities:
- `to_planar_curve` → `Option<cadkernel::space::PlanarCurve>`

## `EntityCommon`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `handle` | `Handle` | yes |
| `layer` | `String` | yes |
| `color` | `Color` | yes |
| `line_weight` | `LineWeight` | yes |
| `linetype` | `String` | yes |
| `linetype_scale` | `f64` | yes |
| `transparency` | `Transparency` | yes |
| `color_name` | `Option<String>` | no |
| `invisible` | `bool` | yes |
| `extended_data` | `ExtendedData` | yes |
| `reactors` | `Vec<Handle>` | yes |
| `xdictionary_handle` | `Option<Handle>` | no |
| `owner_handle` | `Handle` | yes |
| `full_visual_style_handle` | `Option<Handle>` | no |

## `EntityType`

Kind: `Enum`

| Variant | Fields |
|---|---|
| `Point` | value: Point |
| `Line` | value: Line |
| `Circle` | value: Circle |
| `LwPolyline` | value: LwPolyline |
| ... | (all first-level `acadrust::EntityType` variants) |

## `Extended`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `common` | `Object` | no |
| `data` | `Object` | no |

## `ExtendedData`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `records` | `Vec<ExtendedDataRecord>` | yes |

## `ExtendedDataRecord`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `application_name` | `String` | yes |
| `values` | `Vec<XDataValue>` | yes |

## `Face3D`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `common` | `Object` | no |
| `first_corner` | `Object` | no |
| `fourth_corner` | `Object` | no |
| `invisible_edges` | `Object` | no |
| `second_corner` | `Object` | no |
| `third_corner` | `Object` | no |

Capabilities:
- `to_mesh` → `Option<acadrust::entities::Mesh>` (requires: triangular or quadrilateral face; tessellation not supported)

## `Handle`

Kind: `NewTypeStruct`

## `Hatch`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `common` | `Object` | no |
| `elevation` | `f64` | no |
| `gradient_color` | `Object` | no |
| `is_associative` | `bool` | no |
| `is_double` | `bool` | no |
| `is_mpolygon` | `bool` | no |
| `is_solid` | `bool` | no |
| `mpolygon_boundary_handle_count` | `f64` | no |
| `mpolygon_hatch_color` | `String` | no |
| `mpolygon_x_direction` | `Object` | no |
| `normal` | `Object` | no |
| `paths` | `Array` | no |
| `pattern` | `Object` | no |
| `pattern_angle` | `f64` | no |
| `pattern_scale` | `f64` | no |
| `pattern_type` | `String` | no |
| `pixel_size` | `f64` | no |
| `seed_points` | `Array` | no |
| `style` | `String` | no |

Capabilities:
- `to_planar_curve` → `Option<cadkernel::space::PlanarCurve>` (requires: not a curve; boundary extraction not supported)

## `Helix`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `axis_base_point` | `Object` | no |
| `axis_vector` | `Object` | no |
| `common` | `Object` | no |
| `constraint` | `String` | no |
| `handedness` | `bool` | no |
| `maintenance_version` | `f64` | no |
| `major_version` | `f64` | no |
| `radius` | `f64` | no |
| `spline` | `Object` | no |
| `start_point` | `Object` | no |
| `turn_height` | `f64` | no |
| `turns` | `f64` | no |

Capabilities:
- `to_planar_curve` → `Option<cadkernel::space::PlanarCurve>` (requires: planar projection (not supported))

## `Insert`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `attributes` | `Array` | no |
| `block_name` | `String` | no |
| `column_count` | `f64` | no |
| `column_spacing` | `f64` | no |
| `common` | `Object` | no |
| `dwg_minsert` | `bool` | no |
| `insert_point` | `Object` | no |
| `normal` | `Object` | no |
| `rotation` | `f64` | no |
| `row_count` | `f64` | no |
| `row_spacing` | `f64` | no |
| `seqend_handle` | `Option<unknown>` | no |
| `view_rep_handle` | `Option<unknown>` | no |
| `x_scale` | `f64` | no |
| `y_scale` | `f64` | no |
| `z_scale` | `f64` | no |

Capabilities:
- `explode` → `Vec<acadrust::entities::EntityType>` (requires: block definition must be available (not supported))
- `bounding_box` → `acadrust::types::BoundingBox3D` (requires: block definition must be available (not supported))

## `Leader`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `attributes` | `Array` | no |
| `block_name` | `String` | no |
| `column_count` | `f64` | no |
| `column_spacing` | `f64` | no |
| `common` | `Object` | no |
| `dwg_minsert` | `bool` | no |
| `insert_point` | `Object` | no |
| `normal` | `Object` | no |
| `rotation` | `f64` | no |
| `row_count` | `f64` | no |
| `row_spacing` | `f64` | no |
| `seqend_handle` | `Option<unknown>` | no |
| `view_rep_handle` | `Option<unknown>` | no |
| `x_scale` | `f64` | no |
| `y_scale` | `f64` | no |
| `z_scale` | `f64` | no |

## `Leader`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `annotation_handle` | `f64` | no |
| `annotation_offset` | `Object` | no |
| `arrow_enabled` | `bool` | no |
| `arrow_size` | `f64` | no |
| `arrowhead_type` | `f64` | no |
| `block_offset` | `Object` | no |
| `byblock_color` | `f64` | no |
| `common` | `Object` | no |
| `creation_type` | `String` | no |
| `dimension_gap` | `f64` | no |
| `dimension_style` | `String` | no |
| `dwg_unknown_bit1` | `bool` | no |
| `dwg_unknown_bit2` | `bool` | no |
| `dwg_unknown_bit3` | `bool` | no |
| `dwg_unknown_bit4` | `bool` | no |
| `dwg_unknown_bit5` | `bool` | no |
| `dwg_unknown_short1` | `f64` | no |
| `hookline_direction` | `String` | no |
| `hookline_enabled` | `bool` | no |
| `horizontal_direction` | `Object` | no |
| `normal` | `Object` | no |
| `origin` | `Object` | no |
| `override_color` | `String` | no |
| `path_type` | `String` | no |
| `text_height` | `f64` | no |
| `text_width` | `f64` | no |
| `vertices` | `Array` | no |

## `Light`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `attenuation_end_limit` | `f64` | no |
| `attenuation_start_limit` | `f64` | no |
| `attenuation_type` | `f64` | no |
| `cast_shadows` | `bool` | no |
| `class_version` | `f64` | no |
| `common` | `Object` | no |
| `falloff_angle` | `f64` | no |
| `hotspot_angle` | `f64` | no |
| `intensity` | `f64` | no |
| `light_color` | `Object` | no |
| `light_type` | `f64` | no |
| `name` | `String` | no |
| `photometric_data` | `Option<unknown>` | no |
| `photometric_mode` | `bool` | no |
| `plot_glyph` | `bool` | no |
| `position` | `Object` | no |
| `shadow_map_size` | `f64` | no |
| `shadow_map_softness` | `f64` | no |
| `shadow_type` | `f64` | no |
| `status` | `bool` | no |
| `target` | `Object` | no |
| `use_attenuation_limits` | `bool` | no |

## `Line`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `common` | `EntityCommon` | yes |
| `start` | `Vector3` | yes |
| `end` | `Vector3` | yes |
| `thickness` | `f64` | yes |
| `normal` | `Vector3` | yes |

Capabilities:
- `to_planar_curve` → `Option<cadkernel::space::PlanarCurve>`

## `LineWeight`

Kind: `Enum`

| Variant | Fields |
|---|---|
| `ByLayer` | - |
| `ByBlock` | - |
| `Default` | - |
| `Value` | value: i16 |

## `LwPolyline`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `common` | `EntityCommon` | yes |
| `vertices` | `Vec<LwVertex>` | yes |
| `is_closed` | `bool` | yes |
| `plinegen` | `bool` | yes |
| `constant_width` | `f64` | yes |
| `elevation` | `f64` | yes |
| `thickness` | `f64` | yes |
| `normal` | `Vector3` | yes |

Capabilities:
- `to_planar_curve` → `Option<cadkernel::space::PlanarCurve>` (requires: planar, non-self-intersecting)
- `offset` → `Vec<acadrust::entities::LwPolyline>` (requires: planar, non-self-intersecting)

## `LwVertex`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `location` | `Vector2` | yes |
| `bulge` | `f64` | yes |
| `start_width` | `f64` | yes |
| `end_width` | `f64` | yes |
| `vertex_id` | `i32` | yes |

## `MLine`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `common` | `Object` | no |
| `flags` | `String` | no |
| `justification` | `String` | no |
| `normal` | `Object` | no |
| `scale_factor` | `f64` | no |
| `start_point` | `Object` | no |
| `style_element_count` | `f64` | no |
| `style_handle` | `Option<unknown>` | no |
| `style_name` | `String` | no |
| `vertices` | `Array` | no |

## `MText`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `attachment_point` | `String` | no |
| `background_color` | `String` | no |
| `background_fill_flags` | `f64` | no |
| `background_scale` | `f64` | no |
| `background_transparency` | `f64` | no |
| `column_data` | `Object` | no |
| `common` | `Object` | no |
| `drawing_direction` | `String` | no |
| `dwg_x_direction` | `Option<unknown>` | no |
| `extents_height` | `f64` | no |
| `extents_width` | `f64` | no |
| `height` | `f64` | no |
| `insertion_point` | `Object` | yes |
| `is_annotative` | `bool` | no |
| `line_spacing_factor` | `f64` | no |
| `line_spacing_style` | `String` | no |
| `normal` | `Object` | no |
| `rectangle_height` | `Option<unknown>` | no |
| `rectangle_width` | `f64` | no |
| `rotation` | `f64` | no |
| `style` | `String` | no |
| `value` | `String` | no |

Capabilities:
- `to_planar_curve` → `Option<cadkernel::space::PlanarCurve>` (requires: not a curve; text glyph extraction not supported)

## `Mesh`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `blend_crease` | `bool` | no |
| `common` | `Object` | no |
| `edges` | `Array` | no |
| `faces` | `Array` | no |
| `override_option` | `f64` | no |
| `subdivision_level` | `f64` | no |
| `version` | `f64` | no |
| `vertices` | `Array` | no |

Capabilities:
- `to_mesh` → `Option<acadrust::entities::Mesh>` (requires: mesh data already available; copy returned)

## `MultiLeader`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `arrowhead_handle` | `Option<unknown>` | no |
| `arrowhead_overrides` | `Array` | no |
| `arrowhead_size` | `f64` | no |
| `block_attributes` | `Array` | no |
| `block_connection_type` | `String` | no |
| `block_content_color` | `String` | no |
| `block_content_handle` | `Option<unknown>` | no |
| `block_rotation` | `f64` | no |
| `block_scale` | `Object` | no |
| `common` | `Object` | no |
| `content_type` | `String` | no |
| `context` | `Object` | no |
| `dogleg_length` | `f64` | no |
| `dwg_version` | `f64` | no |
| `enable_annotation_scale` | `bool` | no |
| `enable_dogleg` | `bool` | no |
| `enable_landing` | `bool` | no |
| `extend_leader_to_text` | `bool` | no |
| `line_color` | `String` | no |
| `line_type_handle` | `Option<unknown>` | no |
| `line_weight` | `String` | no |
| `path_type` | `String` | no |
| `property_override_flags` | `String` | no |
| `scale_factor` | `f64` | no |
| `style_handle` | `Option<unknown>` | no |
| `text_align_in_ipe` | `f64` | no |
| `text_alignment` | `String` | no |
| `text_angle_type` | `String` | no |
| `text_attachment_direction` | `String` | no |
| `text_attachment_point` | `String` | no |
| `text_bottom_attachment` | `String` | no |
| `text_color` | `String` | no |
| `text_direction_negative` | `bool` | no |
| `text_frame` | `bool` | no |
| `text_height` | `f64` | no |
| `text_left_attachment` | `String` | no |
| `text_right_attachment` | `String` | no |
| `text_style_handle` | `Option<unknown>` | no |
| `text_top_attachment` | `String` | no |

## `Ole2Frame`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `common` | `Object` | no |
| `dwg_mode` | `f64` | no |
| `envelope` | `String` | no |
| `is_paper_space` | `bool` | no |
| `lock_aspect` | `f64` | no |
| `lower_right_corner` | `Object` | no |
| `ole_object_type` | `String` | no |
| `source_application` | `String` | no |
| `storage` | `Object` | no |
| `upper_left_corner` | `Object` | no |
| `version` | `f64` | no |

## `Point`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `common` | `EntityCommon` | yes |
| `location` | `Vector3` | yes |
| `thickness` | `f64` | yes |
| `normal` | `Vector3` | yes |
| `x_axis_angle` | `f64` | yes |

Capabilities:
- `to_planar_curve` → `Option<cadkernel::space::PlanarCurve>` (requires: not a curve; represented as point marker)

## `PolyfaceMesh`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `common` | `Object` | no |
| `elevation` | `f64` | no |
| `end_width` | `f64` | no |
| `faces` | `Array` | no |
| `flags` | `String` | no |
| `normal` | `Object` | no |
| `seqend_handle` | `Option<unknown>` | no |
| `smooth_surface` | `String` | no |
| `start_width` | `f64` | no |
| `thickness` | `f64` | no |
| `vertices` | `Array` | no |

Capabilities:
- `to_mesh` → `Option<acadrust::entities::Mesh>` (requires: mesh data already available; copy returned)

## `PolygonMesh`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `common` | `Object` | no |
| `elevation` | `f64` | no |
| `flags` | `String` | no |
| `m_smooth_density` | `f64` | no |
| `m_vertex_count` | `f64` | no |
| `n_smooth_density` | `f64` | no |
| `n_vertex_count` | `f64` | no |
| `normal` | `Object` | no |
| `smooth_type` | `String` | no |
| `vertices` | `Array` | no |

Capabilities:
- `to_mesh` → `Option<acadrust::entities::Mesh>` (requires: mesh data already available; copy returned)

## `Polyline`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `common` | `Object` | no |
| `flags` | `Object` | no |
| `vertices` | `Array` | no |

Capabilities:
- `to_planar_curve` → `Option<cadkernel::space::PlanarCurve>` (requires: planar)

## `Polyline2D`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `common` | `Object` | no |
| `elevation` | `f64` | no |
| `end_width` | `f64` | no |
| `flags` | `Object` | no |
| `normal` | `Object` | no |
| `smooth_surface` | `String` | no |
| `start_width` | `f64` | no |
| `thickness` | `f64` | no |
| `vertices` | `Array` | no |

Capabilities:
- `to_planar_curve` → `Option<cadkernel::space::PlanarCurve>` (requires: planar)
- `offset` → `Vec<acadrust::entities::LwPolyline>` (requires: planar, non-self-intersecting)

## `Polyline3D`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `common` | `Object` | no |
| `default_end_width` | `f64` | no |
| `default_start_width` | `f64` | no |
| `elevation` | `f64` | no |
| `flags` | `Object` | no |
| `mesh_m_count` | `f64` | no |
| `mesh_n_count` | `f64` | no |
| `normal` | `Object` | no |
| `smooth_m_density` | `f64` | no |
| `smooth_n_density` | `f64` | no |
| `smooth_type` | `String` | no |
| `vertices` | `Array` | no |

Capabilities:
- `to_planar_curve` → `Option<cadkernel::space::PlanarCurve>` (requires: planar)

## `RasterImage`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `brightness` | `f64` | no |
| `class_version` | `f64` | no |
| `clip_boundary` | `Object` | no |
| `clipping_enabled` | `bool` | no |
| `common` | `Object` | no |
| `contrast` | `f64` | no |
| `definition_handle` | `Option<unknown>` | no |
| `definition_reactor_handle` | `Option<unknown>` | no |
| `fade` | `f64` | no |
| `file_path` | `String` | no |
| `flags` | `String` | no |
| `insertion_point` | `Object` | no |
| `size` | `Object` | no |
| `u_vector` | `Object` | no |
| `v_vector` | `Object` | no |

## `Ray`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `base_point` | `Object` | no |
| `common` | `Object` | no |
| `direction` | `Object` | no |

Capabilities:
- `to_planar_curve` → `Option<cadkernel::space::PlanarCurve>`

## `Region`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `acis_data` | `Object` | no |
| `common` | `Object` | no |
| `history_handle` | `Option<unknown>` | no |
| `point_of_reference` | `Object` | no |
| `silhouettes` | `Array` | no |
| `uid` | `String` | no |
| `wires` | `Array` | no |

Capabilities:
- `to_mesh` → `Option<acadrust::entities::Mesh>` (requires: ACIS data available; tessellation not supported)

## `SectionSymbol`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `common` | `Object` | no |
| `end_a` | `Array` | no |
| `end_b` | `Array` | no |
| `label` | `String` | no |
| `points` | `Array` | no |
| `raw_flags_90` | `f64` | no |
| `raw_point_count_90` | `f64` | no |
| `raw_point_record_count` | `f64` | no |
| `raw_view_symbol_70` | `f64` | no |
| `style_handle` | `f64` | no |
| `symbol_scale` | `f64` | no |
| `tick_a` | `f64` | no |
| `tick_b` | `f64` | no |
| `version` | `f64` | no |
| `view_rep_handle` | `f64` | no |
| `view_symbol_version` | `f64` | no |

## `Seqend`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `common` | `Object` | no |

## `Shape`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `common` | `Object` | no |
| `insertion_point` | `Object` | no |
| `normal` | `Object` | no |
| `oblique_angle` | `f64` | no |
| `relative_x_scale` | `f64` | no |
| `rotation` | `f64` | no |
| `shape_name` | `String` | no |
| `shape_number` | `f64` | no |
| `size` | `f64` | no |
| `style_handle` | `Option<unknown>` | no |
| `style_name` | `String` | no |
| `thickness` | `f64` | no |

Capabilities:
- `to_planar_curve` → `Option<cadkernel::space::PlanarCurve>` (requires: not a curve; shape glyph extraction not supported)

## `Solid`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `common` | `Object` | no |
| `first_corner` | `Object` | no |
| `fourth_corner` | `Object` | no |
| `is_trace` | `bool` | no |
| `normal` | `Object` | no |
| `second_corner` | `Object` | no |
| `thickness` | `f64` | no |
| `third_corner` | `Object` | no |

Capabilities:
- `to_mesh` → `Option<acadrust::entities::Mesh>` (requires: triangular or quadrilateral face; tessellation not supported)

## `Solid3D`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `acis_data` | `Object` | no |
| `common` | `Object` | no |
| `history_handle` | `Option<unknown>` | no |
| `point_of_reference` | `Object` | no |
| `silhouettes` | `Array` | no |
| `uid` | `String` | no |
| `wires` | `Array` | no |

Capabilities:
- `to_mesh` → `Option<acadrust::entities::Mesh>` (requires: ACIS data available; tessellation not supported)

## `Spline`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `begin_tangent` | `Object` | no |
| `common` | `Object` | no |
| `control_points` | `Array` | no |
| `control_tolerance` | `f64` | no |
| `cv_frame_visible` | `bool` | no |
| `degree` | `f64` | no |
| `dwg_flags1` | `f64` | no |
| `end_tangent` | `Object` | no |
| `fit_points` | `Array` | no |
| `fit_tolerance` | `f64` | no |
| `flags` | `Object` | no |
| `knot_parameterization` | `f64` | no |
| `knot_tolerance` | `f64` | no |
| `knots` | `Array` | no |
| `normal` | `Object` | no |
| `weights` | `Array` | no |

Capabilities:
- `to_planar_curve` → `Option<cadkernel::space::PlanarCurve>` (requires: planar)

## `Surface`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `acis_data` | `Object` | no |
| `common` | `Object` | no |
| `history_handle` | `Option<unknown>` | no |
| `kind` | `String` | no |
| `modeler_format_version` | `f64` | no |
| `point_of_reference` | `Object` | no |
| `silhouettes` | `Array` | no |
| `surface_data` | `String` | no |
| `u_isolines` | `f64` | no |
| `v_isolines` | `f64` | no |
| `wires` | `Array` | no |

Capabilities:
- `to_mesh` → `Option<acadrust::entities::Mesh>` (requires: surface data available; tessellation not supported)

## `Table`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `base_style` | `Option<unknown>` | no |
| `block_name` | `String` | no |
| `block_record_handle` | `Option<unknown>` | no |
| `break_data` | `Array` | no |
| `break_flow_direction` | `String` | no |
| `break_options` | `String` | no |
| `break_ranges` | `Array` | no |
| `break_spacing` | `f64` | no |
| `columns` | `Array` | no |
| `common` | `Object` | no |
| `data_version` | `f64` | no |
| `description` | `String` | no |
| `dwg_unknown_byte` | `f64` | no |
| `dwg_unknown_handle` | `Option<unknown>` | no |
| `dwg_unknown_long1` | `f64` | no |
| `dwg_unknown_long2` | `f64` | no |
| `dwg_unknown_short` | `f64` | no |
| `field_handles` | `Array` | no |
| `horizontal_direction` | `Object` | no |
| `insertion_point` | `Object` | no |
| `legacy_border_colors` | `Option<unknown>` | no |
| `legacy_border_line_weights` | `Option<unknown>` | no |
| `legacy_border_visibility` | `Option<unknown>` | no |
| `legacy_style_override` | `Option<unknown>` | no |
| `merged_ranges` | `Array` | no |
| `name` | `String` | no |
| `normal` | `Object` | no |
| `override_border_color` | `bool` | no |
| `override_border_line_weight` | `bool` | no |
| `override_border_visibility` | `bool` | no |
| `override_flag` | `bool` | no |
| `rows` | `Array` | no |
| `table_style_handle` | `Option<unknown>` | no |
| `value_flags` | `f64` | no |

## `Text`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `alignment_point` | `Option<unknown>` | no |
| `common` | `Object` | no |
| `generation_flags` | `f64` | no |
| `height` | `f64` | no |
| `horizontal_alignment` | `String` | no |
| `insertion_point` | `Object` | yes |
| `normal` | `Object` | no |
| `oblique_angle` | `f64` | no |
| `rotation` | `f64` | no |
| `style` | `String` | no |
| `thickness` | `f64` | no |
| `value` | `String` | no |
| `vertical_alignment` | `String` | no |
| `width_factor` | `f64` | no |

Capabilities:
- `to_planar_curve` → `Option<cadkernel::space::PlanarCurve>` (requires: not a curve; text glyph extraction not supported)

## `Tolerance`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `common` | `Object` | no |
| `dimension_gap` | `f64` | no |
| `dimension_style_handle` | `Option<unknown>` | no |
| `dimension_style_name` | `String` | no |
| `direction` | `Object` | no |
| `dwg_unknown_short` | `f64` | no |
| `insertion_point` | `Object` | no |
| `normal` | `Object` | no |
| `text` | `String` | no |
| `text_height` | `f64` | no |

Capabilities:
- `to_planar_curve` → `Option<cadkernel::space::PlanarCurve>` (requires: not a curve; GD&T frame extraction not supported)

## `Transparency`

Kind: `Enum`

| Variant | Fields |
|---|---|
| `by_layer` | - |
| `by_block` | - |
| `explicit` | value: u8 |

## `Underlay`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `clip_boundary_vertices` | `Array` | no |
| `clip_inverted` | `bool` | no |
| `common` | `Object` | no |
| `contrast` | `f64` | no |
| `definition_handle` | `f64` | no |
| `fade` | `f64` | no |
| `flags` | `String` | no |
| `insertion_point` | `Object` | no |
| `normal` | `Object` | no |
| `rotation` | `f64` | no |
| `underlay_type` | `String` | no |
| `x_scale` | `f64` | no |
| `y_scale` | `f64` | no |
| `z_scale` | `f64` | no |

## `Unknown`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `common` | `Object` | no |
| `dwg_handle_bits` | `f64` | no |
| `dwg_type_code` | `f64` | no |
| `dxf_name` | `String` | no |

## `Vector2`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `x` | `f64` | yes |
| `y` | `f64` | yes |

## `Vector3`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `x` | `f64` | yes |
| `y` | `f64` | yes |
| `z` | `f64` | yes |

## `ViewBorder`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `active_viewport` | `f64` | no |
| `center` | `Array` | no |
| `common` | `Object` | no |
| `max` | `Array` | no |
| `min` | `Array` | no |
| `rotation_angle` | `f64` | no |
| `scale` | `f64` | no |
| `scale_handle` | `f64` | no |
| `version` | `f64` | no |

## `Viewport`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `ambient_color` | `String` | no |
| `back_clip_z` | `f64` | no |
| `background_handle` | `f64` | no |
| `base_ucs_handle` | `f64` | no |
| `brightness` | `f64` | no |
| `center` | `Object` | no |
| `circle_sides` | `f64` | no |
| `clip_boundary_handle` | `f64` | no |
| `common` | `Object` | no |
| `contrast` | `f64` | no |
| `custom_scale` | `f64` | no |
| `default_lighting` | `bool` | no |
| `default_lighting_type` | `f64` | no |
| `elevation` | `f64` | no |
| `front_clip_z` | `f64` | no |
| `frozen_layers` | `Array` | no |
| `grid_flags` | `Object` | no |
| `grid_major` | `f64` | no |
| `grid_spacing` | `Object` | no |
| `height` | `f64` | no |
| `id` | `f64` | no |
| `lens_length` | `f64` | no |
| `render_mode` | `String` | no |
| `shade_plot_handle` | `f64` | no |
| `shade_plot_mode` | `f64` | no |
| `snap_angle` | `f64` | no |
| `snap_base` | `Object` | no |
| `snap_spacing` | `Object` | no |
| `status` | `Object` | no |
| `style_sheet` | `String` | no |
| `sun_handle` | `f64` | no |
| `twist_angle` | `f64` | no |
| `ucs_at_origin` | `bool` | no |
| `ucs_handle` | `f64` | no |
| `ucs_icon_visible` | `bool` | no |
| `ucs_origin` | `Object` | no |
| `ucs_ortho_type` | `f64` | no |
| `ucs_per_viewport` | `bool` | no |
| `ucs_x_axis` | `Object` | no |
| `ucs_y_axis` | `Object` | no |
| `view_center` | `Object` | no |
| `view_direction` | `Object` | no |
| `view_height` | `f64` | no |
| `view_target` | `Object` | no |
| `visual_style_handle` | `f64` | no |
| `width` | `f64` | no |

## `Wipeout`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `brightness` | `f64` | no |
| `class_version` | `f64` | no |
| `clip_boundary_vertices` | `Array` | no |
| `clip_mode` | `String` | no |
| `clip_type` | `String` | no |
| `clipping_enabled` | `bool` | no |
| `common` | `Object` | no |
| `contrast` | `f64` | no |
| `definition_handle` | `Option<unknown>` | no |
| `definition_reactor_handle` | `Option<unknown>` | no |
| `fade` | `f64` | no |
| `flags` | `String` | no |
| `insertion_point` | `Object` | no |
| `size` | `Object` | no |
| `u_vector` | `Object` | no |
| `v_vector` | `Object` | no |

Capabilities:
- `to_planar_curve` → `Option<cadkernel::space::PlanarCurve>` (requires: not a curve; clipping boundary extraction not supported)

## `XDataValue`

Kind: `Enum`

| Variant | Fields |
|---|---|
| `String` | value: String |
| `ControlString` | value: String |
| `LayerName` | value: String |
| `BinaryData` | value: Vec<u8> |
| `Handle` | value: Handle |
| `Point3D` | value: Vector3 |
| `Position3D` | value: Vector3 |
| `Displacement3D` | value: Vector3 |
| `Direction3D` | value: Vector3 |
| `Real` | value: f64 |
| `Distance` | value: f64 |
| `ScaleFactor` | value: f64 |
| `Integer16` | value: i16 |
| `Integer32` | value: i32 |

## `XLine`

Kind: `Struct`

| Field | Type | Required |
|---|---|---|
| `base_point` | `Object` | no |
| `common` | `Object` | no |
| `direction` | `Object` | no |

Capabilities:
- `to_planar_curve` → `Option<cadkernel::space::PlanarCurve>`

