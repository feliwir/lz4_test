#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <lz4.h>

int main(int argc, char **argv)
{
    uint32_t compressed_size, uncompressed_size;
    // Read LZ4 file specified in first argument
    FILE *file = fopen(argv[1], "rb");
    // Read 4 bytes specifying the compressed size
    fread(&compressed_size, 4, 1, file);
    // Read 4 bytes specifying the uncompressed size
    fread(&uncompressed_size, 4, 1, file);
    // Allocate memory for the compressed data
    char *compressed_data = malloc(compressed_size);
    // Read the compressed data
    fread(compressed_data, 1, compressed_size, file);
    // Allocate memory for the uncompressed data
    char *uncompressed_data = malloc(uncompressed_size);
    // Decompress the data
    int result = LZ4_decompress_fast(compressed_data, uncompressed_data, uncompressed_size);
    // Check if the decompression was successful
    if (result != (compressed_size - 4))
    {
        printf("Decompression failed: %i vs %i\n", result, compressed_size);
        return 1;
    }

    return 0;
}