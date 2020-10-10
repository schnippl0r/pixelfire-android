//              _          _  __ _
//        _ __ (_)_  _____| |/ _(_)_ __ ___
//       | '_ \| \ \/ / _ \ | |_| | '__/ _ \
//       | |_) | |>  <  __/ |  _| | | |  __/
//       | .__/|_/_/\_\___|_|_| |_|_|  \___|
//       |_|
//
// pixelfire-android - A pixel fire for Android using Rust, winit & pixels
// Copyright (C) 2020, Mario Kilies <mario@kilies.de>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use pixels::{wgpu::TextureFormat, PixelsBuilder, SurfaceTexture};
use std::time::Duration;
use winit::{event_loop::EventLoop, window::Window};

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
pub fn main() {
    let mut f = pixelfire_lib::Fire::default();
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();

    // This workaround is needed to prevent a crash.
    // Speculation: Maybe a race condition - Pixels acquires the SurcfaceTexture before the winit window creation has finished in the Android system
    std::thread::sleep(Duration::from_millis(500));

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        PixelsBuilder::new(f.width(), f.height(), surface_texture)
            .texture_format(TextureFormat::Rgba8UnormSrgb)
            .render_texture_format(TextureFormat::Rgba8UnormSrgb)
            .build()
            .unwrap()
    };

    event_loop.run(move |event, _, _ /* control_flow */| match event {
        winit::event::Event::RedrawEventsCleared => {
            let frame = pixels.get_frame();
            frame.copy_from_slice(f.as_slice());
            pixels.render().unwrap();
            f.update();
        }
        _ => {}
    });
}
