use slint::Weak;

slint::slint!{
    import { Button, VerticalBox } from "std-widgets.slint";
    export component App inherits Window {
        in property <int> counter: 2;        // in -> make it public
        callback clicked <=> btn.clicked;   //alias
        VerticalBox {
            Text { text: "Hello power of 2: " + counter; }
            btn := Button {text: "yay"; }
        }
    }
}

fn main() {
    let app: App = App::new().unwrap();
    let weak: Weak<App> = app.as_weak();      //weak pointer so we can move into the closure
    app.on_clicked(move || {
        let app: App = weak.upgrade().unwrap();
        app.set_counter(app.get_counter() * 2);
    });
    app.run().unwrap();
}
