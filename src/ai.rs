//! Local AI inference via Candle. Loads quantized Whisper .gguf from device storage.
//! Minimal memory footprint for mobile thermal constraints.

use anyhow::Result;
use std::path::Path;

/// Transcribe audio buffer (mono f32, 16 kHz) to text using Whisper-Tiny GGUF.
/// Model path: e.g. device storage or app bundle.
pub async fn transcribe_audio(path: &Path, samples: &[f32]) -> Result<String> {
    // TODO: Candle Whisper GGUF loading and inference
    // - Load model from path (whisper-tiny.gguf or similar)
    // - Convert f32 samples to candle tensor
    // - Run inference
    // - Return decoded text
    //
    // For now, stub returns placeholder. Full implementation requires:
    // - candle-core GGUF loading
    // - candle-transformers Whisper model forward pass
    // - Token decoding
    let _ = (path, samples);
    Ok("Processed".to_string())
}

/// Parse transcribed text to assign sticker value (0, 1, or 2).
/// Uses simple heuristics: "great", "excellent" -> 2; "ok", "good" -> 1; else 0.
pub fn parse_sticker_from_transcription(text: &str) -> crate::db::StickerValue {
    let lower = text.to_lowercase();
    if lower.contains("great")
        || lower.contains("excellent")
        || lower.contains("awesome")
        || lower.contains("perfect")
    {
        return crate::db::StickerValue::Two;
    }
    if lower.contains("good")
        || lower.contains("ok")
        || lower.contains("okay")
        || lower.contains("fine")
        || lower.contains("did well")
    {
        return crate::db::StickerValue::One;
    }
    crate::db::StickerValue::Zero
}
