slint::slint!{
    import { Button, VerticalBox } from "std-widgets.slint";
    export component App inherits Window {
        property <int> counter: 2;
        VerticalBox {
            Text { text: "Hello power of 2: " + counter; }
            Button {text: "yay"; clicked => {counter *= 2}}
        }
    }
}

fn main() {
    println!("Hello, world!");
}
