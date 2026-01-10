use bevy::{
    app::{App, Plugin, Startup},
    diagnostic::{
        EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    ecs::system::Commands,
    render::diagnostic::RenderDiagnosticsPlugin,
};
use iyes_perf_ui::{PerfUiPlugin, prelude::PerfUiDefaultEntries};

pub struct DiagnosticsPlugin;

impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_plugins(EntityCountDiagnosticsPlugin)
            .add_plugins(SystemInformationDiagnosticsPlugin)
            .add_plugins(RenderDiagnosticsPlugin)
            .add_plugins(PerfUiPlugin)
            .add_systems(Startup, startup);
    }
}

fn startup(mut commands: Commands) {
    commands.spawn(PerfUiDefaultEntries::default());
}
