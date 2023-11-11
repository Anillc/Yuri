use std::{time::{SystemTime, UNIX_EPOCH}, fmt::Debug, collections::LinkedList};

use sdl2::{video::Window, surface::Surface, pixels::PixelMasks, EventPump, event::Event, keyboard::Scancode};

use crate::{device_atomic, hart::Hart, trap::Exception, utils::u32_to_u8};

use super::{Device, bus::Bus};

pub(crate) const YSYX_START: u64 = 0x20000000;
pub(crate) const YSYX_END: u64 = YSYX_START + 0x10000000 - 1;

const VGA_WIDTH: usize = 800;
const VGA_HEIGHT: usize = 600;
const KEYDOWN: u32 = 0x8000;


const YSYX_TIME: u64 = YSYX_START;

const YSYX_VGACTL_ADDR_LOW: u64 = YSYX_START + 0x100;
// sync
const YSYX_VGACTL_ADDR_HIGH: u64 = YSYX_START + 0x100 + 4;

const YSYX_KBD_ADDR: u64 = YSYX_START + 0x200;

const YSYX_FB_START: u64 = YSYX_START + 0x01000000;
const YSYX_FB_END: u64 = YSYX_FB_START + ((VGA_WIDTH * VGA_HEIGHT * 4) as u64) - 1;

pub(crate) struct Ysyx {
  window: Window,
  event_pump: EventPump,
  vgactl: [u32; 2],
  vmem: [u32; VGA_WIDTH * VGA_HEIGHT],
  key_queue: LinkedList<u32>,
}

impl Debug for Ysyx {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Ysyx")
      .field("vgactl", &self.vgactl)
      .field("vmem", &self.vmem)
      .finish()
  }
}

impl Ysyx {
  pub(crate) fn new() -> Ysyx {
    let sdl_ctx = sdl2::init().unwrap();
    let video = sdl_ctx.video().unwrap();
    let window = video.window("ysyx", VGA_WIDTH as u32, VGA_HEIGHT as u32)
      .position_centered()
      .opengl()
      .build().unwrap();
    let event_pump = sdl_ctx.event_pump().unwrap();
    Ysyx {
      window,
      event_pump,
      vgactl: [((VGA_WIDTH << 16) | VGA_HEIGHT) as u32, 0],
      vmem: [0; VGA_WIDTH * VGA_HEIGHT],
      key_queue: LinkedList::new(),
    }
  }
}

impl Device for Ysyx {
  device_atomic!();

  fn step(&mut self, _bus: &mut Bus, _hart: &mut Hart) {
    for event in self.event_pump.poll_iter() {
      match event {
          Event::Quit { .. } => todo!("exit"),
          Event::KeyDown{ scancode: Some(scancode), .. }
        | Event::KeyUp{ scancode: Some(scancode), .. } => {
          if let Some(amcode) = keycode_to_amkey(scancode) {
            let down = if let Event::KeyDown { .. } = event { KEYDOWN } else { 0 };
            self.key_queue.push_back(amcode | down);
          }
        },
        _ => {},
      }
    }
  }

  fn read8(&mut self, address: u64) -> Result<u8, Exception> {
    Err(Exception::LoadAccessFault(address))
  }

  fn read16(&mut self, address: u64) -> Result<u16, Exception> {
    Err(Exception::LoadAccessFault(address))
  }

  fn read32(&mut self, address: u64) -> Result<u32, Exception> {
    match address {
      YSYX_VGACTL_ADDR_LOW => Ok(self.vgactl[0]),
      YSYX_VGACTL_ADDR_HIGH => Ok(self.vgactl[1]),
      YSYX_FB_START..=YSYX_FB_END => Ok(self.vmem[((address - YSYX_FB_START) / 4) as usize]),
      YSYX_KBD_ADDR => Ok(if self.key_queue.len() != 0 {
        self.key_queue.pop_front().unwrap()
      } else { 0 }),
      _ => Err(Exception::LoadAccessFault(address)),
    }
  }

  fn read64(&mut self, address: u64) -> Result<u64, Exception> {
    match address {
      YSYX_TIME => Ok(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()),
      _ => Err(Exception::LoadAccessFault(address))
    }
  }

  fn write8(&mut self, address: u64, _data: u8) -> Result<(), Exception> {
    Err(Exception::StoreAMOAccessFault(address))
  }

  fn write16(&mut self, address: u64, _data: u16) -> Result<(), Exception> {
    Err(Exception::StoreAMOAccessFault(address))
  }

  fn write32(&mut self, address: u64, data: u32) -> Result<(), Exception> {
    match address {
      YSYX_VGACTL_ADDR_HIGH => {
        let surface = Surface::from_data_pixelmasks(
          u32_to_u8(&mut self.vmem),
          VGA_WIDTH as u32, VGA_HEIGHT as u32, VGA_WIDTH as u32 * 4,
          PixelMasks {
            bpp: 32,
            rmask: 0x00ff0000,
            gmask: 0x0000ff00,
            bmask: 0x000000ff,
            amask: 0x00000000,
          }).unwrap();
        let mut w_surface = self.window.surface(&self.event_pump).unwrap();
        surface.blit_scaled(None, &mut w_surface, None).unwrap();
        w_surface.finish().unwrap();
      },
      YSYX_FB_START..=YSYX_FB_END => self.vmem[((address - YSYX_FB_START) / 4) as usize] = data,
      _ => return Err(Exception::StoreAMOAccessFault(address)),
    }
    Ok(())
  }

  fn write64(&mut self, address: u64, _data: u64) -> Result<(), Exception> {
    Err(Exception::StoreAMOAccessFault(address))
  }
}

fn keycode_to_amkey(scancode: Scancode) -> Option<u32> {
  match scancode {
    Scancode::Escape => Some(1),
    Scancode::F1 => Some(2),
    Scancode::F2 => Some(3),
    Scancode::F3 => Some(4),
    Scancode::F4 => Some(5),
    Scancode::F5 => Some(6),
    Scancode::F6 => Some(7),
    Scancode::F7 => Some(8),
    Scancode::F8 => Some(9),
    Scancode::F9 => Some(10),
    Scancode::F10 => Some(11),
    Scancode::F11 => Some(12),
    Scancode::F12 => Some(13),
    Scancode::Grave => Some(14),
    Scancode::Num1 => Some(15),
    Scancode::Num2 => Some(16),
    Scancode::Num3 => Some(17),
    Scancode::Num4 => Some(18),
    Scancode::Num5 => Some(19),
    Scancode::Num6 => Some(20),
    Scancode::Num7 => Some(21),
    Scancode::Num8 => Some(22),
    Scancode::Num9 => Some(23),
    Scancode::Num0 => Some(24),
    Scancode::Minus => Some(25),
    Scancode::Equals => Some(26),
    Scancode::Backspace => Some(27),
    Scancode::Tab => Some(28),
    Scancode::Q => Some(29),
    Scancode::W => Some(30),
    Scancode::E => Some(31),
    Scancode::R => Some(32),
    Scancode::T => Some(33),
    Scancode::Y => Some(34),
    Scancode::U => Some(35),
    Scancode::I => Some(36),
    Scancode::O => Some(37),
    Scancode::P => Some(38),
    Scancode::LeftBracket => Some(39),
    Scancode::RightBracket => Some(40),
    Scancode::Backslash => Some(41),
    Scancode::CapsLock => Some(42),
    Scancode::A => Some(43),
    Scancode::S => Some(44),
    Scancode::D => Some(45),
    Scancode::F => Some(46),
    Scancode::G => Some(47),
    Scancode::H => Some(48),
    Scancode::J => Some(49),
    Scancode::K => Some(50),
    Scancode::L => Some(51),
    Scancode::Semicolon => Some(52),
    Scancode::Apostrophe => Some(53),
    Scancode::Return => Some(54),
    Scancode::LShift => Some(55),
    Scancode::Z => Some(56),
    Scancode::X => Some(57),
    Scancode::C => Some(58),
    Scancode::V => Some(59),
    Scancode::B => Some(60),
    Scancode::N => Some(61),
    Scancode::M => Some(62),
    Scancode::Comma => Some(63),
    Scancode::Period => Some(64),
    Scancode::Slash => Some(65),
    Scancode::RShift => Some(66),
    Scancode::LCtrl => Some(67),
    Scancode::Application => Some(68),
    Scancode::LAlt => Some(69),
    Scancode::Space => Some(70),
    Scancode::RAlt => Some(71),
    Scancode::RCtrl => Some(72),
    Scancode::Up => Some(73),
    Scancode::Down => Some(74),
    Scancode::Left => Some(75),
    Scancode::Right => Some(76),
    Scancode::Insert => Some(77),
    Scancode::Delete => Some(78),
    Scancode::Home => Some(79),
    Scancode::End => Some(80),
    Scancode::PageUp => Some(81),
    Scancode::PageDown => Some(82),
    _ => None,
  }
}
