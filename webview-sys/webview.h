#ifndef WEBVIEW_H
#define WEBVIEW_H

#include <objc/objc-runtime.h>
#include <CoreGraphics/CoreGraphics.h>

typedef const char *(*callback)(const void* data, const char* string);

id create_webview(const void *parent, int width, int height, const char *html, const void* data, const void* callback);

void eval_webview(id webview, const char *js);

void destroy_webview(id webview);

#endif // WEBVIEW_H
