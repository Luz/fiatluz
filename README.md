# fiatluz
An enlightening fast geometry library written in Rust.

## Requirements
- Easy usage of this Rust library
- Stability
- Correctness
- Performance
- Determinism (same input, same output) and therefore platform independence
- Allows convex and concave polygons
- For self intersecting polygons, outputting an error is enough

## Features
- [x] Detecting if a point is inside a polygon
- [ ] Detecting if a point is on a polygon's border?
- [ ] Union
- [ ] Subtract
- [ ] Intersect
- [ ] Offset
- [ ] Detecting holes (Might need a special datastructure, e.g. a nested tree of polygons)

## Optional features
- [ ] Delaunay triangulation

## Implementation thoughts
- Use "repr(C)"-statement on primitives for now, to simplify wrapping

