#[macro_use]
extern crate rocket;
extern crate libh3;
extern crate libh3_sys;

#[get("/deg")]
fn deg() -> std::string::String {
  format!("Degrees suck {}", libh3::rads_to_degs(1.002))
}

#[get("/rad")]
fn rad() -> std::string::String {
  format!("Radians rule {}", libh3::degs_to_rads(57.0001))
}

#[get("/area/<resolution>")]
fn area(resolution: i32) -> std::string::String {
  format!("km^2 {}", libh3::hex_area_km_2(resolution))
}

pub fn compact(h3_set: &Vec<libh3::H3Index>) -> Vec<libh3::H3Index> {
    unsafe {
        let n = h3_set.iter().count();
        let mut c = Vec::<libh3::H3Index>::with_capacity(n);
        libh3_sys::compact(h3_set.as_ptr(), c.as_mut_ptr(), n as i32);
        c.set_len(n);
        c = c.into_iter().filter(|h| *h != 0).collect();
        c
    }
}

#[get("/poly/<resolution>")]
fn poly(resolution: u8) -> std::string::String {
  let sf_verts = vec![
     (0.659966917655, -2.1364398519396),
     (0.6595011102219, -2.1359434279405),
     (0.6583348114025, -2.1354884206045),
     (0.6581220034068, -2.1382437718946),
     (0.6594479998527, -2.1384597563896),
     (0.6599990002976, -2.1376771158464),
  ]
  .iter()
  .map(|v| libh3::GeoCoord::new(v.0, v.1))
  .collect();

  let s = libh3::polyfill(&vec![sf_verts], resolution);
  let m = s.len();

  let c = compact(&s);
  let n = c.len();

  format!("success: {}, compact: {}", m, n)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![area, poly, deg, rad])
}
