use delphinium::entities::info::Info;

use crate::domain::entities::image::Image;
use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct Metadata {
    pub id: i32,
    pub upstream: Info,
    pub images: Vec<Image>,
    pub created_at: chrono::NaiveDateTime,
}

impl Metadata {
    pub fn new(
        id: i32,
        upstream: Info,
        images: Vec<Image>,
        created_at: chrono::NaiveDateTime,
    ) -> Self {
        Metadata {
            id,
            upstream,
            images,
            created_at,
        }
    }
}
