use slint::Weak;

slint::slint!{
    import { Button, VerticalBox } from "std-widgets.slint";
    export component App inherits Window {
        in property <int> counter: 2;        // in -> make it public
        GridLayout {
            padding: 20px;
            spacing: 10px;
            Text { text: "power of 2: " + counter; colspan: 3;}
            Row {
                Button { text: "1";}
                Button { text: "2";}
                Button { text: "3";}
            }
            Row {
                Button { text: "4";}
                Button { text: "5";}
                Button { text: "6";}
            }
            Row {
                Button { text: "7";}
                Button { text: "8";}
                Button { text: "9";}
            }
            Row {
                Button { text: "0"; colspan: 3;}
            }
        }
    }
}

fn main() {
    let app: App = App::new().unwrap();
    let weak: Weak<App> = app.as_weak();      //weak pointer so we can move into the closure

    app.run().unwrap();
}
