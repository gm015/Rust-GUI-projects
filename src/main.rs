use slint::Weak;

slint::slint!{
    import { VerticalBox } from "std-widgets.slint";    // remove button and add it below - customised
 
    component Button {
        in property <string> text;
        Rectangle {
            background: ta.pressed ? red : ta.has-hover ? #93af93 : #52a1f0;
            border-radius: 4px;
            border-width: 2px;
            border-color: self.background.darker(20%);
            ta := TouchArea {}
        }
        Text { text: root.text; }
    }
    export component App inherits Window {
        in property <int> counter: 2;
        GridLayout {
            padding: 20px;
            spacing: 10px;
            Text { text: counter; colspan: 3;}
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
    let weak: Weak<App> = app.as_weak();

    app.run().unwrap();
}
