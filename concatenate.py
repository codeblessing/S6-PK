# files = ["MD-5", "SHA1", "SHA2-224", "SHA2-256", "SHA2-384", "SHA2-512", "SHA2-512-224", "SHA2-512-256", "SHA3-224", "SHA3-256", "SHA3-384", "SHA3-512", "KECCAK-224", "KECCAK-256", "KECCAK-384", "KECCAK-512"]
# collisions = []
# tiny = []
# small = []
# mid = []
# big = []
# huge = []
# sac = []

# for filename in files:
#     with open(f"output/{filename}.csv") as file:
#         collisions.append(file.readline().strip())
#         tiny.append(file.readline().strip())
#         small.append(file.readline().strip())
#         mid.append(file.readline().strip())
#         big.append(file.readline().strip())
#         huge.append(file.readline().strip())
#         sac.append(file.readline().strip())

# with open("output/all.csv", "w+") as file:
#     file.write(",".join(collisions) + '\n')
#     file.write(",".join(tiny) + '\n')
#     file.write(",".join(small) + '\n')
#     file.write(",".join(mid) + '\n')
#     file.write(",".join(big) + '\n')
#     file.write(",".join(huge) + '\n')
#     file.write(",".join(sac) + '\n')

files = ["MD-5", "SHA1", "SHA2-224", "SHA2-256", "SHA2-384", "SHA2-512", "SHA2-512-224", "SHA2-512-256", "SHA3-224", "SHA3-256", "SHA3-384", "SHA3-512", "KECCAK-224", "KECCAK-256", "KECCAK-384", "KECCAK-512"]
contents = []

with open("output/-details.txt") as file:
    for line in file.readlines():
        contents.append([line.strip()])

for filename in files:
    with open(f"output/{filename}-details.txt") as file:
        for index, line in enumerate(file.readlines()):
            contents[index].append(line.strip())

with open("output/details.csv", "w") as file:
    for line in contents:
        file.write(",".join(line) + '\n')