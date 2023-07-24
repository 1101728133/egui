use std::sync::{Arc, RwLock};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
enum Plot {
    Sin,
    Bell,
    Sigmoid,
}

fn gaussian(x: f64) -> f64 {
    let var: f64 = 2.0;
    f64::exp(-(x / var).powi(2)) / (var * f64::sqrt(std::f64::consts::TAU))
}

fn sigmoid(x: f64) -> f64 {
    -1.0 + 2.0 / (1.0 + f64::exp(-x))
}
#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ContextMenusData {
    plot: Plot,
    show_axes: [bool; 2],
    allow_drag: bool,
    allow_zoom: bool,
    allow_scroll: bool,
    center_x_axis: bool,
    center_y_axis: bool,
    width: f32,
    height: f32,
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ContextMenus {
    data: Arc<RwLock<ContextMenusData>>,
}

impl PartialEq for ContextMenus {
    fn eq(&self, other: &Self) -> bool {
        *self.data.read().unwrap() == *other.data.read().unwrap()
    }
}

impl Default for ContextMenusData {
    fn default() -> Self {
        Self {
            plot: Plot::Sin,
            show_axes: [true, true],
            allow_drag: true,
            allow_zoom: true,
            allow_scroll: true,
            center_x_axis: false,
            center_y_axis: false,
            width: 400.0,
            height: 200.0,
        }
    }
}

impl super::Demo for ContextMenus {
    fn name(&self) -> &'static str {
        "☰ Context Menus"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        let clone = self.clone();
        use super::View;
        egui::Window::new(self.name())
            .vscroll(false)
            .resizable(false)
            .open(open)
            .show(ctx, move |ui, _, _| clone.clone().ui(ui));
    }
}

impl super::View for ContextMenus {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.menu_button("Click for menu", Self::nested_menus);
            ui.button("Right-click for menu")
                .context_menu(Self::nested_menus);
        });

        ui.separator();

        ui.label("Right-click plot to edit it!");
        ui.horizontal(|ui| {
            self.example_plot(ui).context_menu(|ui| {
                let data = &mut *self.data.write().unwrap();
                ui.menu_button("Plot", |ui| {
                    if ui.radio_value(&mut data.plot, Plot::Sin, "Sin").clicked()
                        || ui
                            .radio_value(&mut data.plot, Plot::Bell, "Gaussian")
                            .clicked()
                        || ui
                            .radio_value(&mut data.plot, Plot::Sigmoid, "Sigmoid")
                            .clicked()
                    {
                        ui.close_menu();
                    }
                });
                egui::Grid::new("button_grid").show(ui, |ui| {
                    ui.add(
                        egui::DragValue::new(&mut data.width)
                            .speed(1.0)
                            .prefix("Width:"),
                    );
                    ui.add(
                        egui::DragValue::new(&mut data.height)
                            .speed(1.0)
                            .prefix("Height:"),
                    );
                    ui.end_row();
                    ui.checkbox(&mut data.show_axes[0], "x-Axis");
                    ui.checkbox(&mut data.show_axes[1], "y-Axis");
                    ui.end_row();
                    if ui.checkbox(&mut data.allow_drag, "Drag").changed()
                        || ui.checkbox(&mut data.allow_zoom, "Zoom").changed()
                        || ui.checkbox(&mut data.allow_scroll, "Scroll").changed()
                    {
                        ui.close_menu();
                    }
                });
            });
        });
        ui.vertical_centered(|ui| {
            ui.add(crate::egui_github_link_file!());
        });
    }
}

impl ContextMenus {
    fn example_plot(&self, ui: &mut egui::Ui) -> egui::Response {
        let data = &mut *self.data.write().unwrap();
        use egui::plot::{Line, PlotPoints};
        let n = 128;
        let line = Line::new(
            (0..=n)
                .map(|i| {
                    use std::f64::consts::TAU;
                    let x = egui::remap(i as f64, 0.0..=n as f64, -TAU..=TAU);
                    match data.plot {
                        Plot::Sin => [x, x.sin()],
                        Plot::Bell => [x, 10.0 * gaussian(x)],
                        Plot::Sigmoid => [x, sigmoid(x)],
                    }
                })
                .collect::<PlotPoints>(),
        );
        egui::plot::Plot::new("example_plot")
            .show_axes(data.show_axes)
            .allow_drag(data.allow_drag)
            .allow_zoom(data.allow_zoom)
            .allow_scroll(data.allow_scroll)
            .center_x_axis(data.center_x_axis)
            .center_x_axis(data.center_y_axis)
            .width(data.width)
            .height(data.height)
            .data_aspect(1.0)
            .show(ui, |plot_ui| plot_ui.line(line))
            .response
    }

    fn nested_menus(ui: &mut egui::Ui) {
        if ui.button("Open...").clicked() {
            ui.close_menu();
        }
        ui.menu_button("SubMenu", |ui| {
            ui.menu_button("SubMenu", |ui| {
                if ui.button("Open...").clicked() {
                    ui.close_menu();
                }
                let _ = ui.button("Item");
            });
            ui.menu_button("SubMenu", |ui| {
                if ui.button("Open...").clicked() {
                    ui.close_menu();
                }
                let _ = ui.button("Item");
            });
            let _ = ui.button("Item");
            if ui.button("Open...").clicked() {
                ui.close_menu();
            }
        });
        ui.menu_button("SubMenu", |ui| {
            let _ = ui.button("Item1");
            let _ = ui.button("Item2");
            let _ = ui.button("Item3");
            let _ = ui.button("Item4");
            if ui.button("Open...").clicked() {
                ui.close_menu();
            }
        });
        let _ = ui.button("Very long text for this item");
    }
}
