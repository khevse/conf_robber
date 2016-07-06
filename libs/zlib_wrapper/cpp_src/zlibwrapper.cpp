#include "zlibwrapper.h"
#include "zlib.h"

#include <stdlib.h>
#include <exception>

#define BUFFER_SIZE 16384

/**
 * Сжатие данных
 *
 * @param исходные данные
 * @param размер исходных данных
 * @param сжатые данные
 * @param размер сжатых данных
 *
 * @result true - сжатие выполненно успешно
 */
bool compress_data(const BYTE *sourceData, usize sourceDataSize, BYTE **compressData, usize *compressDataSize)
{
    *compressData = nullptr;
    *compressDataSize = 0;

    z_stream strm;
    strm.zalloc = Z_NULL;
    strm.zfree  = Z_NULL;
    strm.opaque = Z_NULL;

    int ret = deflateInit2(&strm, Z_BEST_COMPRESSION, Z_DEFLATED, -MAX_WBITS, 8, Z_DEFAULT_STRATEGY);

    if (ret != Z_OK) {
        return false;
    }

    BYTE buffer[BUFFER_SIZE] = {0};
    usize readDataSize     = 0;
    usize balanceDataSize  = sourceDataSize;

    BYTE *pDataNotConst = const_cast<BYTE*>(sourceData);

    BYTE *compressDataTemp = nullptr;
    usize compressDataTempSize = 0;

    int flush = 0;

    while ( balanceDataSize > 0 ) {
        const usize currentDataSize = (balanceDataSize > BUFFER_SIZE) ? BUFFER_SIZE : balanceDataSize;

        strm.avail_in = currentDataSize;
        strm.next_in  = pDataNotConst + readDataSize;

        readDataSize += currentDataSize;
        balanceDataSize -= currentDataSize;

        do {
            strm.avail_out = BUFFER_SIZE;
            strm.next_out  = buffer;

            ret = deflate(&strm, Z_BLOCK);
            if (ret == Z_STREAM_ERROR) {
                if (compressData) {
                    free (compressData);
                }

                *compressDataSize = 0;
                break;
            }

            const usize deflateLen   = BUFFER_SIZE - strm.avail_out;
            const usize current_size = *compressDataSize;

            if ( current_size + deflateLen > compressDataTempSize ) {
                compressDataTempSize = (compressDataTempSize == 0 ? sourceDataSize : compressDataTempSize) * 2;
                compressDataTemp = (BYTE*) realloc (*compressData, compressDataTempSize);
            }

            if ( compressDataTemp != nullptr ) {
                *compressData = compressDataTemp;
                *compressDataSize += deflateLen;
                memcpy(*compressData+current_size, buffer, deflateLen);
            } else {
                if (compressData) {
                    free (compressData);
                }
                *compressDataSize = 0;
                break;
            }

        } while (strm.avail_out == 0 && ret != Z_STREAM_END);
    }

    (void)deflateEnd(&strm);

    return (sourceDataSize > 0) && (*compressDataSize == 0) ? false : true;
}

/**
 * Распаковка данных
 *
 * @param исходные данные
 * @param размер исходных данных
 * @param распакованные данные
 * @param размер распакованных данных
 *
 * @result true - распаковка выполненна успешно
 */
bool decompress_data(const BYTE *sourceData, usize sourceDataSize, BYTE **decompressData, usize *decompressDataSize)
{
    *decompressData = nullptr;
    *decompressDataSize = 0;

    usize resultBufferSize(BUFFER_SIZE), resultSize(0);
    BYTE *resultBuffer, *resultBufferOld;

    if( (resultBuffer = (BYTE *) malloc(resultBufferSize)) == NULL ) {
        return false;
    }

    z_stream strm;
    strm.zalloc = Z_NULL;
    strm.zfree = Z_NULL;
    strm.opaque = 0;

    BYTE *pDataNotConst = const_cast<BYTE*>(sourceData);
    strm.avail_in = sourceDataSize;
    strm.next_in  = pDataNotConst;

    int ret = inflateInit2(&strm, -MAX_WBITS);
    if ( ret != Z_OK ) {
        free( resultBuffer );
        return false;
    }

    BYTE buffer[BUFFER_SIZE] = {0};

    do {
        strm.avail_out = BUFFER_SIZE;
        strm.next_out  = buffer;
        ret = inflate(&strm, Z_BLOCK);

        if ( ret == Z_STREAM_ERROR) {
            free( resultBuffer );
            resultSize = 0;
            break;
        }

        const usize inflateLen = BUFFER_SIZE - strm.avail_out;
        const usize current_size = resultSize;

        if ( (current_size + inflateLen) > resultBufferSize ) {
            resultBufferSize = resultBufferSize * 2;
            resultBufferOld  = resultBuffer;
            if( (resultBuffer = (BYTE*) realloc(resultBuffer, resultBufferSize)) ==  NULL ) {
                free( resultBufferOld );
                resultSize = 0;
                break;
            }
        }

        resultSize += inflateLen;
        memcpy(resultBuffer+current_size, buffer, inflateLen);

    } while ( strm.total_in < sourceDataSize );

    if ( resultSize > 0 ) {
        resultBufferOld = resultBuffer;
        if( (resultBuffer = (BYTE*) realloc(resultBuffer, resultSize)) ==  NULL ) {
            free( resultBufferOld );
        } else {
            *decompressData = resultBuffer;
            *decompressDataSize = resultSize;
        }
    }

    (void) inflateEnd(&strm);

    return (sourceDataSize > 0) && (*decompressDataSize > 0);
}

/**
 * Освобождение выделенной памяти
 * @param динамически выделенная память, которую необходимо освободить
 */
void free_data(BYTE **data)
{
    free (*data);
}