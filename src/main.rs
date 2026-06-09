pub mod structures;
pub mod enums;


fn main() {
	for i in 0..10 {
    let area = Rect::new(0, i, frame.area().width, 1);
    frame.render_widget(Paragraph::new("Hello world!"), area);
}
	println!("Hello, world!");

}