//! # Concept: Frame Capture (FFmpeg Bridge)
//! 
//! This module is responsible for the 'Offline Rendering' pipeline. 
//! it captures raw canvas pixel buffers and pipes them to an FFmpeg 
//! subprocess to generate high-fidelity, lag-free gameplay video.