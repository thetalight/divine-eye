import sys
from fields import S, M, B, ES, EM, EB
from fft import fft, inv_fft, log2

fri_proof = None
test_stark_output = None

def test_fft():
    INPUT_SIZE = 8
    data = [B(i+1) for i in range(INPUT_SIZE)]
    coeffs = fft(data)
    data2 = inv_fft(coeffs)
    assert data2 == data
    print("Basic FFT test passed")


## reference: https://github.com/ethereum/research
if __name__ == '__main__':
        test_fft()