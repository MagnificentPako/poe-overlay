use chrono::{DateTime, Local};
use futures_util::stream::StreamExt;
use leptos::*;
use leptos_use::*;
use shared::{Log, LogEvent};

use crate::base_window::BaseWindow;

#[component]
pub fn LogWindow() -> impl IntoView {
    let timestamp = use_timestamp();
    let (last_ts, set_last_ts) = create_signal(DateTime::from(
        DateTime::from_timestamp(timestamp.get_untracked() as i64 / 1000, 0).unwrap(),
    ));
    let (current_zone, set_current_zone) = create_signal("".into());

    create_effect(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let mut events = tauri_wasm::api::event::listen::<Log>("log-parse")
                .await
                .unwrap();
            let mut last_server_ip: String = "".into();
            let mut current_map_ip: String = "".into();
            while let Some(evt) = events.next().await {
                match evt.payload.event {
                    LogEvent::ZoneChange { zone } => {
                        set_current_zone.set(zone);
                    }
                    LogEvent::Connect { server } => {
                        last_server_ip = server;
                    }
                    LogEvent::GenerateZone { zone_name, .. } => {
                        tauri_wasm::js::console::info(&zone_name);
                        if zone_name.starts_with("Map") {
                            if current_map_ip != last_server_ip {
                                set_last_ts.set(evt.payload.dt);
                                current_map_ip = last_server_ip.clone();
                            }
                        } else {
                        }
                    }
                    _ => {}
                }
            }
        });
    });

    view! {
        <BaseWindow title="Instance Timer".into()>
            {move || {
                let last = last_ts.get();
                let current: DateTime<Local> = DateTime::from(DateTime::from_timestamp(timestamp.get() as i64 / 1000, 0).unwrap());
                let diff = current - last;
                let seconds = diff.num_seconds() % 60;
                let minutes = (diff.num_seconds() / 60) % 60;
                let hours = (diff.num_seconds() / 60) / 60;

                view! {
                    <h1>{format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds)}</h1>
                }
            }}
            <p>"Zone:" {current_zone}</p>
        </BaseWindow>
    }
}
