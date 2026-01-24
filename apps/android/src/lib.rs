slint::slint! {
    export component MainWindow inherits Window {
        in property<string> message;
        width: 360px;
        height: 240px;

        Text {
            text: message;
            color: #0f5132;
            font-size: 20px;
            horizontal-alignment: center;
            vertical-alignment: center;
        }
    }
}

#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).expect("failed to init Slint Android backend");
    let ui = MainWindow::new().expect("failed to create UI");
    ui.set_message(shared::hello_text().into());
    ui.run().expect("failed to run UI");
}
