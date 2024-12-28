use clay_layout::{
    color::Color,
    elements::{
        containers::border::BorderContainer, rectangle::Rectangle, text::Text, CornerRadius,
    },
    id::Id,
    layout::{padding::Padding, sizing::Sizing, Layout},
    math::Dimensions,
    Clay,
};

fn main() {
    let clay = Clay::new(Dimensions::new(800.0, 600.0));
    clay.measure_text_function(|_, _| Dimensions::default());
    clay.begin();
    clay.with(
        [
            Layout::new()
                .width(Sizing::Fixed(100.0))
                .height(Sizing::Fixed(100.0))
                .padding(clay_layout::layout::padding::Padding::new(10, 10))
                .end(),
            Rectangle::new()
                .color(Color::rgb(255., 255., 255.))
                .end(Id::new("parent_rect")),
        ],
        |clay| {
            clay.with(
                [
                    Layout::new()
                        .width(Sizing::Fixed(100.0))
                        .height(Sizing::Fixed(100.0))
                        .padding(clay_layout::layout::padding::Padding::new(10, 10))
                        .end(),
                    Rectangle::new()
                        .color(Color::rgb(255., 255., 255.))
                        .end(Id::new("rect_under_rect")),
                ],
                |_clay| {},
            );
            clay.text(
                "test",
                Text::new()
                    .color(Color::rgb(255., 255., 255.))
                    .font_size(24)
                    .end(),
            );
        },
    );
    clay.with(
        [
            Layout::new().padding(Padding::new(16, 16)).end(),
            BorderContainer::new()
                .all_directions(2, Color::rgb(255., 255., 0.))
                .corner_radius(CornerRadius::All(25.))
                .end(Id::new_index("Border_container", 1)),
        ],
        |clay| {
            clay.with(
                [
                    Layout::new()
                        .width(Sizing::Fixed(50.0))
                        .height(Sizing::Fixed(50.0))
                        .end(),
                    Rectangle::new()
                        .color(Color::rgb(0., 255., 255.))
                        .end(Id::new("rect_under_border")),
                ],
                |_clay| {},
            );
        },
    );

    let _items = clay.end();


    let test = 

    
}
