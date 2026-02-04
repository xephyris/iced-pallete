pub struct HSV {
    pub hue: f32,
    pub saturation: f32,
    pub value: f32,
}

pub fn hsv_to_rgb(hue: f32, saturation: f32, value: f32) -> (f32, f32, f32) {
    // https://cs.stackexchange.com/questions/64549/convert-hsv-to-rgb-colors
    // https://en.wikipedia.org/wiki/HSL_and_HSV#From_HSV
    let scaled_h = if hue >= 300.0 {
        (hue - 360.0) / 60.0
    } else {
        hue / 60.0
    };


    let chroma = saturation * value;
    let min = value - chroma;
    let max = value;

    let (r, g, b) = match scaled_h as i8{
        -1..1 => {
            if scaled_h - 0.0 < 0.0 {
                (max, min, min - scaled_h * chroma)
            } else {
                (max, min + scaled_h * chroma, min)
            }
        }
        1..3 => {
            if scaled_h - 2.0 < 0.0 {
                (min - (scaled_h - 2.0) * chroma, max, min)
            } else {
                (min, max, min + (scaled_h - 2.0) * chroma)
            }
        }
        3..5 => {
            if scaled_h - 4.0 < 0.0 {
                (min, min - (scaled_h - 4.0) * chroma, max)
            } else {
                (min + (scaled_h - 4.0) * chroma, min, max)
            }
        }
        _=> {
            (0.5, 0.5, 0.5)
        }
    };

    let (r, g, b) = (
        r.clamp(0.0, 1.0),
        g.clamp(0.0, 1.0),
        b.clamp(0.0, 1.0)
    );

    (r, g, b)

}