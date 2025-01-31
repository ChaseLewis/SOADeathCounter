use anyhow::Result;
use dolphin_memory::Dolphin;

pub struct SOAMemoryMap {
    dolphin: Dolphin
}

impl SOAMemoryMap {

    pub fn wait_for_dolphin() -> Self {
        let dolphin = loop {
            if let Ok(dolphin) = Dolphin::new() {
                break dolphin;
            }
        };

        return Self {
            dolphin
        };
    }

    const RNG_SEED_ADDRESS: usize = 0x803469A8;
    const AREA_ID_ADDRESS: usize = 0x80311AC4;
    const AREA_SUB_ID_ADDRESS: usize = 0x80311AC8;
    const ENEMIES_KILLED_ADDRESS: usize = 0x8030BB44;

    pub fn is_running(&self) -> bool {
        return self.dolphin.is_emulation_running();
    }

    pub fn read_enemies_killed(&self) -> Result<i32> {
        return Ok(self.dolphin.read_i32(Self::ENEMIES_KILLED_ADDRESS, None)?);
    }

    pub fn read_rng_seed(&self) -> Result<i32> {
        return Ok(self.dolphin.read_i32(Self::RNG_SEED_ADDRESS, None)?);
    }

    pub fn write_rng_seed(&self,seed: i32) -> Result<()> {
        return Ok(self.dolphin.write_i32(seed, Self::RNG_SEED_ADDRESS, None)?);
    }

    pub fn read_back_attack_chance(&self) -> Result<i8> {
        return Ok(self.dolphin.read_i8(0x8030B7AA,None)?);
    }

    pub fn write_back_attack_chance(&self, value: i8) -> Result<()> {
        return Ok(self.dolphin.write_i8(value,0x8030B7AA,None)?);
    }

    pub fn read_first_strike_chance(&self) -> Result<i8> {
        return Ok(self.dolphin.read_i8(0x8030B7A9,None)?);
    }

    pub fn write_first_strike_chance(&self, value: i8) -> Result<()> {
        return Ok(self.dolphin.write_i8(value,0x8030B7A9,None)?);
    }

    pub fn read_escape_chance(&self) -> Result<i8> {
        return Ok(self.dolphin.read_i8(0x80347390,Some(&[0x2]))?);
    }

    pub fn write_escape_chance(&self, value: i8) -> Result<()> {
        return Ok(self.dolphin.write_i8(value,0x80347390,Some(&[0x2]))?);
    }

    pub fn read_encounter_mod(&self) -> Result<i8> {
        return Ok(self.dolphin.read_i8(0x8030B7AD,None)?);          
    }

    pub fn write_encounter_mod(&self,value: i8) -> Result<()> {
        return Ok(self.dolphin.write_i8(value, 0x8030B7AD,None)?);     
    }

    pub fn read_u32(&self,address: usize, pointer_offsets: Option<&[usize]>) -> Result<u32> {
        return Ok(self.dolphin.read_u32(address, pointer_offsets)?)
    }

    pub fn read_i32(&self,address: usize, pointer_offsets: Option<&[usize]>) -> Result<i32> {
        return Ok(self.dolphin.read_i32(address, pointer_offsets)?)
    }

    pub fn read_u16(&self,address: usize, pointer_offsets: Option<&[usize]>) -> Result<u16> {
        return Ok(self.dolphin.read_u16(address, pointer_offsets)?);
    }

    pub fn read_i16(&self,address: usize, pointer_offsets: Option<&[usize]>) -> Result<i16> {
        return Ok(self.dolphin.read_i16(address, pointer_offsets)?);
    }

    pub fn read_area_id(&self) -> Result<i32> {
        return Ok(self.dolphin.read_i32(0x80311AC4,None)?);
    }

    pub fn read_area_sub_id(&self) -> Result<char> {
        let ascii_char = self.dolphin.read_u8(0x80311AC8, None)?;
        return Ok(char::from_u32(ascii_char as u32).unwrap());
    }


}