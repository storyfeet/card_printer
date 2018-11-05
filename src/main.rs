extern crate lazy_conf;
extern crate mksvg;

use lazy_conf::{config};
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
    let mut cfg = config("-c",&["conf.lz"]);

    let w:usize = cfg.grab().fg("-w").cf("width").help("Columns").t().unwrap_or(4);
    let h:usize = cfg.grab().fg("-h").cf("height").t().unwrap_or(4);

    let imgs_ = cfg.grab().fg("-imgs").cf("images").s_req("Image names needed ':' separated in quotes");
    let counts = cfg.grab().fg("-nn").cf("img_count")
                    .s().unwrap_or("1".to_string());

    let out_base = cfg.grab().fg("-out").cf("out")
                    .s().unwrap_or("out/cards".to_string());

    let do_pdf = cfg.grab().fg("-pdf").cf("pdf").s();

    let landscape = cfg.grab().fg("-landscape").cf("landscape").help("Make landscape").is_present();


    let counts:Vec<i32> = counts.split(':').map(|s|s.parse().unwrap_or(1)).collect();

    if cfg.help("Card Printer") {return};

    let mut cards = Vec::new();

    for (i,img) in imgs_.unwrap().split(':').enumerate() {
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



