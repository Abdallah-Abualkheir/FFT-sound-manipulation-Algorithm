# Final Project
## Fast Fourier Transform
By: Caleb Eby, Abed Abu Al Kheir, Adam Callahan, Travis Plunkett

# Description and Goals
For our final project we wanted to implement a fast fourier transform to process audio files. Our initial idea was to take an audio file (.wav) and put it through our FFT, change a few values in the output array, and then put it back through an inverse FFT function, and play back an output audio file (.wav). Currently we have a working FFT algorithm and an IFFT (inverse) algorithm. The inverse algorithm uses a conjugation of the frequencies array and then passes it through our forward FFT method again.

We are able to process wave files (.wav), and put them through our FFT algorithm, manipulate the frequencies array, and then pass it through our inverse FFT function, then play an output.wav file with the distorted frequencies. If we have no distortions there is no noticable difference between the input.wav file and the output.wav file.


# To run our program:
You need to have rust and the creek library installed, and the appropriate input.wav file installed in the folder with the code.


# Assumptions made:
We assumed our user knows how to work with Visual Studio Code, and can run a program in rust. In order to manipulate the frequencies of the output.wav file the user must understand how to cut off certain frequencies and what numbers correlate with certain pitches in the audio.


# Code Description:
We made our own Complex Number class since the FFT algorithm deals with the imaginary number i, equal to the sqrt of -1. We made functions so that our class can do addition, subtraction, multiplication, as well as conjugate and magnitude functions.

Our key functions are:
fft - our Fast Fourier Transform function which takes in a vector of complex numbers and outputs a transformed vector of complex numbers. We use an algorithm based on the Cooley-Tukey algorithm to transform our vector and output the frequencies vector.

ifft - our Inverse Fast Fourier Transform function which takes in a vector of complex numbers and outputs a vector of f32 (32 bit floating point). We tested several different methods for this but eventually landed on a method that involves taking the conjugate of the input vector of frequencies, passing that conjugated vector back through our forward FFT algorithm, then performing another conjugation on each value of the vector returned from that then converting it to f32.


**TODO**
Audio stuff from creek library

Audio manipulation through frequency array transforms


# Works Cited

https://www.dsprelated.com/showarticle/800.php <br>
https://github.com/MeadowlarkDAW/creek <br>
https://en.wikipedia.org/wiki/Cooley%E2%80%93Tukey_FFT_algorithm
