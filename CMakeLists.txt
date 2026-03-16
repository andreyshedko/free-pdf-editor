cmake_minimum_required(VERSION 3.20)

project(PDFEditor LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTORCC ON)

if(WIN32 AND MINGW)
    get_filename_component(_mingw_bin_dir "${CMAKE_CXX_COMPILER}" DIRECTORY)
    cmake_path(CONVERT "${_mingw_bin_dir};$ENV{PATH}" TO_NATIVE_PATH_LIST _mingw_launch_path)
    set(_mingw_env_launcher "\"${CMAKE_COMMAND}\" -E env \"PATH=${_mingw_launch_path}\"")
    set_property(GLOBAL PROPERTY RULE_LAUNCH_COMPILE "${_mingw_env_launcher}")
    set_property(GLOBAL PROPERTY RULE_LAUNCH_CUSTOM "${_mingw_env_launcher}")
endif()

if(MSVC)
    add_compile_options(/Zc:__cplusplus)
endif()

if(WIN32 AND DEFINED Qt6_DIR)
    string(TOLOWER "${Qt6_DIR}" _qt6_dir_lc)
    if(MSVC AND _qt6_dir_lc MATCHES "mingw")
        message(WARNING "Qt6_DIR points to a MinGW Qt kit, but the active compiler is MSVC. Re-detecting a compatible Qt installation.")
        unset(Qt6_DIR CACHE)
        unset(Qt6_DIR)
    elseif(MINGW AND _qt6_dir_lc MATCHES "msvc")
        message(WARNING "Qt6_DIR points to an MSVC Qt kit, but the active compiler is MinGW. Re-detecting a compatible Qt installation.")
        unset(Qt6_DIR CACHE)
        unset(Qt6_DIR)
    endif()
endif()

# Help CMake locate Qt6 on Windows without manual cache editing.
if(WIN32 AND NOT DEFINED Qt6_DIR)
    set(_qt_arch_suffixes)
    if(MSVC)
        list(APPEND _qt_arch_suffixes
            msvc2022_64 msvc2022
            msvc2019_64 msvc2019
            clang_64
        )
    elseif(MINGW)
        list(APPEND _qt_arch_suffixes
            mingw_64 mingw64
        )
    else()
        list(APPEND _qt_arch_suffixes
            msvc2022_64 msvc2022
            msvc2019_64 msvc2019
            mingw_64 mingw64
            clang_64
        )
    endif()

    # 1. Honor explicit environment variables first.
    foreach(_env_root "$ENV{QTDIR}" "$ENV{QT_DIR}" "$ENV{Qt6_DIR}")
        if(_env_root AND EXISTS "${_env_root}/lib/cmake/Qt6/Qt6Config.cmake")
            set(_is_compatible TRUE)
            string(TOLOWER "${_env_root}" _env_root_lc)
            if(MSVC AND _env_root_lc MATCHES "mingw")
                set(_is_compatible FALSE)
            elseif(MINGW AND _env_root_lc MATCHES "msvc")
                set(_is_compatible FALSE)
            endif()

            if(NOT _is_compatible)
                continue()
            endif()

            set(Qt6_DIR "${_env_root}/lib/cmake/Qt6" CACHE PATH "Qt6Config.cmake dir" FORCE)
            break()
        endif()
    endforeach()

    # 2. Auto-scan C:/Qt/<version>/<arch> for any installed version/arch.
    if(NOT DEFINED Qt6_DIR)
        file(GLOB _qt_version_dirs "C:/Qt/6.*")
        foreach(_vdir IN LISTS _qt_version_dirs)
            if(IS_DIRECTORY "${_vdir}")
                foreach(_arch IN LISTS _qt_arch_suffixes)
                    set(_candidate "${_vdir}/${_arch}/lib/cmake/Qt6/Qt6Config.cmake")
                    if(EXISTS "${_candidate}")
                        set(Qt6_DIR "${_vdir}/${_arch}/lib/cmake/Qt6"
                            CACHE PATH "Qt6Config.cmake dir" FORCE)
                        break()
                    endif()
                endforeach()
            endif()
            if(DEFINED Qt6_DIR)
                break()
            endif()
        endforeach()
    endif()

    if(DEFINED Qt6_DIR)
        # Qt6_DIR is <prefix>/lib/cmake/Qt6 – go up 3 levels to <prefix>
        get_filename_component(_qt_prefix "${Qt6_DIR}/../../.." ABSOLUTE)
        list(PREPEND CMAKE_PREFIX_PATH "${_qt_prefix}")
        message(STATUS "Auto-detected Qt6 at: ${_qt_prefix}")
    endif()
endif()

find_package(Qt6 REQUIRED COMPONENTS
    Widgets
    Gui
    Core
    LinguistTools
)

add_subdirectory(src)
