from flask import Flask, request, jsonify
from scripts.data_loader import load_wells, load_aquifers
from scripts.wasm_integration import rust_wasm_pip
from scripts.geobert import geobert_predict
from scripts.config import WELLS_GEOJSON, AQUIFER_FILES, WASM_MODULE


# Initialize Flask app
app = Flask(__name__)

# Load data at startup
wells = load_wells(WELLS_GEOJSON)
aquifers_list = load_aquifers(AQUIFER_FILES)

@app.route('/predict', methods=['POST'])
def predict():
    """API endpoint to predict aquifer context and GeoBERT classification."""
    try:
        # Parse request data
        data = request.json
        well_id = data.get("well_id")
        
        # Validate well_id
        if well_id is None or not isinstance(well_id, int):
            return jsonify({"error": "Invalid or missing well_id"}), 400

        # Get the well from the dataset
        well = wells[wells["ID"] == well_id].iloc[0]

        # Process each aquifer file
        for aquifers in aquifers_list:
            pip_results = rust_wasm_pip(well.geometry, aquifers, WASM_MODULE)

            if pip_results and any(result["inside"] for result in pip_results):
                # Find matching polygon
                matched_result = next(result for result in pip_results if result["inside"])
                aquifer_id = matched_result["polygon_id"]
                aquifer = aquifers.iloc[aquifer_id]

                # Create input for GeoBERT
                geobert_input = (
                    f"Well at Longitude {well.geometry.x}, Latitude {well.geometry.y} in {well['NMMUN']}, {well['SGUF']} "
                    f"is inside the {aquifer['SAQ_NM_NOM']} aquifer."
                )

                # Perform GeoBERT classification
                classification = geobert_predict(geobert_input)

                # Return result
                return jsonify({
                    "well_id": well_id,
                    "aquifer": aquifer["SAQ_NM_NOM"],
                    "context": geobert_input,
                    "classification": classification
                })

        # If no match found
        return jsonify({
            "well_id": well_id,
            "message": "The well is not inside any aquifer."
        })

    except Exception as e:
        return jsonify({"error": str(e)}), 500

if __name__ == "__main__":
    app.run(port=5000, debug=True)
