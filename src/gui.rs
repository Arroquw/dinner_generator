use crate::generate::Generate;
use eframe::egui;

#[derive(Eq, PartialEq, Clone)]
enum ActiveModal {
    AddToPool,
    EditEntry(usize, bool),
    RemoveFromPool(usize),
    EditPoolEntry(usize, bool),
    None,
}

enum ActiveState {
    Swapping { first_index: Option<usize> },
}

pub struct DinnerViewer {
    entries: Generate,
    active_modal: ActiveModal,
    input_entry: String,
    active_state: ActiveState,
    showing_pool: bool,
}

impl DinnerViewer {
    pub fn new(g: Generate) -> Self {
        DinnerViewer {
            entries: g,
            ..Self::default()
        }
    }

    fn view_pool(&mut self, ui: &mut egui::Ui) {
        let style = ui.style_mut();
        style.spacing.button_padding = egui::vec2(30.0, 10.0); // Adjust padding (influences size)
        style.visuals.widgets.active.rounding = egui::Rounding::same(45.0); // Optional rounding for button

        let scroll_area = egui::ScrollArea::vertical()
            .max_height(600.0)
            .auto_shrink(false);

        let mut string_set = false;
        scroll_area.show(ui, |ui| {
            for (index, entry) in self.entries.pool().clone().iter().enumerate() {
                ui.vertical(|ui| {
                    ui.menu_button(entry, |ui| {
                        if ui.button("Edit").clicked() {
                            if !string_set {
                                self.input_entry = entry.to_owned();
                                string_set = true;
                            }

                            self.active_modal = ActiveModal::EditPoolEntry(index, string_set);
                        }
                        if ui.button("Remove").clicked() {
                            self.active_modal = ActiveModal::RemoveFromPool(index);
                        }
                    });
                });
            }
        });
        if ui.button("Add entry").clicked() {
            self.active_modal = ActiveModal::AddToPool;
        }
    }

    fn view_days(&mut self, ui: &mut egui::Ui) {
        let style = ui.style_mut();
        style.spacing.button_padding = egui::vec2(30.0, 10.0); // Adjust padding (influences size)
        style.visuals.widgets.active.rounding = egui::Rounding::same(45.0); // Optional rounding for button

        let scroll_area = egui::ScrollArea::vertical()
            .max_height(600.0)
            .auto_shrink(false);

        let mut string_set = false;
        scroll_area.show(ui, |ui| {
            for (index, day) in self.entries.days().clone().iter().enumerate() {
                ui.vertical(|ui| {
                    ui.menu_button(day, |ui| {
                        if ui.button("Regenerate Entry").clicked() {
                            let _ = self.entries.regenerate_entry(index);
                        }
                        if ui.button("Edit entry").clicked() {
                            if !string_set {
                                self.input_entry = day.to_owned();
                                string_set = true;
                            }
                            self.active_modal = ActiveModal::EditEntry(index, string_set);
                        }
                        if ui.button("Select for Swap").clicked() {
                            match &mut self.active_state {
                                ActiveState::Swapping { first_index: None } => {
                                    self.active_state = ActiveState::Swapping {
                                        first_index: Some(index),
                                    };
                                    ui.close_menu();
                                }
                                ActiveState::Swapping {
                                    first_index: Some(first_index),
                                } => {
                                    // Perform the swap
                                    let _ = self.entries.swap_days_entries(*first_index, index);
                                    // Reset the state to Normal after swap
                                    self.active_state = ActiveState::Swapping { first_index: None };
                                    ui.close_menu();
                                }
                            }
                        }
                    });
                });
            }
        });
    }

    fn show_modal(&mut self, ctx: &egui::Context) {
        let mut entries = self.entries.clone();
        match self.active_modal {
            ActiveModal::AddToPool => {
                self.open_window("add_to_pool", "Add entry to pool", ctx, |entry| {
                    entries.add_to_pool(entry);
                })
            }
            ActiveModal::EditPoolEntry(index, mut set) => {
                self.open_window("edit_entry", "Edit day entry", ctx, |entry| {
                    let _ = entries.edit_pool_entry(index, entry);
                    set = false;
                })
            }
            ActiveModal::EditEntry(index, mut set) => {
                self.open_window("edit_entry", "Edit day entry", ctx, |entry| {
                    let _ = entries.edit_days_entry(index, entry);
                    set = false;
                })
            }
            ActiveModal::RemoveFromPool(index) => {
                let _ = entries.remove_from_pool(index, "input.txt");
            }
            ActiveModal::None => (),
        }
        self.entries = entries;
    }

    fn open_window<F>(&mut self, title: &str, label: &str, ctx: &egui::Context, on_submit: F)
    where
        F: FnOnce(String),
    {
        egui::Window::new(title).show(ctx, |ui| {
            ui.label(label);
            ui.text_edit_singleline(&mut self.input_entry);

            if ui.button("Submit").clicked() && !self.input_entry.is_empty() {
                let entry = self.input_entry.clone(); // Take ownership of input
                on_submit(entry);
                self.input_entry.clear(); // Clear input after submitting
                self.active_modal = ActiveModal::None;
            }

            if ui.button("Close").clicked() {
                self.active_modal = ActiveModal::None;
            }
        });
    }
}
impl Default for DinnerViewer {
    fn default() -> Self {
        Self {
            entries: Generate::read_entries("input.txt", "output.txt", 7, false).unwrap(),
            active_modal: ActiveModal::None,
            input_entry: "".to_owned(),
            active_state: ActiveState::Swapping { first_index: None },
            showing_pool: false,
        }
    }
}

impl eframe::App for DinnerViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_pixels_per_point(1.5);
            ui.heading("dinner viewer");
            if self.showing_pool {
                self.view_pool(ui);
            } else {
                self.view_days(ui);
            }

            if ui
                .button(if self.showing_pool {
                    "View Days"
                } else {
                    "View Pool"
                })
                .clicked()
            {
                self.showing_pool = !self.showing_pool;
            }

            if self.active_modal != ActiveModal::None {
                self.show_modal(ctx);
            }
        });
    }
}
