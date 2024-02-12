slint::slint!{
    import { Button, VerticalBox } from "std-widgets.slint";
    export component App inherits Window {
        property <int> counter: 2;
        callback clicked <=> btn.clicked;   //alias
        VerticalBox {
            Text { text: "Hello power of 2: " + counter; }
            btn := Button {text: "yay"; }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
