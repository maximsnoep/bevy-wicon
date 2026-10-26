//! The window icon on Windows (title bar, taskbar and Alt+Tab), from one PNG image built into the program, so it does not
//! depend on the working directory. (On Linux with X11, the title bar too; other platforms ignore it.)
//!
//! The window is created by winit after the startup systems may have run, so the icon is set from the first frame
//! that has a window.

use bevy::ecs::system::NonSendMarker;
use bevy::prelude::*;
use image::imageops::FilterType;

// The size of the title bar icon (16 x 16 at 100% display scale, 32 x 32 at 200%). Windows scales a large image down
// poorly, so it is resized here. The taskbar (and Alt+Tab) use the image as it is, at most 256 x 256.
const TITLE_BAR: u32 = 32;

#[derive(Debug, Clone)]
pub struct WindowIconPlugin {
    png: &'static [u8],
}

impl WindowIconPlugin {
    /// The icon from a square PNG image (e.g., `include_bytes!`), preferably 256 x 256.
    #[must_use]
    pub const fn new(png: &'static [u8]) -> Self {
        Self { png }
    }
}

// The icon for the title bar and for the taskbar, until they are set.
#[derive(Resource)]
struct WindowIcon {
    title_bar: winit::window::Icon,
    taskbar: winit::window::Icon,
}

impl Plugin for WindowIconPlugin {
    fn build(&self, app: &mut App) {
        match icons(self.png) {
            Ok((title_bar, taskbar)) => {
                app.insert_resource(WindowIcon { title_bar, taskbar })
                    .add_systems(Update, set.run_if(resource_exists::<WindowIcon>));
            }
            Err(err) => warn!("WindowIconPlugin: cannot read the icon: {err}"),
        }
    }
}

// The title bar and the taskbar icon from the image.
fn icons(png: &[u8]) -> Result<(winit::window::Icon, winit::window::Icon)> {
    let image = image::load_from_memory_with_format(png, image::ImageFormat::Png)?.into_rgba8();
    let title_bar = image::imageops::resize(&image, TITLE_BAR, TITLE_BAR, FilterType::Lanczos3);
    let icon = |image: image::RgbaImage| {
        let (width, height) = image.dimensions();
        winit::window::Icon::from_rgba(image.into_raw(), width, height)
    };
    Ok((icon(title_bar)?, icon(image)?))
}

fn set(mut commands: Commands, icon: Res<WindowIcon>, _: NonSendMarker) {
    let set = bevy::winit::WINIT_WINDOWS.with_borrow(|winit_windows| {
        for window in winit_windows.windows.values() {
            window.set_window_icon(Some(icon.title_bar.clone()));
            #[cfg(target_os = "windows")]
            winit::platform::windows::WindowExtWindows::set_taskbar_icon(&**window, Some(icon.taskbar.clone()));
        }
        !winit_windows.windows.is_empty()
    });
    if set {
        commands.remove_resource::<WindowIcon>();
    }
}
