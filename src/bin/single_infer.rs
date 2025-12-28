use clap::Parser;
use image::io::Reader as ImageReader;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use colon_sim::vision::{DetectorHandle, InferenceThresholds};
use colon_sim::vision_interfaces::Frame;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Run detector on a single image and emit a boxed PNG"
)]
struct Args {
    /// Input image path (any format supported by the `image` crate).
    #[arg(long)]
    image: PathBuf,
    /// Output path for the boxed image (defaults to <stem>_boxed.png alongside the input).
    #[arg(long)]
    out: Option<PathBuf>,
    /// Objectness threshold.
    #[arg(long, default_value_t = 0.3)]
    infer_obj_thresh: f32,
    /// IoU threshold for NMS.
    #[arg(long, default_value_t = 0.5)]
    infer_iou_thresh: f32,
}

fn default_out_path(input: &Path) -> PathBuf {
    let parent = input.parent().unwrap_or_else(|| Path::new("."));
    let stem = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    parent.join(format!("{stem}_boxed.png"))
}

fn draw_rect(img: &mut image::RgbaImage, bbox_norm: [f32; 4], color: image::Rgba<u8>, thickness: u32) {
    let (w, h) = img.dimensions();
    let clamp = |v: f32, max: u32| -> u32 { v.max(0.0).min((max as i32 - 1) as f32) as u32 };
    let x0 = clamp(bbox_norm[0] * w as f32, w);
    let y0 = clamp(bbox_norm[1] * h as f32, h);
    let x1 = clamp(bbox_norm[2] * w as f32, w);
    let y1 = clamp(bbox_norm[3] * h as f32, h);
    if x0 >= w || y0 >= h || x1 >= w || y1 >= h {
        return;
    }
    for t in 0..thickness {
        let xx0 = x0.saturating_add(t);
        let yy0 = y0.saturating_add(t);
        let xx1 = x1.saturating_sub(t);
        let yy1 = y1.saturating_sub(t);
        if xx0 >= w || yy0 >= h || xx1 >= w || yy1 >= h || xx0 > xx1 || yy0 > yy1 {
            continue;
        }
        for x in xx0..=xx1 {
            if yy0 < h {
                img.put_pixel(x, yy0, color);
            }
            if yy1 < h {
                img.put_pixel(x, yy1, color);
            }
        }
        for y in yy0..=yy1 {
            if xx0 < w {
                img.put_pixel(xx0, y, color);
            }
            if xx1 < w {
                img.put_pixel(xx1, y, color);
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let in_path = &args.image;
    if !in_path.exists() {
        anyhow::bail!("input image not found: {}", in_path.display());
    }
    let out_path = args.out.unwrap_or_else(|| default_out_path(in_path));

    let img = ImageReader::open(in_path)?.decode()?.to_rgba8();
    let (w, h) = img.dimensions();
    let rgba = img.as_raw().clone();

    let thresh = InferenceThresholds {
        obj_thresh: args.infer_obj_thresh,
        iou_thresh: args.infer_iou_thresh,
    };
    let mut handle = DetectorHandle::with_thresholds(thresh);

    let ts = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0);
    let frame = Frame {
        id: 0,
        timestamp: ts,
        rgba: Some(rgba),
        size: (w, h),
        path: Some(in_path.clone()),
    };

    let result = handle.detector.detect(&frame);
    let mut boxed = img.clone();
    if result.boxes.is_empty() {
        eprintln!("no detections (confidence {})", result.confidence);
    } else {
        for (i, bbox) in result.boxes.iter().enumerate() {
            let color = if i == 0 {
                image::Rgba([255, 64, 192, 255])
            } else {
                image::Rgba([64, 192, 255, 255])
            };
            draw_rect(&mut boxed, *bbox, color, 2);
        }
    }
    boxed.save(&out_path)?;
    println!(
        "saved boxed image to {} ({} boxes)",
        out_path.display(),
        result.boxes.len()
    );
    Ok(())
}
