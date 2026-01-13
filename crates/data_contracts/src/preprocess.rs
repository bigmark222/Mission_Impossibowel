use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub struct ImageStats {
    pub mean: [f32; 3],
    pub std: [f32; 3],
    pub aspect: f32,
}

impl ImageStats {
    pub fn tiny_input(&self) -> [f32; 4] {
        [self.mean[0], self.mean[1], self.mean[2], self.aspect]
    }

    pub fn feature_vector(&self, box_count: f32) -> [f32; 8] {
        [
            self.mean[0],
            self.mean[1],
            self.mean[2],
            self.std[0],
            self.std[1],
            self.std[2],
            self.aspect,
            box_count,
        ]
    }
}

#[derive(Debug, Error)]
pub enum ImageStatsError {
    #[error("expected {expected} values, got {actual}")]
    InvalidLength { expected: usize, actual: usize },
    #[error("image dimensions must be non-zero")]
    ZeroDimensions,
}

pub fn stats_from_rgb_u8(
    width: u32,
    height: u32,
    rgb: &[u8],
) -> Result<ImageStats, ImageStatsError> {
    if width == 0 || height == 0 {
        return Err(ImageStatsError::ZeroDimensions);
    }
    let expected = width as usize * height as usize * 3;
    if rgb.len() != expected {
        return Err(ImageStatsError::InvalidLength {
            expected,
            actual: rgb.len(),
        });
    }

    let mut sum = [0f32; 3];
    let mut sumsq = [0f32; 3];
    for chunk in rgb.chunks_exact(3) {
        let r = chunk[0] as f32 / 255.0;
        let g = chunk[1] as f32 / 255.0;
        let b = chunk[2] as f32 / 255.0;
        sum[0] += r;
        sum[1] += g;
        sum[2] += b;
        sumsq[0] += r * r;
        sumsq[1] += g * g;
        sumsq[2] += b * b;
    }

    stats_from_sums(width as usize, height as usize, sum, sumsq)
}

pub fn stats_from_rgba_u8(
    width: u32,
    height: u32,
    rgba: &[u8],
) -> Result<ImageStats, ImageStatsError> {
    if width == 0 || height == 0 {
        return Err(ImageStatsError::ZeroDimensions);
    }
    let expected = width as usize * height as usize * 4;
    if rgba.len() != expected {
        return Err(ImageStatsError::InvalidLength {
            expected,
            actual: rgba.len(),
        });
    }

    let mut sum = [0f32; 3];
    let mut sumsq = [0f32; 3];
    for chunk in rgba.chunks_exact(4) {
        let r = chunk[0] as f32 / 255.0;
        let g = chunk[1] as f32 / 255.0;
        let b = chunk[2] as f32 / 255.0;
        sum[0] += r;
        sum[1] += g;
        sum[2] += b;
        sumsq[0] += r * r;
        sumsq[1] += g * g;
        sumsq[2] += b * b;
    }

    stats_from_sums(width as usize, height as usize, sum, sumsq)
}

pub fn stats_from_chw_f32(
    width: usize,
    height: usize,
    chw: &[f32],
) -> Result<ImageStats, ImageStatsError> {
    if width == 0 || height == 0 {
        return Err(ImageStatsError::ZeroDimensions);
    }
    let expected = width * height * 3;
    if chw.len() != expected {
        return Err(ImageStatsError::InvalidLength {
            expected,
            actual: chw.len(),
        });
    }

    let mut sum = [0f32; 3];
    let mut sumsq = [0f32; 3];
    let pixels_per_channel = width * height;
    for c in 0..3 {
        let start = c * pixels_per_channel;
        let slice = &chw[start..start + pixels_per_channel];
        for v in slice {
            sum[c] += *v;
            sumsq[c] += *v * *v;
        }
    }

    stats_from_sums(width, height, sum, sumsq)
}

fn stats_from_sums(
    width: usize,
    height: usize,
    sum: [f32; 3],
    sumsq: [f32; 3],
) -> Result<ImageStats, ImageStatsError> {
    let pix_count = (width * height) as f32;
    let mean = [sum[0] / pix_count, sum[1] / pix_count, sum[2] / pix_count];
    let std = [
        (sumsq[0] / pix_count - mean[0] * mean[0]).max(0.0).sqrt(),
        (sumsq[1] / pix_count - mean[1] * mean[1]).max(0.0).sqrt(),
        (sumsq[2] / pix_count - mean[2] * mean[2]).max(0.0).sqrt(),
    ];
    Ok(ImageStats {
        mean,
        std,
        aspect: width as f32 / height as f32,
    })
}
