from Crypto.Cipher import AES

key = b"128-bit long key"

cipher = AES.new(key, AES.MODE_ECB)