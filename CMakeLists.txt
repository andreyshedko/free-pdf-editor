cmake_minimum_required(VERSION 3.20)

project(PDFEditor LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTORCC ON)

# Help CMake locate Qt6 on Windows without manual cache editing.
if(WIN32 AND NOT DEFINED Qt6_DIR)
    # 1. Honor explicit environment variables first.
    foreach(_env_root "$ENV{QTDIR}" "$ENV{QT_DIR}" "$ENV{Qt6_DIR}")
        if(_env_root AND EXISTS "${_env_root}/lib/cmake/Qt6/Qt6Config.cmake")
            set(Qt6_DIR "${_env_root}/lib/cmake/Qt6" CACHE PATH "Qt6Config.cmake dir" FORCE)
            break()
        endif()
    endforeach()

    # 2. Auto-scan C:/Qt/<version>/<arch> for any installed version/arch.
    if(NOT DEFINED Qt6_DIR)
        set(_qt_arch_suffixes
            mingw_64 mingw64
            msvc2022_64 msvc2022
            msvc2019_64 msvc2019
            clang_64
        )
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
