use pengui::{
	UserInterface
};

fn main() {
	let mut locked_ui = UserInterface::new();
	loop {
		locked_ui.register_event();
		let ui = locked_ui.new_frame();


		locked_ui = ui.end_frame();
		locked_ui.draw();
	}
}