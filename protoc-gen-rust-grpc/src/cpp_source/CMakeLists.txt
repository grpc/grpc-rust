cmake_minimum_required(VERSION 3.14)
project(protoc-gen-rust-grpc LANGUAGES CXX)

# Parse version from Cargo.toml unless already set (e.g. from CI or environment)
if(NOT RUST_GRPC_VERSION)
    if(DEFINED ENV{RUST_GRPC_VERSION})
        set(RUST_GRPC_VERSION "$ENV{RUST_GRPC_VERSION}")
    else()
        file(READ "${CMAKE_CURRENT_SOURCE_DIR}/../../Cargo.toml" CARGO_TOML_CONTENT)
        string(REGEX MATCH "\nversion[ \t]*=[ \t]*\"([^\"]+)\"" _ "${CARGO_TOML_CONTENT}")
        set(RUST_GRPC_VERSION ${CMAKE_MATCH_1})
    endif()
endif()
add_compile_definitions(RUST_GRPC_VERSION="${RUST_GRPC_VERSION}")

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

if(MINGW)
    add_compile_options("-Wa,-mbig-obj")
    add_compile_definitions(_WIN32_WINNT=0x0601)
elseif(MSVC)
    add_compile_options("/bigobj")
endif()

# Options
set(PROTOBUF_VERSION "34.0" CACHE STRING "Version of protobuf to download")
option(BUILD_SHARED_LIBS "Build shared libraries" OFF)
option(BUILD_PROTOC "Build protoc executable" ON)
option(BUILD_PLUGIN "Build protoc-gen-rust-grpc plugin" ON)

# Add cmake directory to module path
list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake")

# Include modules
include(FetchContent)
include(FetchProtobuf)

# Download and configure abseil
# Abseil version that's compatible with protobuf
set(ABSEIL_VERSION "20240722.0")
set(ABSEIL_URL "https://github.com/abseil/abseil-cpp/archive/refs/tags/${ABSEIL_VERSION}.tar.gz")

message(STATUS "Downloading abseil-cpp ${ABSEIL_VERSION}")

FetchContent_Declare(
    absl
    URL ${ABSEIL_URL}
    URL_HASH SHA256=f50e5ac311a81382da7fa75b97310e4b9006474f9560ac46f54a9967f07d4ae3
    DOWNLOAD_EXTRACT_TIMESTAMP TRUE
)

# Set abseil build options
set(ABSL_PROPAGATE_CXX_STD ON CACHE BOOL "" FORCE)
set(ABSL_BUILD_TESTING OFF CACHE BOOL "" FORCE)
set(BUILD_TESTING OFF CACHE BOOL "" FORCE)

# Force using the fetched abseil instead of system abseil
set(CMAKE_FIND_PACKAGE_PREFER_CONFIG TRUE)
set(absl_DIR "${CMAKE_CURRENT_BINARY_DIR}/_deps/absl-build" CACHE PATH "" FORCE)
# Prevent finding system libraries
set(CMAKE_PREFIX_PATH "" CACHE STRING "" FORCE)

FetchContent_MakeAvailable(absl)

# Prevent protobuf from using a local version of abseil
set(protobuf_ABSL_PROVIDER "package" CACHE STRING "Use external Abseil" FORCE)

# Download and configure protobuf
fetch_protobuf(${PROTOBUF_VERSION})

# Add the protoc-gen-rust-grpc executable and installation rules
if(BUILD_PLUGIN)
    add_executable(protoc-gen-rust-grpc
        src/grpc_rust_plugin.cc
        src/grpc_rust_generator.cc
        src/grpc_rust_generator.h
    )

    # Link against protobuf and abseil
    target_link_libraries(protoc-gen-rust-grpc
        PRIVATE
            protobuf::libprotoc
            protobuf::libprotobuf
            absl::flat_hash_map
            absl::strings
            absl::string_view
    )

    # Include directories
    target_include_directories(protoc-gen-rust-grpc
        PRIVATE
            ${protobuf_SOURCE_DIR}/src
            ${CMAKE_CURRENT_SOURCE_DIR}
    )

    # Set output directory
    set_target_properties(protoc-gen-rust-grpc
        PROPERTIES
            RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin
    )

    install(TARGETS protoc-gen-rust-grpc
        RUNTIME DESTINATION bin
    )
endif()

if(BUILD_PROTOC)
    # Add protoc executable as a custom target that copies the built protoc and installation rules
    # We explicitly name it "protoc" without version suffix
    if(WIN32)
        set(PROTOC_OUTPUT_NAME "protoc.exe")
    else()
        set(PROTOC_OUTPUT_NAME "protoc")
    endif()

    add_custom_target(copy-protoc ALL
        COMMAND ${CMAKE_COMMAND} -E copy_if_different
            $<TARGET_FILE:protoc>
            ${CMAKE_BINARY_DIR}/bin/${PROTOC_OUTPUT_NAME}
        DEPENDS protoc
        COMMENT "Copying protoc to output directory"
    )

    install(PROGRAMS ${CMAKE_BINARY_DIR}/bin/${PROTOC_OUTPUT_NAME}
        DESTINATION bin
    )

    install(DIRECTORY ${protobuf_SOURCE_DIR}/src/google/protobuf/
        DESTINATION include/google/protobuf
        FILES_MATCHING PATTERN "*.proto"
    )
endif()

# Print summary
message(STATUS "")
message(STATUS "protoc-gen-rust-grpc configuration summary:")
message(STATUS "  Crate Version: ${RUST_GRPC_VERSION}")
message(STATUS "  Protobuf version: ${PROTOBUF_VERSION}")
message(STATUS "  Abseil version: ${ABSEIL_VERSION}")
message(STATUS "  Build type: ${CMAKE_BUILD_TYPE}")
message(STATUS "  C++ standard: ${CMAKE_CXX_STANDARD}")
message(STATUS "  Output directory: ${CMAKE_BINARY_DIR}/bin")
message(STATUS "  Build protoc: ${BUILD_PROTOC}")
message(STATUS "  Build plugin: ${BUILD_PLUGIN}")
