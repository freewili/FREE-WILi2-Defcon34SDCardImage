/* Freestanding libc-subset for wasm32 guest modules: -O0 keeps memcpy and
 * friends as real calls, and without these definitions wasm-ld
 * (--allow-undefined) would turn them into unresolvable env imports that
 * fault on device. */
#include <string.h>

void * memcpy(void * dst, const void * src, size_t n)
{
    unsigned char * d = (unsigned char *)dst;
    const unsigned char * s = (const unsigned char *)src;
    while (n--) *d++ = *s++;
    return dst;
}

void * memmove(void * dst, const void * src, size_t n)
{
    unsigned char * d = (unsigned char *)dst;
    const unsigned char * s = (const unsigned char *)src;
    if (d < s) { while (n--) *d++ = *s++; }
    else if (d > s) { d += n; s += n; while (n--) *--d = *--s; }
    return dst;
}

void * memset(void * dst, int v, size_t n)
{
    unsigned char * d = (unsigned char *)dst;
    while (n--) *d++ = (unsigned char)v;
    return dst;
}

int memcmp(const void * a, const void * b, size_t n)
{
    const unsigned char * x = (const unsigned char *)a;
    const unsigned char * y = (const unsigned char *)b;
    for (; n--; ++x, ++y)
        if (*x != *y) return (int)*x - (int)*y;
    return 0;
}

size_t strlen(const char * s)
{
    const char * p = s;
    while (*p) ++p;
    return (size_t)(p - s);
}
