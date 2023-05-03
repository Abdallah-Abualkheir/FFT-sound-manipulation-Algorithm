mod complex_number;
mod fft;
mod ifft;
use std::fs::OpenOptions;
use std::io::Write;

use creek::{
    wav_bit_depth, ReadDiskStream, SymphoniaDecoder, WavEncoder, WriteDiskStream,
    WriteStreamOptions,
};

use crate::complex_number::ComplexNumber;
use crate::fft::fft;
use crate::ifft::ifft;

fn main() {
    let mut read_disk_stream = ReadDiskStream::<SymphoniaDecoder>::new(
        // "./AudioFiles/The-Weeknd-The-Hills-instrumentalfx-clipped-2.wav",
        "./AudioFiles/kent.wav",
        0,
        Default::default(),
    )
    .unwrap();
    let _ = read_disk_stream.cache(0, 0);
    read_disk_stream.seek(0, Default::default()).unwrap();
    read_disk_stream.block_until_ready().unwrap();
    let num_channels = read_disk_stream.info().num_channels;
    let num_frames = read_disk_stream.info().num_frames;
    let write_options: WriteStreamOptions<WavEncoder<wav_bit_depth::Float32>> = Default::default();
    let block_size = write_options.block_size;
    let mut write_disk_stream = WriteDiskStream::<WavEncoder<wav_bit_depth::Float32>>::new(
        "./output.wav",
        num_channels,
        read_disk_stream.info().sample_rate.unwrap(),
        write_options,
    )
    .unwrap();
    let mut input_buf = vec![vec![0.0; num_frames]; num_channels as usize];

    read_disk_stream
        .fill_buffer_blocking(&mut input_buf)
        .unwrap();

    fn pad_zeros_power_of_2(input: &[f32]) -> Vec<f32> {
        let channel_size = input.len();
        // println!("ChannelSize: {channel_size}");
        // Determine the upper bound to the power of 2 we are adding
        let power_of_2 = f32::log2(channel_size as f32).ceil() as i32;
        // println!("power_of_2: {power_of_2}");
        let zeros_to_add = (2.0f64).powi(power_of_2) - channel_size as f64;
        // println!("zeros_to_add: {zeros_to_add}");
        let mut channel = input.to_vec();
        // Add corresponding amount of 0.0 entries
        channel.extend(std::iter::repeat(0.0).take(zeros_to_add as usize));

        let channel_new_size = channel.len();
        println!("new size: {channel_new_size}");
        channel
    }
    let mut frequencies_buf: Vec<Vec<ComplexNumber>> = vec![];
    // Loops through each audio channel individually and processes it
    let output_buf: Vec<Vec<f32>> = input_buf
        .iter()
        .map(|channel| {
            let channel = pad_zeros_power_of_2(channel);
            println!("start fft");
            let frequencies = fft(&channel
                .iter()
                .map(|v| ComplexNumber {
                    real: *v,
                    imaginary: 0.0,
                })
                .collect::<Vec<ComplexNumber>>());
            println!("finished fft");

            // Now we have the signal in frequency domain, so we can manipulate it.
            // All the code until we call ifft is frequency-manipulation code.

            // Since the frequencies graph is symmetric across N/2 (the lower/upper half are complex conjugates),
            // we manipulate the frequencies in the top half, and then mirror it across automatically (with conjugation)
            let positive_frequencies: Vec<ComplexNumber> = frequencies[(frequencies.len() / 2)..]
                .iter()
                .rev()
                .cloned()
                .enumerate()
                .map(|(i, amplitude)| {
                    if i > 90000 {
                        // Clip frequencies above a threshold (units of threshold is 1/(audio file length))
                        &amplitude * 0.0
                    } else {
                        // Preserve other frequencies
                        &amplitude * 1.0
                    }
                })
                .collect();
            // Shifts our frequency buffer to change the pitch
            // Note: since shifting the buffer results in shifting in 1/n (inverse function),
            // and the musical scale is exponential/logarithmic,
            // Our kind of shifting is not one-to-one with "musical shifting", e.g. transposition.
            // Different frequencies end up being shifted different amounts.
            // This was just much easier to implement than a true "musical shift" or transposition.
            let shift_amount = 25000;
            let mut output_frequencies: Vec<ComplexNumber> =
                vec![ComplexNumber::zero(); shift_amount];
            output_frequencies.extend_from_slice(&positive_frequencies);
            output_frequencies.truncate(positive_frequencies.len());
            output_frequencies.reverse();
            // Frequencies_buf is just used for plotting
            frequencies_buf.push(output_frequencies.iter().rev().cloned().collect());
            let frequencies: Vec<ComplexNumber> = std::iter::once(frequencies[0].clone())
                .chain(output_frequencies.iter().rev().map(|f| f.conjugate()))
                .chain(output_frequencies.iter().cloned())
                .collect();
            // Print statements for helping debug or to measure how long it takes
            println!("start inverse");
            let reconstructed = ifft(&frequencies);
            println!("finished inverse");
            reconstructed
        })
        .collect();
    let mut i = 0;
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("frequencies.csv")
        .unwrap();

    // This was test code used to ensure that the forward transform -> inverse transform
    // resulted in (approximately) the original data:
    // let input_padded = pad_zeros_power_of_2(&input_buf[0]);
    // assert_approx_eq!(input_padded, output_buf[0]);
    writeln!(file, "Amplitude").unwrap();
    let out_string: String = frequencies_buf[0]
        .iter()
        .map(|v| format!("{}\n", v.magnitude()))
        .collect();
    writeln!(file, "{}", out_string).unwrap();
    loop {
        println!("write loop chunk {i}");
        let end = std::cmp::min(i + block_size, num_frames);
        let buf_to_write: Vec<&[f32]> = output_buf
            .iter()
            .map(|channel_data| &channel_data[i..end])
            .collect();
        // write_disk_stream.write(&buf_to_write).unwrap();
        let result = write_disk_stream.write(&buf_to_write);
        if let Err(err) = result {
            println!("exit early {err:?}: {}", err);
            break;
        }
        i += block_size;
        if i > num_frames {
            break;
        }
    }
    println!("done with loop");

    write_disk_stream.finish_and_close().unwrap();
}
