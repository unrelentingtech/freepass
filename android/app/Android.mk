LOCAL_PATH := $(call my-dir)

include $(CLEAR_VARS)
LOCAL_MODULE := libsodium-prebuilt
ifeq ($(TARGET_ARCH_ABI),x86)
    LOCAL_SRC_FILES := $(LOCAL_PATH)/../../libsodium/libsodium-android-i686/lib/libsodium.so
    LOCAL_EXPORT_C_INCLUDES := $(LOCAL_PATH)/../../libsodium/libsodium-android-i686/include
else ifeq ($(TARGET_ARCH_ABI),armeabi)
    LOCAL_SRC_FILES := $(LOCAL_PATH)/../../libsodium/libsodium-android-armv6/lib/libsodium.so
    LOCAL_EXPORT_C_INCLUDES := $(LOCAL_PATH)/../../libsodium/libsodium-android-armv6/include
else ifeq ($(TARGET_ARCH_ABI),armeabi-v7a)
    LOCAL_SRC_FILES := $(LOCAL_PATH)/../../libsodium/libsodium-android-armv7-a/lib/libsodium.so
    LOCAL_EXPORT_C_INCLUDES := $(LOCAL_PATH)/../../libsodium/libsodium-android-armv7-a/include
else ifeq ($(TARGET_ARCH_ABI),arm64-v8a)
    LOCAL_SRC_FILES := $(LOCAL_PATH)/../../libsodium/libsodium-android-armv8-a/lib/libsodium.so
    LOCAL_EXPORT_C_INCLUDES := $(LOCAL_PATH)/../../libsodium/libsodium-android-armv8-a/include
endif
include $(PREBUILT_SHARED_LIBRARY)


include $(CLEAR_VARS)
LOCAL_MODULE := freepass-capi-prebuilt
LOCAL_SRC_FILES := $(LOCAL_PATH)/../../capi/target/android-all/release/$(TARGET_ARCH_ABI)/libfreepass_capi.so
LOCAL_EXPORT_C_INCLUDES := $(LOCAL_PATH)/../../capi/
LOCAL_SHARED_LIBRARIES += libsodium-prebuilt
include $(PREBUILT_SHARED_LIBRARY)


include $(CLEAR_VARS)
LOCAL_CPP_FEATURES += exceptions
LOCAL_MODULE := jniVault
LOCAL_CFLAGS += -std=c++11
# Fucking NDK! Using LOCAL_SHARED_LIBRARIES results in THE FULL ABSOLUTE PATH OF THE DEPENDENCY BEING WRITTEN INTO THE LIBRARY
LOCAL_LDFLAGS += -Wl,--build-id -L$(LOCAL_PATH)/../../capi/target/android-all/release/$(TARGET_ARCH_ABI) -lfreepass_capi
LOCAL_LDLIBS += -llog
LOCAL_C_INCLUDES += $(LOCAL_PATH)/../../capi/
LOCAL_SRC_FILES += $(LOCAL_PATH)/src/main/jni/jniVault.cpp
include $(BUILD_SHARED_LIBRARY)