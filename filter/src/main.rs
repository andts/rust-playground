//use std::f32::consts::PI;
use std::i16;

extern crate hound;

fn main() {
    let file_spec = hound::WavSpec {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("out.wav", file_spec).unwrap();

    let current_cutoff_freq = 0.40f32;
    let current_resonance = 0.8f32;

    //filter coefficients
    let mut q = 1.0f32 - current_cutoff_freq;
    let p = current_cutoff_freq + 0.8f32 * current_cutoff_freq * q;
    let f = p + p - 1.0f32;
    q = current_resonance * (1.0f32 + 0.5f32 * q * (1.0f32 - q + 5.6f32 * q * q));

    println!("q {:?}", q);
    println!("p {:?}", p);
    println!("f {:?}", f);

    //filter output buffer
    let mut b: [f32; 5] = [0f32; 5];

    println!("b {:?}", b);
    println!("b0 {:?}", b[0]);
    println!("b1 {:?}", b[1]);
    println!("b2 {:?}", b[2]);
    println!("b3 {:?}", b[3]);
    println!("b4 {:?}", b[4]);

    let mut reader = hound::WavReader::open("downlow.wav").unwrap();
    for sample in reader.samples::<i16>()/*.take(1000)*/ {
//        println!("b before {:?}", b);

        //filter processing
        let sample_val = sample.unwrap() as f32;
//        println!("sample_val {}", sample_val);
        let input2 = sample_val - q * b[4];                          //feedback
//        println!("input2 {}", input2);

        let t1 = b[1];
        b[1] = (input2 + b[0]) * p - b[1] * f;
        let t2 = b[2];
        b[2] = (b[1] + t1) * p - b[2] * f;
        let t1_1 = b[3];
        b[3] = (b[2] + t2) * p - b[3] * f;
        b[4] = (b[3] + t1_1) * p - b[4] * f;
        //b[4] = b[4] - (b[4] * b[4] * b[4] * 0.366667f32);    //clipping
        b[0] = input2;
//        println!("b after {:?}", b);

        //filtering result
        let sample = b[4];

//        println!("sample {}", sample);
//        let amplitude = i16::MAX as f32;
        writer.write_sample(sample as i16).unwrap();
//        break;
    }
}
