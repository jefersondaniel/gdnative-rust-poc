use std::collections::HashMap;

use gdnative::godot_error;

use crate::core::error::DataError;

use super::animation::{ Animation, AnimationElement };

#[derive(Clone)]
struct  AnimationManager {
    pub filepath: String,
    pub foreignanimation: bool,
    pub currentanimation: Option<Animation>,
    pub currentelement: Option<AnimationElement>,
    pub finishedanimation: bool,
    pub animationtime: i32,
    animations: HashMap<usize, Animation>,
    animationinloop: bool,
    elementswitchtime: i32,
}

impl AnimationManager {
    pub fn new(filepath: String, animations: HashMap<usize, Animation>) -> Self {
        AnimationManager {
            filepath,
            foreignanimation: false,
            currentanimation: None,
            currentelement: None,
            finishedanimation: false,
            animationtime: 0,
            animations: animations.clone(),
            animationinloop: false,
            elementswitchtime: 0,
        }
    }

    pub fn has_animation(&self, number: usize) -> bool {
        self.animations.contains_key(&number)
    }

    pub fn set_local_animation(&mut self, animationnumber: usize, elementnumber: usize) -> Result<(), DataError> {
        let animation = self.animations.get(&animationnumber)
            .ok_or_else(|| DataError::new(format!("Animation not found: {}", animationnumber)))?
            .clone();

        let element = animation.elements.get(elementnumber)
            .ok_or_else(|| DataError::new(format!("Animation element not found: {},{}", animationnumber, elementnumber)))?
            .clone();

        self.foreignanimation = false;
        self.set_animation(&animation, &element);

        Ok(())
    }

    pub fn set_foreign_animation(&mut self, manager: AnimationManager, animationnumber: usize, elementnumber: usize) -> Result<(), DataError>  {
        let animation = manager.animations.get(&animationnumber)
            .ok_or_else(|| DataError::new(format!("Foreign animation not found: {}", animationnumber)))?
            .clone();

        let element = animation.elements.get(elementnumber)
            .ok_or_else(|| DataError::new(format!("Foreign animation element not found: {},{}", animationnumber, elementnumber)))?
            .clone();

        self.foreignanimation = true;
        self.set_animation(&animation, &element);

        Ok(())
    }

    pub fn set_animation(&mut self, animation: &Animation, element: &AnimationElement) {
        self.currentanimation = Some(animation.clone());
        self.currentelement = Some(element.clone());

        self.finishedanimation = false;
        self.animationinloop = false;
        self.animationtime = animation.get_element_start_time(element.id);
        self.elementswitchtime = element.gameticks;
    }

    pub fn update(&mut self) -> Result<(), DataError> {
        self.finishedanimation = false;
        self.animationtime += 1;

        if self.elementswitchtime == -1 {
            return Ok(());
        }

        if self.elementswitchtime > 1{
            self.elementswitchtime -= 1;
        } else {
            let currentanimation = self.currentanimation
                .as_ref()
                .ok_or_else(|| DataError::new("Animation manager animation is null".to_string()))?;

            let currentelement = self.currentelement
                .as_ref()
                .ok_or_else(|| DataError::new("Animation manager element is null".to_string()))?;

            let newlement_option = currentanimation.get_next_element(currentelement.id);

            if let Some(newlement) = newlement_option {
                if newlement.id <= currentelement.id {
                    self.animationinloop = true;
                    self.finishedanimation = true;
                }

                self.currentelement = Some(newlement.clone());
                self.elementswitchtime = newlement.gameticks;
            } else {
                godot_error!("Unexpected error during animation element switch");
            }
        }

        Ok(())
    }
}
