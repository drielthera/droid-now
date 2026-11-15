use crate::core::{self, Device, RomInspection, ScanResult, ToolLocator, ToolStatus};
use crate::i18n::{self, Locale};
use crate::safety;
use crate::usb_modes::{self, DetectedUsbMode};
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
const AMBER: Color32 = Color32::from_rgb(225, 187, 112);
const RED: Color32 = Color32::from_rgb(235, 127, 136);
const CONTROL_EDGE: Color32 = Color32::from_rgb(120, 132, 148);

#[derive(Clone, Copy)]
enum Icon {
    Home,
    Activity,
    About,
    Help,
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
    let text_color = if primary {
        Color32::from_rgb(14, 35, 32)
    } else if enabled {
        TEXT
    } else {
        MUTED
    };
    let mut button = egui::Button::new(RichText::new(label).size(11.0).color(text_color))
        .min_size(egui::vec2(width, height))
        .corner_radius(egui::CornerRadius::same(5));
    if primary {
        button = button.fill(GREEN);
    } else if !enabled {
        button = button.fill(PANEL_DARK);
    }
    let response = ui.add_enabled(enabled, button);
    paint_icon(
        ui.painter(),
        kind,
        egui::Rect::from_center_size(
            egui::pos2(response.rect.left() + 18.0, response.rect.center().y),
            egui::vec2(17.0, 17.0),
        ),
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
    Reboot(Result<String, String>),
    Rom(PathBuf, Result<RomInspection, String>),
    ToolsInstalled(Result<PathBuf, String>),
    UsbModes(Result<Vec<DetectedUsbMode>, String>),
}

pub struct DroidNowApp {
    tools: ToolLocator,
    tool_status: ToolStatus,
    locale: Locale,
    devices: Vec<Device>,
    usb_modes: Vec<DetectedUsbMode>,
    usb_scanning: bool,
    usb_scan_error: Option<String>,
    selected_index: Option<usize>,
    device_info: Option<(String, Result<String, String>)>,
    rom: Option<(PathBuf, RomInspection)>,
    rom_folder: String,
    inspecting_rom: bool,
    rom_error: Option<String>,
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
            locale: Locale::default(),
            devices: Vec::new(),
            usb_modes: Vec::new(),
            usb_scanning: false,
            usb_scan_error: None,
            selected_index: None,
            device_info: None,
            rom: None,
            rom_folder: String::new(),
            inspecting_rom: false,
            rom_error: None,
            log: Vec::new(),
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
        self.usb_scanning = true;
        self.usb_scan_error = None;
        let usb_sender = self.sender.clone();
        let usb_ctx = ctx.clone();
        thread::spawn(move || {
            let result = usb_modes::scan_usb_modes();
            let _ = usb_sender.send(UiMessage::UsbModes(result));
            usb_ctx.request_repaint();
        });
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
            let _ = sender.send(UiMessage::Reboot(result));
            ctx.request_repaint();
        });
        self.log(format!(
            "Mengirim perintah mulai ulang: {}…",
            action_label(action)
        ));
    }

    fn start_rom_inspection(&mut self, path: PathBuf, ctx: egui::Context) {
        self.inspecting_rom = true;
        self.rom = None;
        self.rom_error = None;
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
                UiMessage::UsbModes(result) => {
                    self.usb_scanning = false;
                    match result {
                        Ok(modes) => {
                            self.usb_modes = modes;
                            self.usb_scan_error = None;
                        }
                        Err(error) => {
                            self.usb_modes.clear();
                            self.usb_scan_error = Some(error);
                        }
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
                UiMessage::Reboot(result) => {
                    self.rebooting = false;
                    match result {
                        Ok(message) => {
                            self.log(message);
                            self.log("Tunggu perangkat masuk ke mode tujuan, lalu pindai ulang.");
                        }
                        Err(error) => self.log(format!("Mulai ulang gagal: {error}")),
                    }
                }
                UiMessage::Rom(path, result) => {
                    self.inspecting_rom = false;
                    match result {
                        Ok(inspection) => {
                            self.log(format!(
                                "Folder ROM selesai diperiksa: {}. Ditemukan {} berkas terkait.",
                                path.display(),
                                inspection.entries.len()
                            ));
                            self.rom = Some((path, inspection));
                        }
                        Err(error) => {
                            self.rom_error = Some(error.clone());
                            self.log(format!("Pemeriksaan folder gagal: {error}"));
                        }
                    }
                }
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
            .set_title(i18n::tr(
                self.locale,
                "Pilih folder Android SDK Platform-Tools",
            ))
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
            .set_title(i18n::tr(self.locale, "Pilih folder ROM hasil ekstrak"))
            .pick_folder()
        {
            self.rom_folder = path.display().to_string();
            self.rom = None;
            self.rom_error = None;
        }
    }

    fn inspect_selected_rom(&mut self, ctx: &egui::Context) {
        let folder = self.rom_folder.trim();
        if folder.is_empty() {
            let message = "Pilih folder ROM sebelum memulai pemeriksaan.".to_string();
            self.rom_error = Some(message.clone());
            self.log(message);
            return;
        }
        self.start_rom_inspection(PathBuf::from(folder), ctx.clone());
    }

    fn export_case_archive(&mut self) {
        let Some(path) = rfd::FileDialog::new()
            .set_title(i18n::tr(self.locale, "Simpan laporan diagnostik DroidNow"))
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
                        ui.label(RichText::new("DroidNow").size(21.0).strong().color(TEXT));
                        ui.label(
                            RichText::new("Your devices. One solution")
                                .size(11.0)
                                .color(MUTED),
                        );
                    });
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let mut locale = self.locale;
                        egui::ComboBox::from_id_salt("locale-switcher")
                            .selected_text(locale.tag())
                            .width(74.0)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut locale,
                                    Locale::Indonesian,
                                    "Bahasa Indonesia",
                                );
                                ui.selectable_value(&mut locale, Locale::English, "English");
                            });
                        self.locale = locale;
                        if draw_icon_button(
                            ui,
                            Icon::Folder,
                            i18n::tr(self.locale, "Pilih tools"),
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
                for (page, label, icon_kind) in [
                    (Page::Home, i18n::tr(self.locale, "Ringkasan"), Icon::Home),
                    (
                        Page::Log,
                        i18n::tr(self.locale, "Aktivitas"),
                        Icon::Activity,
                    ),
                    (Page::About, i18n::tr(self.locale, "Tentang"), Icon::About),
                    (Page::Help, i18n::tr(self.locale, "Panduan"), Icon::Help),
                ] {
                    let selected = self.page == page;
                    let width = ui.available_width();
                    let response = ui.add_sized(
                        [width, 40.0],
                        egui::Button::selectable(
                            selected,
                            RichText::new(label).size(12.0).color(if selected {
                                TEXT
                            } else {
                                MUTED
                            }),
                        )
                        .corner_radius(egui::CornerRadius::same(5)),
                    );
                    paint_icon(
                        ui.painter(),
                        icon_kind,
                        egui::Rect::from_center_size(
                            egui::pos2(response.rect.left() + 38.0, response.rect.center().y),
                            egui::vec2(18.0, 18.0),
                        ),
                        if selected { GREEN } else { MUTED },
                    );
                    if response.clicked() {
                        self.page = page;
                    }
                    ui.add_space(5.0);
                }
            });
    }

    fn draw_connection_banner(&self, ui: &mut egui::Ui) {
        let (title, message, color) = match self.selected_device() {
            None if self.scanning => (
                i18n::tr(self.locale, "Memindai perangkat"),
                i18n::tr(self.locale, "Menunggu hasil ADB/Fastboot."),
                MUTED,
            ),
            None if !self.tool_status.adb_available && !self.tool_status.fastboot_available => (
                i18n::tr(self.locale, "ADB/Fastboot tidak ditemukan"),
                i18n::tr(
                    self.locale,
                    "Siapkan tools atau pilih folder yang sudah ada.",
                ),
                AMBER,
            ),
            None => (
                i18n::tr(self.locale, "Tidak ada perangkat"),
                i18n::tr(self.locale, "Sambungkan USB, lalu pindai."),
                MUTED,
            ),
            Some(device) if device.mode == "adb" && device.state == "unauthorized" => (
                i18n::tr(self.locale, "Izin USB debugging diperlukan"),
                i18n::tr(self.locale, "Setujui permintaan akses di perangkat."),
                AMBER,
            ),
            Some(device) if device.mode == "adb" && device.state == "offline" => (
                i18n::tr(self.locale, "Perangkat offline"),
                i18n::tr(self.locale, "Periksa kabel atau port, lalu pindai."),
                AMBER,
            ),
            Some(device) if device.mode == "fastboot" => {
                (i18n::tr(self.locale, "Fastboot terhubung"), "", GREEN)
            }
            Some(_) => (i18n::tr(self.locale, "Perangkat terhubung"), "", GREEN),
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
                        if !message.is_empty() {
                            ui.label(RichText::new(message).size(10.0).color(MUTED));
                        }
                    });
                });
            });
    }

    fn draw_device_card(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        card(ui, |ui| {
            ui.horizontal(|ui| {
                draw_icon(ui, Icon::Device, 18.0, MUTED);
                ui.label(
                    RichText::new(i18n::tr(self.locale, "Informasi perangkat"))
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
                            .unwrap_or_else(|| i18n::tr(self.locale, "Pilih perangkat").into());
                        let mut selected = self.selected_index;
                        let combo_id = ui.make_persistent_id(egui::Id::new("Perangkat"));
                        let response =
                            egui::ComboBox::from_label(i18n::tr(self.locale, "Perangkat"))
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
                                })
                                .response;
                        let popup_open = egui::ComboBox::is_open(ctx, combo_id);
                        let key_step = if response.has_focus() && !popup_open {
                            ctx.input(|input| {
                                if input.key_pressed(egui::Key::ArrowDown) {
                                    1isize
                                } else if input.key_pressed(egui::Key::ArrowUp) {
                                    -1
                                } else {
                                    0
                                }
                            })
                        } else {
                            0
                        };
                        if selected == self.selected_index && key_step != 0 {
                            let len = self.devices.len();
                            let index = selected.unwrap_or(0) % len;
                            selected = Some(if key_step > 0 {
                                (index + 1) % len
                            } else {
                                (index + len - 1) % len
                            });
                        }
                        if selected != self.selected_index {
                            self.select_device(selected, ctx);
                        }
                    } else {
                        if let Some(device) = self.selected_device() {
                            ui.label(
                                RichText::new(device.mode.to_uppercase())
                                    .size(9.0)
                                    .color(MUTED),
                            );
                        } else {
                            ui.label(
                                RichText::new(i18n::tr(self.locale, "Belum dipilih"))
                                    .size(9.0)
                                    .color(MUTED),
                            );
                        }
                    }
                });
            });
            ui.add_space(9.0);
            if let Some(device) = self.selected_device() {
                match self.device_info.as_ref() {
                    Some((serial, Ok(_))) if serial == &device.serial => {}
                    Some((serial, Err(error)))
                        if serial == &device.serial && error == "Membaca identitas perangkat…" =>
                    {
                        ui.label(
                            RichText::new(i18n::tr(
                                self.locale,
                                "Membaca informasi dasar perangkat…",
                            ))
                            .size(10.0)
                            .color(MUTED),
                        );
                    }
                    Some((serial, Err(error))) if serial == &device.serial => {
                        ui.label(
                            RichText::new(i18n::translate_text(self.locale, &format!(
                                "Informasi dasar tidak dapat dibaca: {error}. Periksa koneksi dan ketersediaan tools."
                            )))
                            .size(10.0)
                            .color(RED),
                        );
                    }
                    _ => {
                        ui.label(
                            RichText::new(i18n::tr(
                                self.locale,
                                "Menunggu hasil pembacaan informasi dasar…",
                            ))
                            .size(10.0)
                            .color(MUTED),
                        );
                    }
                }
            }
            ui.add_space(6.0);
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
            let missing_value = if self.selected_device().is_some() {
                i18n::tr(self.locale, "Belum terbaca")
            } else {
                i18n::tr(self.locale, "Belum tersedia")
            };
            let model = values
                .get("model")
                .cloned()
                .unwrap_or_else(|| missing_value.into());
            let codename = values
                .get("codename")
                .or_else(|| values.get("product"))
                .cloned()
                .unwrap_or_else(|| missing_value.into());
            let android = values
                .get("android")
                .cloned()
                .unwrap_or_else(|| missing_value.into());
            let bootloader = values
                .get("bootloader unlocked")
                .cloned()
                .unwrap_or_else(|| missing_value.into());
            let serial = self
                .selected_device()
                .map(|device| device.serial.clone())
                .unwrap_or_else(|| missing_value.into());
            let connection = self
                .selected_device()
                .map(|device| format!("{} · {}", device.mode.to_uppercase(), device.state))
                .unwrap_or_else(|| missing_value.into());
            ui.columns(2, |columns| {
                kv_row(&mut columns[0], i18n::tr(self.locale, "Model"), &model);
                kv_row(
                    &mut columns[0],
                    i18n::tr(self.locale, "Kode perangkat"),
                    &codename,
                );
                kv_row(
                    &mut columns[0],
                    i18n::tr(self.locale, "Versi Android"),
                    &android,
                );
                kv_row(
                    &mut columns[1],
                    i18n::tr(self.locale, "Status bootloader"),
                    &bootloader,
                );
                kv_row(
                    &mut columns[1],
                    i18n::tr(self.locale, "Nomor seri"),
                    &serial,
                );
                kv_row(
                    &mut columns[1],
                    i18n::tr(self.locale, "Koneksi"),
                    &connection,
                );
            });
            if let Some((info_serial, Ok(info))) = self.device_info.as_ref() {
                if self
                    .selected_device()
                    .is_some_and(|device| device.serial == *info_serial)
                {
                    egui::CollapsingHeader::new(i18n::tr(self.locale, "Detail teknis perangkat"))
                        .default_open(false)
                        .show(ui, |ui| {
                            ui.label(RichText::new(i18n::tr(self.locale, "Nilai kosong atau gagal dibaca tidak membuktikan perangkat aman.")).size(9.0).color(AMBER));
                            for line in info.lines() {
                                ui.label(RichText::new(i18n::translate_text(self.locale, line)).size(9.0).monospace().color(MUTED));
                            }
                        });
                }
            }
            if let Some(device) = self.selected_device() {
                if device.mode == "adb" && device.state != "device" {
                    ui.add_space(5.0);
                    ui.label(
                        RichText::new(if device.state == "unauthorized" {
                            i18n::tr(
                                self.locale,
                                "Buka layar ponsel dan setujui permintaan USB debugging.",
                            )
                        } else {
                            i18n::tr(
                                self.locale,
                                "Coba kabel atau port USB lain, lalu sambungkan kembali.",
                            )
                        })
                        .size(9.0)
                        .color(AMBER),
                    );
                }
            }
        });
    }

    fn draw_usb_mode_card(&self, ui: &mut egui::Ui) {
        card(ui, |ui| {
            ui.horizontal(|ui| {
                draw_icon(ui, Icon::Device, 18.0, MUTED);
                ui.label(
                    RichText::new(i18n::tr(self.locale, "Mode USB teramati"))
                        .size(14.0)
                        .strong()
                        .color(TEXT),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(
                        RichText::new(i18n::tr(self.locale, "Baca-saja"))
                            .size(9.0)
                            .color(AMBER),
                    );
                });
            });
            ui.add_space(6.0);
            ui.label(
                RichText::new(i18n::tr(self.locale, "Petunjuk dari VID:PID teramati."))
                    .size(10.0)
                    .color(MUTED),
            );
            if self.usb_scanning {
                ui.label(
                    RichText::new(i18n::tr(self.locale, "Memeriksa daftar perangkat USB…"))
                        .size(10.0)
                        .color(MUTED),
                );
            } else if self.usb_scan_error.is_some() {
                ui.label(
                    RichText::new(i18n::tr(
                        self.locale,
                        "Daftar perangkat USB tidak dapat dibaca saat ini.",
                    ))
                    .size(10.0)
                    .color(AMBER),
                );
            } else if self.usb_modes.is_empty() {
                ui.label(
                    RichText::new(i18n::tr(
                        self.locale,
                        "Tidak ada ID USB yang dikenali untuk petunjuk mode.",
                    ))
                    .size(10.0)
                    .color(MUTED),
                );
            } else {
                ui.add_space(3.0);
                for detected in &self.usb_modes {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(i18n::translate_text(self.locale, detected.mode.label()))
                                .size(10.0)
                                .color(TEXT),
                        );
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.label(
                                RichText::new(detected.usb_id())
                                    .size(9.0)
                                    .monospace()
                                    .color(MUTED),
                            );
                        });
                    });
                }
            }
        });
    }

    fn draw_actions(&mut self, ui: &mut egui::Ui) {
        card(ui, |ui| {
            ui.label(
                RichText::new(i18n::tr(self.locale, "Mulai ulang perangkat"))
                    .size(14.0)
                    .strong()
                    .color(TEXT),
            );
            ui.label(
                RichText::new(i18n::tr(self.locale, "Konfirmasi diperlukan."))
                    .size(10.0)
                    .color(MUTED),
            );
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                let button_width = ((ui.available_width() - 16.0) / 3.0).max(120.0);
                for (label, action, icon_kind) in [
                    (
                        i18n::tr(self.locale, "Mulai ulang ke Android"),
                        "system",
                        Icon::Reboot,
                    ),
                    (
                        i18n::tr(self.locale, "Buka Recovery"),
                        "recovery",
                        Icon::Reboot,
                    ),
                    (
                        i18n::tr(self.locale, "Buka Fastboot"),
                        "bootloader",
                        Icon::Reboot,
                    ),
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
                draw_icon(ui, Icon::Folder, 18.0, MUTED);
                ui.label(
                    RichText::new(i18n::tr(self.locale, "Periksa berkas ROM lokal"))
                        .size(14.0)
                        .strong()
                        .color(TEXT),
                );
            });
            ui.label(
                RichText::new(i18n::tr(
                    self.locale,
                    "Menghitung SHA-256 lokal. Berkas tidak dijalankan atau dikirim ke perangkat.",
                ))
                .size(10.0)
                .color(MUTED),
            );
            ui.add_space(8.0);
            ui.label(
                RichText::new(i18n::tr(self.locale, "Folder yang akan diperiksa"))
                    .size(9.0)
                    .color(MUTED),
            );
            ui.horizontal(|ui| {
                let browse_width = 146.0;
                let inspect_width = 144.0;
                let text_width =
                    (ui.available_width() - browse_width - inspect_width - 16.0).max(120.0);
                let can_edit_folder = !self.inspecting_rom;
                let can_inspect = can_edit_folder && !self.rom_folder.trim().is_empty();
                ui.add_enabled_ui(can_edit_folder, |ui| {
                    ui.add_sized(
                        [text_width, 32.0],
                        egui::TextEdit::singleline(&mut self.rom_folder)
                            .hint_text(i18n::tr(self.locale, "Pilih folder ROM hasil ekstrak")),
                    );
                });
                if draw_icon_button(
                    ui,
                    Icon::Folder,
                    i18n::tr(self.locale, "Pilih folder"),
                    can_edit_folder,
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
                    i18n::tr(self.locale, "Periksa berkas"),
                    can_inspect,
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
            if self.inspecting_rom {
                ui.label(
                    RichText::new(i18n::tr(
                        self.locale,
                        "Memeriksa isi folder dan menghitung SHA-256 berkas lokal…",
                    ))
                    .size(10.0)
                    .color(MUTED),
                );
            } else if let Some(error) = &self.rom_error {
                ui.label(
                    RichText::new(i18n::translate_text(
                        self.locale,
                        &format!("Pemeriksaan gagal: {error}"),
                    ))
                    .size(10.0)
                    .color(RED),
                );
                ui.label(
                    RichText::new(i18n::tr(
                        self.locale,
                        "Pilih folder yang benar, lalu jalankan pemeriksaan lagi.",
                    ))
                    .size(9.0)
                    .color(MUTED),
                );
            } else if let Some((_path, inspection)) = &self.rom {
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new(&inspection.folder_name)
                            .size(10.0)
                            .strong()
                            .monospace()
                            .color(TEXT),
                    );
                });
                let summary = if inspection.entries.is_empty() {
                    i18n::tr(self.locale, "Folder ini tidak berisi berkas gambar atau metadata yang dikenali. Pilih folder ROM hasil ekstrak.").to_string()
                } else {
                    format!(
                        "{} berkas terkait · {}",
                        inspection.entries.len(),
                        inspection.script_hint
                    )
                };
                ui.label(
                    RichText::new(i18n::translate_text(self.locale, &summary))
                        .size(9.0)
                        .color(MUTED),
                );
                ui.label(
                    RichText::new(i18n::tr(
                        self.locale,
                        "Sumber dan kecocokan firmware belum dapat dipastikan.",
                    ))
                    .size(9.0)
                    .color(AMBER),
                );
                egui::ScrollArea::vertical()
                    .max_height(78.0)
                    .show(ui, |ui| {
                        for entry in inspection.entries.iter().take(12) {
                            let kind_color = if entry.kind == "IMAGE" { MUTED } else { AMBER };
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
                                        .unwrap_or(i18n::tr(self.locale, "SHA-256 tidak tersedia"))
                                        .to_string()
                                },
                                |hash| format!("{} B · SHA-256 {hash}", entry.size_bytes),
                            );
                            ui.label(
                                RichText::new(i18n::translate_text(self.locale, &hash_text))
                                    .size(8.0)
                                    .monospace()
                                    .color(MUTED),
                            );
                        }
                    });
                ui.label(
                    RichText::new(i18n::translate_text(self.locale, &inspection.warning))
                        .size(10.0)
                        .color(AMBER),
                );
            } else {
                ui.label(
                    RichText::new(
                        i18n::tr(self.locale, "Pilih folder untuk melihat nama, ukuran, dan SHA-256 berkas yang ditemukan."),
                    )
                    .size(10.0)
                    .color(MUTED),
                );
            }
        });
    }

    fn draw_confirm_dialog(&mut self, ctx: &egui::Context) {
        let Some(action) = self.pending_action else {
            return;
        };
        if ctx.input(|input| input.key_pressed(egui::Key::Escape)) {
            self.pending_action = None;
            return;
        }
        let device = self.selected_device().cloned();
        let mut confirm = false;
        let mut cancel = false;
        egui::Window::new(i18n::tr(self.locale, "Konfirmasi mulai ulang"))
            .id(egui::Id::new("reboot-confirm"))
            .collapsible(false)
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.set_min_width(330.0);
                ui.label(
                    RichText::new(action_prompt(self.locale, action))
                        .size(16.0)
                        .strong()
                        .color(TEXT),
                );
                ui.label(
                    RichText::new(action_description(self.locale, action))
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
                        .color(MUTED),
                    );
                }
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    if ui.button(i18n::tr(self.locale, "Batal")).clicked() {
                        cancel = true;
                    }
                    if ui
                        .button(
                            RichText::new(i18n::tr(self.locale, "Mulai ulang sekarang"))
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
        egui::TopBottomPanel::bottom("footer")
            .frame(
                egui::Frame::new()
                    .fill(Color32::from_rgb(22, 25, 30))
                    .inner_margin(egui::Margin::symmetric(20, 7)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let reading_device = self.device_info.as_ref().is_some_and(|(_, result)| {
                        matches!(result, Err(message) if message == "Membaca identitas perangkat…")
                    });
                    let app_status = if self.installing_tools {
                        i18n::tr(self.locale, "Menyiapkan ADB/Fastboot…")
                    } else if self.scanning {
                        i18n::tr(self.locale, "Memindai…")
                    } else if self.rebooting {
                        i18n::tr(self.locale, "Memulai ulang…")
                    } else if self.inspecting_rom {
                        i18n::tr(self.locale, "Memeriksa berkas…")
                    } else if self.usb_scanning {
                        i18n::tr(self.locale, "Memeriksa USB…")
                    } else if reading_device {
                        i18n::tr(self.locale, "Membaca informasi perangkat…")
                    } else {
                        i18n::tr(self.locale, "Siap")
                    };
                    let is_loading = self.scanning
                        || self.installing_tools
                        || self.rebooting
                        || self.inspecting_rom
                        || self.usb_scanning
                        || reading_device;
                    ui.label(
                        RichText::new(i18n::tr(self.locale, "Status aplikasi:"))
                            .size(9.0)
                            .color(MUTED),
                    );
                    ui.label(RichText::new(app_status).size(9.0).strong().color(GREEN));
                    if is_loading {
                        ui.add(egui::Spinner::new().size(13.0).color(GREEN));
                    }
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
                                ui.heading(RichText::new(i18n::tr(self.locale, "Perangkat dan koneksi")).size(24.0).strong().color(TEXT));
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    let tools_ready = self.tool_status.adb_available && self.tool_status.fastboot_available;
                                    let text = if self.scanning {
                                        i18n::tr(self.locale, "Memindai…")
                                    } else if self.installing_tools {
                                        i18n::tr(self.locale, "Menyiapkan…")
                                    } else if tools_ready {
                                        i18n::tr(self.locale, "Pindai perangkat")
                                    } else {
                                        i18n::tr(self.locale, "Siapkan ADB/Fastboot")
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
                            if self.selected_device().is_some() {
                                ui.add_space(9.0);
                                self.draw_device_card(ui, ctx);
                            }
                            ui.add_space(9.0);
                            self.draw_usb_mode_card(ui);
                            ui.add_space(9.0);
                            self.draw_actions(ui);
                            ui.add_space(9.0);
                            self.draw_rom_card(ui, ctx);
                        }
                        Page::Log => {
                            page_heading(ui, self.locale, i18n::tr(self.locale, "Riwayat aktivitas"), i18n::tr(self.locale, "Catatan sesi ini tersimpan di komputer dan tidak dikirim secara otomatis."));
                            ui.add_space(10.0);
                            card(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new(i18n::translate_text(self.locale, &format!("{} catatan aktivitas", self.log.len()))).size(10.0).color(MUTED));
                                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                        if ui.button(i18n::tr(self.locale, "Ekspor laporan…")).clicked() {
                                            request_case_export = true;
                                        }
                                    });
                                });
                                ui.add_space(8.0);
                                if self.log.is_empty() {
                                    ui.label(RichText::new(i18n::tr(self.locale, "Belum ada aktivitas.")).size(10.0).color(MUTED));
                                } else {
                                    egui::ScrollArea::vertical().max_height(620.0).show(ui, |ui| {
                                        for (index, entry) in self.log.iter().enumerate().rev() {
                                            let color = if entry.contains("gagal") || entry.contains("tidak ditemukan") { RED } else if entry.contains("berhasil") || entry.contains("Ditemukan") { GREEN } else { MUTED };
                                            ui.horizontal_top(|ui| {
                                                ui.label(RichText::new(format!("{:03}", index + 1)).size(9.0).monospace().color(MUTED));
                                                ui.label(RichText::new(i18n::translate_text(self.locale, entry)).size(10.0).color(color));
                                            });
                                            ui.separator();
                                        }
                                    });
                                }
                            });
                        }
                        Page::About => {
                            page_heading(ui, self.locale, i18n::tr(self.locale, "Tentang DroidNow"), i18n::tr(self.locale, "Diagnostik Android melalui ADB dan Fastboot."));
                            ui.add_space(10.0);
                            card(ui, |ui| {
                                ui.label(RichText::new(i18n::tr(self.locale, "Membaca informasi perangkat dan memeriksa berkas lokal.")).size(11.0).color(MUTED));
                                ui.add_space(8.0);
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new(i18n::tr(self.locale, "Versi aplikasi")).size(10.0).color(MUTED));
                                    ui.label(RichText::new(format!("v{}", env!("CARGO_PKG_VERSION"))).size(10.0).monospace().color(TEXT));
                                });
                            });
                        }
                        Page::Help => {
                            page_heading(ui, self.locale, i18n::tr(self.locale, "Panduan singkat"), "");
                            ui.add_space(10.0);
                            card(ui, |ui| {
                                help_step(ui, "1", i18n::tr(self.locale, "Siapkan tools"), i18n::tr(self.locale, "Pilih folder Platform-Tools, atau siapkan tools resmi dari Google."));
                                help_step(ui, "2", i18n::tr(self.locale, "Hubungkan perangkat"), i18n::tr(self.locale, "ADB: aktifkan USB debugging dan setujui akses. Fastboot: masuk sesuai petunjuk perangkat."));
                                help_step(ui, "3", i18n::tr(self.locale, "Tinjau hasil"), i18n::tr(self.locale, "Pindai koneksi. DroidNow hanya membaca perangkat."));
                            });
                            egui::CollapsingHeader::new(i18n::tr(self.locale, "Lokasi tools"))
                                .default_open(false)
                                .show(ui, |ui| {
                                    ui.label(
                                        RichText::new(i18n::translate_text(
                                            self.locale,
                                            &self.tool_status.location,
                                        ))
                                        .size(10.0)
                                        .monospace()
                                        .color(MUTED),
                                    );
                                });
                            ui.add_space(9.0);
                            card(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        draw_icon(ui, Icon::Shield, 17.0, AMBER);
                                        ui.label(RichText::new(i18n::tr(self.locale, "Batas operasi")).size(13.0).strong().color(AMBER));
                                    });
                                    ui.label(RichText::new(i18n::tr(self.locale, "Reboot perlu konfirmasi. Pemasangan firmware, penulisan partisi, perubahan bootloader, dan penghapusan data tidak tersedia.")).size(10.0).color(MUTED));
                                    ui.add_space(5.0);
                                    ui.label(RichText::new(i18n::tr(self.locale, "Tools dari folder pilihan tidak diverifikasi. Hash lokal tidak membuktikan sumber atau kecocokan firmware.")).size(10.0).color(MUTED));
                                    ui.label(RichText::new(i18n::tr(self.locale, "Deteksi USB hanya membaca VID:PID teramati. Perintah USB tidak dikirim; protokol tidak diverifikasi.")).size(10.0).color(AMBER));
                                    if cfg!(target_os = "windows") {
                                        ui.add_space(8.0);
                                        if draw_icon_button(ui, Icon::Shield, i18n::tr(self.locale, "Petunjuk driver USB"), true, 210.0, 36.0, false).clicked() {
                                            self.show_driver_dialog = true;
                                        }
                                    }
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

fn page_heading(ui: &mut egui::Ui, locale: Locale, title: &str, subtitle: &str) {
    ui.heading(RichText::new(title).size(23.0).strong().color(TEXT));
    if !subtitle.is_empty() {
        ui.label(
            RichText::new(i18n::translate_text(locale, subtitle))
                .size(10.0)
                .color(MUTED),
        );
    }
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
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, CONTROL_EDGE);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, TEXT);
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(49, 55, 64);
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, CONTROL_EDGE);
    visuals.widgets.active.bg_fill = Color32::from_rgb(42, 91, 81);
    visuals.widgets.open.bg_fill = Color32::from_rgb(38, 44, 52);
    visuals.widgets.open.bg_stroke = Stroke::new(1.0_f32, CONTROL_EDGE);
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

fn action_prompt(locale: Locale, action: &str) -> &'static str {
    match action {
        "recovery" => i18n::tr(locale, "Masuk ke Recovery?"),
        "bootloader" => i18n::tr(locale, "Masuk ke Fastboot?"),
        _ => i18n::tr(locale, "Mulai ulang perangkat?"),
    }
}

impl DroidNowApp {
    fn draw_driver_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_driver_dialog || !cfg!(target_os = "windows") {
            return;
        }
        if ctx.input(|input| input.key_pressed(egui::Key::Escape)) {
            self.show_driver_dialog = false;
            return;
        }
        let mut close = false;
        egui::Window::new(i18n::tr(self.locale, "Petunjuk driver USB"))
            .id(egui::Id::new("generic-driver-help"))
            .collapsible(false)
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.set_min_width(390.0);
                ui.label(RichText::new(i18n::tr(self.locale, "Pemasangan driver memerlukan tindakan Anda di Windows.")).size(13.0).strong().color(TEXT));
                ui.add_space(6.0);
                ui.label(RichText::new(i18n::tr(self.locale, "DroidNow tidak mengunduh atau menjalankan installer driver secara otomatis. Driver yang diperlukan bergantung pada perangkat dan versi Windows.")).size(10.0).color(MUTED));
                ui.add_space(7.0);
                ui.label(RichText::new(i18n::tr(self.locale, "Jika perangkat tidak terdeteksi, periksa kabel dan port USB. Pasang driver ADB/Fastboot hanya dari sumber tepercaya, lalu tinjau penerbit serta permintaan izin Windows sebelum melanjutkan.")).size(10.0).color(AMBER));
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if ui.button(i18n::tr(self.locale, "Tutup")).clicked() {
                        close = true;
                    }
                });
            });
        if close {
            self.show_driver_dialog = false;
        }
    }
}

fn action_description(locale: Locale, action: &str) -> &'static str {
    match action {
        "recovery" => {
            i18n::tr(locale, "Perangkat akan dimulai ulang ke Recovery. Tindakan ini tidak memasang atau menghapus apa pun.")
        }
        "bootloader" => {
            i18n::tr(locale, "Perangkat akan dimulai ulang ke Fastboot. Tindakan ini tidak membuka kunci bootloader.")
        }
        _ => i18n::tr(locale, "Perangkat akan dimulai ulang ke Android. Simpan pekerjaan yang sedang berlangsung."),
    }
}
