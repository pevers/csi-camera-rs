extern crate opencv;

use opencv::{
    core,
    highgui,
    prelude::*,
    videoio
};

fn gstreamer_pipeline(
    capture_width: u32,
    capture_height: u32,
    display_width: u32,
    display_height: u32,
    framerate: u32,
    flip_method: u32,
) -> String {
    format!(
        "nvarguscamerasrc ! \
        video/x-raw(memory:NVMM), \
        width=(int){}, height=(int){}, \
        format=(string)NV12, framerate=(fraction){}/1 ! \
        nvvidconv flip-method={} !  \
        video/x-raw, width=(int){}, height=(int){}, format=(string)BGRx ! \
        videoconvert ! \
        video/x-raw, format=(string)BGR ! appsink",
        capture_width, capture_height, framerate, flip_method, display_width, display_height,
    )
}

fn run() -> opencv::Result<()> {
    let window = "video capture";
    highgui::named_window(window, 1)?;
    let pipeline = gstreamer_pipeline(1280, 720, 1280, 720, 60, 0);
    println!("Creating pipeline {}", pipeline);
    let mut cam = videoio::VideoCapture::from_file(&pipeline, videoio::CAP_GSTREAMER)?;
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }
    loop {
        let mut frame = core::Mat::default()?;
        cam.read(&mut frame)?;
        if frame.size()?.width > 0 {
            highgui::imshow(window, &mut frame)?;
        }
        let key = highgui::wait_key(10)?;
        if key > 0 && key != 255 {
            break;
        }
    }
    Ok(())
}

fn main() {
    run().unwrap();
}
