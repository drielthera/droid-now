use crate::core::{self, Device, RomInspection, ScanResult, ToolLocator, ToolStatus};
use crate::safety;
use eframe::egui::{self, Align, Align2, Color32, FontId, Layout, RichText, Stroke};
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

const BG: Color32 = Color32::from_rgb(20, 23, 27);
const PANEL: Color32 = Color32::from_rgb(29, 32, 38);
const PANEL_DARK: Color32 = Color32::from_rgb(23, 26, 32);
const LINE: Color32 = Color32::from_rgb(48, 53, 61);
const TEXT: Color32 = Color32::from_rgb(232, 234, 239);
const MUTED: Color32 = Color32::from_rgb(160, 166, 176);
const GREEN: Color32 = Color32::from_rgb(73, 201, 177);
const BLUE: Color32 = Color32::from_rgb(151, 177, 218);
const AMBER: Color32 = Color32::from_rgb(225, 187, 112);
const RED: Color32 = Color32::from_rgb(235, 127, 136);

#[derive(Clone, Copy)]
enum Icon {
    Home,
    Activity,
    About,
    Help,
    Settings,
    Scan,
    Device,
    Shield,
    Folder,
    Info,
    Reboot,
}

fn draw_icon(ui: &mut egui::Ui, kind: Icon, size: f32, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(size, size), egui::Sense::hover());
    paint_icon(ui.painter(), kind, rect, color);
}

fn paint_icon(painter: &egui::Painter, kind: Icon, rect: egui::Rect, color: Color32) {
    let unit = rect.width().min(rect.height());
    let offset_x = (rect.width() - unit) * 0.5;
    let offset_y = (rect.height() - unit) * 0.5;
    let point = |x: f32, y: f32| {
        egui::pos2(
            rect.left() + offset_x + x * unit,
            rect.top() + offset_y + y * unit,
        )
    };
    let stroke = Stroke::new((unit * 0.075).max(1.3), color);
    let line = |a, b| painter.line_segment([a, b], stroke);
    let closed = |points: Vec<egui::Pos2>| painter.add(egui::Shape::closed_line(points, stroke));

    match kind {
        Icon::Home => {
            line(point(0.12, 0.48), point(0.50, 0.16));
            line(point(0.50, 0.16), point(0.88, 0.48));
            line(point(0.24, 0.43), point(0.24, 0.84));
            line(point(0.76, 0.43), point(0.76, 0.84));
            line(point(0.24, 0.84), point(0.76, 0.84));
            line(point(0.44, 0.84), point(0.44, 0.61));
            line(point(0.56, 0.84), point(0.56, 0.61));
            line(point(0.44, 0.61), point(0.56, 0.61));
        }
        Icon::Activity => {
            closed(vec![
                point(0.25, 0.12),
                point(0.68, 0.12),
                point(0.82, 0.26),
                point(0.82, 0.88),
                point(0.25, 0.88),
            ]);
            line(point(0.68, 0.12), point(0.68, 0.27));
            line(point(0.68, 0.27), point(0.82, 0.27));
            line(point(0.36, 0.45), point(0.70, 0.45));
            line(point(0.36, 0.61), point(0.70, 0.61));
            line(point(0.36, 0.77), point(0.62, 0.77));
        }
        Icon::About | Icon::Info => {
            painter.circle_stroke(point(0.50, 0.50), unit * 0.39, stroke);
            painter.circle_filled(point(0.50, 0.34), stroke.width * 0.72, color);
            line(point(0.50, 0.47), point(0.50, 0.69));
        }
        Icon::Help => {
            painter.circle_stroke(point(0.50, 0.50), unit * 0.39, stroke);
            painter.text(
                point(0.50, 0.50),
                Align2::CENTER_CENTER,
                "?",
                FontId::proportional(unit * 0.60),
                color,
            );
        }
        Icon::Settings => {
            let center = point(0.50, 0.50);
            painter.circle_stroke(center, unit * 0.25, stroke);
            painter.circle_stroke(center, unit * 0.09, stroke);
            for index in 0..8 {
                let angle = std::f32::consts::TAU * index as f32 / 8.0;
                let direction = egui::vec2(angle.cos(), angle.sin());
                line(
                    center + direction * (unit * 0.30),
                    center + direction * (unit * 0.42),
                );
            }
        }
        Icon::Scan => {
            painter.circle_stroke(point(0.42, 0.42), unit * 0.25, stroke);
            line(point(0.60, 0.60), point(0.86, 0.86));
            line(point(0.42, 0.29), point(0.42, 0.55));
            line(point(0.29, 0.42), point(0.55, 0.42));
        }
        Icon::Device => {
            closed(vec![
                point(0.30, 0.10),
                point(0.70, 0.10),
                point(0.77, 0.17),
                point(0.77, 0.83),
                point(0.70, 0.90),
                point(0.30, 0.90),
                point(0.23, 0.83),
                point(0.23, 0.17),
            ]);
            line(point(0.40, 0.20), point(0.60, 0.20));
            painter.circle_filled(point(0.50, 0.81), stroke.width * 0.62, color);
        }
        Icon::Shield => {
            closed(vec![
                point(0.50, 0.10),
                point(0.82, 0.23),
                point(0.78, 0.58),
                point(0.66, 0.77),
                point(0.50, 0.90),
                point(0.34, 0.77),
                point(0.22, 0.58),
                point(0.18, 0.23),
            ]);
            line(point(0.35, 0.50), point(0.46, 0.62));
            line(point(0.46, 0.62), point(0.68, 0.38));
        }
        Icon::Folder => {
            closed(vec![
                point(0.12, 0.28),
                point(0.43, 0.28),
                point(0.52, 0.40),
                point(0.88, 0.40),
                point(0.82, 0.82),
                point(0.16, 0.82),
            ]);
            line(point(0.12, 0.28), point(0.16, 0.82));
        }
        Icon::Reboot => {
            let center = point(0.50, 0.50);
            painter.circle_stroke(center, unit * 0.31, stroke);
            line(point(0.60, 0.13), point(0.83, 0.16));
            line(point(0.83, 0.16), point(0.78, 0.38));
            line(point(0.40, 0.87), point(0.17, 0.84));
            line(point(0.17, 0.84), point(0.22, 0.62));
            line(point(0.50, 0.26), point(0.50, 0.53));
        }
    }
}

fn draw_icon_button(
    ui: &mut egui::Ui,
    kind: Icon,
    label: &str,
    enabled: bool,
    width: f32,
    height: f32,
    primary: bool,
) -> egui::Response {
    let sense = if enabled {
        egui::Sense::click()
    } else {
        egui::Sense::hover()
    };
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), sense);
    let fill = if primary {
        if response.hovered() {
            Color32::from_rgb(91, 218, 192)
        } else {
            GREEN
        }
    } else if !enabled {
        PANEL_DARK
    } else if response.hovered() {
        Color32::from_rgb(49, 55, 64)
    } else {
        Color32::from_rgb(38, 42, 49)
    };
    let text_color = if primary {
        Color32::from_rgb(14, 35, 32)
    } else if enabled {
        TEXT
    } else {
        MUTED
    };
    ui.painter()
        .rect_filled(rect, egui::CornerRadius::same(5), fill);
    ui.painter().rect_stroke(
        rect,
        egui::CornerRadius::same(5),
        Stroke::new(1.0_f32, LINE),
        egui::StrokeKind::Inside,
    );
    paint_icon(
        ui.painter(),
        kind,
        egui::Rect::from_center_size(
            egui::pos2(rect.left() + 25.0, rect.center().y),
            egui::vec2(17.0, 17.0),
        ),
        text_color,
    );
    ui.painter().text(
        egui::pos2(rect.left() + 43.0, rect.center().y),
        Align2::LEFT_CENTER,
        label,
        FontId::proportional(11.0),
        text_color,
    );
    response
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Page {
    Home,
    Log,
    About,
    Help,
}

#[derive(Debug)]
enum UiMessage {
    Scan(ScanResult),
    Info(String, Result<String, String>),
    Reboot(String, Result<String, String>),
    Rom(PathBuf, Result<RomInspection, String>),
    ToolsInstalled(Result<PathBuf, String>),
}

pub struct DroidNowApp {
    tools: ToolLocator,
    tool_status: ToolStatus,
    devices: Vec<Device>,
    selected_index: Option<usize>,
    device_info: Option<(String, Result<String, String>)>,
    rom: Option<(PathBuf, RomInspection)>,
    rom_folder: String,
    log: Vec<String>,
    sender: Sender<UiMessage>,
    receiver: Receiver<UiMessage>,
    scanning: bool,
    rebooting: bool,
    installing_tools: bool,
    show_driver_dialog: bool,
    pending_action: Option<&'static str>,
    page: Page,
}

impl DroidNowApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        apply_visuals(&cc.egui_ctx);
        let (sender, receiver) = mpsc::channel();
        let mut app = Self {
            tools: ToolLocator::default(),
            tool_status: ToolLocator::default().status(),
            devices: Vec::new(),
            selected_index: None,
            device_info: None,
            rom: None,
            rom_folder: String::new(),
            log: vec!["DroidNow siap. Sambungkan ponsel melalui kabel USB.".into()],
            sender,
            receiver,
            scanning: false,
            rebooting: false,
            installing_tools: false,
            show_driver_dialog: false,
            pending_action: None,
            page: Page::Home,
        };
        app.start_scan(cc.egui_ctx.clone());
        app
    }

    fn log(&mut self, message: impl Into<String>) {
        self.log.push(message.into());
        if self.log.len() > 100 {
            self.log.drain(0..self.log.len() - 100);
        }
    }

    fn start_scan(&mut self, ctx: egui::Context) {
        if self.scanning {
            return;
        }
        self.scanning = true;
        let tools = self.tools.clone();
        let sender = self.sender.clone();
        thread::spawn(move || {
            let result = core::scan_devices(&tools);
            let _ = sender.send(UiMessage::Scan(result));
            ctx.request_repaint();
        });
        self.log("Memindai perangkat ADB dan Fastboot…");
    }

    fn start_platform_tools_install(&mut self, ctx: egui::Context) {
        if self.installing_tools || self.scanning || self.rebooting {
            return;
        }
        self.installing_tools = true;
        let sender = self.sender.clone();
        thread::spawn(move || {
            let result = core::install_platform_tools();
            let _ = sender.send(UiMessage::ToolsInstalled(result));
            ctx.request_repaint();
        });
        self.log("Mengunduh Platform-Tools resmi Google melalui HTTPS…");
    }

    fn start_info(&mut self, device: Device, ctx: egui::Context) {
        self.device_info = Some((
            device.serial.clone(),
            Err("Membaca identitas perangkat…".into()),
        ));
        let tools = self.tools.clone();
        let sender = self.sender.clone();
        thread::spawn(move || {
            let result = core::read_device_info(&device, &tools);
            let _ = sender.send(UiMessage::Info(device.serial, result));
            ctx.request_repaint();
        });
    }

    fn start_reboot(&mut self, action: &'static str, device: Device, ctx: egui::Context) {
        self.rebooting = true;
        let tools = self.tools.clone();
        let sender = self.sender.clone();
        thread::spawn(move || {
            let result = core::reboot_device(&device, action, &tools);
            let _ = sender.send(UiMessage::Reboot(action.to_string(), result));
            ctx.request_repaint();
        });
        self.log(format!(
            "Mengirim perintah mulai ulang: {}…",
            action_label(action)
        ));
    }

    fn start_rom_inspection(&mut self, path: PathBuf, ctx: egui::Context) {
        let sender = self.sender.clone();
        thread::spawn(move || {
            let result = core::inspect_rom_folder(&path);
            let _ = sender.send(UiMessage::Rom(path, result));
            ctx.request_repaint();
        });
        self.log("Membaca folder ROM tanpa menjalankan berkas…");
    }

    fn handle_messages(&mut self, ctx: &egui::Context) {
        while let Ok(message) = self.receiver.try_recv() {
            match message {
                UiMessage::Scan(result) => {
                    self.scanning = false;
                    self.tool_status = result.tools;
                    self.devices = result.devices;
                    self.selected_index = if self.devices.is_empty() {
                        None
                    } else {
                        Some(0)
                    };
                    self.device_info = None;
                    if self.devices.is_empty() {
                        self.log("Tidak ditemukan perangkat melalui ADB atau Fastboot.");
                    } else {
                        let summary = self
                            .devices
                            .iter()
                            .map(|d| format!("{} {}", d.mode.to_uppercase(), d.serial))
                            .collect::<Vec<_>>()
                            .join(", ");
                        self.log(format!(
                            "Ditemukan {} perangkat: {summary}",
                            self.devices.len()
                        ));
                    }
                    for message in result.messages {
                        self.log(message);
                    }
                    if let Some(device) = self.selected_device().cloned() {
                        self.start_info(device, ctx.clone());
                    }
                }
                UiMessage::Info(serial, result) => {
                    let still_selected = self
                        .selected_device()
                        .is_some_and(|device| device.serial == serial);
                    if still_selected {
                        match &result {
                            Ok(_) => self.log(format!("Informasi {serial} berhasil dibaca.")),
                            Err(error) if error != "Membaca identitas perangkat…" => {
                                self.log(format!("Info perangkat tidak terbaca: {error}"))
                            }
                            _ => {}
                        }
                        self.device_info = Some((serial, result));
                    }
                }
                UiMessage::Reboot(action, result) => {
                    self.rebooting = false;
                    match result {
                        Ok(message) => {
                            self.log(message);
                            self.log("Tunggu perangkat masuk ke mode tujuan, lalu pindai ulang.");
                        }
                        Err(error) => self.log(format!("Mulai ulang gagal: {error}")),
                    }
                    let _ = action;
                }
                UiMessage::Rom(path, result) => match result {
                    Ok(inspection) => {
                        self.log(format!(
                            "Folder ROM selesai diperiksa: {} — {} berkas terkait.",
                            path.display(),
                            inspection.entries.len()
                        ));
                        self.rom = Some((path, inspection));
                    }
                    Err(error) => self.log(format!("Folder ROM gagal diperiksa: {error}")),
                },
                UiMessage::ToolsInstalled(result) => {
                    self.installing_tools = false;
                    match result {
                        Ok(path) => match self.tools.set_directory(path.clone()) {
                            Ok(()) => {
                                self.tool_status = self.tools.status();
                                self.log(format!(
                                    "Platform-Tools berhasil dipasang di {}.",
                                    path.display()
                                ));
                                self.start_scan(ctx.clone());
                            }
                            Err(error) => self.log(format!(
                                "Platform-Tools terpasang, tetapi lokasinya gagal dipilih: {error}"
                            )),
                        },
                        Err(error) => self.log(format!("Pemasangan Platform-Tools gagal: {error}")),
                    }
                }
            }
        }
    }

    fn selected_device(&self) -> Option<&Device> {
        self.selected_index
            .and_then(|index| self.devices.get(index))
    }

    fn select_device(&mut self, index: Option<usize>, ctx: &egui::Context) {
        if self.selected_index == index {
            return;
        }
        self.selected_index = index;
        self.device_info = None;
        if let Some(device) = self.selected_device().cloned() {
            self.start_info(device, ctx.clone());
        }
    }

    fn choose_tools_folder(&mut self, ctx: &egui::Context) {
        if let Some(path) = rfd::FileDialog::new()
            .set_title("Pilih folder Android SDK Platform-Tools")
            .pick_folder()
        {
            match self.tools.set_directory(path.clone()) {
                Ok(()) => {
                    self.tool_status = self.tools.status();
                    self.log(format!("Folder Platform-Tools diatur: {}", path.display()));
                    self.start_scan(ctx.clone());
                }
                Err(error) => self.log(format!("Folder tools gagal diatur: {error}")),
            }
        }
    }

    fn choose_rom_folder(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .set_title("Pilih folder ROM Fastboot Xiaomi yang sudah diekstrak")
            .pick_folder()
        {
            self.rom_folder = path.display().to_string();
        }
    }

    fn inspect_selected_rom(&mut self, ctx: &egui::Context) {
        let folder = self.rom_folder.trim();
        if folder.is_empty() {
            self.log("Pilih folder ROM yang sudah diekstrak terlebih dahulu.");
            return;
        }
        self.start_rom_inspection(PathBuf::from(folder), ctx.clone());
    }

    fn export_case_archive(&mut self) {
        let Some(path) = rfd::FileDialog::new()
            .set_title("Simpan laporan diagnostik DroidNow")
            .set_file_name("DroidNow-case.json")
            .save_file()
        else {
            return;
        };
        let selected = self.selected_device().cloned();
        let selected_serial = selected.as_ref().map(|device| device.serial.as_str());
        let info = self.device_info.as_ref().and_then(|(serial, result)| {
            (Some(serial.as_str()) == selected_serial)
                .then_some(result.as_ref().ok().map(String::as_str))
                .flatten()
        });
        let rom_inspection = self.rom.as_ref().map(|(_, inspection)| inspection);
        let archive = safety::create_case_archive(
            &self.devices,
            selected.as_ref(),
            info,
            rom_inspection,
            &self.log,
        );
        match safety::write_case_archive(&path, &archive) {
            Ok(()) => self.log(format!(
                "Laporan diagnostik disimpan di {}. Beberapa data sensitif disamarkan; periksa kembali sebelum membagikan.",
                path.display()
            )),
            Err(error) => self.log(error),
        }
    }

    fn draw_header(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("header")
            .frame(
                egui::Frame::new()
                    .fill(Color32::from_rgb(22, 25, 30))
                    .inner_margin(egui::Margin::symmetric(22, 13)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    draw_icon(ui, Icon::Device, 29.0, GREEN);
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("DroidNow").size(21.0).strong().color(TEXT));
                            ui.add_space(3.0);
                            ui.label(
                                RichText::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                                    .size(10.0)
                                    .color(MUTED),
                            );
                        });
                        ui.label(
                            RichText::new("Diagnostik perangkat Android")
                                .size(11.0)
                                .color(MUTED),
                        );
                    });
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if draw_icon_button(
                            ui,
                            Icon::Settings,
                            "Pengaturan",
                            true,
                            137.0,
                            36.0,
                            false,
                        )
                        .clicked()
                        {
                            self.choose_tools_folder(ui.ctx());
                        }
                    });
                });
            });
    }

    fn draw_navigation(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("navigation")
            .exact_width(188.0)
            .resizable(false)
            .frame(
                egui::Frame::new()
                    .fill(PANEL_DARK)
                    .inner_margin(egui::Margin::symmetric(12, 14)),
            )
            .show(ctx, |ui| {
                ui.label(
                    RichText::new("MENU")
                        .size(9.0)
                        .monospace()
                        .strong()
                        .color(MUTED),
                );
                ui.add_space(10.0);
                for (page, label, icon_kind) in [
                    (Page::Home, "Ringkasan", Icon::Home),
                    (Page::Log, "Aktivitas", Icon::Activity),
                    (Page::About, "Tentang", Icon::About),
                    (Page::Help, "Panduan", Icon::Help),
                ] {
                    let selected = self.page == page;
                    let (rect, response) = ui.allocate_exact_size(
                        egui::vec2(ui.available_width(), 40.0),
                        egui::Sense::click(),
                    );
                    let fill = if selected {
                        Color32::from_rgb(34, 91, 83)
                    } else if response.hovered() {
                        Color32::from_rgb(35, 39, 46)
                    } else {
                        PANEL_DARK
                    };
                    ui.painter()
                        .rect_filled(rect, egui::CornerRadius::same(5), fill);
                    ui.painter().rect_stroke(
                        rect,
                        egui::CornerRadius::same(5),
                        Stroke::new(1.0_f32, if selected { GREEN } else { PANEL_DARK }),
                        egui::StrokeKind::Inside,
                    );
                    paint_icon(
                        ui.painter(),
                        icon_kind,
                        egui::Rect::from_center_size(
                            egui::pos2(rect.left() + 24.0, rect.center().y),
                            egui::vec2(18.0, 18.0),
                        ),
                        if selected { GREEN } else { MUTED },
                    );
                    ui.painter().text(
                        egui::pos2(rect.left() + 45.0, rect.center().y),
                        Align2::LEFT_CENTER,
                        label,
                        FontId::proportional(12.0),
                        if selected { TEXT } else { MUTED },
                    );
                    if response.clicked() {
                        self.page = page;
                    }
                    ui.add_space(5.0);
                }
                ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
                    ui.label(
                        RichText::new("Xiaomi Redmi 4A · pemeriksaan terbatas")
                            .size(9.0)
                            .color(AMBER),
                    );
                    ui.label(
                        RichText::new(format!("DroidNow · v{}", env!("CARGO_PKG_VERSION")))
                            .size(9.0)
                            .color(MUTED),
                    );
                });
            });
    }

    fn draw_right_sidebar(&mut self, ctx: &egui::Context) {
        if self.page != Page::Home {
            return;
        }
        egui::SidePanel::right("right_info")
            .exact_width(330.0)
            .resizable(false)
            .frame(egui::Frame::new().fill(BG).inner_margin(egui::Margin::symmetric(9, 15)))
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                    card(ui, |ui| {
                        ui.horizontal(|ui| {
                            draw_icon(ui, Icon::Help, 17.0, BLUE);
                            ui.label(RichText::new("Persiapan").size(14.0).strong().color(TEXT));
                        });
                        ui.add_space(9.0);
                        bullet(ui, "1", "ADB: aktifkan USB debugging dan sambungkan ponsel.");
                        bullet(ui, "2", "Setujui permintaan akses di layar ponsel jika muncul.");
                        bullet(ui, "3", "Fastboot: matikan ponsel, tahan Volume− + Power, lalu sambungkan USB.");
                        ui.add_space(5.0);
                        ui.separator();
                        ui.horizontal(|ui| {
                            draw_icon(ui, Icon::Scan, 16.0, GREEN);
                            ui.label(RichText::new("ADB & Fastboot").size(13.0).strong().color(TEXT));
                        });
                        let tools_ready = self.tool_status.adb_available && self.tool_status.fastboot_available;
                        let tools_color = if tools_ready { GREEN } else { AMBER };
                        let status_text = if tools_ready { "Siap digunakan" } else { "Belum ditemukan" };
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Status").size(10.0).color(MUTED));
                            ui.label(RichText::new(status_text).size(10.0).color(tools_color));
                        });
                        ui.label(RichText::new(&self.tool_status.location).size(9.0).monospace().color(MUTED));
                        if !tools_ready {
                            let missing = [
                                (!self.tool_status.adb_available).then_some("adb"),
                                (!self.tool_status.fastboot_available).then_some("fastboot"),
                            ]
                            .into_iter()
                            .flatten()
                            .collect::<Vec<_>>()
                            .join(", ");
                            ui.label(RichText::new(format!("Belum ditemukan: {missing}")).size(10.0).color(AMBER));
                        }
                        ui.add_space(7.0);
                        ui.label(RichText::new("Jika belum tersedia, gunakan tombol utama untuk menyiapkan ADB dan Fastboot dari Google.").size(9.0).color(MUTED));
                        ui.label(RichText::new("Program dari folder pilihan tidak diperiksa tanda tangan atau hash-nya. Gunakan sumber tepercaya.").size(9.0).color(AMBER));
                        ui.add_space(5.0);
                        if draw_icon_button(ui, Icon::Folder, "Pilih folder Platform-Tools", true, ui.available_width(), 36.0, false).clicked() {
                            self.choose_tools_folder(ui.ctx());
                        }
                        if cfg!(target_os = "windows") && draw_icon_button(ui, Icon::Shield, "Pasang driver USB Windows", true, ui.available_width(), 36.0, false).clicked() {
                            self.show_driver_dialog = true;
                        }
                    });
                    ui.add_space(10.0);
                    card(ui, |ui| {
                        ui.horizontal(|ui| {
                            draw_icon(ui, Icon::Shield, 17.0, AMBER);
                            ui.label(RichText::new("Batasan keamanan").size(14.0).strong().color(TEXT));
                        });
                        ui.add_space(7.0);
                        ui.label(RichText::new("DroidNow hanya membaca informasi perangkat dan dapat memulai ulang setelah Anda menyetujui. Aplikasi tidak memasang firmware, membuka bootloader, menghapus data, atau menulis partisi.").size(10.0).color(MUTED));
                        ui.add_space(7.0);
                        ui.label(RichText::new("Baca & mulai ulang saja").size(10.0).strong().color(AMBER));
                    });
                    ui.add_space(10.0);
                    card(ui, |ui| {
                        ui.horizontal(|ui| {
                            draw_icon(ui, Icon::Activity, 17.0, BLUE);
                            ui.label(RichText::new("Aktivitas terbaru").size(14.0).strong().color(TEXT));
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                if ui.button(RichText::new("Lihat semua").size(10.0).color(GREEN)).clicked() {
                                    self.page = Page::Log;
                                }
                            });
                        });
                        ui.add_space(4.0);
                        for (index, entry) in self.log.iter().rev().take(4).enumerate() {
                            let color = if entry.contains("gagal") || entry.contains("tidak ditemukan") {
                                RED
                            } else if entry.contains("berhasil") || entry.contains("Ditemukan") {
                                GREEN
                            } else {
                                MUTED
                            };
                            ui.horizontal_top(|ui| {
                                ui.label(RichText::new(format!("{:02}", self.log.len().saturating_sub(index))).size(9.0).monospace().color(MUTED));
                                ui.label(RichText::new(entry).size(9.0).color(color));
                            });
                            ui.add_space(4.0);
                        }
                    });
                });
            });
    }

    fn draw_connection_banner(&self, ui: &mut egui::Ui) {
        let (title, message, color) = match self.selected_device() {
            None => (
                "Belum ada perangkat terhubung",
                "Sambungkan ponsel melalui USB. DroidNow akan memindai koneksi ADB dan Fastboot.",
                MUTED,
            ),
            Some(device) if device.mode == "adb" && device.state == "unauthorized" => (
                "Menunggu izin USB debugging",
                "Buka layar ponsel, lalu setujui permintaan akses dari komputer ini.",
                AMBER,
            ),
            Some(device) if device.mode == "adb" && device.state == "offline" => (
                "Perangkat tidak merespons",
                "Periksa kabel atau port USB, lalu pindai kembali.",
                AMBER,
            ),
            Some(device) if device.mode == "fastboot" => (
                "Fastboot terhubung",
                "Perangkat ditemukan dalam mode Fastboot.",
                GREEN,
            ),
            Some(_) => (
                "Perangkat terhubung",
                "Koneksi siap untuk membaca informasi dasar perangkat.",
                GREEN,
            ),
        };
        let available_width = ui.available_width();
        egui::Frame::new()
            .fill(Color32::from_rgb(32, 35, 42))
            .stroke(Stroke::new(1.0_f32, LINE))
            .inner_margin(egui::Margin::symmetric(14, 11))
            .show(ui, |ui| {
                ui.set_min_width((available_width - 28.0).max(0.0));
                ui.horizontal(|ui| {
                    draw_icon(ui, Icon::Info, 21.0, color);
                    ui.vertical(|ui| {
                        ui.label(RichText::new(title).size(12.0).strong().color(TEXT));
                        ui.label(RichText::new(message).size(10.0).color(MUTED));
                    });
                });
            });
    }

    fn draw_device_card(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        card(ui, |ui| {
            ui.horizontal(|ui| {
                draw_icon(ui, Icon::Device, 18.0, BLUE);
                ui.label(
                    RichText::new("Informasi perangkat")
                        .size(14.0)
                        .strong()
                        .color(TEXT),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if self.devices.len() > 1 {
                        let selected_label = self
                            .selected_device()
                            .map(|device| {
                                format!("{} · {}", device.mode.to_uppercase(), device.serial)
                            })
                            .unwrap_or_else(|| "Pilih perangkat".into());
                        let mut selected = self.selected_index;
                        egui::ComboBox::from_id_salt("device-picker")
                            .selected_text(RichText::new(selected_label).size(9.0).color(TEXT))
                            .show_ui(ui, |ui| {
                                for (index, device) in self.devices.iter().enumerate() {
                                    ui.selectable_value(
                                        &mut selected,
                                        Some(index),
                                        format!(
                                            "{} · {} · {}",
                                            device.mode.to_uppercase(),
                                            device.serial,
                                            device.state
                                        ),
                                    );
                                }
                            });
                        if selected != self.selected_index {
                            self.select_device(selected, ctx);
                        }
                    } else {
                        ui.label(RichText::new("Android").size(9.0).color(MUTED));
                    }
                });
            });
            ui.add_space(9.0);
            let mut values = std::collections::HashMap::<String, String>::new();
            if let (Some(device), Some((serial, Ok(info)))) =
                (self.selected_device(), self.device_info.as_ref())
            {
                if &device.serial == serial {
                    for line in info.lines() {
                        if let Some((key, value)) = line.split_once(':') {
                            values
                                .insert(key.trim().to_ascii_lowercase(), value.trim().to_string());
                        }
                    }
                }
            }
            let model = values.get("model").cloned().unwrap_or_else(|| "—".into());
            let codename = values
                .get("codename")
                .or_else(|| values.get("product"))
                .cloned()
                .unwrap_or_else(|| "—".into());
            let android = values.get("android").cloned().unwrap_or_else(|| "—".into());
            let bootloader = values
                .get("bootloader unlocked")
                .cloned()
                .unwrap_or_else(|| "—".into());
            let serial = self
                .selected_device()
                .map(|device| device.serial.clone())
                .unwrap_or_else(|| "—".into());
            let connection = self
                .selected_device()
                .map(|device| format!("{} · {}", device.mode.to_uppercase(), device.state))
                .unwrap_or_else(|| "—".into());
            ui.columns(2, |columns| {
                kv_row(&mut columns[0], "Model", &model);
                kv_row(&mut columns[0], "Kode perangkat", &codename);
                kv_row(&mut columns[0], "Versi Android", &android);
                kv_row(&mut columns[1], "Status Bootloader", &bootloader);
                kv_row(&mut columns[1], "Nomor Seri", &serial);
                kv_row(&mut columns[1], "Koneksi", &connection);
            });
            if let Some((info_serial, Ok(info))) = self.device_info.as_ref() {
                if self
                    .selected_device()
                    .is_some_and(|device| device.serial == *info_serial)
                {
                    egui::CollapsingHeader::new("Detail teknis perangkat")
                        .default_open(false)
                        .show(ui, |ui| {
                            ui.label(RichText::new("Nilai kosong atau gagal dibaca tidak membuktikan perangkat aman.").size(9.0).color(AMBER));
                            for line in info.lines() {
                                ui.label(RichText::new(line).size(9.0).monospace().color(MUTED));
                            }
                        });
                }
            }
            if let Some(device) = self.selected_device() {
                if device.mode == "adb" && device.state != "device" {
                    ui.add_space(5.0);
                    ui.label(
                        RichText::new(if device.state == "unauthorized" {
                            "Buka layar ponsel dan setujui permintaan USB debugging."
                        } else {
                            "Coba kabel atau port USB lain, lalu sambungkan kembali."
                        })
                        .size(9.0)
                        .color(AMBER),
                    );
                }
            }
        });
    }

    fn draw_actions(&mut self, ui: &mut egui::Ui) {
        card(ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("Mulai ulang perangkat")
                            .size(14.0)
                            .strong()
                            .color(TEXT),
                    );
                    ui.label(
                        RichText::new(
                            "Konfirmasi tetap diperlukan sebelum perangkat dimulai ulang.",
                        )
                        .size(10.0)
                        .color(MUTED),
                    );
                });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(
                        RichText::new("Aksi yang tersedia")
                            .size(9.0)
                            .monospace()
                            .strong()
                            .color(GREEN),
                    );
                });
            });
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                let button_width = ((ui.available_width() - 16.0) / 3.0).max(120.0);
                for (label, action, icon_kind) in [
                    ("Mulai ulang ke Android", "system", Icon::Reboot),
                    ("Buka Recovery", "recovery", Icon::Info),
                    ("Buka Fastboot", "bootloader", Icon::Scan),
                ] {
                    let enabled = self.selected_device().is_some_and(|device| {
                        ((device.mode == "adb" && device.state == "device")
                            || (device.mode == "fastboot" && action == "system"))
                            && !self.rebooting
                    });
                    if draw_icon_button(ui, icon_kind, label, enabled, button_width, 42.0, false)
                        .clicked()
                    {
                        self.pending_action = Some(action);
                    }
                }
            });
        });
    }

    fn draw_rom_card(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        card(ui, |ui| {
            ui.horizontal(|ui| {
                draw_icon(ui, Icon::Folder, 18.0, BLUE);
                ui.label(
                    RichText::new("Periksa berkas ROM lokal")
                        .size(14.0)
                        .strong()
                        .color(TEXT),
                );
            });
            ui.label(
                RichText::new(
                    "Menghitung SHA-256 berkas lokal. Skrip tidak dijalankan dan ponsel tidak diubah.",
                )
                .size(10.0)
                .color(MUTED),
            );
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                let browse_width = 146.0;
                let inspect_width = 144.0;
                let text_width =
                    (ui.available_width() - browse_width - inspect_width - 16.0).max(120.0);
                ui.add_sized(
                    [text_width, 32.0],
                    egui::TextEdit::singleline(&mut self.rom_folder)
                        .hint_text("Pilih folder ROM yang sudah diekstrak"),
                );
                if draw_icon_button(
                    ui,
                    Icon::Folder,
                    "Pilih folder…",
                    true,
                    browse_width,
                    36.0,
                    false,
                )
                .clicked()
                {
                    self.choose_rom_folder();
                }
                if draw_icon_button(
                    ui,
                    Icon::Scan,
                    "Periksa berkas",
                    true,
                    inspect_width,
                    36.0,
                    false,
                )
                .clicked()
                {
                    self.inspect_selected_rom(ctx);
                }
            });
            ui.add_space(7.0);
            if let Some((_path, inspection)) = &self.rom {
                let hint_color = if inspection.codename_hint == "rolex" {
                    GREEN
                } else {
                    AMBER
                };
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new(&inspection.folder_name)
                            .size(10.0)
                            .strong()
                            .monospace()
                            .color(TEXT),
                    );
                    ui.label(
                        RichText::new(format!(
                            "Petunjuk kode perangkat: {}",
                            inspection.codename_hint
                        ))
                        .size(9.0)
                        .monospace()
                        .color(hint_color),
                    );
                });
                ui.label(
                    RichText::new(format!(
                        "{} berkas terkait · {}",
                        inspection.entries.len(),
                        inspection.script_hint
                    ))
                    .size(9.0)
                    .color(MUTED),
                );
                ui.label(
                    RichText::new(if inspection.verified_catalog_match {
                        "Sumber resmi dan kecocokan firmware: terverifikasi"
                    } else {
                        "Sumber resmi dan kecocokan firmware: belum dapat dipastikan"
                    })
                    .size(9.0)
                    .color(AMBER),
                );
                egui::ScrollArea::vertical()
                    .max_height(78.0)
                    .show(ui, |ui| {
                        for entry in inspection.entries.iter().take(12) {
                            let kind_color = if entry.kind == "IMAGE" { BLUE } else { AMBER };
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new(&entry.kind)
                                        .size(8.0)
                                        .monospace()
                                        .color(kind_color),
                                );
                                ui.label(
                                    RichText::new(&entry.name).size(9.0).monospace().color(TEXT),
                                );
                            });
                            let hash_text = entry.sha256.as_deref().map_or_else(
                                || {
                                    entry
                                        .hash_error
                                        .as_deref()
                                        .unwrap_or("SHA-256 tidak tersedia")
                                        .to_string()
                                },
                                |hash| format!("{} B · SHA-256 {hash}", entry.size_bytes),
                            );
                            ui.label(RichText::new(hash_text).size(8.0).monospace().color(MUTED));
                        }
                    });
                ui.label(RichText::new(&inspection.warning).size(10.0).color(AMBER));
            } else {
                ui.label(RichText::new("Nama, ukuran, dan SHA-256 lokal dapat diperiksa. Hash ini tidak memastikan sumber resmi atau kecocokan firmware.").size(10.0).color(MUTED));
            }
        });
    }

    fn draw_confirm_dialog(&mut self, ctx: &egui::Context) {
        let Some(action) = self.pending_action else {
            return;
        };
        let device = self.selected_device().cloned();
        let mut confirm = false;
        let mut cancel = false;
        egui::Window::new("Konfirmasi mulai ulang")
            .id(egui::Id::new("reboot-confirm"))
            .collapsible(false)
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.set_min_width(330.0);
                ui.label(
                    RichText::new("KONFIRMASI")
                        .size(9.0)
                        .monospace()
                        .strong()
                        .color(AMBER),
                );
                ui.add_space(5.0);
                ui.label(
                    RichText::new(action_prompt(action))
                        .size(16.0)
                        .strong()
                        .color(TEXT),
                );
                ui.label(
                    RichText::new(action_description(action))
                        .size(10.0)
                        .color(MUTED),
                );
                if let Some(device) = &device {
                    ui.add_space(7.0);
                    ui.label(
                        RichText::new(format!(
                            "{}  ·  {}",
                            device.mode.to_uppercase(),
                            device.serial
                        ))
                        .size(10.0)
                        .monospace()
                        .color(BLUE),
                    );
                }
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    if ui.button("Batal").clicked() {
                        cancel = true;
                    }
                    if ui
                        .button(
                            RichText::new("Mulai ulang sekarang")
                                .strong()
                                .color(Color32::from_rgb(8, 31, 25)),
                        )
                        .clicked()
                    {
                        confirm = true;
                    }
                });
            });
        if cancel {
            self.pending_action = None;
        }
        if confirm {
            self.pending_action = None;
            if let Some(device) = device {
                self.start_reboot(action, device, ctx.clone());
            }
        }
    }
}

impl eframe::App for DroidNowApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_messages(ctx);
        self.draw_header(ctx);
        self.draw_navigation(ctx);
        self.draw_right_sidebar(ctx);
        egui::TopBottomPanel::bottom("footer")
            .frame(
                egui::Frame::new()
                    .fill(Color32::from_rgb(22, 25, 30))
                    .inner_margin(egui::Margin::symmetric(20, 7)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let app_status = if self.scanning {
                        "Memindai…"
                    } else if self.installing_tools {
                        "Menyiapkan ADB/Fastboot…"
                    } else {
                        "Siap"
                    };
                    ui.label(RichText::new("Status aplikasi:").size(9.0).color(MUTED));
                    ui.label(RichText::new(app_status).size(9.0).strong().color(GREEN));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let fastboot_connected =
                            self.devices.iter().any(|device| device.mode == "fastboot");
                        let adb_connected = self
                            .devices
                            .iter()
                            .any(|device| device.mode == "adb" && device.state == "device");
                        let fastboot_color = if fastboot_connected { GREEN } else { MUTED };
                        let adb_color = if adb_connected { GREEN } else { MUTED };
                        ui.label(
                            RichText::new(format!(
                                "Fastboot: {}",
                                if fastboot_connected {
                                    "Aktif"
                                } else {
                                    "Tidak terhubung"
                                }
                            ))
                            .size(9.0)
                            .color(fastboot_color),
                        );
                        ui.separator();
                        ui.label(
                            RichText::new(format!(
                                "ADB: {}",
                                if adb_connected {
                                    "Terhubung"
                                } else {
                                    "Tidak terhubung"
                                }
                            ))
                            .size(9.0)
                            .color(adb_color),
                        );
                    });
                });
            });
        let mut request_case_export = false;
        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(BG)
                    .inner_margin(egui::Margin::symmetric(18, 18)),
            )
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    match self.page {
                        Page::Home => {
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.label(RichText::new("DIAGNOSTIK ANDROID").size(10.0).monospace().strong().color(MUTED));
                                    ui.heading(RichText::new("Perangkat & koneksi").size(24.0).strong().color(TEXT));
                                    ui.label(RichText::new("Periksa koneksi dan informasi perangkat. Pemeriksaan model terbatas pada Xiaomi Redmi 4A.").size(11.0).color(MUTED));
                                });
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    let tools_ready = self.tool_status.adb_available && self.tool_status.fastboot_available;
                                    let text = if self.scanning {
                                        "Memindai…"
                                    } else if self.installing_tools {
                                        "Menyiapkan…"
                                    } else if tools_ready {
                                        "Pindai perangkat"
                                    } else {
                                        "Siapkan ADB/Fastboot"
                                    };
                                    let enabled = !self.scanning && !self.installing_tools && !self.rebooting;
                                    if draw_icon_button(ui, Icon::Scan, text, enabled, 198.0, 40.0, true).clicked() {
                                        if tools_ready {
                                            self.start_scan(ctx.clone());
                                        } else {
                                            self.start_platform_tools_install(ctx.clone());
                                        }
                                    }
                                });
                            });
                            ui.add_space(10.0);
                            self.draw_connection_banner(ui);
                            ui.add_space(9.0);
                            self.draw_device_card(ui, ctx);
                            ui.add_space(9.0);
                            self.draw_actions(ui);
                            ui.add_space(9.0);
                            self.draw_rom_card(ui, ctx);
                            ui.add_space(8.0);
                            egui::Frame::new()
                                .fill(Color32::from_rgb(31, 40, 54))
                                .stroke(Stroke::new(1.0_f32, Color32::from_rgb(55, 70, 92)))
                                .inner_margin(egui::Margin::symmetric(11, 7))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        draw_icon(ui, Icon::Shield, 17.0, BLUE);
                                        ui.label(RichText::new("Pemeriksaan terbatas: Xiaomi Redmi 4A (rolex). DroidNow tidak memasang firmware, membuka bootloader, menghapus data, atau menulis partisi.").size(10.0).color(TEXT));
                                    });
                                });
                        }
                        Page::Log => {
                            page_heading(ui, "Riwayat aktivitas", "Catatan sesi ini tersimpan di komputer dan tidak dikirim secara otomatis.");
                            ui.add_space(10.0);
                            card(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new(format!("{} catatan aktivitas", self.log.len())).size(10.0).color(MUTED));
                                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                        if ui.button("Ekspor laporan…").clicked() {
                                            request_case_export = true;
                                        }
                                    });
                                });
                                ui.add_space(8.0);
                                egui::ScrollArea::vertical().max_height(620.0).show(ui, |ui| {
                                    for (index, entry) in self.log.iter().enumerate().rev() {
                                        let color = if entry.contains("gagal") || entry.contains("tidak ditemukan") { RED } else if entry.contains("berhasil") || entry.contains("Ditemukan") { GREEN } else { MUTED };
                                        ui.horizontal_top(|ui| {
                                            ui.label(RichText::new(format!("{:03}", index + 1)).size(9.0).monospace().color(MUTED));
                                            ui.label(RichText::new(entry).size(10.0).color(color));
                                        });
                                        ui.separator();
                                    }
                                });
                            });
                        }
                        Page::About => {
                            page_heading(ui, "Tentang DroidNow", "Diagnostik perangkat Android.");
                            ui.add_space(10.0);
                            card(ui, |ui| {
                                ui.label(RichText::new(format!("DroidNow v{}", env!("CARGO_PKG_VERSION"))).size(15.0).strong().color(TEXT));
                                ui.add_space(6.0);
                                ui.label(RichText::new("DroidNow membaca informasi perangkat dan memeriksa berkas lokal tanpa memasang apa pun ke ponsel.").size(11.0).color(MUTED));
                                ui.add_space(10.0);
                                ui.label(RichText::new("Fungsi").size(11.0).strong().color(TEXT));
                                ui.label(RichText::new("Memindai koneksi ADB/Fastboot, membaca informasi perangkat, menghitung SHA-256 berkas terpilih, dan mengekspor laporan lokal dengan penyamaran data tertentu.").size(10.0).color(MUTED));
                                ui.add_space(10.0);
                                ui.label(RichText::new("Perangkat yang didukung").size(11.0).strong().color(TEXT));
                                ui.label(RichText::new("Pemeriksaan terbatas pada Xiaomi Redmi 4A (rolex). Perangkat lain dapat muncul dalam pemindaian, tetapi belum divalidasi sebagai model yang didukung.").size(10.0).color(MUTED));
                                ui.add_space(10.0);
                                ui.label(RichText::new("Keamanan").size(11.0).strong().color(AMBER));
                                ui.label(RichText::new("DroidNow tidak memasang firmware, membuka atau mengunci bootloader, menghapus data, atau menulis partisi. Perintah mulai ulang selalu meminta konfirmasi.").size(10.0).color(MUTED));
                            });
                        }
                        Page::Help => {
                            page_heading(ui, "Mulai cepat", "Siapkan koneksi, lalu tinjau hasil pemeriksaan.");
                            ui.add_space(10.0);
                            card(ui, |ui| {
                                help_step(ui, "1", "Siapkan ADB dan Fastboot", "Jika belum tersedia, tekan Siapkan ADB/Fastboot di halaman utama. DroidNow mengunduh tools resmi Google dan memindai otomatis setelah selesai.");
                                help_step(ui, "2", "Sambungkan ponsel", "Untuk ADB, aktifkan USB debugging dan setujui permintaan akses. Untuk Fastboot, matikan ponsel lalu tahan Volume− + Power.");
                                help_step(ui, "3", "Tinjau hasil", "DroidNow memindai saat dibuka dan setelah menyiapkan tools. Tekan Pindai perangkat untuk memindai lagi.");
                            });
                            ui.add_space(9.0);
                            card(ui, |ui| {
                                ui.horizontal(|ui| {
                                    draw_icon(ui, Icon::Shield, 17.0, AMBER);
                                    ui.label(RichText::new("Catatan keamanan").size(13.0).strong().color(AMBER));
                                });
                                ui.label(RichText::new("Hash berkas lokal tidak memastikan sumber resmi, model, wilayah, atau keamanan firmware.").size(10.0).color(MUTED));
                                ui.add_space(5.0);
                                ui.label(RichText::new("Jangan gunakan hasil pemeriksaan sebagai izin untuk memasang firmware atau mengubah perangkat.").size(10.0).color(AMBER));
                            });
                        }
                    }
                });
            });
        if request_case_export {
            self.export_case_archive();
        }
        self.draw_confirm_dialog(ctx);
        self.draw_driver_dialog(ctx);
    }
}

fn card<R>(ui: &mut egui::Ui, content: impl FnOnce(&mut egui::Ui) -> R) -> R {
    egui::Frame::group(ui.style())
        .fill(PANEL)
        .stroke(Stroke::new(1.0_f32, LINE))
        .inner_margin(egui::Margin::same(14))
        .show(ui, content)
        .inner
}

fn kv_row(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.add_sized(
            [112.0, 18.0],
            egui::Label::new(RichText::new(label).size(10.0).color(MUTED)),
        );
        ui.label(RichText::new(value).size(10.0).color(TEXT));
    });
}

fn page_heading(ui: &mut egui::Ui, title: &str, subtitle: &str) {
    ui.label(
        RichText::new("DROIDNOW")
            .size(9.0)
            .monospace()
            .strong()
            .color(MUTED),
    );
    ui.heading(RichText::new(title).size(23.0).strong().color(TEXT));
    ui.label(RichText::new(subtitle).size(10.0).color(MUTED));
}

fn help_step(ui: &mut egui::Ui, number: &str, title: &str, description: &str) {
    ui.horizontal_top(|ui| {
        ui.label(
            RichText::new(number)
                .size(10.0)
                .monospace()
                .strong()
                .color(GREEN),
        );
        ui.vertical(|ui| {
            ui.label(RichText::new(title).size(10.0).strong().color(TEXT));
            ui.label(RichText::new(description).size(10.0).color(MUTED));
        });
    });
    ui.add_space(9.0);
}

fn bullet(ui: &mut egui::Ui, number: &str, text: &str) {
    ui.horizontal_top(|ui| {
        ui.label(
            RichText::new(number)
                .size(10.0)
                .monospace()
                .strong()
                .color(BLUE),
        );
        ui.label(RichText::new(text).size(10.0).color(MUTED));
    });
    ui.add_space(6.0);
}

fn apply_visuals(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = BG;
    visuals.window_fill = PANEL;
    visuals.extreme_bg_color = PANEL_DARK;
    visuals.faint_bg_color = Color32::from_rgb(22, 32, 44);
    visuals.selection.bg_fill = Color32::from_rgb(37, 89, 75);
    visuals.selection.stroke = Stroke::new(1.0_f32, GREEN);
    visuals.widgets.noninteractive.bg_fill = PANEL;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, TEXT);
    visuals.widgets.inactive.bg_fill = Color32::from_rgb(34, 38, 45);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, TEXT);
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(49, 55, 64);
    visuals.widgets.active.bg_fill = Color32::from_rgb(42, 91, 81);
    visuals.widgets.open.bg_fill = Color32::from_rgb(38, 44, 52);
    visuals.window_stroke = Stroke::new(1.0_f32, LINE);
    ctx.set_visuals(visuals);
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(8.0, 7.0);
    style
        .text_styles
        .insert(egui::TextStyle::Body, FontId::proportional(11.0));
    ctx.set_style(style);
}

fn action_label(action: &str) -> &'static str {
    match action {
        "recovery" => "membuka Recovery",
        "bootloader" => "membuka Fastboot",
        _ => "memulai ulang ke Android",
    }
}

fn action_prompt(action: &str) -> &'static str {
    match action {
        "recovery" => "Masuk ke Recovery?",
        "bootloader" => "Masuk ke Fastboot?",
        _ => "Mulai ulang perangkat?",
    }
}

impl DroidNowApp {
    fn draw_driver_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_driver_dialog || !cfg!(target_os = "windows") {
            return;
        }
        const XIAOMI_DRIVER_GUIDE: &str = "https://new.c.mi.com/mx/post/11443";
        let mut close = false;
        let mut open_guide = false;
        egui::Window::new("Driver USB Xiaomi untuk Windows")
            .id(egui::Id::new("xiaomi-driver-guide"))
            .collapsible(false)
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.set_min_width(390.0);
                ui.label(RichText::new("Pemasangan driver memerlukan tindakan Anda di Windows.").size(13.0).strong().color(TEXT));
                ui.add_space(6.0);
                ui.label(RichText::new("DroidNow tidak mengunduh atau menjalankan installer driver secara otomatis. Buka panduan Xiaomi resmi untuk paket MiFlash dan instruksi memasang ulang driver melalui menu Driver.").size(10.0).color(MUTED));
                ui.add_space(7.0);
                ui.label(RichText::new("Jalankan MiFlash secara manual dan setujui prompt Administrator/UAC Windows sendiri hanya jika Anda ingin memasang driver. Jangan lanjutkan bila sumber atau penerbit paket tidak cocok.").size(10.0).color(AMBER));
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if ui.button("Tutup").clicked() {
                        close = true;
                    }
                    if ui.button(RichText::new("Buka panduan resmi Xiaomi").strong()).clicked() {
                        open_guide = true;
                    }
                });
            });
        if close {
            self.show_driver_dialog = false;
        }
        if open_guide {
            match webbrowser::open(XIAOMI_DRIVER_GUIDE) {
                Ok(()) => self.log("Panduan driver resmi Xiaomi dibuka di browser; pemasangan tetap dilakukan manual di Windows."),
                Err(error) => self.log(format!("Panduan Xiaomi gagal dibuka: {error}")),
            }
        }
    }
}

fn action_description(action: &str) -> &'static str {
    match action {
        "recovery" => {
            "Perangkat akan dimulai ulang ke Recovery. Tindakan ini tidak memasang atau menghapus apa pun."
        }
        "bootloader" => {
            "Perangkat akan dimulai ulang ke Fastboot. Tindakan ini tidak membuka kunci bootloader."
        }
        _ => "Perangkat akan dimulai ulang ke Android. Simpan pekerjaan yang sedang berlangsung.",
    }
}
