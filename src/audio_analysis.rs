//! # Concept: Audio Spectrum Analysis
//! 
//! This module provides the engine's rhythmic intelligence. It uses Fast 
//! Fourier Transform (FFT) analysis to identify rhythmic onsets (beats) in 
//! WAV files, enabling gameplay elements to synchronize with the music.

use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit};
use spectrum_analyzer::scaling::divide_by_N;
use std::path::Path;
use std::fs;
use std::io::Write;

/// Represents a detected rhythmic onset in an audio track.
#[derive(Debug, Clone, Copy)]
pub struct DetectedBeat {
    pub time: f32,
    pub intensity: f32,
}

/// A utility for analyzing audio files and extracting beat maps.
pub struct BeatDetector;

impl BeatDetector {
    /// Performs an automated analysis of a WAV file to generate a beat map.
    pub fn analyze_beats(file_path: &str, target_bpm_hint: Option<f32>) -> Result<Vec<DetectedBeat>, String> {
        let path = Path::new(file_path);
        let cache_path = path.with_extension("beats");

        // 1. Check for a sidecar '.beats' cache file to skip expensive FFT analysis.
        if cache_path.exists()
            && let Ok(content) = fs::read_to_string(&cache_path) {
                let mut cached_beats = Vec::new();
                let mut valid = true;
                for line in content.lines() {
                    let parts: Vec<&str> = line.split(',').collect();
                    if parts.len() == 2 {
                        if let (Ok(time), Ok(intensity)) = (parts[0].parse::<f32>(), parts[1].parse::<f32>()) {
                            cached_beats.push(DetectedBeat { time, intensity });
                        } else {
                            valid = false;
                            break;
                        }
                    }
                }
                if valid && !cached_beats.is_empty() {
                    println!("[Analysis] Loaded {} beats from cache.", cached_beats.len());
                    return Ok(cached_beats);
                }
            }

        // 2. Perform raw spectral flux calculation if no valid cache is found.
        let (spectral_flux, sample_rate) = Self::calculate_spectral_flux(file_path)?;
        
        // 3. Iteratively tune peak-picking sensitivity to match the target BPM.
        let duration_minutes = (spectral_flux.len() as f32 * 1024.0 / sample_rate as f32) / 60.0;
        let target_bpm = target_bpm_hint.unwrap_or(116.0);
        
        let mut best_beats = Vec::new();
        let mut best_diff = f32::MAX;
        let mut best_multiplier = 1.5;

        for m in 11..=100 {
            let multiplier = m as f32 / 10.0;
            let beats = Self::pick_peaks(&spectral_flux, multiplier, sample_rate as f32);
            
            let bpm = beats.len() as f32 / duration_minutes;
            let diff = (bpm - target_bpm).abs();
            
            if diff < best_diff {
                best_diff = diff;
                best_beats = beats;
                best_multiplier = multiplier;
            }
        }

        println!("[Analysis] Selected Multiplier {:.1} (Error: {:.1} BPM). Found {} beats.", best_multiplier, best_diff, best_beats.len());

        // 4. Save the optimized beat map to a cache file for instant loading next time.
        if let Ok(mut file) = fs::File::create(&cache_path) {
            for beat in &best_beats {
                let _ = writeln!(file, "{},{}", beat.time, beat.intensity);
            }
        }

        Ok(best_beats)
    }

    /// Computes the change in frequency energy (flux) across overlapping FFT windows.
    fn calculate_spectral_flux(file_path: &str) -> Result<(Vec<f32>, u32), String> {
        let path = Path::new(file_path);
        let mut reader = hound::WavReader::open(path).map_err(|e| e.to_string())?;
        
        let spec = reader.spec();
        let channels = spec.channels as usize;

        // 1. Decode raw WAV samples into normalized floating-point values.
        let samples: Vec<f32> = match spec.sample_format {
            hound::SampleFormat::Float => reader.samples::<f32>()
                .map(|s| s.unwrap_or(0.0))
                .collect(),
            hound::SampleFormat::Int => {
                let max_val = 2_u32.pow(spec.bits_per_sample as u32 - 1) as f32;
                reader.samples::<i32>()
                    .map(|s| s.unwrap_or(0) as f32 / max_val)
                    .collect()
            }
        };

        // 2. Downmix multi-channel audio to mono for consistent analysis.
        let mono_samples: Vec<f32> = if channels > 1 {
            samples.chunks(channels)
                .map(|chunk| chunk.iter().sum::<f32>() / channels as f32)
                .collect()
        } else {
            samples
        };

        let window_size = 2048;
        let hop_size = 1024;
        let mut spectral_flux = Vec::new();
        let mut prev_spectrum: Option<Vec<f32>> = None;

        // 3. Slide an overlapping window across the audio and perform FFT.
        let mut start_idx = 0;
        while start_idx < mono_samples.len() {
            let end_idx = (start_idx + window_size).min(mono_samples.len());
            let window = &mono_samples[start_idx..end_idx];
            
            let mut padded_window = window.to_vec();
            if padded_window.len() < window_size {
                padded_window.resize(window_size, 0.0);
            }

            let spectrum_result = samples_fft_to_spectrum(
                &padded_window,
                spec.sample_rate,
                FrequencyLimit::Range(40.0, 200.0), 
                Some(&divide_by_N),
            );

            if let Ok(spectrum) = spectrum_result {
                let magnitudes: Vec<f32> = spectrum.data().iter().map(|(_, val)| val.val()).collect();
                
                // 4. Calculate the positive difference in energy between this frame and the last (Flux).
                if let Some(prev) = &prev_spectrum {
                    let mut flux = 0.0;
                    for (curr, old) in magnitudes.iter().zip(prev.iter()) {
                        let diff = curr - old;
                        if diff > 0.0 { flux += diff; }
                    }
                    spectral_flux.push(flux);
                } else {
                    spectral_flux.push(0.0);
                }
                prev_spectrum = Some(magnitudes);
            }
            
            start_idx += hop_size;
        }
        Ok((spectral_flux, spec.sample_rate))
    }

    /// Identifies local flux maxima that exceed a dynamic sensitivity threshold.
    fn pick_peaks(spectral_flux: &[f32], threshold_multiplier: f32, sample_rate: f32) -> Vec<DetectedBeat> {
        let mut beats: Vec<DetectedBeat> = Vec::new();
        let history_size = 43;
        let hop_size = 1024;

        for i in 0..spectral_flux.len() {
            let start = i.saturating_sub(history_size / 2);
            let end = (i + history_size / 2).min(spectral_flux.len());
            let local_window = &spectral_flux[start..end];
            
            // 1. Calculate a local average flux to adapt to changing audio volume.
            let local_average: f32 = local_window.iter().sum::<f32>() / local_window.len() as f32;
            let current_flux = spectral_flux[i];

            // 2. Identify if the current frame is a 'peak' relative to its neighbors and the threshold.
            if current_flux > local_average * threshold_multiplier && current_flux > 0.0001 {
                let time = (i * hop_size) as f32 / sample_rate;
                
                // 3. Debounce peaks to prevent double-triggering on single rhythmic events.
                if let Some(last_beat) = beats.last() 
                    && time - last_beat.time < 0.2 { 
                        if current_flux > last_beat.intensity {
                            beats.pop(); 
                        } else {
                            continue;
                        }
                }
                
                beats.push(DetectedBeat {
                    time,
                    intensity: current_flux,
                });
            }
        }
        beats
    }
}
