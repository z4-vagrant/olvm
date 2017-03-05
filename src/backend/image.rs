/*
 * Backend image management
 */

use common::{Context, Result, Error};
use common::structs::Image;
use database;

pub fn script_create(ctx: &Context, img: &mut Image) -> Result<()> {
    let backend = try!(ctx.conf.get_backend(img.backend.as_str()).ok_or(Error::new("Invalid or unknown backend")));

    if let Some(ref path) = backend.image.create {
        let params = try!(super::script(path, try!(img.to_json()).as_str()));

        if params.len() > 0 {
            for (key, val) in params {
                img.parameters.insert(key.to_string(), val.to_string());
            }

            try!(database::image::update(&ctx.db, img));
        }
    }

    Ok(())
}

pub fn script_delete(ctx: &Context, img: &Image) -> Result<()> {
    let backend = try!(ctx.conf.get_backend(img.backend.as_str()).ok_or(Error::new("Invalid or unknown backend")));

    if let Some(ref path) = backend.image.delete {
        try!(super::script(path, try!(img.to_json()).as_str()));
    }

    Ok(())
}
