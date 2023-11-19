macro_rules! html_element {
    ($tag:expr, { $($attr:ident=$value:expr),* }, [$($content:expr),*]) => {{
        let mut element = String::new();
        element.push_str(&format!("<{} ", $tag));
        $(element.push_str(&format!("{}=\"{}\" ", stringify!($attr), $value));)*
        element.push_str(">");
        element.push_str(&format!("{}", html_content!($($content),*)));
        element.push_str(&format!("</{}>", $tag));
        element
    }};
}

macro_rules! html_content {
    ($($content:expr),*) => {
        format!($($content),*)
    };
    () => {
        String::new()
    };
}

fn main() {

    // copy 网上的例子, 体现了rust宏的强大

    let name = "Alice";
    let age = 30;

    let html = html_element!(
        "div",
        {
            class="container",
            id="user-info",
            data="user-data"
        },
        [
            "Name: {}, Age: {}", name, age
        ]
    );

    println!("{}", html);
}
