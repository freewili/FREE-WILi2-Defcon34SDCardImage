/* Freestanding stddef.h for wiliclang wasm32 guest builds. */
#ifndef _OW_STDDEF_H
#define _OW_STDDEF_H
typedef __SIZE_TYPE__    size_t;
typedef __PTRDIFF_TYPE__ ptrdiff_t;
#ifndef NULL
#ifdef __cplusplus
#define NULL 0
#else
#define NULL ((void*)0)
#endif
#endif
#endif
