// use core::intrinsics::size_of;

use libsa::endian::{BigEndianU32, BigEndianU64};
// use limine::Framebuffer;

#[derive(Debug)]
#[repr(packed)]
pub struct RAMFBConfig {
    address: BigEndianU64,
    /// Format, should always be 0x34325241
    _fourcc: BigEndianU32,
    /// Not used?
    _flags: BigEndianU32,
    _width: BigEndianU32,
    height: BigEndianU32,
    stride: BigEndianU32,
}

impl RAMFBConfig {
    pub fn new(width: u32, height: u32) -> Self {
        let stride = width * 4;
        let size = crate::arch::paging::PageSize::from_size_ceil(height as usize * stride as usize)
            as usize;
        let layout = vmem::Layout::new(size);
        let layout = layout.align(size);

        let physaddr = crate::mem::PHYS
            .alloc_constrained(layout, vmem::AllocStrategy::NextFit)
            .unwrap();

        Self {
            address: BigEndianU64::new(physaddr as _),
            _fourcc: BigEndianU32::new(0x34325241),
            _flags: BigEndianU32::new(0x0),
            _width: BigEndianU32::new(width),
            height: BigEndianU32::new(height),
            stride: BigEndianU32::new(stride),
        }
    }

    pub fn addr(&self) -> u64 {
        self.address.get()
    }

    pub fn byte_size(&self) -> usize {
        self.height.get() as usize * self.stride.get() as usize
    }
}

// pub struct Window {
//     pub x: u16,
//     pub y: u16,
//     pub extent_x: u16,
//     pub extent_y: u16,
//     framebuffer: RAMFBConfig,
//     buf: *mut u8,
// }

#[derive(Copy, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}


impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }

    pub fn new_from_hex(hex: u32) -> Color {
        let red = (hex >> 16) as u8;
        let green = ((hex >> 8) & 0xff) as u8;
        let blue = (hex & 0xff) as u8;
        Color { red, green, blue }
    }

    #[inline]
    pub fn white() -> Color {
        Color{ red: 0xff, green: 0xff, blue: 0xff }
    }

    #[inline]
    pub fn black() -> Color {
        Color{ red: 0x00, green: 0x00, blue: 0x00 }
    }

    #[inline]
    pub fn red() -> Color {
        Color{ red: 0xff, green: 0x00, blue: 0x00 }
    }

    #[inline]
    pub fn green() -> Color {
        Color{ red: 0x00, green: 0xff, blue: 0x00 }
    }

    #[inline]
    pub fn blue() -> Color {
        Color{ red: 0x00, green: 0x00, blue: 0xff }
    }
}

impl From<u32> for Color {
    fn from(x: u32) -> Self { 
        Color::new_from_hex(x)
    }
}

fn get_pointer_for_coords(x: usize, y: usize, ramfb: &RAMFBConfig) -> *mut u8 {
    let stride = TryInto::<usize>::try_into(u32::from(ramfb.stride)).unwrap();
    unsafe {
        let ptr = ramfb.addr() as *mut u8;
        // vram + y*pitch + x*pixelwidth;
        ptr.add(y * stride + x * 4)
    }
}

pub fn set(x: usize, y: usize, color: Color, ramfb: &RAMFBConfig) {
    let ptr = get_pointer_for_coords(x, y, ramfb);

    unsafe {
        ptr.write_volatile(color.blue);
        ptr.add(1).write_volatile(color.green);
        ptr.add(2).write_volatile(color.red);

    }
}

/*
static void fillrect(unsigned char *vram, unsigned char r, unsigned char g, unsigned   char b, unsigned char w, unsigned char h) {
    unsigned char *where = vram;
    int i, j;
 
    for (i = 0; i < w; i++) {
        for (j = 0; j < h; j++) {
            //putpixel(vram, 64 + j, 64 + i, (r << 16) + (g << 8) + b);
            where[j*pixelwidth] = r;
            where[j*pixelwidth + 1] = g;
            where[j*pixelwidth + 2] = b;
        }
        where+=pitch;
    }
} */

pub fn fillrect(ramfb: &RAMFBConfig, color: Color, width: usize, height: usize) {
    let mut donde = ramfb.addr() as *mut u8;
    let stride = TryInto::<usize>::try_into(u32::from(ramfb.stride)).unwrap();
    for _ in 0..width {
        for j in 0..height {
            unsafe {
                donde.add(j * 4).write_volatile(color.blue);
                donde.add((j * 4) + 1).write_volatile(color.green);
                donde.add((j * 4) + 2).write_volatile(color.red);
            }
        }
        unsafe {
            donde = donde.add(stride);
        }
    }
}

pub fn render_char(ramfb: &RAMFBConfig, chr: char, color: Color) {
    let data = include_bytes!("assets/Tamzen6x12.psf");
    let font = psf2::Font::new(data).unwrap();
    let glyph = font.get_ascii(chr as u8).unwrap();

    let mut y = 0;
    let mut x = 0;
    for row in glyph {
        for pixel in row {
            if pixel {
                set(x, y, color, &ramfb);
            }
            x += 1;
        }
        x = 0;
        y += 1;
    }

}

// impl Window {
//     pub fn new(x: u16, y: u16, extent_x: u16, extent_y: u16, framebuffer: RAMFBConfig) -> Window {
//         unsafe {
//             let buf = alloc::alloc::alloc(alloc::alloc::Layout::new::<[u8; 300]>());
//             Window {
//                 x,
//                 y,
//                 extent_x,
//                 extent_y,
//                 framebuffer,
//                 buf,
//             }
//         }
//     }

//     pub fn fill(&self, color: Color) {
//         for y in (0..self.framebuffer.byte_size())
//             .step_by(u32::from(self.framebuffer._width).try_into().unwrap())
//         {
//             // let y_pos: usize =
//             //     y / TryInto::<usize>::try_into(u32::from(self.framebuffer._width)).unwrap();
//             // if (y_pos < (self.y as usize)) || (((self.y + self.extent_y) as usize) < y_pos) {
//             //     continue;
//             // }
//             for x in (y..(y + TryInto::<usize>::try_into(u32::from(self.framebuffer._width))
//                 .unwrap()))
//                 .step_by(3)
//             {
//                 // let x_pos = (x - y) / 3;
//                 // if (x_pos < (self.x as usize)) || (((self.x + self.extent_x) as usize) < x_pos) {
//                 //     continue;
//                 // }
//                 unsafe {
//                     let ptr = self.buf;
//                     ptr.add(x).write_volatile(color.red);
//                     ptr.add(x + 1).write_volatile(color.green);
//                     ptr.add(x + 2).write_volatile(color.blue);
//                 }
//             }
//         }
//     }

//     pub fn render(&self) {
//         for y in (0..self.framebuffer.byte_size())
//             .step_by(u32::from(self.framebuffer._width).try_into().unwrap())
//         {
//             // let y_pos: usize =
//             //     y / TryInto::<usize>::try_into(u32::from(self.framebuffer._width)).unwrap();
//             // if (y_pos < (self.y as usize)) || (((self.y + self.extent_y) as usize) < y_pos) {
//             //     continue;
//             // }
//             for x in
//                 y..(y + TryInto::<usize>::try_into(u32::from(self.framebuffer._width)).unwrap())
//             {
//                 // let x_pos = (x - y) / 3;
//                 // if (x_pos < (self.x as usize)) || (((self.x + self.extent_x) as usize) < x_pos) {
//                 //     continue;
//                 // }
//                 unsafe {
//                     let ptr = self.framebuffer.addr() as *mut u8;
//                     ptr.add(x).write_volatile(self.buf.add(x).read_volatile());
//                 }
//             }
//         }
//     }
// }
