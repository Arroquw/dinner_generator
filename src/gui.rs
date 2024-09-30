use crate::generate::Generate;
use eframe::egui;

#[derive(Eq, PartialEq)]
enum ActiveModal {
    AddToPool,
    EditEntry(usize, bool),
    RemoveFromPool,
    EditPoolEntry(usize),
    SwapEntries,
    None,
}

pub struct DinnerViewer {
    entries: Generate,
    active_modal: ActiveModal,
    input_entry: String,
    selected_day: Option<usize>,
}

impl DinnerViewer {
    pub fn new(g: Generate) -> Self {
        DinnerViewer {
            entries: g,
            ..Self::default()
        }
    }
    fn show_modal(&mut self, ctx: &egui::Context) {
        let mut entries = self.entries.clone();
        match self.active_modal {
            ActiveModal::AddToPool => {
                self.open_window("add_to_pool", "Add entry to pool", ctx, |entry| {
                    entries.add_to_pool(entry);
                })
            }
            ActiveModal::EditEntry(index, mut set) => {
                self.open_window("edit_entry", "Edit day entry", ctx, |entry| {
                    let _ = entries.edit_days_entry(index, entry);
                    set = false;
                })
            }
            ActiveModal::RemoveFromPool => {
                //     ui.label("Select an entry to remove:");
                //     for entry in &*self.entries.pool().clone() {
                //         if ui.button(entry).clicked() {
                //             let _ = self.entries.remove_from_pool(entry, "input.txt");
                //         }
                //     }
                todo!()
            }
            ActiveModal::EditPoolEntry(_) => todo!(),
            ActiveModal::SwapEntries => todo!(),
            ActiveModal::None => todo!(),
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
            selected_day: Some(0_usize),
        }
    }
}

impl eframe::App for DinnerViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut string_set = false;
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.heading("dinner viewer");
            for (index, day) in self.entries.days().clone().iter().enumerate() {
                ui.horizontal(|ui| {
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
                    ui.label(day);
                });

                //ui.menu_button(day, |ui| {
                // if ui.button("Add to Pool").clicked() {
                //     let mut new_entry = "";
                //
                //
                //     if !new_entry.is_empty() {
                //         self.entries.add_to_pool(new_entry.to_owned());
                //     }
                //     ui.close_menu();
                // }
                //
                // if ui.button("Edit Day Entry").clicked() {
                //     let mut new_value = String::new();
                //     ui.text_edit_singleline(&mut new_value);
                //     if !new_value.is_empty() {
                //         let _ = self.entries.edit_days_entry(index, new_value);
                //     }
                // }
                //
                // if ui.button("Swap Entries").clicked() {
                //     if self.swap_indices.is_none() {
                //         self.swap_indices = Some((index, 0)); // select first entry
                //     } else if let Some((first, _)) = self.swap_indices {
                //         self.swap_indices = Some((first, index)); // select second entry
                //         let _ = self.entries.swap_days_entries(first, index);
                //         self.swap_indices = None;
                //     }
                // }
                //
                // if ui.button("Edit Pool Entry").clicked() {
                //     ui.label("Select a pool entry to edit:");
                //     for entry in &*self.entries.pool().clone() {
                //         if ui.button(entry).clicked() {
                //             let mut new_value = String::new();
                //             ui.text_edit_singleline(&mut new_value);
                //             if !new_value.is_empty() {
                //                 let _ = self.entries.edit_pool_entry(entry, new_value);
                //             }
                //         }
                //     }
                // }
                //});
            }
            if ui.button("Remove entry from pool").clicked() {
                self.active_modal = ActiveModal::RemoveFromPool;
            }
            if ui.button("Add new entry to pool").clicked() {
                self.active_modal = ActiveModal::AddToPool;
            }
            if ui.button("Swap Entries").clicked() {
                self.active_modal = ActiveModal::SwapEntries;
            }
            if ui.button("Edit pool entry").clicked() {
                self.active_modal = ActiveModal::EditPoolEntry(0);
            }

            if self.active_modal != ActiveModal::None {
                self.show_modal(ctx);
            }
        });
    }
}
