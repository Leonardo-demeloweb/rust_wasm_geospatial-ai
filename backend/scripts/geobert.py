from transformers import AutoTokenizer, TFAutoModelForSequenceClassification
import tensorflow as tf
def geobert_predict(text, model_name="botryan96/GeoBERT"):
   """Predict classification using GeoBERT."""
   tokenizer = AutoTokenizer.from_pretrained(model_name)
   model = TFAutoModelForSequenceClassification.from_pretrained(model_name)

   inputs = tokenizer(text, return_tensors="tf", truncation=True, padding=True, max_length=128)
   outputs = model(**inputs)
   predicted_class = tf.argmax(outputs.logits, axis=-1).numpy()[0]
   return predicted_class