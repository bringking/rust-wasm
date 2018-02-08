#![feature(proc_macro)]
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

wasm_bindgen! {
  #[derive(Clone)]
  pub struct LatLng {
    lat: f32,
    lng: f32
  }

  impl LatLng {
    pub fn new(lat: f32, lng: f32) -> LatLng {
      LatLng { lat, lng }
    }

    pub fn to_json(&mut self) -> String {
      format!("{{\"lat\": {}, \"lng\": {}}}", self.lat, self.lng)
    }
  }

  pub struct Listing {
      name: String,
      location: LatLng
  }

  impl Listing {
    pub fn new(name: &str, location: LatLng) -> Listing {
      Listing { name: name.to_string(), location }
    }

    pub fn get_name(&mut self) -> String {
     self.name.to_string()
    }

    pub fn get_location(&mut self) -> LatLng {
      self.location.clone()
    }

    pub fn distance_to(&mut self, listing: Listing) -> f32 {
      let (dx, dy) = (self.location.lat - listing.location.lat, self.location.lng - listing.location.lng);
      (dx * dx + dy * dy).sqrt()
    }
  }
}
