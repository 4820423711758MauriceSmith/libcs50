#include <jni.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "cs50.h"
#include "CS50.h"

// 定义Java异常类
#define EOF_EXCEPTION "LCS50$EOFException;"

// 抛出EOF异常
JNIEXPORT void JNICALL throwEOFException(JNIEnv *env, const char *message) {
    jclass eofExceptionClass = (*env)->FindClass(env, EOF_EXCEPTION);
    if (eofExceptionClass != NULL) {
        (*env)->ThrowNew(env, eofExceptionClass, message);
    }
}

// 实现getChar方法
JNIEXPORT jchar JNICALL Java_CS50_getChar(JNIEnv *env, jobject obj, jstring prompt) {
    const char *c_prompt = NULL;
    if (prompt != NULL) {
        c_prompt = (*env)->GetStringUTFChars(env, prompt, NULL);
        if (c_prompt == NULL) {
            return 0;
        }
    }
    
    char result = get_char((char *)c_prompt);
    
    if (c_prompt != NULL) {
        (*env)->ReleaseStringUTFChars(env, prompt, c_prompt);
    }
    
    if (result == CHAR_MAX) {
        throwEOFException(env, "输入错误或EOF");
        return 0;
    }
    
    return (jchar)result;
}

// 实现getDouble方法
JNIEXPORT jdouble JNICALL Java_CS50_getDouble(JNIEnv *env, jobject obj, jstring prompt) {
    const char *c_prompt = NULL;
    if (prompt != NULL) {
        c_prompt = (*env)->GetStringUTFChars(env, prompt, NULL);
        if (c_prompt == NULL) {
            return 0.0;
        }
    }
    
    double result = get_double((char *)c_prompt);
    
    if (c_prompt != NULL) {
        (*env)->ReleaseStringUTFChars(env, prompt, c_prompt);
    }
    
    if (result == DBL_MAX) {
        throwEOFException(env, "输入错误或EOF");
        return 0.0;
    }
    
    return (jdouble)result;
}

// 实现getFloat方法
JNIEXPORT jfloat JNICALL Java_CS50_getFloat(JNIEnv *env, jobject obj, jstring prompt) {
    const char *c_prompt = NULL;
    if (prompt != NULL) {
        c_prompt = (*env)->GetStringUTFChars(env, prompt, NULL);
        if (c_prompt == NULL) {
            return 0.0f;
        }
    }
    
    float result = get_float((char *)c_prompt);
    
    if (c_prompt != NULL) {
        (*env)->ReleaseStringUTFChars(env, prompt, c_prompt);
    }
    
    if (result == FLT_MAX) {
        throwEOFException(env, "输入错误或EOF");
        return 0.0f;
    }
    
    return (jfloat)result;
}

// 实现getInt方法
JNIEXPORT jint JNICALL Java_CS50_getInt(JNIEnv *env, jobject obj, jstring prompt) {
    const char *c_prompt = NULL;
    if (prompt != NULL) {
        c_prompt = (*env)->GetStringUTFChars(env, prompt, NULL);
        if (c_prompt == NULL) {
            return 0;
        }
    }
    
    int result = get_int((char *)c_prompt);
    
    if (c_prompt != NULL) {
        (*env)->ReleaseStringUTFChars(env, prompt, c_prompt);
    }
    
    if (result == INT_MAX) {
        throwEOFException(env, "输入错误或EOF");
        return 0;
    }
    
    return (jint)result;
}

// 实现getLong方法
JNIEXPORT jlong JNICALL Java_CS50_getLong(JNIEnv *env, jobject obj, jstring prompt) {
    const char *c_prompt = NULL;
    if (prompt != NULL) {
        c_prompt = (*env)->GetStringUTFChars(env, prompt, NULL);
        if (c_prompt == NULL) {
            return 0;
        }
    }
    
    long result = get_long((char *)c_prompt);
    
    if (c_prompt != NULL) {
        (*env)->ReleaseStringUTFChars(env, prompt, c_prompt);
    }
    
    if (result == LONG_MAX) {
        throwEOFException(env, "输入错误或EOF");
        return 0;
    }
    
    return (jlong)result;
}

// 实现getString方法
JNIEXPORT jstring JNICALL Java_CS50_getString(JNIEnv *env, jobject obj, jstring prompt) {
    const char *c_prompt = NULL;
    if (prompt != NULL) {
        c_prompt = (*env)->GetStringUTFChars(env, prompt, NULL);
        if (c_prompt == NULL) {
            return NULL;
        }
    }
    
    string result = get_string((char *)c_prompt);
    
    if (c_prompt != NULL) {
        (*env)->ReleaseStringUTFChars(env, prompt, c_prompt);
    }
    
    if (result == NULL) {
        throwEOFException(env, "输入错误或EOF");
        return NULL;
    }
    
    jstring java_string = (*env)->NewStringUTF(env, result);
    free_string(result);
    
    return java_string;
}