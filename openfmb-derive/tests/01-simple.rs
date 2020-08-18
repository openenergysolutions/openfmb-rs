
#[derive(Debug, Default, ::openfmb_derive::OpenFMB)]
struct Power {
    p: f64,
    q: f64,
    s: f64,
    i: Option<f64>,
}

#[derive(Debug, Default, ::openfmb_derive::OpenFMB)]
struct Meter {
    #[openfmb(inherit)]
    pow: Option<Power>
}

fn main() {
    let pwr = Power{
        p: 1.2,
        q: 2.3,
        s: 3.4,
        i: Some(4.5),
    };
    println!("{:?}", pwr);
}
