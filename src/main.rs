use std::{thread, time, env, io, process};
use log::LevelFilter;

#[macro_use]
extern crate log;

const XUSER_MAX_COUNT: u32 = 4;
const DEFAULT_REFRESH_MILLIS: u64 = 10;

fn parse_refresh_rate(value: String) -> u64 {
	match value.trim().parse() {
		Ok(num) => num,
		Err(e) => {
			warn!("Incorrect refresh rate parameter provided, using default value, error {:?}", e);
			DEFAULT_REFRESH_MILLIS
		}
	}
}

fn read_refresh_rate() -> u64 {
	match env::args().nth(1) {
		Some(input) => parse_refresh_rate(input),
		None => DEFAULT_REFRESH_MILLIS
	}
} 

fn is_gamepad_zero_connected(handle: &rusty_xinput::XInputHandle) -> bool {
	match &handle.get_state(0) {
		Ok(_) => true,
		Err(_) => false
	}
}

fn setup_pads(pads: &mut Vec<u32>, handle: &rusty_xinput::XInputHandle) {
	for pad_num in 1..XUSER_MAX_COUNT { // We skip 0, cause it is a new virtual gamepad
		match &handle.get_state(pad_num) {
			Ok(_) => {
				pads.push(pad_num);
				trace!("Gamepad {pad_num} found");
			}
			Err(e) => {
				trace!("Gamepad {} not found, error: {:?}", pad_num, e);
			}
		}
	}

	if pads.len() < 2 || pads.len() > 3 {
		error!("Wrong number of gamepads connected, minimum is 2 and maximum 3, exiting...");
		process::exit(1);
	}
}

fn merge_thumb_joystick(src_thumb: i16, gamepad_thumb: i16) -> i16 {
	let tmp_src_thumb: i32 = src_thumb.into();
	let tmp_gamepad_thumb: i32 = gamepad_thumb.into();

	if tmp_gamepad_thumb.abs() > tmp_src_thumb.abs() {
		gamepad_thumb
	} else {
		src_thumb
	}
}

fn merge_pad_inputs(src: &rusty_xinput::XInputState, gamepad: &mut vigem_client::XGamepad) {
	gamepad.buttons.raw = gamepad.buttons.raw | src.raw.Gamepad.wButtons;
	gamepad.left_trigger |= src.raw.Gamepad.bLeftTrigger;
	gamepad.right_trigger |= src.raw.Gamepad.bRightTrigger;

	gamepad.thumb_lx = merge_thumb_joystick(src.raw.Gamepad.sThumbLX, gamepad.thumb_lx);
	gamepad.thumb_rx = merge_thumb_joystick(src.raw.Gamepad.sThumbRX, gamepad.thumb_rx);
	gamepad.thumb_ly = merge_thumb_joystick(src.raw.Gamepad.sThumbLY, gamepad.thumb_ly);
	gamepad.thumb_ry = merge_thumb_joystick(src.raw.Gamepad.sThumbRY, gamepad.thumb_ry);
}

fn init_merged_gamepad() -> vigem_client::XTarget {
	let client = vigem_client::Client::connect().unwrap();
	let id = vigem_client::TargetId::XBOX360_WIRED;
	let mut target = vigem_client::Xbox360Wired::new(client, id);
	target.plugin().unwrap();
	target.wait_ready().unwrap();
	thread::sleep(time::Duration::from_millis(1000)); // Wait more
	target
}

fn main() {
	simple_logger::SimpleLogger::new()
		.with_level(LevelFilter::Warn)
        .env()
		.init()
		.unwrap();

	let refresh_rate = read_refresh_rate();
	trace!("Using refresh rate of {refresh_rate} ms");
	let handle = rusty_xinput::XInputHandle::load_default().unwrap();

	if is_gamepad_zero_connected(&handle) {
		error!("Please disconnect all gamepads, and restart application, exiting...");
		process::exit(1);
	}

	let mut target = init_merged_gamepad();

	if is_gamepad_zero_connected(&handle) {
		debug!("Virtual gamepad initialized successfully");
	} else {
		error!("Error, problem with registering virtual gamepad as gamepad nr 0, exiting...");
		process::exit(1);
	}

	println!("Connect gamepads to merge, then press ENTER :)");
	io::stdin().read_line(&mut String::new()).unwrap();
	
	let mut pads = Vec::new();
	setup_pads(&mut pads, &handle);
	println!("Merging started!");

	loop {
		let mut gamepad = vigem_client::XGamepad::default();

		for pad_num in &pads {
			match handle.get_state(*pad_num) {
				Ok(state) => {
					merge_pad_inputs(&state, &mut gamepad)
				}
				Err(e) => {
					error!("Problem with reading xinput_get_state, gamepad {}, error: {:?}", pad_num, e);
					break;
				}
			}
		}

		match target.update(&gamepad) {
			Ok(_) => {}
			Err(e) => { 
				error!("Problem with updating gamepad state, error: {:?}", e);
			}
		}

		thread::sleep(time::Duration::from_millis(refresh_rate));
	}
}