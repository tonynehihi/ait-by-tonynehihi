//! Help → About Illustrator by Tonynehihi.

use crate::VectorcraftApp;
use crate::theme;

/// The About window, while `UiState::about` is set.
pub(super) fn show(app: &mut VectorcraftApp, ctx: &egui::Context) {
    if !app.ui.about {
        return;
    }
    let mut open = true;
    egui::Window::new(tl!("About Illustrator by Tonynehihi"))
        .collapsible(false)
        .resizable(false)
        .open(&mut open)
        .pivot(egui::Align2::CENTER_CENTER)
        .default_pos(ctx.content_rect().center())
        .frame(egui::Frame::window(&ctx.global_style()).inner_margin(egui::Margin::same(18)))
        .show(ctx, |ui| {
            ui.set_width(380.0);
            ui.label(egui::RichText::new("Illustrator by Tonynehihi — by Tony.").font(theme::semibold(16.0)));
            ui.add_space(6.0);
            ui.label(crate::i18n::fmt(tl!("Version {version}"), &[("version", env!("CARGO_PKG_VERSION"))]));
            ui.add_space(6.0);
            ui.label("Personal build by Tonynehihi, based on VectorCraft by the ArtCraft team.");
            ui.add_space(10.0);
            crate::updates::about_section(app, ui);
        });
    app.ui.about = open;
}
