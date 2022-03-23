from time import time
from ntpath import basename
from Crypto.Cipher import AES
from Crypto.Util.Padding import pad, unpad


def measure(method, *args, **kwargs):
    ts = time()
    result = method(*args)
    te = time()

    if 'log_sink' in kwargs and 'log_name' in kwargs:
        name = kwargs.get('log_name')
        kwargs['log_sink'][name] = int((te - ts) * 1000)
    else:
        print(f"{method.__name__}: {int((te - ts) * 1000)}")

    return result


key = b"128-bit long key"

encryptors = {
    "ECB": AES.new(key, AES.MODE_ECB),
    "CBC": AES.new(key, AES.MODE_CBC, b'_initialization_'),
    "CTR": AES.new(key, AES.MODE_CTR, nonce=b'==none=='),
    "CFB": AES.new(key, AES.MODE_CFB, b'_initialization_'),
    "OFB": AES.new(key, AES.MODE_OFB, b'_initialization_')
}

decryptors = {
    "ECB": AES.new(key, AES.MODE_ECB),
    "CBC": AES.new(key, AES.MODE_CBC, b'_initialization_'),
    "CTR": AES.new(key, AES.MODE_CTR, nonce=b'==none=='),
    "CFB": AES.new(key, AES.MODE_CFB, b'_initialization_'),
    "OFB": AES.new(key, AES.MODE_OFB, b'_initialization_')
}

files = ["data/small.txt", "data/medium.txt", "data/large.txt"]

sink = {}

for file in files:
    with open(file, 'rb') as file:
        contents = file.read(-1)
        for name, encryptor, name, decryptor in zip(encryptors.keys(),
                                                    encryptors.values(),
                                                    decryptors.keys(),
                                                    decryptors.values()):
            ciphertext = measure(
                encryptor.encrypt,
                pad(contents, AES.block_size),
                log_sink=sink,
                log_name=f"{name}_{basename(file.name)}_encrypt")
            # with open(f"modified/{basename(file.name)}_encrypted.png",
            #           'wb') as encrypted:
            #     encrypted.write(ciphertext)
            decrypted = unpad(
                measure(decryptor.decrypt,
                        ciphertext,
                        log_sink=sink,
                        log_name=f"{name}_{basename(file.name)}_decrypt"),
                AES.block_size)
            # with open(f"modified/{basename(file.name)}_decrypted.png",
            #           'wb') as crypted:
            #     crypted.write(decrypted)

            assert (contents == decrypted)

with open("results.txt", "w") as results:
    for key, val in sink.items():
        results.write(f"{key}: {val}ms\n")

with open("data/error-test.txt", "rb") as file:
    contents = file.read(-1)
    for name, encryptor, decryptor in zip(encryptors.keys(),
                                          encryptors.values(),
                                          decryptors.values()):
        ciphertext = bytearray(encryptor.encrypt(contents))
        ciphertext[0:5] = b'00000'
        ciphertext = bytes(ciphertext)
        decrypted = decryptor.decrypt(ciphertext)
        with open(f"errors/{name}_result.txt", "wb") as result:
            result.write(decrypted)