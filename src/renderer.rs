use 

pub struct Renderer {
	display: SimulatorDisplay,
}

impl Renderer {

	pub fn new () -> Result<Renderer, String> {
		let display = SimulatorDisplay::<BinaryColor>::new(Size::new(128,64));
		
	pub fn draw(&mut self) -> Result<(), String> {
		draw_background();
		draw_wlan();
		draw_temp();
		draw_history;
	}
	
	fn draw_background(&mut self);
	fn draw_wlan(&mut self);
	fn draw_temp(&mut self);
	fn draw_history(&mut self);
	
}


