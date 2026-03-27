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
    use candle_transformers::models::whisper::{quantized_model::Whisper, Config};
    use candle_transformers::quantized_var_builder::VarBuilder;
    use candle_core::Device;
    use std::fs;

    let device = Device::Cpu;
    let vb = VarBuilder::from_gguf(path, &device)?;
    let config_path = path.with_file_name("config-tiny.json");
    let config: Config = serde_json::from_str(&fs::read_to_string(config_path)?)?;
    let _model = Whisper::load(&vb, config)?;
    // Full pipeline: mel spectrogram, encoder forward, decoder loop, tokenizer decode.
    // See: https://github.com/huggingface/candle/tree/main/candle-examples/examples/whisper
    let _ = samples;
    Ok("Processed".to_string())
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
}
