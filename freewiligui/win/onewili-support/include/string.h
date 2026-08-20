/* Freestanding string.h for wiliclang wasm32 guest builds. Implementations
 * live in ow_freestanding.c, compiled into every guest module so no env
 * imports leak into the wasm (the device resolves only wiliwasm.* imports). */
#ifndef _OW_STRING_H
#define _OW_STRING_H
#include <stddef.h>
#ifdef __cplusplus
extern "C" {
#endif
void * memcpy(void * dst, const void * src, size_t n);
void * memmove(void * dst, const void * src, size_t n);
void * memset(void * dst, int v, size_t n);
int    memcmp(const void * a, const void * b, size_t n);
size_t strlen(const char * s);
#ifdef __cplusplus
}
#endif
#endif
