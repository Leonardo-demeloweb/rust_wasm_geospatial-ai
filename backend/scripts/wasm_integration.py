import json
import subprocess
def rust_wasm_pip(well_point, aquifers, wasm_module):
   """Perform point-in-polygon calculations using Rust WASM."""
   polygons = []
   for idx, geom in enumerate(aquifers.geometry):
       if geom.geom_type == "Polygon":
           polygons.append({
               "id": idx,
               "coordinates": [{"longitude": coord[0], "latitude": coord[1]} for coord in geom.exterior.coords]
           })
       elif geom.geom_type == "MultiPolygon":
           for poly in geom.geoms:
               polygons.append({
                   "id": idx,
                   "coordinates": [{"longitude": coord[0], "latitude": coord[1]} for coord in poly.exterior.coords]
               })
   input_data = {
       "points": [{"id": 0, "longitude": well_point.x, "latitude": well_point.y}],
       "polygons": polygons,
   }
   try:
       input_json = json.dumps(input_data)
       result = subprocess.run(
           ["wasmer", "run", wasm_module, "--invoke", "point_in_polygon"],
           input=input_json,
           text=True,
           capture_output=True,
       )
       if result.returncode != 0:
           raise Exception(result.stderr)
       return json.loads(result.stdout)
   except Exception as e:
       print(f"Error in Rust WASM: {e}")
       return []