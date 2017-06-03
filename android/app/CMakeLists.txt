cmake_minimum_required(VERSION 3.6.0)

set(ANDROID_BUILD_MODE debug)

if(ANDROID_ABI STREQUAL x86)
    set(SODIUM_ABI i686)
elseif(ANDROID_ABI STREQUAL armeabi)
    set(SODIUM_ABI armv7)
elseif(ANDROID_ABI STREQUAL armeabi-v7a)
    set(SODIUM_ABI armv7-a)
elseif(ANDROID_ABI STREQUAL arm64-v8a)
    set(SODIUM_ABI armv8-a)
endif()

message("LIB DIR ${CMAKE_LIBRARY_OUTPUT_DIRECTORY}")

add_library(libsodium SHARED IMPORTED)
set_target_properties(libsodium PROPERTIES
                      IMPORTED_LOCATION ${CMAKE_SOURCE_DIR}/../../libsodium/libsodium-android-${SODIUM_ABI}/lib/libsodium.so
                      IMPORTED_NO_SONAME true)
include_directories( ${CMAKE_SOURCE_DIR}/../../libsodium/libsodium-android-${SODIUM_ABI}/include )
file(COPY ${CMAKE_SOURCE_DIR}/../../libsodium/libsodium-android-${SODIUM_ABI}/lib/libsodium.so
     DESTINATION ${CMAKE_LIBRARY_OUTPUT_DIRECTORY})

add_library(freepass-capi SHARED IMPORTED)
set_target_properties(freepass-capi PROPERTIES
                      IMPORTED_LOCATION ${CMAKE_SOURCE_DIR}/../../capi/target/android-all/${ANDROID_BUILD_MODE}/${ANDROID_ABI}/libfreepass_capi.so
                      IMPORTED_NO_SONAME true)
include_directories( ${CMAKE_SOURCE_DIR}/../../capi )
file(COPY ${CMAKE_SOURCE_DIR}/../../capi/target/android-all/${ANDROID_BUILD_MODE}/${ANDROID_ABI}/libfreepass_capi.so
     DESTINATION ${CMAKE_LIBRARY_OUTPUT_DIRECTORY})

find_library(log-lib log)

add_library(jniVault SHARED src/main/jni/jniVault.cpp)

target_link_libraries(jniVault freepass-capi libsodium ${log-lib})