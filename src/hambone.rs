use serde::{Serialize, Deserialize};
use actix_web::{get, post, web, HttpResponse, HttpServer, Responder, Result};

#[derive(Serialize)]
pub struct Hambone {
    pub name: String,
    pub img: String // TODO: types for URL
}

impl Hambone {
    pub fn new(name: &str, img_url: &str) -> Self {
        Hambone {
            name: name.to_string(),
            img: img_url.to_string()
        }
    }
}

#[derive(Serialize)]
pub struct HambonesContainer {
    api: String,
    hambones: Vec<Hambone>
}

impl HambonesContainer {
    pub fn new(hambones: Vec<Hambone>) -> Self {
        HambonesContainer {
            api: "v1".to_string(),
            hambones
        }
    }
}

#[get("/v1/hambones")]
pub async fn all_v1_hambones() -> Result<impl Responder> {
    // TODO: Implement data backend (hah)
    let hambones_list = vec![
        Hambone::new("Mark Hamill", "https://upload.wikimedia.org/wikipedia/commons/thumb/9/99/Mark_Hamill_%282017%29.jpg/1200px-Mark_Hamill_%282017%29.jpg"),
        Hambone::new("Mark Harmon", "https://upload.wikimedia.org/wikipedia/commons/1/13/Mark_Harmon_1_edit1.jpg"),
        Hambone::new("Matt Damon", "https://upload.wikimedia.org/wikipedia/commons/8/83/Matt_Damon_TIFF_2015.jpg"),
        Hambone::new("Matt Dillon", "https://upload.wikimedia.org/wikipedia/commons/c/ce/Matt_Dillon_2010.jpg"),
        Hambone::new("Dylan McDermot", "https://upload.wikimedia.org/wikipedia/commons/2/2d/Dylan_McDermott_2014.jpg"),
        Hambone::new("Dermot Mulroney", "https://upload.wikimedia.org/wikipedia/commons/c/cf/Dermot_Mulroney_2013_TIFF_%28cropped%29.jpg"),
        Hambone::new("Kyle Mooney", "https://upload.wikimedia.org/wikipedia/commons/a/a9/Kyle_Mooney_by_Gage_Skidmore.jpg"),
        Hambone::new("Mickey Rooney", "https://upload.wikimedia.org/wikipedia/commons/f/f6/Mickey_Rooney_2_Allan_Warren.jpg"),
        Hambone::new("George Clooney", "https://upload.wikimedia.org/wikipedia/commons/8/8d/George_Clooney_2016.jpg"),
    ];
    Ok(web::Json(HambonesContainer::new(hambones_list)))
}