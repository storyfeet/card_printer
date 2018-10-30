extern crate lazyf;
extern crate mksvg;

use lazyf::{SGetter,Cfg};
use mksvg::{page,Card,SvgWrite,Args,SvgArg};

#[derive(Clone,Debug)]
pub struct ImgCard{
    img:String,
}

impl ImgCard{
    pub fn new(img:String)->Self{
        ImgCard{
            img,
        }
    }
}

impl Card<f64> for ImgCard{
    fn front<S:SvgWrite>(&self,svg:&mut S,w:f64,h:f64){
        svg.rect(0.,0.,w,h,Args::new().stroke_width(5).stroke("black").fill("none"));

        svg.img(&self.img,0.,0.,w,h);
    }
}




fn main() {
    let cfg = Cfg::load_first("-c",&["conf.lz"]);

    let w:usize= cfg.get_t(("-w","width")).unwrap_or(4);
    let h:usize= cfg.get_t(("-h","height")).unwrap_or(4);

    let imgs = cfg.get_s(("-imgs","images")).unwrap();//no image no svg
    let counts = cfg.get_s(("-nn","img_count")).unwrap_or("1".to_string());
    let out_base = cfg.get_s(("-out","out")).unwrap_or("out/cards".to_string());

    let do_pdf = cfg.get_s(("-pdf","pdf"));

    let landscape = cfg.get_s(("-landscape","landscape")).is_some();


    let counts:Vec<i32> = counts.split(':').map(|s|s.parse().unwrap_or(1)).collect();

    let mut cards = Vec::new();

    for (i,img) in imgs.split(':').enumerate() {
        let cn = i % counts.len();
        for _ in 0..counts[cn]{
            cards.push(ImgCard::new(img.to_string()));
        }
    }
    println!("CARDS:{:?}",cards);

    
    let pages = if landscape {
        page::pages(out_base,page::a4_height(),page::a4_width(),w,h,&cards)
    }else {
        page::pages_a4(out_base,w,h,&cards)
    };

    if let Some(pdpath) = do_pdf{
        page::unite_as_pdf(pages,pdpath);
    }
}



