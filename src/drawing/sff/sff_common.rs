use super::image::{Palette, RawImage};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct SffPal {
    pub pal: Rc<Palette>,
    pub itemno: i32,
    pub groupno: i32,
    pub is_used: bool,
    pub usedby: i32,
    pub reserved: i32,
}

#[derive(Clone)]
pub struct SffData {
    pub image: Rc<RefCell<RawImage>>,
    pub groupno: i16,
    pub imageno: i16,
    pub x: i16,
    pub y: i16,
    pub palindex: i16,
    pub linked: i16,
}

#[derive(Clone)]
pub struct SffMetadata {
    pub verlo3: u8,
    pub verlo2: u8,
    pub verlo1: u8,
    pub verhi: u8,
}
