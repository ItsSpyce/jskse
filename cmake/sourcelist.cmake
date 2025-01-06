set(headers ${headers}
    include/PCH.h
    include/macrel.h
    include/log.h
    include/util.h
    include/graphics.h
    include/bridge/cosave.h
    include/bridge/hooks.h
    include/bridge/rimgui.h
    include/bridge/papyrus.h
    include/bridge/sinks.h
)
set(sources ${sources}
    ${headers}
    src/main.cpp
    src/util.cpp
    src/bridge/cosave.cpp
    src/bridge/hooks.cpp
    src/bridge/papyrus.cpp
    src/bridge/rimgui.cpp
    src/bridge/sinks.cpp
)
