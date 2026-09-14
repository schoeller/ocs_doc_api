# Object Model

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
- `offset` → `Vec<cadkernel::geom2d::Polyline>` (requires: planar, non-self-intersecting)

