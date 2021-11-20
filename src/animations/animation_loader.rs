use std::collections::HashMap;

use gdnative::core_types::{Point2, Rect2, Size2, Vector2};
use gdnative::{godot_error, godot_warn};

use crate::core::attribute_value::AttributeValue;
use crate::core::blending::Blending;
use crate::core::enumerations::SpriteEffects;
use crate::core::sprite_id::SpriteId;
use crate::core::{enumerations::ClsnType};
use crate::core::regex::RegEx;
use crate::io::text_file::TextFile;
use crate::io::text_section::TextSection;

use super::{animation::{Animation, AnimationElement}, clsn::Clsn};

pub struct AnimationLoader {
    animationtitleregex: RegEx,
    clsnregex: RegEx,
    clsnlineregex: RegEx,
    elementregex: RegEx,
}

impl AnimationLoader {
    pub fn new() -> Self {
        let animationtitleregex = RegEx::new(r"^\s*begin action\s+(-?\d+)(,.+)?\s*$");
        let clsnregex = RegEx::new(r"clsn([12])(default)?:\s*(\d+)");
        let clsnlineregex = RegEx::new(r"clsn([12])?\[(-?\d+)\]\s*=\s*(-?\d+)\s*,\s*(-?\d+)\s*,\s*(-?\d+)\s*,\s*(-?\d+)");
        let elementregex = RegEx::new(r"\s*,\s*");

        AnimationLoader {
            animationtitleregex: animationtitleregex,
            clsnregex: clsnregex,
            clsnlineregex: clsnlineregex,
            elementregex: elementregex
        }
    }

    pub fn load_animations(&self, text_file: TextFile) -> HashMap<usize, Animation> {
        let mut animations = HashMap::new();

        for section in text_file.sections.iter() {
            let animation_option = self.create_animation(section);

            if let Some(animation) = animation_option {
                if !animations.contains_key(&animation.number) {
                    animations.insert(animation.number, animation);
                } else {
                    godot_warn!("Invalid duplicated animation: {}", section.title)
                }
            }
        }

        animations
    }

    fn create_animation(&self, section: &TextSection) -> Option<Animation> {
        let title_match = self.animationtitleregex.search(&section.title)?;
        let animation_number = title_match.get_usize(1)?;

        let mut loopstart = 0;
        let mut starttick = 0;
        let mut elements = Vec::<AnimationElement>::new();

        let mut loading_type1 = Vec::<Clsn>::new();
        let mut loading_type2 = Vec::<Clsn>::new();
        let mut default_type1 = Vec::<Clsn>::new();
        let mut default_type2 = Vec::<Clsn>::new();

        let mut loaddefault = false;
        let mut loadtype = ClsnType::None;
        let mut loadcount = 0;

        for line in &section.lines {
            let line_string = String::from(line);

            if loadcount > 0 {
                let clsn_option = self.create_clsn(line, loadtype);

                if let Some(clsn) = clsn_option {
                    if loaddefault {
                        if loadtype == ClsnType::Type1Attack {
                            default_type1.push(clsn);
                        }
                        if loadtype == ClsnType::Type2Normal {
                            default_type2.push(clsn);
                        }
                    } else {
                        if loadtype == ClsnType::Type1Attack {
                            loading_type1.push(clsn);
                        }
                        if loadtype == ClsnType::Type2Normal {
                            loading_type2.push(clsn);
                        }
                    }
                } else {
                    godot_warn!("Could not create Clsn from line: {}", String::from(line));
                }

                loadcount = loadcount - 1;
                continue;
            }

            let clsnmatch_option = self.clsnregex.search(&line_string);

            if let Some(clsn_match) = clsnmatch_option {
                let mut clsntype = ClsnType::None;

                if clsn_match.get_string(1) == "1" {
                    clsntype = ClsnType::Type1Attack;
                }

                if clsn_match.get_string(1) == "2" {
                    clsntype = ClsnType::Type2Normal;
                }

                let isdefault = clsn_match.get_string(2).to_lowercase() == "default";

                if isdefault {
                    if clsntype == ClsnType::Type1Attack {
                        default_type1.clear();
                    }
                    if clsntype == ClsnType::Type2Normal {
                        default_type2.clear();
                    }
                }

                loadcount = clsn_match.get_i32(3).unwrap_or_default();
                loaddefault = isdefault;
                loadtype = clsntype;
                continue;
            }

            if line.to_string().to_lowercase() == "loopstart" {
                loopstart = elements.len();
                continue;
            }

            let element_option = self.create_element(
                line,
                elements.len(),
                starttick,
                default_type1.clone(),
                default_type2.clone(),
                loading_type1.clone(),
                loading_type2.clone()
            );

            if let Some(element) = element_option {
                if element.gameticks == -1 {
                    starttick = -1;
                } else {
                    starttick += element.gameticks;
                }

                elements.push(element.clone());
                loading_type1.clear();
                loading_type2.clear();
            } else {
                godot_error!("Invalid animation element. Anim No: {}, line: {}", animation_number, line.to_string());
            }
        }

        if elements.len() == 0 {
            godot_error!("Invalid animation {}, no elements", animation_number);
            return None
        }

        if loopstart == elements.len() {
            loopstart = 0;
        }

        Some(Animation::new(
            animation_number,
            loopstart,
            elements
        ))
    }

    fn create_clsn(&self, line: &AttributeValue, overridetype: ClsnType) -> Option<Clsn> {
        if !line.compare("clsn", 0, 0, 4) {
            return None
        }

        let line_string = String::from(line);
        let clsn_match = self.clsnlineregex.search(&line_string)?;

        let mut x1 = clsn_match.get_i32(3)?;
        let mut y1 = clsn_match.get_i32(4)?;
        let mut x2 = clsn_match.get_i32(5)?;
        let mut y2 = clsn_match.get_i32(6)?;

        if x1 > x2 {
            std::mem::swap(&mut x1, &mut x2);
        }

        if y1 > y2 {
            std::mem::swap(&mut y1, &mut y2);
        }

        Some(Clsn::new(overridetype, Rect2::new(
            Point2::new(x1 as f32, y1 as f32),
            Size2::new((x2 - x1) as f32, (y2 - y1) as f32))
        ))
    }

    fn create_element(
        &self,
        line: &AttributeValue,
        elementid: usize,
        starttick: i32,
        default_type1: Vec<Clsn>,
        default_type2: Vec<Clsn>,
        loading_type1: Vec<Clsn>,
        loading_type2: Vec<Clsn>
    ) -> Option<AnimationElement> {
        let line_string = line.to_string();
        let elements = self.elementregex.split(&line_string)?;

        if elements.len() < 5 {
            return None
        }

        let groupnumber = elements[0].to_string().parse::<i32>().ok()?;
        let imagenumber = elements[1].to_string().parse::<i32>().ok()?;
        let offset_x = elements[2].to_string().parse::<i32>().ok()?;
        let offset_y = elements[3].to_string().parse::<i32>().ok()?;
        let gameticks = elements[4].to_string().parse::<i32>().ok()?;

        let mut flip = SpriteEffects::None;

        if elements.len() >= 6 {
            let flip_text = elements[5].to_lowercase();

            if flip_text.contains("h") {
                flip |= SpriteEffects::FlipHorizontally;
            }

            if flip_text.contains("v") {
                flip |= SpriteEffects::FlipVertically;
            }
        }

        let mut blending = Blending::default();

        if elements.len() >= 7 {
            blending = Blending::from(line);
        }

        let mut clsn = Vec::<Clsn>::new();;
        clsn.extend(if loading_type1.len() != 0 { loading_type1.clone() } else { default_type1.clone() });
        clsn.extend(if loading_type2.len() != 0 { loading_type2.clone() } else { default_type2.clone() });

        let element = AnimationElement::new(
            elementid,
            gameticks,
            SpriteId::new(groupnumber, imagenumber),
            Vector2::new(offset_x as f32, offset_y as f32),
            flip,
            blending,
            starttick
        );

        Some(element)
    }
}

impl Default for AnimationLoader {
    fn default() -> Self {
        Self::new()
    }
}
