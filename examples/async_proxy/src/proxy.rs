use std::collections::HashMap;
use reqwasm::http::Request;
use serde_json::Value;

use yewdux::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Status {
    Loading,
    Ready,
    Error,
}

#[derive(Default, Clone, PartialEq, Eq, Store)]
pub struct State {
    timezones: HashMap<String, (String, Status)>,
}

impl State {
    pub fn get(&self, timezone: &String) -> Option<(String, Status)> {
        self.timezones
            .get(timezone.as_str())
            .map(|result| (result.0.clone(), result.1.clone()))
    }

    pub fn timezones(&self) -> Vec<String> {
        self.timezones.keys().map(|t| t.clone()).collect()
    }

    pub fn add(&mut self, timezone: String) {
        self.timezones.insert(timezone.clone(), ("...".into(), Status::Loading));
        self.refresh(timezone);
    }

    pub fn refresh(&mut self, timezone: String) {
        if let Some(e) = self.timezones.get_mut(timezone.as_str()) {
            (*e).1 = Status::Loading;
        }
        yew::platform::spawn_local(async move {
            let url = "http://worldtimeapi.org/api/timezone/".to_string() + timezone.as_str();
            let resp = Request::get(url.as_str())
                .send()
                .await
                .unwrap();
            let dispatch = Dispatch::<State>::new();
            if resp.ok() {
                let resp = resp.text().await.unwrap();
                let resp: Value = serde_json::from_str(resp.as_str()).unwrap();
                let datetime = resp["datetime"].to_string();
                dispatch.reduce_mut(move |state|
                    state.timezones.insert(timezone, (datetime, Status::Ready))
                );
            } else {
                dispatch.reduce_mut(move |state|
                    state.timezones.insert(timezone, (resp.status_text(), Status::Error))
                );
            }
        });
    }

    pub fn delete(&mut self, timezone: &String) {
        self.timezones.remove(timezone);
    }
}