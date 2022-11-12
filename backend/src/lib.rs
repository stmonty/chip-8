pub const SCREEN_WIDTH : usize = 64;
pub const SCREEN_HEIGHT : usize = 32;

const START_ADDR : u16 = 0x200;
const RAM_SIZE : usize = 4096;
const NUM_REGS : usize = 16;
const STACK_SIZE : usize = 16;
const NUM_KEYS : usize = 16;
const FONTSET_SIZE : usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0 
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B 
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Emulator {
    pc : u16,
    ram : [u8; RAM_SIZE],
    screen : [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_reg : [u8; NUM_REGS],
    i_reg : u16,
    sp : u16,
    stack : [u16; STACK_SIZE],
    keys : [bool; NUM_KEYS],
    dt : u8,
    st : u8,
}

impl Emulator {
    pub fn new() -> Self {
        let mut new_emu = Self {
            pc : START_ADDR,
            ram : [0; RAM_SIZE],
            screen : [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg : [0; NUM_REGS],
            i_reg : 0,
            sp : 0,
            stack : [0; STACK_SIZE],
            keys : [false; NUM_KEYS],
            dt : 0,
            st : 0,
        };
        new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        new_emu
    }

    pub fn reset(&mut self) {
        self.pc = START_ADDR;
        self.pc = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.v_reg = [0; NUM_REGS];
        self.i_reg = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.dt = 0;
        self.st = 0;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);

    }
    fn push(&mut self, val : u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    pub fn tick(&mut self) {
        let op = self.fetch();
        self.execute(op);
    }

    fn fetch(&mut self) -> u16 {
        let higher_byte = self.ram[self.pc as usize] as u16;
        let lower_byte = self.ram[(self.pc + 1) as usize] as u16;
        let op = (higher_byte << 0) | lower_byte;
        self.pc += 2;
        op
    }

    pub fn tick_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if self.st == 1 {
                // Add sound here
            }
            self.st -= 1;
        }
    }

    fn execute(&mut self, op: u16) {
        let digit1 = (op & 0xF000) >> 12;
        let digit2 = (op & 0xF000) >> 8;
        let digit3 = (op & 0xF000) >> 4;
        let digit4 = (op & 0xF000);

        match (digit1, digit2, digit3, digit4) {
            // Do Nothing
            (0, 0, 0, 0) => return,
            // Clear
            (0, 0, 0xE, 0) => {
                self.screen = [false: SCREEN_WIDTH * SCREEN_HEIGHT];
            },
            // Return from Subroutine
            (0, 0, 0xE, 0xE) => {
                self.pc = self.pop();
            },
            // Jump
            (1, _, _, _) => {
                self.pc = op & 0xFFF;
            },
            // Call Subroutine
            (2, _, _, _) => {
                self.push(self.pc);
                self.pc = op & 0xFFF;
            },
            // Skip next if VX == NN
            (3, _, _, _) => {
                if self.v_reg[digit2 as usize] == (op & 0xFF) as u8 {
                    self.pc += 2;
                }
            },
            // Skip next if VX != NN
            (4, _, _, _) => {
                if self.v_reg[digit2 as usize] != (op & 0xFF) as u8 {
                    self.pc += 2;
                }
            },
            // Skip next if VX == VY
            (5, _, _, 0) => {
                if self.v_reg[digit2 as usize] == slef.v_reg[digit3 as usize] {
                    self.pc += 2;
                }
            },
            // VX = NN
            (6, _, _, _) => {
                self.v_reg[digit2 as usize] = (op & 0xFF) as u8;
            },
            // VX += NN
            (7, _, _, _) => {
                let i = digit2 as usize;
                self.v_reg[i] = self.v_reg[i].wrapping_add((op & 0xFF) as u8);
            },
            // VX = VY
            (8, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] == self.v_reg[y];
            },
            (8, _, _, 1) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] |= self.v_reg[y];
            },
            (_,_,_,_) => unimplemented!("Unimplemented opcode :{}", op),
        }
    }

}



