from Crypto.Cipher import AES

key = b"128-bit long key"

encryptors = {
    "ECB": AES.new(key, AES.MODE_ECB),
    "CBC": AES.new(key, AES.MODE_CBC),
    "CTR": AES.new(key, AES.MODE_CTR),
    "CFB": AES.new(key, AES.MODE_CFB),
    "OFB": AES.new(key, AES.MODE_OFB)
}

files = [
    "data/small.txt",
    "data/medium.txt",
    "data/large.txt"
]

with open("results.txt", "w") as results:
    for file in files:
        with open(file) as file:
            pass

# with open("data/")