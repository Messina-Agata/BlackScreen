use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Fullscreen},
};

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let monitors: Vec<_> = event_loop.available_monitors().collect();

    let mut windows = Vec::new();

    for monitor in &monitors {
        let window = WindowBuilder::new()
            .with_visible(false) // inizialmente invisibile
            .with_decorations(false)
            .build(&event_loop)
            .unwrap();

        window.set_fullscreen(Some(Fullscreen::Borderless(Some(monitor.clone()))));

        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
        let pixels = Pixels::new(size.width, size.height, surface_texture)?;

        window.set_visible(true);
        windows.push((window, pixels));
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested | WindowEvent::MouseInput { .. } => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            Event::RedrawRequested(window_id) => {
                if let Some((_, pixels)) = windows.iter_mut().find(|(w, _)| w.id() == window_id) {
                    let frame: &mut [u8] = pixels.frame_mut();
                    for pixel in frame.chunks_exact_mut(4) {
                        pixel.copy_from_slice(&[0u8, 0u8, 0u8, 255u8]); // nero RGBA
                    }
                    pixels.render().unwrap();
                }
            }
            _ => {}
        }
    });
}