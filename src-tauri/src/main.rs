#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Manager, http::HeaderMap};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let _window = app.get_webview_window("main").unwrap();
            Ok(())
        })
        // ✅ 全局注入 Header（所有 HTTPS 请求）
        .register_uri_scheme_protocol("https", |_app, request| {
            let mut headers = HeaderMap::new();

            // ✅ User-Agent
            headers.insert(
                "user-agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/149.0.0.0 Safari/537.36"
                    .parse()
                    .unwrap(),
            );

            // ✅ sec-ch-ua
            headers.insert(
                "sec-ch-ua",
                r#""Google Chrome";v="149", "Chromium";v="149", "Not)A;Brand";v="24""#
                    .parse()
                    .unwrap(),
            );

            // ✅ sec-ch-ua-mobile
            headers.insert(
                "sec-ch-ua-mobile",
                "?0".parse().unwrap(),
            );

            // ✅ sec-ch-ua-platform
            headers.insert(
                "sec-ch-ua-platform",
                r#""Windows""#.parse().unwrap(),
            );

            // ✅ 转发请求
            let client = reqwest::blocking::Client::new();
            let resp = client
                .get(request.uri())
                .headers(headers)
                .send()
                .unwrap();

            let status = resp.status();
            let resp_headers = resp.headers().clone();
            let body = resp.bytes().unwrap();

            let mut builder = tauri::http::Response::builder().status(status);
            for (k, v) in resp_headers.iter() {
                builder = builder.header(k, v);
            }

            builder.body(body).unwrap()
        })
        .run(app_lib::run)
        .expect("error while running tauri application");
}