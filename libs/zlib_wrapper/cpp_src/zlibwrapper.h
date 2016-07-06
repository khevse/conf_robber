#ifndef __zlibwrapper_h__
#define __zlibwrapper_h__

#include <_mingw.h>
#include <stdint.h>
#include <stdbool.h>

typedef uint32_t usize;
typedef uint8_t BYTE;

#ifdef __cplusplus
extern "C" {
#endif

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
bool  compress_data(const BYTE *sourceData, usize sourceDataSize, BYTE **compressData, usize *compressDataSize);

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
bool  decompress_data(const BYTE *sourceData, usize sourceDataSize, BYTE **decompressData, usize *decompressDataSize);

/**
 * Освобождение выделенной памяти
 * @param динамически выделенная память, которую необходимо освободить
 */
void  free_data(BYTE **data);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif // __zlibwrapper_h__