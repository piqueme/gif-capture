extern crate sdl2;

use std::cmp;

use sdl2::VideoSubsystem;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::mouse::Cursor;
use sdl2::mouse::SystemCursor;
use sdl2::rect::Rect;
use sdl2::rect::Point;

struct CaptureContext {
    screen_dimensions: (u32, u32),
    capture_area: Rect,
}

fn get_screen_dimensions(
    video_subsystem: &VideoSubsystem
) -> Result<(u32, u32), String> {
    let display_mode = video_subsystem
        .current_display_mode(0)?;
    let dimensions = (
        display_mode.w as u32,
        display_mode.h as u32,
    );
    Ok(dimensions)
}

fn get_capture_rect(corner1: &Point, corner2: &Point) -> Rect {
    let min_x = cmp::min(corner1.x(), corner2.x());
    let min_y = cmp::min(corner1.y(), corner2.y());
    let max_x = cmp::max(corner1.x(), corner2.x());
    let max_y = cmp::max(corner1.y(), corner2.y());

    let width = (max_x - min_x).try_into().unwrap();
    let height = (max_y - min_y).try_into().unwrap();

    Rect::new(min_x, min_y, width, height)
}

fn get_capture_area(
    video_subsystem: &VideoSubsystem,
    screen_dimensions: &(u32, u32),
) -> Result<Rect, String> {
    let mut window = video_subsystem
        .window("Screenshot", screen_dimensions.0, screen_dimensions.1)
        .position_centered()
        .borderless()
        .input_grabbed()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
    window.set_opacity(0.5)?;
    
    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;
    canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
    canvas.clear();
    canvas.present();

    let cursor = Cursor::from_system(SystemCursor::Crosshair)
        .map_err(|err| format!("failed to load cursor: {}", err))?;
    cursor.set();

    let mut events = video_subsystem.sdl().event_pump()?;

    let mut selected_corners: (Option<Point>, Option<Point>) = (None, None);

    'running: loop {
        for event in events.poll_iter() {
            // quit 
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => break 'running,
                _ => {}
            }

            // update area
            match event {
                Event::MouseButtonDown { x, y, .. } => {
                    selected_corners.0 = Some(Point::new(x, y));
                    canvas.present();
                },
                Event::MouseMotion { x, y, .. } => {
                    if let Some(point) = selected_corners.0 {
                        let corner = Point::new(x, y);
                        let draw_rect = get_capture_rect(&corner, &point);
                        canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
                        canvas.clear();
                        canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));
                        canvas.fill_rect(draw_rect)?;
                        canvas.present();
                    }
                },
                Event::MouseButtonUp { x, y, .. } => {
                    if selected_corners.0.is_some() {
                        let end_point = Point::new(x, y);
                        selected_corners.1 = Some(end_point);
                        break 'running;
                    }
                }
                _ => {}
            }
        }
    }

    match selected_corners {
        (Some(start), Some(end)) => Ok(get_capture_rect(&start, &end)),
        _ => Err(String::from("Failed to select area for capture."))
    }
}

fn get_capture_context() -> Result<CaptureContext, String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let screen_dimensions = get_screen_dimensions(&video_subsystem)?;
    let capture_area = get_capture_area(&video_subsystem, &screen_dimensions)?;

    let capture_context = CaptureContext {
        screen_dimensions,
        capture_area
    };
    Ok(capture_context)
}

fn main() -> Result<(), String> {
    let capture_context = get_capture_context()?;
    let screen_dimensions = capture_context.screen_dimensions;
    let capture_area = capture_context.capture_area;

    println!("Capture Screen Dimensions: {:?}", screen_dimensions);
    println!("Capture Area: {:?}", capture_area);

    Ok(())
}
