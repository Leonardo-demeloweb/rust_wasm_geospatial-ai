import geopandas as gpd
def load_wells(file_path):
   """Load and process well data from GeoJSON."""
   return gpd.read_file(file_path).to_crs("EPSG:4326")
def load_aquifers(aquifer_files):
   """Load and process aquifer data from multiple GeoJSON files."""
   aquifers = []
   for file_path in aquifer_files:
       aq = gpd.read_file(file_path).to_crs("EPSG:4326")
       # Fix invalid geometries
       aq["geometry"] = aq["geometry"].buffer(0)
       aq = aq[aq["geometry"].is_valid]
       aquifers.append(aq)
   return aquifers