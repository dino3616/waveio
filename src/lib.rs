use std::{io::{Read,BufReader,BufWriter,SeekFrom,prelude::*},fs::File,convert::TryFrom};
use byteorder::{LittleEndian,ByteOrder};

pub struct MonoPcm{
    pub sample_freq: u32,    //標本化周波数
    pub bits: u32,    //量子化精度
    pub length: u32,    //波長
    pub sound: Vec<f64>,    //音データ
}

pub struct StereoPcm{
    pub sample_freq: u32,    //標本化周波数
    pub bits: u32,    //量子化精度
    pub length: u32,    //波長
    pub sound_l: Vec<f64>,    //音データ
    pub sound_r: Vec<f64>,    //音データ
}

impl MonoPcm{
    pub fn new()->MonoPcm{
        MonoPcm{
            sample_freq: 0,
            bits: 0,
            length: 0,
            sound: Vec::new(),
        }
    }

    pub fn wav_read(&mut self,filename: &str)->Result<(),&'static str>{
        let mut reader=BufReader::new(File::open(filename).unwrap());

        let mut buf=[0;4];
        reader.seek(SeekFrom::Current(24)).unwrap();
        reader.read(&mut buf).unwrap();
        self.sample_freq=LittleEndian::read_u32(&mut buf);

        let mut buf=[0;2];
        reader.seek(SeekFrom::Current(6)).unwrap();
        reader.read(&mut buf).unwrap();
        let mut buf=[buf[0],buf[1],0,0];
        self.bits=LittleEndian::read_u32(&mut buf);

        let mut buf=[0;4];
        reader.seek(SeekFrom::Current(4)).unwrap();
        reader.read(&mut buf).unwrap();
        self.length=LittleEndian::read_u32(&mut buf)/2;

        for _ in 0..self.length{
            let mut buf=[0;2];
            reader.read(&mut buf).unwrap();
            let mut buf=[buf[0],buf[1]];
            self.sound.push((LittleEndian::read_i16(&mut buf)) as f64/32768.0);
        }

        Ok(())
    }

    pub fn wav_write(&self,filename: &str)->Result<(),&'static str> {
        let mut writer=BufWriter::new(File::create(filename).unwrap());

        writer.write(b"RIFF").unwrap();
        let mut buf=[0;4];
        LittleEndian::write_u32(&mut buf,36+self.length*2);
        let mut buf=[0;4];
        LittleEndian::write_u32(&mut buf,36+self.length*2);
        writer.write(&buf).unwrap();

        writer.write(b"WAVE").unwrap();

        writer.write(b"fmt ").unwrap();
        let mut buf=[0;4];
        LittleEndian::write_u32(&mut buf,self.bits);
        writer.write(&buf).unwrap();
        let mut buf=[0;2];
        LittleEndian::write_u16(&mut buf,1);
        writer.write(&buf).unwrap();
        let mut buf=[0;2];
        LittleEndian::write_u16(&mut buf,1);
        writer.write(&buf).unwrap();
        let mut buf=[0;4];
        LittleEndian::write_u32(&mut buf,self.sample_freq);
        writer.write(&buf).unwrap();
        let mut buf=[0;4];
        LittleEndian::write_u32(&mut buf,self.sample_freq*self.bits/8);
        writer.write(&buf).unwrap();
        let mut buf=[0;2];
        LittleEndian::write_u16(&mut buf,TryFrom::try_from(self.bits/8).unwrap());
        writer.write(&buf).unwrap();
        let mut buf=[0;2];
        LittleEndian::write_u16(&mut buf,TryFrom::try_from(self.bits).unwrap());
        writer.write(&buf).unwrap();

        writer.write(b"data").unwrap();
        let mut buf=[0;4];
        LittleEndian::write_u32(&mut buf,self.length*2);
        writer.write(&buf).unwrap();

        for i in 0..self.length as usize{
            let mut sound=(self.sound[i]+1.0)/2.0*65536.0;
            if sound>65536.0{
                sound=65536.0;
            }
            else if sound<0.0{
                sound=0.0;
            }

            let sound=(sound+0.5-32768.0) as i16;

            let mut buf=[0;2];
            LittleEndian::write_i16(&mut buf,sound);
            writer.write(&buf).unwrap();
        }

        Ok(())
    }
}

impl StereoPcm{
    pub fn new()->StereoPcm{
        StereoPcm{
            sample_freq: 0,
            bits: 0,
            length: 0,
            sound_l: Vec::new(),
            sound_r: Vec::new(),
        }
    }

    pub fn wav_read(&mut self,filename: &str)->Result<(),&'static str>{
        let mut reader=BufReader::new(File::open(filename).unwrap());

        let mut buf=[0;4];
        reader.seek(SeekFrom::Current(24)).unwrap();
        reader.read(&mut buf).unwrap();
        self.sample_freq=LittleEndian::read_u32(&mut buf);

        let mut buf=[0;2];
        reader.seek(SeekFrom::Current(6)).unwrap();
        reader.read(&mut buf).unwrap();
        let mut buf=[buf[0],buf[1],0,0];
        self.bits=LittleEndian::read_u32(&mut buf);

        let mut buf=[0;4];
        reader.seek(SeekFrom::Current(4)).unwrap();
        reader.read(&mut buf).unwrap();
        self.length=LittleEndian::read_u32(&mut buf)/(2*2);

        for _ in 0..self.length{
            let mut buf=[0;2];
            reader.read(&mut buf).unwrap();
            let mut buf=[buf[0],buf[1]];
            self.sound_l.push((LittleEndian::read_i16(&mut buf)) as f64/32768.0);

            let mut buf=[0;2];
            reader.read(&mut buf).unwrap();
            let mut buf=[buf[0],buf[1]];
            self.sound_r.push((LittleEndian::read_i16(&mut buf)) as f64/32768.0);
        }

        Ok(())
    }

    pub fn wav_write(&self,filename: &str)->Result<(),&'static str> {
        let mut writer=BufWriter::new(File::create(filename).unwrap());

        writer.write(b"RIFF").unwrap();
        let mut buf=[0;4];
        LittleEndian::write_u32(&mut buf,36+self.length*2*2);
        writer.write(&buf).unwrap();

        writer.write(b"WAVE").unwrap();

        writer.write(b"fmt ").unwrap();
        let mut buf=[0;4];
        LittleEndian::write_u32(&mut buf,self.bits);
        writer.write(&buf).unwrap();
        let mut buf=[0;2];
        LittleEndian::write_u16(&mut buf,1);
        writer.write(&buf).unwrap();
        let mut buf=[0;2];
        LittleEndian::write_u16(&mut buf,2);
        writer.write(&buf).unwrap();
        let mut buf=[0;4];
        LittleEndian::write_u32(&mut buf,self.sample_freq);
        writer.write(&buf).unwrap();
        let mut buf=[0;4];
        LittleEndian::write_u32(&mut buf,self.sample_freq*self.bits/8*2);
        writer.write(&buf).unwrap();
        let mut buf=[0;2];
        LittleEndian::write_u16(&mut buf,TryFrom::try_from(self.bits/8*2).unwrap());
        writer.write(&buf).unwrap();
        let mut buf=[0;2];
        LittleEndian::write_u16(&mut buf,TryFrom::try_from(self.bits).unwrap());
        writer.write(&buf).unwrap();

        writer.write(b"data").unwrap();
        let mut buf=[0;4];
        LittleEndian::write_u32(&mut buf,self.length*2*2);
        writer.write(&buf).unwrap();

        for i in 0..self.length as usize{
            let mut sound=(self.sound_l[i]+1.0)/2.0*65536.0;
            if sound>65536.0{
                sound=65536.0;
            }
            else if sound<0.0{
                sound=0.0;
            }

            let sound=(sound+0.5-32768.0) as i16;

            let mut buf=[0;2];
            LittleEndian::write_i16(&mut buf,sound);
            writer.write(&buf).unwrap();

            let mut sound=(self.sound_r[i]+1.0)/2.0*65536.0;
            if sound>65536.0{
                sound=65536.0;
            }
            else if sound<0.0{
                sound=0.0;
            }

            let sound=(sound+0.5-32768.0) as i16;

            let mut buf=[0;2];
            LittleEndian::write_i16(&mut buf,sound);
            writer.write(&buf).unwrap();
        }

        Ok(())
    }
}