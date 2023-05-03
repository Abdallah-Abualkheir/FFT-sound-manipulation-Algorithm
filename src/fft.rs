use std::f32::consts::PI;

use crate::complex_number::ComplexNumber;

pub(crate) fn fft(input: &[ComplexNumber]) -> Vec<ComplexNumber> {
    if input.len() == 1 {
        //Base case when input is size 1
        input.to_vec()
    } else {
        let mut output: Vec<ComplexNumber> = vec![]; //output vector of complex numbers
        let even_terms: Vec<ComplexNumber> = input
            .iter()
            .cloned()
            .enumerate()
            .filter_map(|(i, val)| if i % 2 == 0 { Some(val) } else { None })
            .collect();
        let even: Vec<ComplexNumber> = fft(&even_terms); //Call FFT recursively output
        let odd_terms: Vec<ComplexNumber> = input
            .iter()
            .cloned()
            .enumerate()
            .filter_map(|(i, val)| if i % 2 == 1 { Some(val) } else { None })
            .collect();
        let odd: Vec<ComplexNumber> = fft(&odd_terms); //Call FFT recursively output

        //even and odd returns go to output array
        output.extend_from_slice(&even); //combine even half and odd half into a single output array
        output.extend_from_slice(&odd);

        for k in 0..(input.len() / 2) {
            let p = output[k].clone();
            let theta = -2.0 * PI * (k as f32) / (input.len() as f32);
            let q = &ComplexNumber {
                real: f32::cos(theta),
                imaginary: f32::sin(theta),
            } * &output[k + input.len() / 2]; //We use eulers formula: cos + i*sin to replace e^-2pik/N
            output[k] = &p + &q;
            output[k + input.len() / 2] = &p - &q;
        }
        output
    }
}

/// Checks if all the floating-point numbers in a slice areal approx. equal
#[macro_export]
macro_rules! assert_approx_eq {
    ($lhs:expr, $rhs:expr) => {
        if $lhs.len() != $rhs.len() {
            panic!(
                "assertion failed, expected the floating-point arrays to approximately equal:\n\
                            {:?}\n\
                            {:?}\n\
                But they had different lengths: {} and {}",
                $lhs, $rhs,
                $lhs.len(), $rhs.len()
            );
        }

        let min_diff = 1e-5;

        for i in 0..$lhs.len() {
            if ($lhs[i] - $rhs[i]).abs() > min_diff {
                let ext = 20;
                let nearby_values = (std::cmp::max((i as isize)-ext as isize, 0) as usize..std::cmp::min(i+ext, $lhs.len()))
                    .map(|i| format!("[{i}]: {} {}\n", $lhs[i], $rhs[i]))
                    .collect::<String>();
                panic!(
                    "assertion failed, expected the floating-point arrays to approximately equal:\n\
                                {:?}\n\
                                {:?}
                    The items at [{i}] differed: {:?} and {:?}\n\
                    {}",
                    $lhs, $rhs, $lhs[i], $rhs[i], nearby_values
                );
            }
        }
    };
}

//Test Code

// function that turns a range of integer numbers ex: 0,1,2,3,4,5,... to floating points that increase by steps-
//- in this case the steps areal increments of 0.01. so 0,1,2,3,4,5 is gonna be 0.00,0.01, 0.02, 0.03,0.0.04,0.05
fn floating_point_range(max: f32, step: f32) -> impl Iterator<Item = f32> {
    (0..(max / step) as isize).map(move |t| (t as f32) * step)
}

#[cfg(test)]
mod tests {

    use crate::ifft::ifft;

    use super::*;

    #[test]
    fn test_fourier_transform() {
        let step = 0.03125;
        let period = 2.0;
        //create input_1 vector with values from 0 to 1 in steps of 0.01
        let input_1: Vec<ComplexNumber> = floating_point_range(period, step)
            //call the sin function
            .map(|t| ComplexNumber {
                real: 7.0 * f32::sin(2.0 * PI * t) + 2.0 * f32::sin(3.0 * PI * t) + 0.1,
                imaginary: 0.0,
            })
            //storeal results
            .collect();

        let output_1 = fft(&input_1);
        println!("{output_1:?}");
        // let output_1: Vec<_> = output_1
        //     .iter()
        //     .enumerate()
        //     .map(|(i, v)| {
        //         if i >= output_1.len() - 3 {
        //             ComplexNumber {
        //                 real: 0.0,
        //                 imaginary: 0.0,
        //             }
        //         } else {
        //             v.clone()
        //         }
        //     })
        //     .collect();
        println!("fixed: {output_1:?}");
        let output_scaled: Vec<_> = output_1
            .iter()
            .map(|v| ComplexNumber {
                real: v.real * step,
                imaginary: v.imaginary * step,
            })
            .collect();

        println!("{output_scaled:?}");
        let reconstructed = ifft(&output_1);
        assert_approx_eq!(
            &input_1.iter().map(|i| i.magnitude()).collect::<Vec<f32>>(),
            reconstructed
        );
        // assert_approx_eq!(
        //     &output_1,
        //     &[0.2, 0.0, 7.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
        // );
    }
}
