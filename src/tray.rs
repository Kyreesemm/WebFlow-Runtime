use tray_icon::menu::{Menu, MenuItem};
use tray_icon::{Icon, TrayIcon, TrayIconBuilder};

pub const SHOW_ID: &str = "show-manager";
pub const EXIT_ID: &str = "exit-manager";

pub struct ManagerTray {
    pub _icon: TrayIcon,
    pub show_item: MenuItem,
    pub exit_item: MenuItem,
}

impl ManagerTray {
    pub fn create(icon_bytes: &[u8]) -> Result<Self, String> {
        let icon = load_icon(icon_bytes)
            .ok_or_else(|| "Failed to decode manager tray icon".to_string())?;
        let show_item = MenuItem::with_id(SHOW_ID, "Показать WebFlow Runtime Manager", true, None);
        let exit_item = MenuItem::with_id(EXIT_ID, "Выйти", true, None);
        let menu = Menu::new();
        menu.append(&show_item).map_err(|e| e.to_string())?;
        menu.append(&exit_item).map_err(|e| e.to_string())?;

        let tray = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("WebFlow Runtime Manager")
            .with_icon(icon)
            .build()
            .map_err(|e| e.to_string())?;

        Ok(Self {
            _icon: tray,
            show_item,
            exit_item,
        })
    }

    pub fn set_interface_visible(&self, visible: bool) {
        self.show_item.set_text(if visible {
            "Скрыть интерфейс"
        } else {
            "Показать интерфейс"
        });
    }
}

fn load_icon(bytes: &[u8]) -> Option<Icon> {
    let mut decoder = png::Decoder::new(std::io::BufReader::new(std::io::Cursor::new(bytes)));
    decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut reader = decoder.read_info().ok()?;
    let mut buffer = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buffer).ok()?;
    let data = &buffer[..info.buffer_size()];
    let rgba = match info.color_type {
        png::ColorType::Rgba => data.to_vec(),
        png::ColorType::Rgb => data
            .chunks_exact(3)
            .flat_map(|pixel| [pixel[0], pixel[1], pixel[2], 255])
            .collect(),
        png::ColorType::Grayscale => data
            .iter()
            .flat_map(|&value| [value, value, value, 255])
            .collect(),
        png::ColorType::GrayscaleAlpha => data
            .chunks_exact(2)
            .flat_map(|pixel| [pixel[0], pixel[0], pixel[0], pixel[1]])
            .collect(),
        _ => return None,
    };
    Icon::from_rgba(rgba, info.width, info.height).ok()
}
