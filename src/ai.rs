// Unlicense — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
//! Local AI inference via Candle. Loads quantized Whisper .gguf from device storage.
//! Minimal memory footprint for mobile thermal constraints.

use anyhow::Result;
use std::path::Path;

/// f119=transcribe_audio. Mono f32 16kHz → text via Whisper-Tiny GGUF.
/// When candle feature + model at path: runs inference. Else: returns placeholder.
pub async fn f119(path: &Path, samples: &[f32]) -> Result<String> {
    #[cfg(feature = "candle")]
    {
        if path.exists() {
            if let Ok(Ok(t)) = tokio::task::spawn_blocking({
                let p = path.to_path_buf();
                let s = samples.to_vec();
                move || f137(&p, &s)
            })
            .await
            {
                return Ok(t);
            }
        }
    }
    let _ = (path, samples);
    Ok("Processed".to_string())
}

/// f137=transcribe_audio_sync. Load GGUF, run mel→encoder→decoder→tokenizer (candle).
#[cfg(feature = "candle")]
fn f137(path: &Path, samples: &[f32]) -> Result<String> {
    use candle_transformers::models::whisper::{
        audio::pcm_to_mel, quantized_model::Whisper, Config, EOT_TOKEN, N_SAMPLES,
        NO_TIMESTAMPS_TOKEN, SOT_TOKEN, TRANSCRIBE_TOKEN,
    };
    use candle_transformers::quantized_var_builder::VarBuilder;
    use candle_core::{Device, IndexOp, Tensor, D};
    use std::fs;

    let device = Device::Cpu;
    let vb = VarBuilder::from_gguf(path, &device)?;
    let config_path = path.with_file_name("config-tiny.json");
    let config: Config = serde_json::from_str(&fs::read_to_string(config_path)?)?;
    let mut model = Whisper::load(&vb, config.clone())?;

    // Load mel filter bank (80 x 201 = 16,080 f32 values, little-endian)
    let filters_path = path.with_file_name("melfilters.bytes");
    let filters_bytes = fs::read(&filters_path)
        .map_err(|e| anyhow::anyhow!("melfilters.bytes not found at {}: {}", filters_path.display(), e))?;
    let filters: Vec<f32> = filters_bytes
        .chunks_exact(4)
        .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect();

    // Pad or truncate audio to 30s (N_SAMPLES = 480,000 at 16kHz)
    let mut audio = samples.to_vec();
    if audio.len() > N_SAMPLES {
        audio.truncate(N_SAMPLES);
    }

    // Mel spectrogram: audio → [n_mel * n_len] flat vec
    let mel = pcm_to_mel(&config, &audio, &filters);
    let n_mel = config.num_mel_bins; // 80
    let n_len = mel.len() / n_mel;
    let mel_tensor = Tensor::from_vec(mel, (1, n_mel, n_len), &device)?;

    // Encoder forward
    let encoder_out = model.encoder.forward(&mel_tensor, true)?;

    // Load tokenizer for decoding + token ID lookup
    let tokenizer_path = path.with_file_name("tokenizer-tiny.json");
    let tokenizer = tokenizers::Tokenizer::from_file(&tokenizer_path)
        .map_err(|e| anyhow::anyhow!("tokenizer-tiny.json: {}", e))?;

    // Resolve special token IDs
    let sot_id = tokenizer.token_to_id(SOT_TOKEN).unwrap_or(50258);
    let eot_id = tokenizer.token_to_id(EOT_TOKEN).unwrap_or(50256);
    let transcribe_id = tokenizer.token_to_id(TRANSCRIBE_TOKEN).unwrap_or(50359);
    let no_ts_id = tokenizer.token_to_id(NO_TIMESTAMPS_TOKEN).unwrap_or(50363);
    let en_id = sot_id + 1; // English = SOT + 1 = 50259

    // Decoder loop: autoregressive greedy decoding
    let mut tokens: Vec<u32> = vec![sot_id, en_id, transcribe_id, no_ts_id];
    let max_tokens = config.max_target_positions / 2; // 224

    model.decoder.reset_kv_cache();
    for step in 0..max_tokens {
        let token_tensor = Tensor::new(tokens.as_slice(), &device)?.unsqueeze(0)?;
        let dec_out = model.decoder.forward(&token_tensor, &encoder_out, step == 0)?;
        let logits = model.decoder.final_linear(&dec_out)?;

        // Get logits for the last token position, greedy argmax
        let last_logits = logits.i((.., tokens.len() - 1, ..))?;

        // Suppress special tokens (> 50256) except EOT to avoid hallucinated tags
        let vocab_size = last_logits.dim(D::Minus1)?;
        let suppress_mask: Vec<f32> = (0..vocab_size)
            .map(|i| {
                let id = i as u32;
                if id == eot_id {
                    0.0
                } else if id > eot_id {
                    f32::NEG_INFINITY
                } else {
                    0.0
                }
            })
            .collect();
        let mask_tensor = Tensor::from_vec(suppress_mask, last_logits.shape(), &device)?;
        let masked = (last_logits + mask_tensor)?;

        let next_id = masked
            .argmax(D::Minus1)?
            .squeeze(0)?
            .to_scalar::<u32>()?;

        if next_id == eot_id {
            break;
        }
        tokens.push(next_id);
    }

    // Decode tokens to text (skip the 4 prompt tokens)
    let text_tokens: Vec<u32> = tokens.into_iter().skip(4).collect();
    let text = tokenizer
        .decode(&text_tokens, true)
        .map_err(|e| anyhow::anyhow!("tokenizer decode: {}", e))?;

    let trimmed = text.trim().to_string();
    if trimmed.is_empty() {
        Ok("(no speech detected)".to_string())
    } else {
        Ok(trimmed)
    }
}

/// t124=BehaviorResult. s10=score, s11=note, s12=tags.
#[derive(Debug, Clone, Default)]
pub struct t124 {
    pub s10: super::db::t119,
    pub s11: String,
    pub s12: Vec<String>,
}

/// f134=extract_behavior. Parse text → score + note. Uses f120 heuristics; LLM optional later.
pub fn f134(text: &str) -> t124 {
    let s10 = f120(text);
    let s11 = text.trim().to_string();
    let s12 = f138(text);
    t124 { s10, s11, s12 }
}

/// f138=extract_tags. Heuristic tag extraction from transcription.
fn f138(text: &str) -> Vec<String> {
    let lower = text.to_lowercase();
    let mut tags = Vec::new();
    for (phrase, tag) in [
        ("elopement", "elopement"),
        ("refus", "refusal"),
        ("combative", "combative"),
        ("stay in", "stay_in_space"),
        ("finish", "finish_work"),
        ("great", "positive"),
        ("excellent", "positive"),
    ] {
        if lower.contains(phrase) {
            tags.push(tag.to_string());
        }
    }
    tags
}

/// f120=parse_sticker_from_transcription. Heuristics: great/excellent→2, ok/good→1, else 0.
pub fn f120(text: &str) -> super::db::t119 {
    let lower = text.to_lowercase();
    if lower.contains("great")
        || lower.contains("excellent")
        || lower.contains("awesome")
        || lower.contains("perfect")
    {
        return super::db::t119::Two;
    }
    if lower.contains("good")
        || lower.contains("ok")
        || lower.contains("okay")
        || lower.contains("fine")
        || lower.contains("did well")
        || lower.contains("acceptable")
    {
        return super::db::t119::One;
    }
    if lower.contains("refus")
        || lower.contains("elopement")
        || lower.contains("combative")
        || lower.contains("no work")
        || lower.contains("didn't")
    {
        return super::db::t119::Zero;
    }
    super::db::t119::Zero
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::t119;

    /// f120=parse_sticker_great_returns_two
    #[test]
    fn parse_sticker_great_returns_two() {
        assert_eq!(f120("He did great!"), t119::Two);
    }

    /// f120=parse_sticker_excellent_returns_two
    #[test]
    fn parse_sticker_excellent_returns_two() {
        assert_eq!(f120("Excellent work"), t119::Two);
    }

    /// f120=parse_sticker_good_returns_one
    #[test]
    fn parse_sticker_good_returns_one() {
        assert_eq!(f120("Good job today"), t119::One);
    }

    /// f120=parse_sticker_ok_returns_one
    #[test]
    fn parse_sticker_ok_returns_one() {
        assert_eq!(f120("Ok, fine"), t119::One);
    }

    /// f120=parse_sticker_empty_returns_zero
    #[test]
    fn parse_sticker_empty_returns_zero() {
        assert_eq!(f120(""), t119::Zero);
    }

    /// f120=parse_sticker_neutral_returns_zero
    #[test]
    fn parse_sticker_neutral_returns_zero() {
        assert_eq!(f120("needs improvement"), t119::Zero);
    }

    /// f120=parse_sticker_refusal_returns_zero
    #[test]
    fn parse_sticker_refusal_returns_zero() {
        assert_eq!(
            f120("Refused to do work"),
            t119::Zero
        );
    }

    /// f120=parse_sticker_awesome_returns_two
    #[test]
    fn parse_sticker_awesome_returns_two() {
        assert_eq!(f120("Awesome job!"), t119::Two);
    }

    /// f120=parse_sticker_perfect_returns_two
    #[test]
    fn parse_sticker_perfect_returns_two() {
        assert_eq!(f120("Perfect day"), t119::Two);
    }

    /// f120=parse_sticker_fine_returns_one
    #[test]
    fn parse_sticker_fine_returns_one() {
        assert_eq!(f120("Did fine today"), t119::One);
    }

    /// f120=parse_sticker_did_well_returns_one
    #[test]
    fn parse_sticker_did_well_returns_one() {
        assert_eq!(f120("He did well"), t119::One);
    }

    /// f120=parse_sticker_acceptable_returns_one
    #[test]
    fn parse_sticker_acceptable_returns_one() {
        assert_eq!(f120("Acceptable behavior"), t119::One);
    }

    /// f120=parse_sticker_elopement_returns_zero
    #[test]
    fn parse_sticker_elopement_returns_zero() {
        assert_eq!(
            f120("Elopement incident"),
            t119::Zero
        );
    }

    /// f120=parse_sticker_combative_returns_zero
    #[test]
    fn parse_sticker_combative_returns_zero() {
        assert_eq!(
            f120("Combative with staff"),
            t119::Zero
        );
    }

    /// f120=parse_sticker_no_work_returns_zero
    #[test]
    fn parse_sticker_no_work_returns_zero() {
        assert_eq!(
            f120("No work completed"),
            t119::Zero
        );
    }

    /// f120=parse_sticker_didnt_returns_zero
    #[test]
    fn parse_sticker_didnt_returns_zero() {
        assert_eq!(
            f120("He didn't participate"),
            t119::Zero
        );
    }

    /// f134=extract_behavior
    #[test]
    fn extract_behavior_returns_score_and_note() {
        let r = f134("He did great today!");
        assert_eq!(r.s10, t119::Two);
        assert_eq!(r.s11, "He did great today!");
        assert!(r.s12.contains(&"positive".to_string()));
    }

    /// f138=extract_tags via f134: elopement, refusal, combative
    #[test]
    fn extract_behavior_tags_elopement_refusal_combative() {
        let r = f134("Elopement and refused to stay in. Combative.");
        assert!(r.s12.contains(&"elopement".to_string()));
        assert!(r.s12.contains(&"refusal".to_string()));
        assert!(r.s12.contains(&"combative".to_string()));
    }

    /// f138=extract_tags via f134: stay_in_space, finish_work
    #[test]
    fn extract_behavior_tags_stay_in_finish() {
        let r = f134("Had to stay in his seat and helped finish the work.");
        assert!(r.s12.contains(&"stay_in_space".to_string()));
        assert!(r.s12.contains(&"finish_work".to_string()));
    }

    /// f119=transcribe_audio placeholder when no model
    #[tokio::test]
    async fn transcribe_audio_returns_processed_without_model() {
        let path = std::path::Path::new("/nonexistent/whisper.gguf");
        let samples = vec![0.0f32; 1600]; // 0.1s at 16kHz
        let text = super::f119(path, &samples).await.unwrap();
        assert_eq!(text, "Processed");
    }

    #[tokio::test]
    async fn transcribe_audio_empty_samples() {
        let path = std::path::Path::new("/nonexistent/whisper.gguf");
        let samples: Vec<f32> = vec![];
        let text = super::f119(path, &samples).await.unwrap();
        assert_eq!(text, "Processed");
    }

    // ===== f134 edge cases =====

    #[test]
    fn f134_empty_input() {
        let r = f134("");
        assert_eq!(r.s10, t119::Zero);
        assert_eq!(r.s11, "");
        assert!(r.s12.is_empty());
    }

    #[test]
    fn f134_mixed_case_great() {
        assert_eq!(f120("GREAT job"), t119::Two);
        assert_eq!(f120("GrEaT work"), t119::Two);
        assert_eq!(f120("EXCELLENT"), t119::Two);
    }

    #[test]
    fn f134_unicode_text() {
        let r = f134("Très bien! 👍 Great");
        assert_eq!(r.s10, t119::Two);
        assert_eq!(r.s11, "Très bien! 👍 Great");
        assert!(r.s12.contains(&"positive".to_string()));
    }

    #[test]
    fn f134_very_long_text() {
        let long = "great ".repeat(5000);
        let r = f134(&long);
        assert_eq!(r.s10, t119::Two);
    }

    #[test]
    fn f134_numbers_only() {
        let r = f134("12345 67890");
        assert_eq!(r.s10, t119::Zero);
        assert!(r.s12.is_empty());
    }

    #[test]
    fn f134_whitespace_only() {
        let r = f134("   \t\n  ");
        assert_eq!(r.s10, t119::Zero);
    }

    #[test]
    fn f134_conflicting_keywords_first_wins() {
        // "great" matches first → Two, even though "refusal" is also present
        let r = f134("great but also refusal");
        assert_eq!(r.s10, t119::Two); // "great" checked first in f120
    }

    // ===== f138 tag extraction edge cases =====

    #[test]
    fn f138_empty_returns_no_tags() {
        let r = f134("");
        assert!(r.s12.is_empty());
    }

    #[test]
    fn f138_tag_in_middle_of_word() {
        // "unfinished" contains "finish"
        let r = f134("unfinished business");
        assert!(r.s12.contains(&"finish_work".to_string()));
    }

    #[test]
    fn f138_all_tags_at_once() {
        let r = f134("elopement refusal combative stay in finish great excellent");
        assert!(r.s12.contains(&"elopement".to_string()));
        assert!(r.s12.contains(&"refusal".to_string()));
        assert!(r.s12.contains(&"combative".to_string()));
        assert!(r.s12.contains(&"stay_in_space".to_string()));
        assert!(r.s12.contains(&"finish_work".to_string()));
        assert!(r.s12.contains(&"positive".to_string()));
        assert_eq!(r.s12.len(), 7); // "great" and "excellent" both → "positive"
    }

    #[test]
    fn f138_case_insensitive() {
        let r = f134("ELOPEMENT REFUSAL COMBATIVE");
        assert!(r.s12.contains(&"elopement".to_string()));
        assert!(r.s12.contains(&"refusal".to_string()));
        assert!(r.s12.contains(&"combative".to_string()));
    }

    #[test]
    fn f138_partial_match_not_exact() {
        // "refus" is the pattern, so "refused" should match
        let r = f134("She refused to participate");
        assert!(r.s12.contains(&"refusal".to_string()));
    }

    // ===== f120 additional keyword coverage =====

    #[test]
    fn parse_sticker_okay_returns_one() {
        assert_eq!(f120("It was okay"), t119::One);
    }

    #[test]
    fn parse_sticker_multiple_keywords_first_match_wins() {
        // "great" is checked before "good", so result is Two
        assert_eq!(f120("great and good"), t119::Two);
    }

    #[test]
    fn parse_sticker_keyword_in_sentence() {
        assert_eq!(f120("He showed excellent behavior during the activity"), t119::Two);
    }

    #[test]
    fn parse_sticker_punctuation_around_keyword() {
        assert_eq!(f120("great!"), t119::Two);
        assert_eq!(f120("(good)"), t119::One);
        assert_eq!(f120("'refusal'"), t119::Zero);
    }
}
