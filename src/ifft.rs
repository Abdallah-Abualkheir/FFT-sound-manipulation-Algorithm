use crate::complex_number::ComplexNumber;
use crate::fft::fft;

pub(crate) fn ifft(frequencies: &[ComplexNumber]) -> Vec<f32> {
    //conjugate method
    //By taking the conjugate of the input frequencies, putting them through our forward fft
    //And then conjugating the output of that
    let frequencies: Vec<ComplexNumber> = frequencies.iter().map(|f| f.conjugate()).collect();
    let samples = fft(&frequencies);
    samples
        .iter()
        .map(|f| f.conjugate().real / samples.len() as f32)
        .collect()
}
