use std::{time::{SystemTime, UNIX_EPOCH}, fmt::Debug};

use sdl2::{video::Window, surface::Surface, Sdl, pixels::PixelMasks};

use crate::{device_atomic, hart::Hart, trap::Exception, utils::u32_to_u8};

use super::{Device, bus::Bus};

pub(crate) const YSYX_START: u64 = 0x20000000;
pub(crate) const YSYX_END: u64 = YSYX_START + 0x10000000 - 1;

const VGA_WIDTH: usize = 800;
const VGA_HEIGHT: usize = 600;


const YSYX_TIME: u64 = YSYX_START;

const YSYX_VGACTL_ADDR_LOW: u64 = YSYX_START + 0x100;
// sync
const YSYX_VGACTL_ADDR_HIGH: u64 = YSYX_START + 0x100 + 4;

const YSYX_FB_START: u64 = YSYX_START + 0x01000000;
const YSYX_FB_END: u64 = YSYX_FB_START + ((VGA_WIDTH * VGA_HEIGHT * 4) as u64) - 1;

pub(crate) struct Ysyx {
  sdl_ctx: Sdl,
  window: Window,
  vgactl: [u32; 2],
  vmem: [u32; VGA_WIDTH * VGA_HEIGHT],
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
    Ysyx {
      sdl_ctx,
      window,
      vgactl: [((VGA_WIDTH << 16) | VGA_HEIGHT) as u32, 0],
      vmem: [0; VGA_WIDTH * VGA_HEIGHT],
    }
  }
}

impl Device for Ysyx {
  device_atomic!();

  fn step(&mut self, _bus: &mut Bus, _hart: &mut Hart) {}

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
        let pump = &mut self.sdl_ctx.event_pump().unwrap();
        let mut w_surface = self.window.surface(pump).unwrap();
        surface.blit_scaled(None, &mut w_surface, None).unwrap();
        w_surface.finish().unwrap();
        for _ in pump.poll_iter() {}
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
