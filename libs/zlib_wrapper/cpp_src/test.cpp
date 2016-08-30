
#include "zlibwrapper.h"

#include <vector>
#include <stdint.h>
#include <exception>
#include <sstream>
#include <stdio.h>
#include <fstream>

typedef uint8_t BYTE;
typedef std::vector<BYTE> BinaryData;

void log(const char *buffer) {

    std::ofstream outfile;
    outfile.exceptions( std::fstream::failbit );
    outfile.open("zlib_wrapper_log.txt", std::ofstream::binary | std::ofstream::app);
    outfile << "\n" << buffer;
    outfile.flush();
    outfile.close();
}

void log(const BYTE *buffer) {
    log((char*)buffer);
}

BinaryData readFile(const char *path)
{
    BinaryData container;

    FILE *fp = fopen( path, "rb" );
    if ( !fp ) {
        throw std::runtime_error("Failed open file.");
    }

    try {
        container.clear();

        fseek( fp, 0, SEEK_SET );
        if ( fgetc( fp ) == EOF && ferror( fp ) != 0 ) {
            throw std::runtime_error("Failed reading begin of the file");
        }

        fseek( fp, 0, SEEK_END );
        const long filelength = ftell( fp );
        fseek( fp, 0, SEEK_SET );
        if ( filelength == -1L ) {
            throw std::runtime_error("Failed reading file size");
        }

        const size_t size = filelength;
        if ( size != 0 ) {
            container.resize(size, 0);

            size_t read = fread(container.data(), 1, size, fp );

            if ( read != size ) {
                container.clear();
                throw std::runtime_error("Failed reading file.");
            }
        }
    } catch (std::exception &ex) {
        fclose(fp);
        throw std::runtime_error( ex.what() );
    }

    fclose(fp);

    return container;
}


int main()
{
    log("Test");

    BinaryData sourceData = readFile("../../../test_data/original.cf");

    char buf[1024] = {0};

    do {
        sprintf(buf,"----\nsourceData.size=%d\n----", sourceData.size());
        log(buf);

        log("Compress");
        BYTE *compressData = nullptr;
        usize compressDataSize = 0;

        if (compress_data(sourceData.data(), sourceData.size(), &compressData, &compressDataSize)){
            log("Compress ok");
        }

        log("-Compress");

        BYTE *decompressData = nullptr;
        usize decompressDataSize = 0;

        log("Decompress");

        sprintf(buf,"compressDataSize=%d", compressDataSize);
        log(buf);

        if (decompress_data(compressData, compressDataSize, &decompressData, &decompressDataSize)){
            log("Decompress ok");
        }

        sprintf(buf,"decompressDataSize=%d", decompressDataSize);
        log(buf);

        log("-Decompress");

        BinaryData resultData(decompressData, decompressData+decompressDataSize);

        if (decompressDataSize > 0){
            free_data(&decompressData);
        }

        if (compressDataSize > 0){
            free_data(&compressData);
        }

        if (sourceData == resultData){
            log("Test ok.");
        } else {
            log("Test error");
            break;
        }

        size_t cut_size = size_t(sourceData.size()/5) > 0 ? sourceData.size()/5 : 1;
        sourceData.resize( sourceData.size() - cut_size );

    } while ( !sourceData.empty() );

    log("-Test");

    return 0;
}
