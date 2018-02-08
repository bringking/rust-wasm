#![feature(proc_macro)]
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

fn hypot(dx: f32, dy: f32) -> f32 {
  (dx * dx + dy * dy).sqrt()
}

fn distance_tween(p1: &LatLng, p2: &LatLng) -> f32 {
    let (dx, dy) = (p1.lat - p2.lat, p1.lng - p2.lng);
    hypot(dx, dy)
}

wasm_bindgen! {
  #[derive(Clone, PartialEq, Debug)]
  pub struct LatLng {
    lat: f32,
    lng: f32
  }

  impl LatLng {
    pub fn new(lat: f32, lng: f32) -> LatLng {
      LatLng { lat, lng }
    }

    pub fn to_json(&self) -> String {
      format!("{{\"lat\": {}, \"lng\": {}}}", self.lat, self.lng)
    }

    fn distance(&self, other: &LatLng) -> f32 {
      distance_tween(self, other)
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

    pub fn name(&self) -> String {
     self.name.to_string()
    }

    pub fn location(&self) -> LatLng {
      self.location.clone()
    }

    pub fn distance_to(&self, listing: Listing) -> f32 {
      self.location.distance(&listing.location)
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_the_hypot() {
        assert_eq!(hypot(3., 4.), 5.);
    }

    #[test]
    fn listings_can_measure_distance_to_other_listings() {
        let listing_one = Listing::new("Kushagram",LatLng::new(0., 3.));
        let listing_two = Listing::new("Kushagram 2",LatLng::new(4., 0.));
        assert_eq!(listing_one.distance_to(listing_two), 5.);
    }

    #[test]
    fn listings_can_retrieve_the_name() {
        let listing_one = Listing::new("Kushagram",LatLng::new(0., 3.));
        assert_eq!(listing_one.name(), "Kushagram");
    }

    #[test]
    fn listings_can_retrieve_the_location() {
        let listing_one = Listing::new("Kushagram", LatLng::new(0., 3.));
        assert_eq!(listing_one.location(), LatLng::new(0., 3.));
    }

    #[test]
    fn latlng_can_be_retrived_as_json() {
        let latlng = LatLng::new(0., 3.);
        assert_eq!(latlng.to_json(), "{\"lat\": 0, \"lng\": 3}")
    }
}
