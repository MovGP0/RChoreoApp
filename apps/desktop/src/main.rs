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

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;
    ui.set_message(shared::hello_text().into());
    ui.run()
}
