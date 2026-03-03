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
pub fn parse_sticker_from_transcription(text: &str) -> super::db::StickerValue {
    let lower = text.to_lowercase();
    if lower.contains("great")
        || lower.contains("excellent")
        || lower.contains("awesome")
        || lower.contains("perfect")
    {
        return super::db::StickerValue::Two;
    }
    if lower.contains("good")
        || lower.contains("ok")
        || lower.contains("okay")
        || lower.contains("fine")
        || lower.contains("did well")
    {
        return super::db::StickerValue::One;
    }
    super::db::StickerValue::Zero
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::StickerValue;

    #[test]
    fn parse_sticker_great_returns_two() {
        assert_eq!(parse_sticker_from_transcription("He did great!"), StickerValue::Two);
    }

    #[test]
    fn parse_sticker_excellent_returns_two() {
        assert_eq!(parse_sticker_from_transcription("Excellent work"), StickerValue::Two);
    }

    #[test]
    fn parse_sticker_good_returns_one() {
        assert_eq!(parse_sticker_from_transcription("Good job today"), StickerValue::One);
    }

    #[test]
    fn parse_sticker_ok_returns_one() {
        assert_eq!(parse_sticker_from_transcription("Ok, fine"), StickerValue::One);
    }

    #[test]
    fn parse_sticker_empty_returns_zero() {
        assert_eq!(parse_sticker_from_transcription(""), StickerValue::Zero);
    }

    #[test]
    fn parse_sticker_neutral_returns_zero() {
        assert_eq!(parse_sticker_from_transcription("needs improvement"), StickerValue::Zero);
    }
}
