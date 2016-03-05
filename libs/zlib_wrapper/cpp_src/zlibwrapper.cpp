#include "zlibwrapper.hpp"
#include "zlib.h"

#include <stdlib.h>
#include <fstream>

/* СЛУЖЕБНЫЕ ФУНКЦИИ ДЛЯ ОТЛАДКИ */

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

/* ОСНОВНЫЕ ФУНКЦИИ */

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

	const int bufferLen    = sourceDataSize;
	BYTE buffer[bufferLen] = {0};
	usize readDataSize     = 0;
	usize balanceDataSize  = sourceDataSize;

	BYTE *pDataNotConst = const_cast<BYTE*>(sourceData);

	while ( balanceDataSize > 0 ) {
		const usize currentDataSize = (balanceDataSize > bufferLen) ? bufferLen : balanceDataSize;

		strm.avail_in = currentDataSize;
		strm.next_in  = pDataNotConst + readDataSize;

		readDataSize += currentDataSize;
		balanceDataSize -= currentDataSize;

		do {
			strm.avail_out = bufferLen;
			strm.next_out  = buffer;

			ret = deflate(&strm, balanceDataSize == 0 ? Z_FINISH : Z_NO_FLUSH);

			const usize deflateLen   = bufferLen - strm.avail_out;
			const usize current_size = *compressDataSize;

			BYTE *compressDataTemp = (BYTE*) realloc (*compressData, current_size + deflateLen);

			if ( compressDataTemp != nullptr ) {
				*compressData = compressDataTemp;
				*compressDataSize += deflateLen;
				memcpy(*compressData+current_size, buffer, deflateLen);
			} else {
				*compressDataSize = 0;
				free (compressData);
				(void)deflateEnd(&strm);
		        return false;
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

	z_stream strm;
	strm.zalloc = Z_NULL;
	strm.zfree = Z_NULL;
	strm.opaque = Z_NULL;
	strm.avail_in = 0;
	strm.next_in = Z_NULL;
	int ret = inflateInit2(&strm, -MAX_WBITS);
	if ( ret != Z_OK ) {
		return false;
	}

	BYTE *pDataNotConst = const_cast<BYTE*>(sourceData);
	strm.avail_in = sourceDataSize;
	strm.next_in  = pDataNotConst;

	const int bufferLen = sourceDataSize + 1024;
	BYTE buffer[bufferLen] = {0};

	do {
		strm.avail_out = bufferLen;
		strm.next_out  = buffer;
		ret = inflate(&strm, Z_NO_FLUSH);
		if ( ret == Z_STREAM_ERROR) {
			*decompressDataSize = 0;
			free (decompressData);
			(void) inflateEnd(&strm);
			return false;
		}

		const usize inflateLen = bufferLen - strm.avail_out;
		const usize current_size = *decompressDataSize;

		BYTE *decompressDataTemp = (BYTE*) realloc (*decompressData, current_size + inflateLen);

		if ( decompressDataTemp != nullptr ) {
			*decompressData = decompressDataTemp;
			*decompressDataSize += inflateLen;
			memcpy(*decompressData+current_size, buffer, inflateLen);
		} else {
			*decompressDataSize = 0;
			free (decompressData);
			(void) inflateEnd(&strm);
	        return false;
		}

	} while ( strm.avail_out == 0 );

	(void) inflateEnd(&strm);

	return (sourceDataSize > 0) && (*decompressDataSize == 0) ? false : true;
}

/**
 * Освобождение выделенной памяти
 * @param динамически выделенная память, которую необходимо освободить
 */
void free_data(BYTE **data)
{
	free (*data);
}