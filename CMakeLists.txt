cmake_minimum_required(VERSION 3.24)

project(ArtificialCulture VERSION 0.2.0 LANGUAGES C CXX)

include(FetchContent)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(FLECS_STATIC ON CACHE BOOL "Build Flecs as a static library" FORCE)
set(FLECS_SHARED OFF CACHE BOOL "Do not build an extra shared Flecs library" FORCE)
set(FLECS_TESTS OFF CACHE BOOL "Do not build Flecs tests as part of this project" FORCE)

FetchContent_Declare(
    flecs
    GIT_REPOSITORY https://github.com/SanderMertens/flecs.git
    GIT_TAG v4.1.6
    GIT_SHALLOW TRUE
)
FetchContent_MakeAvailable(flecs)

add_library(artificial_culture_core STATIC
    src/cognition.cpp
)
add_library(ArtificialCulture::Core ALIAS artificial_culture_core)

target_include_directories(artificial_culture_core
    PUBLIC
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
        $<INSTALL_INTERFACE:include>
)
target_compile_features(artificial_culture_core PUBLIC cxx_std_20)
target_link_libraries(artificial_culture_core PUBLIC flecs_static)

if(MSVC)
    target_compile_options(artificial_culture_core PRIVATE /W4 /permissive-)
else()
    target_compile_options(artificial_culture_core PRIVATE -Wall -Wextra -Wpedantic)
endif()

add_executable(ac_demo src/main.cpp)
target_link_libraries(ac_demo PRIVATE ArtificialCulture::Core)

include(CTest)
if(BUILD_TESTING)
    add_executable(ac_adapter_contract_tests tests/adapter_contract_tests.cpp)
    target_link_libraries(ac_adapter_contract_tests PRIVATE ArtificialCulture::Core)
    add_test(NAME ac_adapter_contract COMMAND ac_adapter_contract_tests)

    add_executable(ac_subjective_event_memory_tests tests/subjective_event_memory_tests.cpp)
    target_link_libraries(ac_subjective_event_memory_tests PRIVATE ArtificialCulture::Core)
    add_test(NAME ac_subjective_event_memory COMMAND ac_subjective_event_memory_tests)

    add_executable(ac_testimony_provenance_tests tests/testimony_provenance_tests.cpp)
    target_link_libraries(ac_testimony_provenance_tests PRIVATE ArtificialCulture::Core)
    add_test(NAME ac_testimony_provenance COMMAND ac_testimony_provenance_tests)

    add_executable(ac_relationship_memory_tests tests/relationship_memory_tests.cpp)
    target_link_libraries(ac_relationship_memory_tests PRIVATE ArtificialCulture::Core)
    add_test(NAME ac_relationship_memory COMMAND ac_relationship_memory_tests)

    add_executable(ac_social_memory_action_tests tests/social_memory_action_tests.cpp)
    target_link_libraries(ac_social_memory_action_tests PRIVATE ArtificialCulture::Core)
    add_test(NAME ac_social_memory_action COMMAND ac_social_memory_action_tests)

    add_executable(ac_two_witnesses_scenario tests/two_witnesses_scenario.cpp)
    target_link_libraries(ac_two_witnesses_scenario PRIVATE ArtificialCulture::Core)
    add_test(NAME ac_two_witnesses_scenario COMMAND ac_two_witnesses_scenario)

    add_executable(ac_relationship_influenced_choice_tests tests/relationship_influenced_choice_tests.cpp)
    target_link_libraries(ac_relationship_influenced_choice_tests PRIVATE ArtificialCulture::Core)
    add_test(NAME ac_relationship_influenced_choice COMMAND ac_relationship_influenced_choice_tests)
endif()
