set(headers ${headers}
    include/PCH.h
    include/bridge/cosave.h
    include/bridge/hooks.h
    include/bridge/rimgui.h
    include/bridge/papyrus.h
    include/bridge/sinks.h
    include/log.h
    include/util.h
)
set(sources ${sources}
    ${headers}
    src/main.cpp
    src/bridge/cosave.cpp
    src/bridge/hooks.cpp
    src/bridge/papyrus.cpp
    src/bridge/rimgui.cpp
    src/bridge/sinks.cpp
    src/util.cpp
)
